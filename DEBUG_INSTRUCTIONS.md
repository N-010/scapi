# 🔍 WalletConnect Debug Instructions

## ✅ What was added

Detailed logging was added to diagnose the wallet-side error “Connection was established via this URL”.

### 📊 New logs
You should now see logs for:
1. **Connection creation** (proposal, topic, symKey, URI)
2. **Wallet responses** (incoming messages)
3. **Session creation** (session details)
4. **Errors** (error code + message)

## 🚀 How to test

### 1. Run with debug logs
```bash
set RUST_LOG=debug
cargo run
```

Or simply:
```bash
cargo run
```

### 2. Scan the QR code
- Open the Qubic wallet
- Find WalletConnect
- Scan the QR code

### 3. Watch the logs
You should see a sequence like:
- “SessionPropose sent successfully”
- “Received message: ...”
- “Session established”

### 4. What to look for

#### ✅ Successful connection
- proposal is sent
- wallet responds with approval
- session becomes active

#### ❌ Connection error
If the wallet rejects, the error code/message should be logged.

#### ⏰ Timeout
If no response arrives before the timeout, verify relay connectivity and wallet behavior.

## 📋 Diagnostics checklist

Check these items:
- [ ] Was the proposal sent?
- [ ] Did the wallet respond?
- [ ] Which response type (approval vs error)?
- [ ] Are the correct namespaces used (Qubic in `required_namespaces`)?

## 🔧 What to do if the error repeats

1. Copy the full error info from logs (code + message)
2. Verify the URI topic changes on each run
3. Compare with the JavaScript version flow (proposal payload, namespaces)
4. Verify relay connectivity

## 📝 Sharing logs for analysis

If the issue persists, provide:
1. Full log output (or saved log file)
2. The generated URI (redact secrets if needed)
3. Time from “URI created” to rejection/timeout
4. Wallet version (if known)

## 🎯 Expected result

With these logs you should be able to see:
- exactly what is sent to the wallet
- what the wallet replies with
- where the flow fails

Next step: run `cargo run`, scan the QR, and review the debug logs.

