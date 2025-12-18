# 🔗 WalletConnect API for Qubic (Rust)

A Rust implementation of a WalletConnect v2 client for connecting a dApp to Qubic wallets via QR code, with WASM bindings for browser usage.

## ✨ Highlights
- ✅ WalletConnect v2 protocol support
- ✅ Qubic namespace methods/events
- ✅ WASM support for browser integration
- ✅ strong, type-safe Rust API
- ✅ async/await-based client
- ✅ event system for session updates

## 📦 Installation

### Rust project (workspace/path)
```toml
[dependencies]
scapi = { path = "../scapi" }
```

### WASM for browsers
```bash
# Install wasm-pack
cargo install wasm-pack

# Build
wasm-pack build --target web
```

## 🚀 Quick start

### Rust (native)
1. Set `WALLETCONNECT_PROJECT_ID`
2. Run an example:
```bash
cargo run --example wallet_connect_basic
```

### JavaScript/TypeScript (WASM)
1. Build WASM: `wasm-pack build --target web`
2. Import `pkg/scapi.js`
3. Call `connect()` and render the returned URI as a QR code

## 📚 Documentation
- `WALLET_CONNECT_INDEX.md` - docs entry point
- `WALLET_CONNECT_QUICKSTART.md` - quick start guide
- `WALLET_CONNECT_API.md` - full API documentation
- `src/wallet_connect/README.md` - internal architecture
- `TROUBLESHOOTING.md` - common issues and fixes

## 🎯 Core capabilities

### 1) Connect to a wallet
Generate a WalletConnect URI and render it as a QR code for the wallet to scan.

### 2) Account management
Request accounts after the session is approved.

### 3) Transactions
Send Qubic or build/sign/send full transactions (depending on wallet support).

### 4) Signing
Sign transactions and messages via the wallet.

### 5) Events
Subscribe to session and wallet events via the event handler.

## 📁 Project structure

```
src/wallet_connect/
├── mod.rs
├── types.rs
├── client.rs
├── session.rs
├── events.rs
├── qubic_namespace.rs
└── wasm_bindings.rs
```

## 🧪 Examples
1. Basic connection: `examples/wallet_connect_basic.rs`
2. Transaction flow: `examples/wallet_connect_transaction.rs`
3. Events: `examples/wallet_connect_events.rs`

## 🔒 Security
- private keys stay inside the wallet
- do not embed secrets in client-side code
- use HTTPS for production browser deployments

## ⚙️ Requirements
- Rust toolchain (stable)
- WalletConnect Project ID (WalletConnect Cloud)
- `wasm-pack` (only for WASM builds)

## 🐛 Debugging
```bash
set RUST_LOG=debug
cargo run --example wallet_connect_basic
```

## 📝 License
Same as the main SCAPI project.

## 🤝 Contributing
Open an issue or a pull request. Keep changes focused and aligned with the existing module structure.

## ✅ Ready to use
Start with `WALLET_CONNECT_QUICKSTART.md` or the examples in `examples/`.

