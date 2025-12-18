# WalletConnect API for Qubic

Rust library to connect a dApp to a Qubic wallet using WalletConnect v2, including QR-code flow and WASM support.

## 📋 Contents
- [Installation](#installation)
- [Quick start (Rust)](#quick-start-rust)
- [Browser usage (WASM)](#browser-usage-wasm)
- [API overview](#api-overview)
- [Data structures](#data-structures)
- [Examples](#examples)
- [TypeScript interoperability](#typescript-interoperability)
- [Requirements](#requirements)
- [Important notes](#important-notes)
- [License](#license)
- [Contributing](#contributing)

## 🚀 Installation

Add to your `Cargo.toml` (workspace/path dependency example):

```toml
[dependencies]
scapi = { path = "../scapi" }
```

## ⚡ Quick start (Rust)

```rust
use scapi::wallet_connect::{WalletConnectClient, WalletConnectConfig, ClientMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WalletConnectConfig {
        project_id: std::env::var("WALLETCONNECT_PROJECT_ID")?,
        qubic_chain_id: "qubic:mainnet".to_string(),
        metadata: ClientMetadata {
            name: "My Qubic dApp".to_string(),
            description: "WalletConnect integration".to_string(),
            url: "https://example.com".to_string(),
            icons: vec![],
        },
        relay_url: None,
    };

    let mut client = WalletConnectClient::new(config)?;
    client.init().await?;

    let uri = client.connect().await?;
    println!("WalletConnect URI (render as QR): {uri}");

    let approved = client.wait_for_connection(120).await?;
    if !approved {
        println!("Connection rejected by wallet");
        return Ok(());
    }

    let accounts = client.request_accounts().await?;
    println!("Accounts: {accounts:?}");
    Ok(())
}
```

## 🌐 Browser usage (WASM)

### Build WASM

```bash
# Install wasm-pack
cargo install wasm-pack

# Build
wasm-pack build --target web
```

### Use from JavaScript
1. Import `pkg/scapi.js`
2. Initialize WASM
3. Create the client and call `connect()`
4. Render the returned URI as a QR code using any QR library

## API overview

### WalletConnectClient

- `new(config)` → create a client
- `init()` → connect/init internal state
- `connect()` → generate WalletConnect URI (for QR rendering)
- `wait_for_connection(timeout_secs)` → wait for wallet approval
- `approve()` → internal approval handling (advanced usage)
- `request_accounts()` → fetch accounts from wallet
- `send_qubic(from, to, amount)` → send Qubic
- `sign_transaction(params)` → sign a transaction
- `send_transaction(params)` → sign and send a transaction
- `sign_message(from, message)` → sign an arbitrary message
- `disconnect()` → disconnect and clear session
- `is_session_active()` → session status
- `get_session()` → session info
- `get_connection_url()` → current URI if available
- `event_handler()` → subscribe to/emit events

## Data structures

### `WalletConnectConfig`
- `project_id: String` - WalletConnect Project ID
- `qubic_chain_id: String` - `qubic:mainnet` or `qubic:testnet`
- `metadata: ClientMetadata` - app metadata
- `relay_url: Option<String>` - optional relay URL override

### `WalletAccount`
- `address: String`
- `amount: Option<u64>`
- `additional_data: HashMap<String, serde_json::Value>`

### `QubicTransactionParams`
- `from: String`
- `to: String`
- `amount: u64`
- `tick: Option<u64>`
- `input_type: Option<u16>`
- `payload: Option<String>`

## Examples

### Example 1: Basic connection
See: `examples/wallet_connect_basic.rs`

### Example 2: Send a transaction
See: `examples/wallet_connect_transaction.rs`

### Example 3: Event handling
See: `examples/wallet_connect_events.rs`

## TypeScript interoperability

If you migrate from the TypeScript implementation, the Rust client aims to keep:
- similar method names
- similar request/response payload shapes
- the same WalletConnect v2 + Qubic namespace approach

## Requirements
- Rust toolchain (stable)
- WalletConnect Project ID (WalletConnect Cloud)
- `wasm-pack` (only for WASM builds)

## Important notes
1. **Project ID**: get it from https://cloud.walletconnect.com/
2. **Chain ID**: use `qubic:mainnet` or `qubic:testnet`
3. **Security**: never store private keys in client-side code
4. **CORS**: for browser integration ensure your environment allows required requests

## License
As part of SCAPI, the project uses the same license as the main repository.

## Contributing
Contributions are welcome. Please open an issue or a pull request.

