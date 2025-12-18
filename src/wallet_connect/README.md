# WalletConnect Module for Qubic

This module provides a WalletConnect v2 implementation for the Qubic blockchain, including a native Rust client and WASM bindings for browser usage.

## 📁 Module layout

```
src/wallet_connect/
├── mod.rs              # Module exports
├── types.rs            # Types and error definitions
├── client.rs           # Main WalletConnect client
├── session.rs          # Session state and helpers
├── events.rs           # Event handler and event types
├── qubic_namespace.rs  # Qubic namespace methods/events
└── wasm_bindings.rs    # WASM/JS bindings
```

## 🔑 Key components

### WalletConnectClient
Main entry point for interacting with WalletConnect:
- create config
- initialize relay connectivity
- generate a URI (`connect()`)
- wait for approval
- execute requests over the Qubic namespace

### Types
Core data structures include:
- `WalletConnectConfig`
- `WalletAccount`
- `QubicTransactionParams`
- `SignatureResponse`
- `WalletConnectionStatus`

### Events
Event system to react to session state changes:
- `WalletConnectEvent` - event types
- `EventCallback` - callback trait
- `EventHandler` - event dispatcher/registry

### Qubic namespace
Qubic-specific WalletConnect methods:
- `qubic_requestAccounts`
- `qubic_sendQubic`
- `qubic_signTransaction`
- `qubic_sendTransaction`
- `qubic_sign`

## 🎯 Core flows

### 1. Connect to a wallet
Generate a URI and show it as a QR code to the user.

### 2. Request accounts
After approval, call `request_accounts()` to get wallet accounts.

### 3. Send transactions
Use `send_qubic()` for a simple transfer, or use transaction params for more control.

### 4. Event handling
Register callbacks on the event handler to observe proposals, session updates, and other events.

## 🌐 WASM support
The module supports WebAssembly builds and exposes a JS-friendly API via `wasm_bindings.rs`.

## 🔒 Security
- crypto operations remain in the wallet
- private keys are never transmitted by this client
- session management aims to avoid stale state and topic reuse

## 📊 Architecture notes
- clear separation of concerns between types/client/session/events
- event-driven design for async session lifecycle

## 🧪 Testing
Run examples:
```bash
# Basic connection
cargo run --example wallet_connect_basic

# Transaction flow
cargo run --example wallet_connect_transaction

# Event handling
cargo run --example wallet_connect_events
```

## 📖 Further reading
- `WALLET_CONNECT_INDEX.md`
- `WALLET_CONNECT_API.md`
- `WALLET_CONNECT_QUICKSTART.md`
- `TROUBLESHOOTING.md`

## 🤝 Contributing
When adding new functionality:
1. keep the module structure and naming consistent
2. add documentation and update examples
3. validate on both native and WASM targets where applicable

## 📝 Changelog
### v0.1.0 (current)
- ✅ basic WalletConnect v2 implementation
- ✅ Qubic namespace support
- ✅ WASM bindings
- ✅ event system
- ✅ examples and documentation

