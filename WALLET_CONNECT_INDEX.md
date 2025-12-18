# 📖 WalletConnect for Qubic - Documentation Index

> A complete guide for using the Rust WalletConnect API for the Qubic blockchain.

## 🚀 Where to start

### Quick start
👉 `WALLET_CONNECT_QUICKSTART.md` - get your first connection in ~5 minutes

### API reference
👉 `WALLET_CONNECT_API.md` - full API documentation with examples

### Overview
👉 `README_WALLET_CONNECT.md` - project-level readme (features, installation, examples)

## 📚 Documentation set

### 1. Quick start
**File:** `WALLET_CONNECT_QUICKSTART.md`  
**Includes:**
- Project ID setup
- installation
- minimal Rust example
- browser/WASM notes
- debugging tips

### 2. Full API docs
**File:** `WALLET_CONNECT_API.md`  
**Includes:**
- installation
- Rust & WASM usage
- API reference
- examples
- TypeScript interop notes

### 3. Main README
**File:** `README_WALLET_CONNECT.md`  
**Includes:**
- project highlights
- setup
- structure
- security notes
- TypeScript comparison

### 4. Internal architecture
**File:** `src/wallet_connect/README.md`  
**Includes:**
- module structure
- key components
- architecture overview
- testing guidance
- contribution notes

### 5. Implementation summary
**File:** `WALLET_CONNECT_SUMMARY.md`  
**Includes:**
- work completed
- stats
- key characteristics
- next steps

### 6. Final report
**File:** `WALLET_CONNECT_REPORT.md`  
**Includes:**
- task description
- deliverables
- conclusions

## 💻 Code examples

1. `examples/wallet_connect_basic.rs` - basic connection flow
2. `examples/wallet_connect_transaction.rs` - building/sending transactions
3. `examples/wallet_connect_events.rs` - event handling

## 🎯 Quick navigation by goal

### Connect a wallet to my dApp
1. Read `WALLET_CONNECT_QUICKSTART.md`
2. Run `cargo run --example wallet_connect_basic`
3. Integrate the same flow into your app

### Use it from the browser (WASM)
1. Read the WASM section in `WALLET_CONNECT_API.md`
2. Build: `wasm-pack build --target web`
3. Use `WalletConnectClient` from JavaScript and render the URI as a QR

### Understand the architecture
1. Read `src/wallet_connect/README.md`
2. Inspect `src/wallet_connect/*`

### Contribute changes
1. Read `src/wallet_connect/README.md` → “Contributing”
2. Follow existing code style and module structure

## 📋 Getting started checklist
- [ ] WalletConnect Project ID obtained
- [ ] Rust installed (1.70+)
- [ ] Read `WALLET_CONNECT_QUICKSTART.md`
- [ ] Ran `cargo run --example wallet_connect_basic`
- [ ] Connected a wallet successfully

## 🔗 Useful links
- WalletConnect Cloud: https://cloud.walletconnect.com/
- WalletConnect docs: https://docs.walletconnect.com/
- Reown docs: https://docs.reown.com/
- wasm-pack: https://rustwasm.github.io/wasm-pack/
- cargo: https://doc.rust-lang.org/cargo/

**Project:** SCAPI - WalletConnect API for Qubic  
**Version:** 0.1.0  
**Updated:** 2025-11-01  

