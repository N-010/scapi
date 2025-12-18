# SCAPI Architecture

Detailed architecture documentation for SCAPI project.

## 📐 Project Structure

```
SCAPI/
├── src/
│   ├── lib.rs          # Library entry point, exports modules
│   ├── main.rs         # CLI application entry point
│   ├── sc_api.rs       # Core API implementation
│   └── wasm.rs         # WASM bindings and JavaScript interop
├── pkg/                # WASM build output (after wasm-pack build)
│   ├── scapi.js        # JavaScript/TypeScript wrapper
│   ├── scapi_bg.wasm   # Compiled WebAssembly binary
│   ├── scapi.d.ts      # TypeScript type definitions
│   └── package.json    # NPM package metadata
├── Cargo.toml          # Rust project configuration
├── README.md           # Main documentation
├── EXAMPLES.md         # Usage examples
├── QUICKSTART.md       # Quick start guide
├── CONTRIBUTING.md     # Contribution guidelines
├── CHANGELOG.md        # Version history
└── LICENSE             # MIT License
```

## 🏗️ Core Components

### 1. RequestDataBuilder (`sc_api.rs`)

**Purpose:** Fluent API for building smart contract queries

**Responsibilities:**
- Managing binary buffer for input data
- Encoding various data types (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64)
- Supporting arrays and special types (m256i for IDs)
- Converting to base64 for HTTP requests
- Sending requests to RPC endpoint

**Key Methods:**
```rust
pub struct RequestDataBuilder {
    contract_index: u32,
    input_type: u32,
    buffer: Vec<u8>,
    little_endian: bool,
}

impl RequestDataBuilder {
    pub fn new() -> Self
    pub fn set_contract_index(mut self, index: u32) -> Self
    pub fn set_input_type(mut self, input_type: u32) -> Self
    pub fn add_uint64(mut self, value: u64) -> Self
    pub fn to_bytes(&self) -> Vec<u8>
    pub async fn send(&self) -> Result<Vec<u8>>
}
```

**Data Flow:**
```
Input → add_* methods → buffer → to_bytes() → send() → RPC → Response bytes
```

### 2. ResponseDecoder (`sc_api.rs`)

**Purpose:** Declarative decoding of binary contract responses

**Responsibilities:**
- Reading binary data with correct endianness
- Parsing various data types
- Validating data availability
- Creating structured JSON representation
- Supporting arrays and nested structures

**Key Methods:**
```rust
pub struct ResponseDecoder<'a> {
    reader: BinaryReader<'a>,
    obj: serde_json::Map<String, serde_json::Value>,
}

impl<'a> ResponseDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self
    pub fn u8(mut self, field: &str) -> Result<Self>
    pub fn u64(mut self, field: &str) -> Result<Self>
    pub fn array_m256i(mut self, field: &str, count: usize) -> Result<Self>
    pub fn array_struct_bytes(mut self, field: &str, count: usize, struct_size: usize) -> Result<Self>
    pub fn to_value(self) -> serde_json::Value
}
```

**Data Flow:**
```
Response bytes → BinaryReader → read_* methods → JSON Map → to_value() → JavaScript object
```

### 3. BinaryReader (`sc_api.rs`)

**Purpose:** Low-level binary data reading

**Responsibilities:**
- Managing read position in buffer
- Handling Little Endian / Big Endian
- Validating data bounds
- Primitive read operations

**Key Methods:**
```rust
pub struct BinaryReader<'a> {
    data: &'a [u8],
    position: usize,
    pub endianness: Endianness,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self
    pub fn read_u8(&mut self) -> Result<u8>
    pub fn read_u64(&mut self) -> Result<u64>
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]>
    fn ensure_available(&self, len: usize) -> Result<()>
}
```

### 4. WASM Bindings (`wasm.rs`)

**Purpose:** JavaScript/TypeScript interface for WASM module

**Responsibilities:**
- Exporting Rust functions to JavaScript
- Converting between Rust and JS types
- Handling asynchronous operations (Promises)
- Managing memory between WASM and JS

**Key Components:**
```rust
#[wasm_bindgen]
pub struct RequestDataBuilderHandle {
    inner: RequestDataBuilder,
}

#[wasm_bindgen]
pub struct ResponseDecoderHandle {
    data: Vec<u8>,
    decoder: ResponseDecoder<'static>,
}

#[wasm_bindgen]
impl RequestDataBuilderHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    
    pub fn set_contract_index(mut self, index: u32) -> Self
    
    pub async fn send(&self) -> Result<Vec<u8>, JsValue>
}
```

**JavaScript Integration:**
```javascript
// JavaScript side
const builder = new RequestDataBuilder()
    .set_contract_index(16)
    .send(); // Returns Promise<Uint8Array>
```

## 🔄 Request/Response Flow

### Complete Flow Diagram

