# WalletConnect Troubleshooting

## Error: “Connection already established”

### Causes
1. **Re-scanning the same QR code** — each URI is unique and can be used only once.
2. **Wallet cache** — the wallet may keep a history of connections.
3. **Program not restarted** — the previous URI is still in memory.

### Fix

#### 1. Restart the program to get a new URI
Each run creates a **new unique** URI with a unique:
- `symKey` (random 32 bytes)
- `topic` (SHA256 hash of `symKey`)
- `expiryTimestamp` (current time + 5 minutes)

#### 2. Clear the wallet cache
**In Qubic Wallet:**
1. Open settings
2. Find the “WalletConnect” / “Connections” section
3. Delete all old sessions
4. Or restart the wallet

#### 3. Verify the connection is unique
The program prints a unique connection ID:

🆔 Connection unique ID: `e10bf2f3a1c3b1bd656d3d0100b642ec...`

Make sure the ID is **different** every time.

## Error: “Invalid URL”

### Causes
1. **Wrong URI format** — must match WalletConnect v2.
2. **Missing required parameters**
3. **Wrong parameter order**

### Fix
The URI must look like:

`wc:<topic>@2?expiryTimestamp=<unix>&relay-protocol=irn&symKey=<hex>`

**Parameter order matters:**
1. `expiryTimestamp` — first
2. `relay-protocol` — second
3. `symKey` — third

**Wallet deep link:**
`qubic-wallet://wc?uri=<encoded_wc_uri>`

## Connection timeout

### Causes
1. **QR code wasn’t scanned** within 120 seconds
2. **Network issues** — no internet
3. **Wallet does not support WalletConnect v2**

### Fix
1. **Scan the QR faster** — you have 120 seconds
2. **Check internet** on both phone and computer
3. **Update the wallet** to the latest version
4. **Use a deep link** instead of a QR code:

`qubic-wallet://wc?uri=<encoded_wc_uri>`

## Project ID problems

### Symptoms
- Connection is never established
- Initialization errors

### Fix
1. **Get your Project ID** at https://cloud.walletconnect.com/
2. **Set an environment variable:**

```bash
set WALLETCONNECT_PROJECT_ID=your_project_id
```

3. **Verify the Project ID is active** in the WalletConnect Cloud dashboard

## Debugging

### Enable debug logging

```bash
set RUST_LOG=debug
cargo run
```

### Inspect generated values
In debug mode you should see:
- `symKey` — 64 hex chars (32 bytes)
- `topic` — 64 hex chars (SHA256 hash)

## Contacts and support
- WalletConnect docs: https://docs.walletconnect.com/
- Reown docs: https://docs.reown.com/

## Useful commands

```bash
# Run with debug logs
set RUST_LOG=debug
cargo run

# Release build (faster)
cargo build --release

# Clean and rebuild
cargo clean
cargo build

# Run examples
cargo run --example wallet_connect_basic
```
