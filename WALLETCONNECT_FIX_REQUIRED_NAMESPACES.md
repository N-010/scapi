# ✅ Root cause fix: `requiredNamespaces` vs `optionalNamespaces`

## 🔴 Problem

The wallet rejected the connection with:
> “Connection was established via this URL”

### Root cause

The Rust implementation used **`optional_namespaces`** while the JavaScript SDK uses **`requiredNamespaces`** for Qubic.

## 🔍 Comparison

### Rust (before)
```rust
required_namespaces: HashMap::new(), // empty
optional_namespaces,                // Qubic was here (wrong)
```

### Rust (after)
```rust
required_namespaces,                 // Qubic is required (correct)
optional_namespaces: HashMap::new(), // empty
```

## ⚡ What changed

**File:** `src/wallet_connect/client.rs`
- moved Qubic namespace from optional → required
- kept optional empty (or used only for truly optional features)

## 🎯 Why this matters

### `requiredNamespaces`
- mandatory requirements for the session
- the wallet must support these chains/methods
- used for core blockchain functionality

### `optionalNamespaces`
- optional capabilities the wallet may support
- the session can still be established without them
- used for non-critical extras

## 📊 Result

| Parameter | Before | After |
|---|---|---|
| `required_namespaces` | `{}` | `{ qubic: {...} }` |
| `optional_namespaces` | `{ qubic: {...} }` | `{}` |
| Wallet behavior | ❌ rejects | ✅ accepts |
| Parity with JS SDK | ❌ no | ✅ yes |

## 🚀 Verification

```bash
set RUST_LOG=debug
cargo run
```

Confirm logs indicate Qubic is placed in `required_namespaces`, then scan the QR code and approve the session.

## 📝 Additional improvements

Along with the root fix:
1. `cleanup_old_state()` clears stale state before reconnects
2. better logging clarifies which namespaces are used
3. documentation and quick start were updated

## ⚠️ Important note

If you build your own WalletConnect client:
- ✅ use `requiredNamespaces` for core chain methods
- ❌ do not put critical methods into `optionalNamespaces`

## 🔗 Related files
- `src/wallet_connect/client.rs` - root fix
- `WALLETCONNECT_FIX_RU.md` - note (now English)
- `QUICKSTART_WALLETCONNECT_RU.md` - quick start (now English)

**Date:** 2025-11-01  
**Status:** ✅ fixed  

