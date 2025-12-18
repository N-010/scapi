# WalletConnect - Changelog

## [Fix] 2025-11-01 - “Connection was established via this URL”

### 🐛 Fixed issue
When scanning the QR code, the wallet rejected the connection with:
> “Connection was established via this URL”

### 🔍 Root cause
The Rust implementation used `optional_namespaces` for Qubic, while the JS SDK uses `requiredNamespaces`. This caused the wallet to reject the proposal.

### ✅ Changes

#### 1. Namespaces fix (`required_namespaces`)
**File:** `src/wallet_connect/client.rs`
- moved Qubic from optional → required

#### 2. Cleanup before reconnect
**File:** `src/wallet_connect/client.rs`
- added `cleanup_old_state()` to clear old listeners/session state before generating a new URI

#### 3. Improved waiting / logs
**File:** `src/wallet_connect/client.rs`
- improved errors and logs around waiting for approval / timeout

#### 4. Updated user-facing instructions
**File:** `src/main.rs`
- clarified that each run generates a fresh URI and stale state is cleaned up

### 📊 Comparison with the JavaScript version

| Aspect | JavaScript (qraw-frontend) | Rust (SCAPI) | Status |
|---|---|---|---|
| Old session cleanup | ✅ automatic | ✅ automatic | aligned |
| URI generation | `client.connect()` | `client.connect()` | aligned |
| Approval flow | `await approval()` | `wait_for_connection()` | equivalent |
| Timeout handling | ✅ | ✅ | aligned |

### 🧪 Testing
```bash
# Run the main program
cargo run

# Or run an example
cargo run --example wallet_connect_basic
```

### 📚 Related files
- `WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md` - root cause explanation
- `WALLETCONNECT_FIX_RU.md` - historical note (now English)

### 🔗 References
- JavaScript analog: `d:\Work\Qubic\qraw-frontend\src\api\wallet-connect-client.ts`

**Status:** ✅ fixed  
**Next:** test with a real Qubic wallet and a full relay implementation

