/// Example: build, sign, and broadcast a Random Lottery ticket purchase using a seed
///
/// This sends the signed transaction through the Qubic live API.
use scapi::{
    build_ticket_tx_bytes_from_seed, openapi_models::live::BroadcastTransactionRequest,
    qubic_transactions::TransactionWithData, qubic_types::traits::FromBytes, PayloadBuilder,
    QubicClient, QubicId,
};
use std::io::{self, Write};

const RL_CONTRACT_INDEX: u32 = 16;
const RL_BUY_TICKET_PROC: u16 = 1;
const DEFAULT_TICKET_PRICE_QUS: u64 = 1_000_000;
const DEFAULT_TICK_OFFSET: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Random Lottery - buy 1 ticket with seed");
    println!("========================================\n");

    let seed = read_line("Enter seed (55 lowercase letters): ")?;
    let amount = read_u64_with_default(
        &format!(
            "Ticket price (QUS) [default {}]: ",
            DEFAULT_TICKET_PRICE_QUS
        ),
        DEFAULT_TICKET_PRICE_QUS,
    )?;

    let scheduled_offset = read_u32_with_default(
        &format!("Scheduled tick offset [default {}]: ", DEFAULT_TICK_OFFSET),
        DEFAULT_TICK_OFFSET,
    )?;
    let client = QubicClient::new();
    let current_tick = client
        .live()
        .get_tick_info()
        .await?
        .tick_info
        .and_then(|info| info.tick)
        .ok_or("tick info missing")?;
    let tick = current_tick.saturating_add(scheduled_offset);

    let payload = PayloadBuilder::new().to_bytes();
    let tx_bytes = build_ticket_tx_bytes_from_seed(
        &seed,
        QubicId::from_contract_id(RL_CONTRACT_INDEX),
        amount,
        tick,
        RL_BUY_TICKET_PROC,
        payload,
    )?;
    let tx = TransactionWithData::from_bytes(&tx_bytes)?;
    let tx_hash: [u8; 32] = tx.clone().into();

    println!("\nSigned transaction created");
    println!("From:   {}", tx.raw_transaction.from.get_identity());
    println!(
        "To:     {} (contract {})",
        QubicId::from_contract_id(RL_CONTRACT_INDEX).get_identity(),
        RL_CONTRACT_INDEX
    );
    println!("Amount: {} QUS", amount);
    println!("Tick:   {} (current: {})", tick, current_tick);
    println!("Proc:   {}", RL_BUY_TICKET_PROC);

    println!("\nTx hash (hex): {}", hex::encode(tx_hash));

    print!("\nBroadcast transaction now? (y/n): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    if confirm.trim().to_lowercase() != "y" {
        println!("Cancelled");
        return Ok(());
    }

    match client
        .live()
        .broadcast_transaction(&BroadcastTransactionRequest {
            encoded_transaction: Some(base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &tx_bytes,
            )),
        })
        .await
    {
        Ok(response) => {
            println!("Broadcasted peers: {:?}", response.peers_broadcasted);
            println!("Transaction id: {:?}", response.transaction_id);
            println!("Encoded tx: {:?}", response.encoded_transaction);
        }
        Err(err) => {
            println!("Broadcast failed: {}", err);
        }
    }

    Ok(())
}

fn read_line(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_u64_with_default(prompt: &str, default: u64) -> Result<u64, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(default);
    }
    Ok(trimmed.parse()?)
}

fn read_u32_with_default(prompt: &str, default: u32) -> Result<u32, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(default);
    }
    Ok(trimmed.parse()?)
}
