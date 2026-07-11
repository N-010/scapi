# SCAPI

SCAPI is a Rust library for Qubic applications. Its primary interface is an instance-based, typed `QubicClient` generated from the Live, Query, Archive, and Stats OpenAPI specifications. The crate also provides an independent Bob JSON-RPC client, helpers for encoding and decoding smart-contract data, transaction support, and WebAssembly bindings.

## Features

- One typed `QubicClient` with separate `live()`, `query()`, `archive()`, and `stats()` service accessors
- 117 generated request and response models and 50 typed OpenAPI operations
- Per-client endpoint configuration through `QubicClientConfig`
- Independent `BobClient` for Bob JSON-RPC nodes
- Smart-contract payload construction with `RequestDataBuilder`
- Binary response parsing with `ResponseDecoder`
- Transaction construction, signing, and broadcasting
- Browser-ready WASM bindings for the client and contract helpers

## Project structure

- `src/client.rs` and `src/client/openapi_api.rs` — service-oriented HTTP client and typed operations
- `src/openapi_models.rs` — generated models grouped under `live`, `query`, `archive`, and `stats`
- `src/bob/` — independent Bob JSON-RPC client
- `src/sc_api.rs` — contract request and response helpers
- `src/transaction.rs` and `src/qubic_transactions.rs` — transaction primitives and builders
- `src/wasm.rs` — WebAssembly bindings
- `examples/` — runnable Rust examples

## Installation

Add the crate from a local checkout:

```toml
[dependencies]
scapi = { path = "../SCAPI" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Rust quickstart

Each accessor uses its own default service URL and returns typed generated models:

```rust
use anyhow::Result;
use scapi::QubicClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = QubicClient::new();

    let tick = client.live().get_tick_info().await?;
    let processed = client.query().get_last_processed_tick().await?;
    let archived = client.archive().get_latest_tick().await?;
    let stats = client.stats().get_latest_data().await?;

    println!("{tick:?}\n{processed:?}\n{archived:?}\n{stats:?}");
    Ok(())
}
```

Endpoint overrides belong to a client instance; they do not change global state:

```rust
use scapi::{QubicClient, QubicClientConfig};

let config = QubicClientConfig::default()
    .live_url("https://example.net/live/v1")
    .query_url("https://example.net/query/v1")
    .archive_url("https://example.net")
    .stats_url("https://example.net");

let client = QubicClient::with_config(config);
```

Generated models are available below `scapi::openapi_models::{live, query, archive, stats}`. The typed methods are defined on the corresponding service accessor, so request parameters and response shapes are checked by Rust.

## Smart-contract queries

Use a generated request model when working directly with the Live API:

```rust
use anyhow::Result;
use scapi::{openapi_models::live::QuerySmartContractRequest, QubicClient};

#[tokio::main]
async fn main() -> Result<()> {
    let request = QuerySmartContractRequest {
        contract_index: Some(16),
        input_type: Some(6),
        input_size: Some(0),
        request_data: Some(String::new()),
    };

    let response = QubicClient::new()
        .live()
        .query_smart_contract(&request)
        .await?;
    println!("{response:?}");
    Ok(())
}
```

For binary contract layouts, `RequestDataBuilder` appends typed values with configurable endianness and can send the completed payload. `ResponseDecoder` reads named primitive fields, arrays, byte ranges, and 256-bit values from the returned bytes:

```rust
use anyhow::Result;
use scapi::{RequestDataBuilder, ResponseDecoder};

#[tokio::main]
async fn main() -> Result<()> {
    let bytes = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(6)
        .send()
        .await?;

    let value = ResponseDecoder::new(&bytes)
        .u8("currentState")?
        .to_value();
    println!("{value}");
    Ok(())
}
```

## Broadcasting a transaction

The Live OpenAPI service accepts a base64-encoded signed transaction through its typed request model:

```rust
use anyhow::Result;
use scapi::{openapi_models::live::BroadcastTransactionRequest, QubicClient};

#[tokio::main]
async fn main() -> Result<()> {
    let signed_transaction_base64 = "...".to_owned();
    let request = BroadcastTransactionRequest {
        encoded_transaction: Some(signed_transaction_base64),
    };

    let response = QubicClient::new()
        .live()
        .broadcast_transaction(&request)
        .await?;
    println!("transaction: {:?}", response.transaction_id);
    Ok(())
}
```

## Bob JSON-RPC

Bob is a separate protocol and is not one of the four OpenAPI services. Connect to it explicitly with `BobClient`; `with_base_url` appends the `/qubic` RPC path.

```rust
use anyhow::Result;
use scapi::bob::BobClient;

#[tokio::main]
async fn main() -> Result<()> {
    let bob = BobClient::with_base_url("http://localhost:40420");
    let status = bob.qubic_status().await?;
    let result = bob.qubic_broadcast_transaction("0x...signed_tx_hex...").await?;
    println!("status: {status}\nbroadcast: {result}");
    Ok(())
}
```

`BobClient::call` is also available for methods that do not yet have a convenience wrapper.

## WebAssembly

Install the Rust target and build the package with `wasm-pack`:

```text
rustup target add wasm32-unknown-unknown
wasm-pack build --target web
```

The WASM API exposes all OpenAPI operations through the `live`, `query`, `archive`, and `stats` service objects, as well as `RequestDataBuilder`, `ResponseDecoder`, and asynchronous smart-contract helpers:

```javascript
import init, { QubicClientConfig, QubicClient } from "./pkg/scapi.js";

await init();

const config = new QubicClientConfig(
  "https://rpc.qubic.org/live/v1",
  undefined,
  undefined,
  undefined,
);
const client = new QubicClient(config);
const tickInfo = await client.live.getTickInfo();
const balance = await client.live.getBalance("IDENTITY");
const archivedTick = await client.archive.getTickData(123456);
const latestStats = await client.stats.getLatestData();
const contractResult = await client.live.querySmartContract({
  contractIndex: 1,
  inputType: 2,
  inputSize: 0,
  requestData: "",
});
```

Method names are the camel-case form of the Rust OpenAPI methods. Path parameters are positional strings or numbers. Request bodies and grouped query parameters are plain JSON-compatible JavaScript objects; when both are present, arguments follow the Rust order: path, body, query. Every operation returns a `Promise` resolving to a JSON-compatible object and rejects with a textual error for HTTP, deserialization, or serialization failures.

## Building and examples

```text
cargo build
cargo test
cargo run --example qubic_client_live
cargo run --example qubic_client_query_smart_contract
cargo run --example bob_qubic_status
```

See the [`examples`](examples/) directory for OpenAPI, Bob, contract, transaction, and WalletConnect programs.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md), keep generated interfaces and examples consistent, and run the Rust checks before opening a change.

## License

Licensed under the [MIT License](LICENSE).
