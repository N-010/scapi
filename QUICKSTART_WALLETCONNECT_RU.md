# 🚀 Quick Start - WalletConnect for Qubic (Rust)

This file used to be written in Russian; it is now fully in English.

## What was fixed?

**Problem:** The wallet displayed “Connection was established via this URL” and refused to connect.  
**Fix:** ✅ The Qubic namespace is now provided via `required_namespaces` (and old state is cleaned up before reconnect).

## ⚡ Run

### 1. Simple run
```bash
cargo run
```

### 2. Follow the console instructions
You should see something like:

```
┌──────────────────────────────────────────────────────────┐
│  🔗 WalletConnect for Qubic - QR connection               │
└──────────────────────────────────────────────────────────┘

Step 1: Creating WalletConnect configuration...
✅ Configuration created

Step 2: Creating WalletConnect client...
✅ Client created

Step 3: Initializing client...
✅ Client initialized successfully

Step 4: Generating URI for QR code...
ℹ️  Old sessions/state are cleaned up automatically
ℹ️  Each run generates a NEW unique URI
✅ URI generated successfully

[QR code renders here]

How to connect:
1) Open your Qubic wallet on your phone
2) Find WalletConnect
3) Scan the QR code
4) Approve the connection in the wallet

Waiting for wallet connection... (timeout: 120s)
```

### 3. Scan the QR code
- Open the Qubic wallet on your phone
- Find WalletConnect
- Scan the QR code
- Approve the connection

### 4. Done ✅
After a successful connection you should see session info and can perform requests.

## 🔧 Use it in your own code

The core flow:
1. create config
2. create client
3. `init()`
4. `connect()` → show URI as QR
5. wait for the wallet to approve
6. call methods (`request_accounts`, `sign_message`, `send_transaction`, etc.)

See `WALLET_CONNECT_QUICKSTART.md` and `WALLET_CONNECT_API.md` for examples.

## ❓ FAQ

### Q: Do I need to manually delete old sessions in the wallet?
**A:** Usually no. The client cleans up old state automatically on each run.

### Q: What if the wallet still says “connection already established”?
**A:** Restart the program to get a fresh URI, and clear old sessions in the wallet if needed.

### Q: How long is a QR code valid?
**A:** The URI includes an expiry (often 5 minutes). The sample waits 120 seconds by default.

### Q: Can I reuse the same QR code multiple times?
**A:** No. Each QR code/URI is single-use.

### Q: Where do I get a Project ID?
**A:** Create a project at https://cloud.walletconnect.com/ and copy the Project ID.

### Q: Does it work with any wallet?
**A:** It works with wallets that support WalletConnect v2 and the Qubic namespace.

## 📚 Additional docs

- Root-cause write-up: `WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md`
- Changelog: `CHANGELOG_WALLETCONNECT.md`
- Full API reference: `WALLET_CONNECT_API.md`
- Examples: `examples/wallet_connect_*.rs`

## 🐛 Troubleshooting

### Error: “Project ID is required”
Set your Project ID via environment variable or in code:

```bash
set WALLETCONNECT_PROJECT_ID=your_project_id
```

### Error: “Connection timeout”
- check your internet connection
- ensure the wallet is open and unlocked
- increase timeout if needed (example): `wait_for_connection(300)`

### Error: “Failed to initialize WalletConnect Client”
- verify the Project ID
- check network connectivity
- retry

### QR code does not render in the console
- this can happen in some terminals/modes
- use the URI directly, or render a QR via an external tool

## ✨ What’s next?

After a successful connection you can:
- request accounts
- sign messages
- sign and send transactions
- subscribe to events

## 🎉 Finished

Last updated: 2025-11-01

