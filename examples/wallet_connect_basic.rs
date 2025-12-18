/// Basic example of using the WalletConnect API for Qubic
///
/// This example demonstrates:
/// - Creating and initializing the client
/// - Generating a connection URI (render as QR)
/// - Basic wallet operations
use scapi::wallet_connect::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging (optional)
    tracing_subscriber::fmt::init();

    println!("🚀 WalletConnect for Qubic - Basic example");
    println!("===========================================\n");

    // Step 1: Create configuration
    println!("📝 Step 1: Creating configuration...");
    let config = WalletConnectConfig::new(
        "your_walletconnect_project_id".to_string(), // Replace with your Project ID
        "qubic:mainnet".to_string(),
    );
    println!("✅ Configuration created\n");

    // Step 2: Create client
    println!("🔧 Step 2: Creating client...");
    let mut client = WalletConnectClient::new(config);
    println!("✅ Client created\n");

    // Step 3: Initialize
    println!("🔌 Step 3: Initializing client...");
    client.init().await?;
    println!("✅ Client initialized\n");

    // Step 4: Generate connection URI (for QR)
    println!("📱 Step 4: Generating connection URI...");
    let uri = client.connect().await?;
    println!("✅ URI generated:");
    println!("   {}\n", uri);

    println!("📲 Scan this QR code in your Qubic wallet:");
    println!("   (In a real app you would render the QR here)\n");

    // In a real app you would wait for the wallet connection here
    println!("⏳ Waiting for wallet connection...");
    println!("   (This happens after the QR is scanned)\n");

    // Note: The next steps require a real wallet connection
    println!("ℹ️  Further operations (accounts, transactions) require");
    println!("   a real Qubic wallet connection via WalletConnect.");

    Ok(())
}
