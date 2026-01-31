/// Example: build, sign, and broadcast a QDraw ticket purchase using a seed
///
/// Contract index: 15
/// Input type: 1
/// Payload: struct buyTicket_input { uint64 ticketCount; }
use scapi::{
    build_ticket_tx_bytes_from_seed, rpc::get::get_tick_info,
    rpc::post::broadcast_transaction_bytes, PayloadBuilder, QubicId,
};
use std::io::{self, Write};

const QDRAW_CONTRACT_INDEX: u32 = 15;
const QDRAW_BUY_TICKET_INPUT_TYPE: u16 = 1;
const DEFAULT_TICKET_PRICE_QUS: u64 = 1_000_000;
const DEFAULT_TICK_OFFSET: u32 = 5;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct BuyTicketInput {
    ticket_count: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("QDraw - buy tickets with seed");
    println!("=============================\n");

    let seed = read_line("Enter seed (55 lowercase letters): ")?;
    let ticket_count = read_u64_with_default("Ticket count [default 1]: ", 1)?;
    let amount = read_u64_with_default(
        &format!("Amount (QUS) [default {}]: ", DEFAULT_TICKET_PRICE_QUS),
        DEFAULT_TICKET_PRICE_QUS,
    )?;

    let scheduled_offset = read_u32_with_default(
        &format!("Scheduled tick offset [default {}]: ", DEFAULT_TICK_OFFSET),
        DEFAULT_TICK_OFFSET,
    )?;
    let current_tick = get_tick_info().await?.tick_info.tick;
    let tick = current_tick.saturating_add(scheduled_offset);

    let input = BuyTicketInput { ticket_count };
    let payload = PayloadBuilder::new()
        .add_uint64(input.ticket_count)
        .to_bytes();

    let tx_bytes = build_ticket_tx_bytes_from_seed(
        &seed,
        QubicId::from_contract_id(QDRAW_CONTRACT_INDEX),
        amount,
        tick,
        QDRAW_BUY_TICKET_INPUT_TYPE,
        payload,
    )?;

    print!("\nBroadcast transaction now? (y/n): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    if confirm.trim().to_lowercase() != "y" {
        println!("Cancelled");
        return Ok(());
    }

    let response = broadcast_transaction_bytes(&tx_bytes).await?;
    println!("Broadcasted peers: {}", response.peers_broadcasted);
    println!("Transaction id: {}", response.transaction_id);
    println!("Encoded tx: {}", response.encoded_transaction);

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
