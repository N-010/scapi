# SCAPI - Smart Contract API for Qubic

## Cargo features

Native WalletConnect support and `scapi-cli` require the `wallet-connect`
feature, which is enabled by default. Applications using only Qubic signing,
transactions, RPC, or Bob can exclude WalletConnect dependencies:

```toml
scapi = { git = "https://github.com/N-010/scapi", default-features = false }
```

The default build retains the existing WalletConnect API and CLI behavior.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-Ready-blue.svg)](https://webassembly.org/)

> 🦀 Rust library for interacting with Qubic smart contracts via HTTP RPC API and WASM

## 📚 Documentation

> 📖 **[Complete Documentation Index](DOCS_INDEX.md)** - Navigate all documents

- **[Quick Start Guide](QUICKSTART.md)** - Get started in 5 minutes
- **[Examples](EXAMPLES.md)** - Practical usage examples
- **[Architecture](ARCHITECTURE.md)** - Architecture and internals
- **[Testing Guide](TESTING.md)** - Testing guidelines
- **[Contributing](CONTRIBUTING.md)** - How to contribute
- **[Changelog](CHANGELOG.md)** - Version history

## 📋 Description

SCAPI (Smart Contract API) is a Rust library providing a convenient interface for:
- 🔍 Executing view queries to Qubic smart contracts
- 📦 Encoding input parameters for contract function calls
- 🔓 Decoding contract responses using a fluent API
- 🌐 Browser usage via WebAssembly (WASM)
- 💻 Command-line interface (CLI)

## ✨ Features

### RequestDataBuilder
Fluent API for building contract queries:
```rust
let response = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(1)
    .add_uint64(1000)
    .send()
    .await?;
```

### ResponseDecoder
Declarative contract response decoding:
```rust
let result = ResponseDecoder::new(&response_bytes)
    .u8("teamFeePercent")
    .u8("distributionFeePercent")
    .u64("playerCounter")
    .array_m256i("players", 1024)
    .to_value();
```

### WASM Support
Full JavaScript/TypeScript integration:
```javascript
import init, { RequestDataBuilder, ResponseDecoder } from './scapi.js';

await init();

const response = await new RequestDataBuilder()
    .set_contract_index(16)
    .set_input_type(2)
    .send();

const decoded = new ResponseDecoder(response)
    .u64("balance")
    .to_value();

console.log(decoded.balance);
```

## 🚀 Installation

### As Rust Library

```toml
[dependencies]
scapi = { git = "https://github.com/your-repo/scapi" }
```

### Build WASM Module

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM module
wasm-pack build --target web --out-dir pkg

# Files will be in pkg/ directory
```

## 📖 Usage

### CLI (Command Line)

```bash
# Run CLI
cargo run --bin scapi-cli

# Or after installation
scapi-cli
```

### Rust Library

```rust
use scapi::{RequestDataBuilder, ResponseDecoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build and send request
    let response = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(2)
        .send()
        .await?;
    
    // Decode response
    let result = ResponseDecoder::new(&response)
        .u8("returnCode")
        .u64("balance")
        .to_value();
    
    println!("{}", result);
    Ok(())
}
```

### WASM in JavaScript

```javascript
// Initialize module
import init, { RequestDataBuilder, ResponseDecoder } from './pkg/scapi.js';

await init('./pkg/scapi_bg.wasm');

// Query contract
const responseBytes = await new RequestDataBuilder()
    .set_contract_index(16)
    .set_input_type(1) // GetFees
    .send();

// Decode response
const fees = new ResponseDecoder(responseBytes)
    .u8("teamFeePercent")
    .u8("distributionFeePercent")
    .u8("winnerFeePercent")
    .u8("burnPercent")
    .to_value();

console.log('Fees:', fees);
```

## 🔧 API Reference

### RequestDataBuilder

Methods for building requests:

| Method | Description |
|-------|----------|
| `new()` | Create new builder |
| `set_contract_index(u32)` | Set contract index |
| `set_input_type(u32)` | Set input type (function) |
| `with_endianness(bool)` | Set byte order (true = Little Endian) |
| `add_bool(bool)` | Add boolean value |
| `add_uint8(u8)` | Add unsigned 8-bit number |
| `add_uint16(u16)` | Add unsigned 16-bit number |
| `add_uint32(u32)` | Add unsigned 32-bit number |
| `add_uint64(u64)` | Add unsigned 64-bit number |
| `add_int8(i8)` | Add signed 8-bit number |
| `add_int16(i16)` | Add signed 16-bit number |
| `add_int32(i32)` | Add signed 32-bit number |
| `add_int64(i64)` | Add signed 64-bit number |
| `add_float(f32)` | Add 32-bit float |
| `add_double(f64)` | Add 64-bit float |
| `add_bytes(Vec<u8>)` | Add byte array |
| `add_m256i_bytes([u8; 32])` | Add 256-bit value (ID) |
| `to_bytes()` | Get encoded bytes |
| `to_base64()` | Get base64 string |
| `send()` | Send request and get response |

### ResponseDecoder

Methods for decoding responses:

| Method | Description |
|-------|----------|
| `new(&[u8])` | Create new decoder |
| `with_endianness(Endianness)` | Set byte order |
| `u8(field)` | Read unsigned 8-bit number |
| `u16(field)` | Read unsigned 16-bit number |
| `u32(field)` | Read unsigned 32-bit number |
| `u64(field)` | Read unsigned 64-bit number |
| `i8(field)` | Read signed 8-bit number |
| `i16(field)` | Read signed 16-bit number |
| `i32(field)` | Read signed 32-bit number |
| `i64(field)` | Read signed 64-bit number |
| `f32(field)` | Read 32-bit float |
| `f64(field)` | Read 64-bit float |
| `bytes(field, len)` | Read byte array |
| `m256i(field)` | Read 256-bit value (ID) |
| `array_u8(field, count)` | Read u8 array |
| `array_u16(field, count)` | Read u16 array |
| `array_u32(field, count)` | Read u32 array |
| `array_u64(field, count)` | Read u64 array |
| `array_m256i(field, count)` | Read ID array (256-bit) |
| `array_struct_bytes(field, count, size)` | Read array of structs as bytes |
| `remaining_bytes(field)` | Read all remaining bytes |
| `to_value()` | Convert to JSON Value |
| `to_json_string()` | Convert to JSON string |

## 📝 Examples

### Example 1: Get Contract Balance

```rust
let response = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(10) // GetBalance
    .send()
    .await?;

let balance = ResponseDecoder::new(&response)
    .u64("balance")
    .to_value();

println!("Contract balance: {} QU", balance["balance"]);
```

### Example 2: Get Player List

```rust
let response = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(2) // GetPlayers
    .send()
    .await?;

let result = ResponseDecoder::new(&response)
    .array_m256i("players", 1024)
    .u64("playerCounter")
    .to_value();

let player_count = result["playerCounter"].as_u64().unwrap();
println!("Total players: {}", player_count);
```

### Example 3: Decode Array of Structs

```rust
let response = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(3) // GetWinners
    .send()
    .await?;

let result = ResponseDecoder::new(&response)
    .array_struct_bytes("winners", 1024, 48) // 48 bytes per struct
    .u64("winnersCounter")
    .to_value();

// Decode each winner separately
for winner_bytes in result["winners"].as_array().unwrap() {
    let winner = ResponseDecoder::new(winner_bytes.as_bytes())
        .bytes("winnerAddress", 32)
        .u64("revenue")
        .u16("epoch")
        .u32("tick")
        .to_value();
    
    println!("Winner: {:?}", winner);
}
```

### Example 4: JavaScript Integration

```javascript
// Initialize
import init, { RequestDataBuilder, ResponseDecoder } from './scapi.js';
await init();

// Get fees
async function getFees() {
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(1)
        .send();
    
    const fees = new ResponseDecoder(response)
        .u8("teamFeePercent")
        .u8("distributionFeePercent")
        .u8("winnerFeePercent")
        .u8("burnPercent")
        .to_value();
    
    return fees;
}

// Usage
const fees = await getFees();
console.log('Team fee:', fees.teamFeePercent, '%');
console.log('Winner fee:', fees.winnerFeePercent, '%');
```

## 🔨 Development

### Requirements

- Rust 1.70+
- wasm-pack (for WASM builds)
- Node.js 16+ (for WASM testing)

### Build Project

```bash
# Regular build
cargo build

# Release build
cargo build --release

# Build WASM
wasm-pack build --target web --out-dir pkg

# Run tests
cargo test
```

### Project Structure

```
SCAPI/
├── src/
│   ├── lib.rs          # Main module
│   ├── main.rs         # CLI application
│   ├── sc_api.rs       # Core API logic
│   └── wasm.rs         # WASM bindings
├── pkg/                # WASM output (after build)
│   ├── scapi.js
│   ├── scapi_bg.wasm
│   └── scapi.d.ts
├── Cargo.toml          # Project configuration
└── README.md           # Documentation
```

## 🌐 RPC Configuration

By default, SCAPI uses `https://rpc.qubic.org` as the RPC endpoint.

To change the endpoint (in future versions):
```rust
// TODO: add support for custom endpoint
```

## 🐛 Known Limitations

1. **Endianness**: Little Endian by default (Qubic standard)
2. **WASM size**: ~200KB after optimization
3. **Browser compatibility**: Modern browsers with WASM support

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first.

### TODO
- [ ] Add support for custom RPC endpoints
- [ ] Add more examples
- [ ] Improve error handling
- [ ] Add transaction support (not just view calls)
- [ ] Optimize WASM bundle size

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Links

- [Qubic Official Website](https://qubic.org)
- [Qubic RPC Documentation](https://docs.qubic.org)
- [WebAssembly](https://webassembly.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## 📧 Contact

For questions and suggestions, open an issue in the repository.

---

Made with ❤️ for Qubic ecosystem