```
┌─────────────────┐
│  JavaScript     │
│  Application    │
└────────┬────────┘
         │
         │ 1. new RequestDataBuilder()
         │    .set_contract_index(16)
         │    .set_input_type(1)
         │    .send()
         ▼
┌─────────────────────────┐
│  WASM Binding Layer     │
│  (wasm.rs)              │
│                         │
│  - RequestDataBuilder   │
│    Handle               │
└────────┬────────────────┘
         │
         │ 2. Convert JS params to Rust
         ▼
┌─────────────────────────┐
│  Core API Layer         │
│  (sc_api.rs)            │
│                         │
│  - RequestDataBuilder   │
│  - BinaryWriter         │
└────────┬────────────────┘
         │
         │ 3. Build binary payload
         │    - Contract index
         │    - Input type
         │    - Encoded parameters
         ▼
┌─────────────────────────┐
│  HTTP Client            │
│  (reqwest)              │
└────────┬────────────────┘
         │
         │ 4. POST to https://rpc.qubic.org
         │    Body: base64 encoded payload
         ▼
┌─────────────────────────┐
│  Qubic RPC Endpoint     │
└────────┬────────────────┘
         │
         │ 5. Execute view function
         ▼
┌─────────────────────────┐
│  Smart Contract         │
└────────┬────────────────┘
         │
         │ 6. Return response bytes
         ▼
┌─────────────────────────┐
│  HTTP Client            │
└────────┬────────────────┘
         │
         │ 7. Decode base64 response
         ▼
┌─────────────────────────┐
│  Core API Layer         │
│                         │
│  - ResponseDecoder      │
│  - BinaryReader         │
└────────┬────────────────┘
         │
         │ 8. Parse binary response
         │    - Read types sequentially
         │    - Build JSON object
         ▼
┌─────────────────────────┐
│  WASM Binding Layer     │
│                         │
│  - Convert to JsValue   │
└────────┬────────────────┘
         │
         │ 9. Return JavaScript object
         ▼
┌─────────────────┐
│  JavaScript     │
│  Application    │
└─────────────────┘
```

## 🔒 Memory Management

### Rust Side
- Stack-allocated for simple types
- Heap-allocated `Vec<u8>` for buffers
- Borrowing (`&[u8]`) for zero-copy operations
- RAII ensures cleanup

### WASM Side
- Linear memory shared between JS and WASM
- `wasm-bindgen` handles marshalling
- JavaScript owns returned data
- Rust side cleaned up when handles drop

### Best Practices
```rust
// ✅ Good: zero-copy with borrowing
pub fn new(data: &'a [u8]) -> Self

// ✅ Good: move ownership to JavaScript
pub fn to_value(self) -> serde_json::Value

// ❌ Bad: unnecessary copy
pub fn to_value(&self) -> serde_json::Value
```

## 🎯 Design Patterns

### 1. Builder Pattern (RequestDataBuilder)

```rust
let request = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(1)
    .add_uint64(1000);
```

**Benefits:**
- Fluent, readable API
- Compile-time validation
- Flexible parameter order

### 2. Method Chaining (ResponseDecoder)

```rust
let result = ResponseDecoder::new(&bytes)
    .u8("field1")
    .u64("field2")
    .to_value();
```

**Benefits:**
- Declarative data structure
- Order matches binary layout
- Easy to read and maintain

### 3. Handle Pattern (WASM)

```rust
pub struct ResponseDecoderHandle {
    data: Vec<u8>,
    decoder: ResponseDecoder<'static>,
}
```

**Benefits:**
- Safely manages lifetimes across WASM boundary
- Ensures data outlives decoder
- JavaScript-friendly interface

## 🧪 Error Handling

### Rust (anyhow)
```rust
pub fn read_u64(&mut self) -> Result<u64> {
    self.ensure_available(8)?;
    // ...
}
```

### WASM (JsValue)
```rust
pub fn u64(self, field: &str) -> Result<Self, JsValue> {
    ResponseDecoderHandle::map_result(data, decoder.u64(field))
}
```

### JavaScript (Promises)
```javascript
try {
    const result = await builder.send();
} catch (error) {
    console.error('Failed:', error);
}
```

## 📊 Data Encoding

### Request Format

```
┌──────────────────────────┐
│ Contract Index (u32)     │ 4 bytes
├──────────────────────────┤
│ Input Type (u32)         │ 4 bytes
├──────────────────────────┤
│ Parameter 1              │ variable
├──────────────────────────┤
│ Parameter 2              │ variable
├──────────────────────────┤
│ ...                      │
└──────────────────────────┘
```

### Response Format (Contract-specific)

Example: GetFees
```
┌──────────────────────────┐
│ teamFeePercent (u8)      │ 1 byte
├──────────────────────────┤
│ distributionFeePercent   │ 1 byte
├──────────────────────────┤
│ winnerFeePercent (u8)    │ 1 byte
├──────────────────────────┤
│ burnPercent (u8)         │ 1 byte
└──────────────────────────┘
```

### Endianness
- **Default:** Little Endian (Qubic standard)
- **Configurable:** via `with_endianness()`

```rust
// Little Endian: 0x1234 → [0x34, 0x12]
// Big Endian:    0x1234 → [0x12, 0x34]
```

## 🚀 Performance Considerations

### Zero-Copy Operations
- `ResponseDecoder` borrows input data
- No unnecessary allocations
- Efficient for large responses

### Minimal WASM Size
```toml
[profile.release]
lto = "thin"  # Link-time optimization
```

### Async I/O
- Non-blocking HTTP requests
- JavaScript Promise integration
- Tokio runtime for Rust native

## 🔮 Future Enhancements

### Planned Features
- [ ] Custom RPC endpoint configuration
- [ ] Transaction execution support
- [ ] Event subscription
- [ ] Contract ABI parsing
- [ ] Automatic type generation

### Optimization Opportunities
- [ ] Response caching
- [ ] Request batching
- [ ] Compression support
- [ ] WebSocket support for subscriptions

---

For more details, see:
- [API Reference](README.md#-api-reference)
- [Examples](EXAMPLES.md)
- [Contributing](CONTRIBUTING.md)

---

Made with ❤️ for Qubic ecosystem
