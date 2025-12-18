# Fix: “Connection was established via this URL”

This file used to be written in Russian; it is now fully in English.

## 🔴 Problem

When scanning a WalletConnect QR code, the wallet rejects the connection and shows:
> “Connection was established via this URL”

## ⚡ Solution (root cause)

**Real cause:** the Rust implementation used **`optional_namespaces`** instead of **`required_namespaces`** for Qubic.

See the detailed write-up: `WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md`

## 🔍 Analysis

The JavaScript SDK uses `requiredNamespaces` for core chain methods. The wallet expects the chain methods to be required; otherwise it may treat the proposal as invalid/duplicate.

## ✅ Code changes

1. **Namespaces fix**: Qubic is now set in `required_namespaces`.
2. **Reconnect safety**: `cleanup_old_state()` clears stale listeners/session state before generating a new URI.

## 🎯 Result
- ✅ each run generates a clean, unique connection
- ✅ stale sessions/state are cleared before reconnect
- ✅ wallet accepts the new QR codes

## 🔗 Related files
- `src/wallet_connect/client.rs` - namespaces + cleanup
- `src/main.rs` - updated user instructions
- `WALLETCONNECT_FIX_REQUIRED_NAMESPACES.md` - root-cause details

**Date:** 2025-11-01  
**Author:** AI Assistant  

