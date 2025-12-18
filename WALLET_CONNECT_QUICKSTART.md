# WalletConnect for Qubic - Quick Start

## 🚀 5-minute integration

### Step 1: Get a Project ID
1. Go to https://cloud.walletconnect.com/
2. Create a new project
3. Copy the **Project ID**

### Step 2: Install
Add this crate via a path dependency (example):

```toml
[dependencies]
scapi = { path = "../scapi" }
```

### Step 3: Minimal code

```rust
use scapi::wallet_connect::{WalletConnectClient, WalletConnectConfig, ClientMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WalletConnectConfig {
        project_id: std::env::var(\"WALLETCONNECT_PROJECT_ID\")?,
        qubic_chain_id: \"qubic:mainnet\".to_string(),
        metadata: ClientMetadata {
            name: \"My Qubic dApp\".to_string(),
            description: \"WalletConnect demo\".to_string(),
            url: \"https://example.com\".to_string(),
            icons: vec![],
        },
        relay_url: None,
    };

    let mut client = WalletConnectClient::new(config)?;
    client.init().await?;

    let uri = client.connect().await?;
    println!(\"Scan this URI as a QR: {uri}\");

    let ok = client.wait_for_connection(120).await?;
    if !ok {
        println!(\"Connection rejected\");
        return Ok(());
    }

    let accounts = client.request_accounts().await?;
    println!(\"Accounts: {accounts:?}\");

    Ok(())
}
```

### Step 4: Run
```bash
cargo run
```

## 🌐 Browser usage (WASM)

### Step 1: Build WASM
```bash
wasm-pack build --target web
```

### Step 2: HTML
Use `pkg/scapi.js` from your app and render the generated URI as a QR using any JS QR library.

## 📱 Supported wallets
- Any wallet that supports WalletConnect v2 and the Qubic namespace

## 🔐 Security

✅ Recommended:
- use HTTPS in production
- keep the Project ID in environment variables / configuration
- verify transaction signatures (server-side where appropriate)

❌ Avoid:
- storing private keys in app code
- sending transactions without explicit user approval

## 🐛 Debugging

### Enable logs
```bash
set RUST_LOG=debug
cargo run
```

### Connection checks
- verify that `connect()` produces a new URI each run
- verify Qubic is in `required_namespaces`

## 📚 Resources
- Full API reference: `WALLET_CONNECT_API.md`
- Examples: `examples/`
- Troubleshooting: `TROUBLESHOOTING.md`

## 💬 Support
If something does not work:
1. Check `TROUBLESHOOTING.md`
2. Check examples in `examples/`
3. Open an issue with logs and the generated URI (redact secrets)

## 🎉 Done
Your app can now connect to Qubic wallets via WalletConnect.

