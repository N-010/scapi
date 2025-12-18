#![cfg(not(target_arch = "wasm32"))]

mod sc_api;
mod wallet_connect;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("scapi-cli binary is not supported on wasm32 targets");
}

#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;
use wallet_connect::*;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with DEBUG level for detailed diagnostics
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🔗 WalletConnect for Qubic - QR connection               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Step 1: Configuration
    println!("📝 Step 1: Creating WalletConnect configuration...");

    let project_id = std::env::var("WALLET_CONNECT_PROJECT_ID")
        .unwrap_or_else(|_| {
            println!("⚠️  WALLET_CONNECT_PROJECT_ID is not set");
            println!("   Use: set WALLET_CONNECT_PROJECT_ID=your_project_id");
            println!("   Or get a Project ID at: https://cloud.walletconnect.com/\n");
            "demo_project_id".to_string()
        });

    let config = WalletConnectConfig::new(project_id.clone(), "qubic:mainnet".to_string());

    println!("✅ Configuration created");
    println!(
        "   Project ID: {}",
        if project_id == "demo_project_id" {
            "demo_project_id (replace with a real one!)"
        } else {
            &project_id
        }
    );
    println!("   Chain ID: qubic:mainnet\n");

    // Step 2: Create client
    println!("🔧 Step 2: Creating WalletConnect client...");
    let mut client = WalletConnectClient::new(config);
    println!("✅ Client created\n");

    // Step 3: Initialize
    println!("🔌 Step 3: Initializing client...");
    match client.init().await {
        Ok(_) => println!("✅ Client initialized successfully\n"),
        Err(e) => {
            println!("❌ Initialization error: {}\n", e);
            return Ok(());
        }
    }

    // Step 4: Generate URI for QR
    println!("📱 Step 4: Generating URI for QR...");
    println!("   ℹ️  Old sessions/state are cleaned up automatically");
    println!("   ℹ️  Each run generates a NEW unique URI");
    println!("   ℹ️  This helps avoid 'connection already established' errors\n");

    let uri = match client.connect().await {
        Ok(uri) => {
            println!("✅ URI generated successfully\n");
            println!(
                "   🆔 Unique connection ID: {}\n",
                uri.split('@').next().unwrap_or("").replace("wc:", "")
            );
            uri
        }
        Err(e) => {
            println!("❌ Failed to generate URI: {}\n", e);
            return Ok(());
        }
    };

    // Render QR code in the console (debug builds)
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    QR CODE TO SCAN                         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Try to render a QR code in the console
    #[cfg(debug_assertions)]
    {
        match qr2term::print_qr(&uri) {
            Ok(_) => println!("\n✅ QR code rendered above ⬆️\n"),
            Err(_) => {
                println!("⚠️  Failed to render QR code in the console\n");
                println!("📋 URI (use it to generate a QR code externally):");
                println!("   {}\n", uri);
            }
        }
    }

    #[cfg(not(debug_assertions))]
    {
        println!("📋 WalletConnect URI:");
        println!("   {}\n", uri);
    }

    // User instructions
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                   HOW TO CONNECT                           ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!("\n1️⃣  Open your Qubic wallet on your phone");
    println!("2️⃣  Find WalletConnect");
    println!("3️⃣  Scan the QR code above");
    println!("4️⃣  Approve the connection in the wallet\n");

    // Extra info
    println!("💡 Deep link:");
    println!("   qubic-wallet://pairwc/{}\n", uri);
    println!("💡 Online QR generator:");
    println!("   https://www.qr-code-generator.com/\n");

    // Wait for wallet connection with a small animation
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║              ⏳ WAITING FOR WALLET CONNECTION              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("   Scan the QR code in your Qubic wallet");
    println!("   Timeout: 120 seconds (2 minutes)\n");

    // Waiting animation in a separate task
    let animation_handle = tokio::spawn(async {
        let mut dots = 0;
        let start = tokio::time::Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs();
            let remaining = 120u64.saturating_sub(elapsed);

            if remaining == 0 {
                break;
            }

            dots = (dots + 1) % 4;
            let dots_str = ".".repeat(dots);
            let spaces_str = " ".repeat(3 - dots);

            print!(
                "\r   ⏳ Waiting{}{} (remaining: {}s)    ",
                dots_str, spaces_str, remaining
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    // Wait for connection with a 120-second timeout
    let connection_result = client.wait_for_connection(120).await;

    // Stop animation
    animation_handle.abort();
    println!("\r                                                              \r");

    // Handle connection result
    match connection_result {
        Ok(true) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ✅ WALLET CONNECTED SUCCESSFULLY!             ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");

            if let Some(session) = client.get_session() {
                println!("📊 Session info:");
                println!("   Topic: {}", session.topic);
                if let Some(ttl) = session.time_until_expiry() {
                    println!("   Expires in: {} hours", ttl / 3600);
                }
                println!("   Relay protocol: {}", session.relay_protocol);
                println!("   Status: Active ✅\n");
            }

            println!("💡 Next, you can use the client methods manually:");
            println!("   • client.request_accounts()");
            println!("   • client.sign_message(...)");
            println!("   • client.send_transaction(...)");
            println!("   • client.disconnect()\n");
        }
        Ok(false) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ❌ CONNECTION REJECTED                        ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");
            println!("   The connection was rejected in the wallet\n");
        }
        Err(e) => {
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║              ⏰ CONNECTION TIMEOUT                          ║");
            println!("╚════════════════════════════════════════════════════════════╝\n");
            println!("   Error: {}\n", e);
            println!("   Possible reasons:");
            println!("   • QR code was not scanned");
            println!("   • Connection was rejected in the wallet");
            println!("   • Timeout expired (120s)\n");

            println!("💡 Try:");
            println!("   • Run again: cargo run");
            println!("   • Check Project ID in the environment variable");
            println!("   • Ensure the wallet supports WalletConnect\n");
        }
    }

    // Next steps
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                  NEXT STEPS                                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("📚 Read the docs:");
    println!("   - WALLET_CONNECT_QUICKSTART.md - quick start");
    println!("   - WALLET_CONNECT_API.md - full API docs");
    println!("   - examples/ - ready-to-run examples\n");

    println!("🧪 Run examples:");
    println!("   cargo run --example wallet_connect_basic");
    println!("   cargo run --example wallet_connect_transaction");
    println!("   cargo run --example wallet_connect_events\n");

    println!("🌐 For browser usage:");
    println!("   wasm-pack build --target web\n");

    println!("✨ Done! SCAPI WalletConnect API is ready to use!");

    Ok(())
}
