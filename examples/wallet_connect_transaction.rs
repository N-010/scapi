/// Example of sending a transaction via WalletConnect
///
/// This example demonstrates:
/// - Connecting to a wallet
/// - Requesting accounts
/// - Creating and sending a transaction
use scapi::wallet_connect::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("💸 WalletConnect for Qubic - Send transaction");
    println!("================================================\n");

    // Client setup
    let metadata = ClientMetadata {
        name: "Qubic Transaction Example".to_string(),
        description: "Example: send a transaction via WalletConnect".to_string(),
        url: "https://qubic.org".to_string(),
        icons: vec!["https://qx.qubic.org/assets/icons/favicon.ico".to_string()],
    };

    let config =
        WalletConnectConfig::new("your_project_id".to_string(), "qubic:mainnet".to_string())
            .with_metadata(metadata);

    let mut client = WalletConnectClient::new(config);

    // Initialize
    println!("🔧 Initializing client...");
    client.init().await?;
    println!("✅ Client is ready\n");

    // Generate QR/URI
    println!("📱 Generating QR URI...");
    let uri = client.connect().await?;
    println!("✅ Scan this QR code:");
    println!("   {}\n", uri);

    println!("⏳ Waiting for connection...");
    // In a real app, you would wait for the session approval event here.

    // Simulated check: in real usage, the session would become active after approval.
    if !client.is_session_active() {
        println!("⚠️  A wallet connection is required to send a transaction");
        return Ok(());
    }

    // Request accounts
    println!("\n📋 Requesting accounts...");
    match client.request_accounts().await {
        Ok(accounts) => {
            println!("✅ Accounts found: {}", accounts.len());
            for (i, account) in accounts.iter().enumerate() {
                println!("   {}. {}", i + 1, account.address);
                if let Some(balance) = account.amount {
                    println!("      Balance: {} Qubic", balance);
                }
            }

            if accounts.is_empty() {
                println!("⚠️  No accounts available");
                return Ok(());
            }

            // Build transaction
            println!("\n💰 Building transaction...");

            let from = accounts[0].address.clone();
            let to = get_recipient_address()?;
            let amount = get_amount()?;

            let tx_params = QubicTransactionParams {
                from: from.clone(),
                to: to.clone(),
                amount,
                tick: None,
                input_type: Some(0),
                payload: None,
            };

            println!("\n📄 Transaction parameters:");
            println!("   From:   {}", from);
            println!("   To:     {}", to);
            println!("   Amount: {} Qubic", amount);

            // Confirm
            print!("\n❓ Send transaction? (y/n): ");
            io::stdout().flush()?;
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;

            if confirm.trim().to_lowercase() != "y" {
                println!("❌ Transaction cancelled");
                return Ok(());
            }

            // Send transaction
            println!("\n🚀 Sending transaction...");
            println!("   (Approve the transaction in your wallet)");

            match client.send_transaction(tx_params).await {
                Ok(result) => {
                    println!("✅ Transaction sent successfully!");
                    println!("   Result: {:?}", result);
                }
                Err(e) => {
                    println!("❌ Failed to send transaction: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to request accounts: {}", e);
        }
    }

    // Disconnect
    println!("\n🔌 Disconnecting from wallet...");
    client.disconnect().await?;
    println!("✅ Disconnected");

    Ok(())
}

fn get_recipient_address() -> Result<String, Box<dyn std::error::Error>> {
    print!("\n📬 Enter recipient address: ");
    io::stdout().flush()?;
    let mut address = String::new();
    io::stdin().read_line(&mut address)?;
    Ok(address.trim().to_string())
}

fn get_amount() -> Result<u64, Box<dyn std::error::Error>> {
    print!("💵 Enter amount (in smallest units): ");
    io::stdout().flush()?;
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    Ok(amount.trim().parse()?)
}
