use scapi::wallet_connect::events::*;
/// WalletConnect event handling example
///
/// This example demonstrates:
/// - Registering event handlers
/// - Reacting to different WalletConnect events
/// - Managing the session lifecycle
use scapi::wallet_connect::*;

// Custom event handler
struct MyEventHandler {
    name: String,
}

impl EventCallback for MyEventHandler {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        println!("\n🔔 [{}] Event received:", self.name);
        println!("   Type: {:?}", event);

        match event {
            WalletConnectEvent::SessionProposal => {
                println!("   📋 Session proposal");
                println!("   Payload: {}", payload);
            }
            WalletConnectEvent::SessionRequest => {
                println!("   📨 Session request");
                println!("   Payload: {}", payload);
            }
            WalletConnectEvent::SessionDelete => {
                println!("   🗑️  Session deleted");
                println!("   Reason: {}", payload);
            }
            WalletConnectEvent::SessionExpire => {
                println!("   ⏰ Session expired");
                println!("   Details: {}", payload);
            }
            WalletConnectEvent::ProposalExpire => {
                println!("   ⌛ Proposal expired");
                println!("   Details: {}", payload);
            }
            WalletConnectEvent::SessionEvent => {
                println!("   🎉 Session event");
                println!("   Payload: {}", payload);
            }
            WalletConnectEvent::SessionUpdate => {
                println!("   🔄 Session update");
                println!("   New data: {}", payload);
            }
            WalletConnectEvent::SessionExtend => {
                println!("   ⏭️  Session extended");
                println!("   Details: {}", payload);
            }
            WalletConnectEvent::SessionPing => {
                println!("   🏓 Session ping");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🎯 WalletConnect for Qubic - Event handling");
    println!("==============================================\n");

    // Create configuration
    let config =
        WalletConnectConfig::new("your_project_id".to_string(), "qubic:mainnet".to_string());

    let mut client = WalletConnectClient::new(config);

    // Register event handlers
    println!("📝 Registering event handlers...");

    // Logging handler (built-in)
    client.event_handler().register(Box::new(LoggingCallback));

    // Custom handler
    client.event_handler().register(Box::new(MyEventHandler {
        name: "CustomHandler".to_string(),
    }));

    println!("✅ Handlers registered\n");

    // Initialize
    println!("🔧 Initializing client...");
    client.init().await?;
    println!("✅ Client initialized\n");

    // Generate URI
    println!("📱 Generating connection URI...");
    let uri = client.connect().await?;
    println!("✅ URI: {}\n", uri);

    println!("📲 Scan the QR code in your wallet to connect");
    println!("   Events will be printed here...\n");

    // Simulate waiting for events
    println!("⏳ Waiting for events (press Ctrl+C to exit)...\n");

    // In a real application there would be an event loop here.
    // For the demo we just wait a bit.
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Simulate some events for demonstration
    println!("\n🧪 Simulating events for demo:");

    // Proposal event
    client.event_handler().emit(
        WalletConnectEvent::SessionProposal,
        serde_json::json!({
            "id": "test-proposal-123",
            "params": {}
        }),
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Session update event
    client.event_handler().emit(
        WalletConnectEvent::SessionUpdate,
        serde_json::json!({
            "topic": "test-topic",
            "params": { "accounts": [] }
        }),
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Disconnect
    println!("\n🔌 Disconnecting...");
    client.disconnect().await?;
    println!("✅ Disconnected\n");

    println!("ℹ️  Example finished. In a real app, events will");
    println!("   arrive from the connected wallet automatically.");

    Ok(())
}
