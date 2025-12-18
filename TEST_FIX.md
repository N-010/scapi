# 🧪 Testing the WalletConnect Fix

## ✅ What was fixed

**Critical fix:** switched from `optional_namespaces` to `required_namespaces`.  
This is the real reason behind the wallet-side rejection/error.

## 🚀 Run the test

### 1. Run the program
```bash
cargo run
```

### 2. Expected console output
You should see:

```
🔗 WalletConnect for Qubic - QR connection
Step 1: Creating configuration...
Step 2: Creating client...
Step 3: Initializing...
Step 4: Generating URI...
ℹ️  Old sessions/state are cleaned up automatically
ℹ️  Each run generates a NEW unique URI
✅ URI generated successfully
```

### 3. Expected debug logs
If you run with `RUST_LOG=debug`, you should see that Qubic is placed in `required_namespaces`.

```bash
set RUST_LOG=debug
cargo run
```

### 4. Scan the QR code
- Open the Qubic wallet on your phone
- Find WalletConnect
- Scan the QR code
- Approve the connection

### 5. Expected result
✅ **Success** — the wallet connects without the previous rejection.

## 🔍 What changed in the code

### Before
```rust
required_namespaces: HashMap::new(), // empty ❌
optional_namespaces,                // Qubic was here
```

### After
```rust
required_namespaces,                 // Qubic is here ✅
optional_namespaces: HashMap::new(), // empty
```

## 📝 Testing checklist
- [ ] `cargo build` succeeds
- [ ] `cargo run` starts
- [ ] QR code/URI is displayed
- [ ] Wallet scans the QR successfully
- [ ] Wallet approves the connection
- [ ] Session becomes active
- [ ] Restarting the program keeps working (fresh URI each run)

## 🐛 If the problem remains

If you still see a rejection:
1. Verify the wallet supports WalletConnect v2 + Qubic namespace
2. Verify the Project ID is valid
3. Clear old WalletConnect sessions in the wallet
4. Inspect logs and confirm `required_namespaces` usage
5. Compare the proposal payload with the JavaScript implementation

## 📚 Documentation
- `WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md` - root cause explanation
- `WALLET_CONNECT_QUICKSTART.md` - quick start
- `TROUBLESHOOTING.md` - common issues

Next step: run `cargo run` and verify a successful wallet connection.

