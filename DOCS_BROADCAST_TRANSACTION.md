# Broadcast Transaction (RPC)

This document explains how to broadcast a signed transaction using the Qubic HTTP RPC endpoint and SCAPI helpers.

## Endpoint

`POST /v1/broadcast-transaction`

Default base URL in SCAPI:
- `https://rpc.qubic.org/live/v1/`
- Can be overridden with `QUBIC_RPC_BASE_URL`

Full URL example:
```
https://rpc.qubic.org/live/v1/broadcast-transaction
```

## Request body

```json
{
  "encodedTransaction": "<BASE64_RAW_TX>"
}
```

Where:
- `encodedTransaction` is Base64 of the **raw binary transaction bytes** (not hex).

## Response body

```json
{
  "peersBroadcasted": 3,
  "encodedTransaction": "<BASE64_RAW_TX>",
  "transactionId": "<tx_id>"
}
```

### Fields
- `peersBroadcasted` (int32): number of peers the node broadcasted the transaction to
- `encodedTransaction` (string): base64 transaction echoed by server
- `transactionId` (string): transaction hash / id

## SCAPI usage (Rust)

### Build `tx_bytes`

```rust
use scapi::{QubicId, QubicWallet, build_ticket_tx_bytes};

let wallet = QubicWallet::from_seed("...")?;
let to = QubicId::from_contract_id(16);
let tx_bytes = build_ticket_tx_bytes(&wallet, to, 1_000_000, 12345, 1, Vec::new())?;
```

Or, if you only have the seed:

```rust
use scapi::{QubicId, build_ticket_tx_bytes_from_seed};

let to = QubicId::from_contract_id(16);
let tx_bytes = build_ticket_tx_bytes_from_seed("...", to, 1_000_000, 12345, 1, Vec::new())?;
```

### Broadcast

```rust
use scapi::rpc::post::broadcast_transaction_bytes;

let response = broadcast_transaction_bytes(&tx_bytes).await?;
println!("Peers: {}", response.peers_broadcasted);
println!("Tx ID: {}", response.transaction_id);
```

## Notes

- Make sure `tick` is set to a **future** tick (commonly `current_tick + 5`).
- For contract procedures, set:
  - `to` = contract id (`QubicId::from_contract_id(...)`)
  - `input_type` = procedure index
  - `payload` = procedure input bytes
- You can fetch current tick via:
  - `GET /v1/tick-info` or `scapi::rpc::get::get_tick_info()`
