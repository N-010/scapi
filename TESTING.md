# Testing Guide for SCAPI

Comprehensive testing guide for SCAPI project.

## 📋 Table of Contents

- [Running Tests](#running-tests)
- [Test Structure](#test-structure)
- [Writing Tests](#writing-tests)
- [WASM Testing](#wasm-testing)
- [Integration Testing](#integration-testing)
- [Manual Testing](#manual-testing)

## 🧪 Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_request_builder

# Run tests with specific name pattern
cargo test decoder

# Run doc tests only
cargo test --doc

# Run integration tests only
cargo test --test '*'
```

### Testing with Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage
```

### Linting and Formatting

```bash
# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy (linter)
cargo clippy -- -D warnings

# Fix clippy suggestions automatically
cargo clippy --fix
```

## 🏗️ Test Structure

```
SCAPI/
├── src/
│   ├── sc_api.rs
│   │   └── #[cfg(test)] mod tests { ... }
│   └── wasm.rs
│       └── #[cfg(test)] mod tests { ... }
└── tests/
    └── integration_tests.rs (future)
```

## ✍️ Writing Tests

### Unit Tests

#### Testing RequestDataBuilder

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder_basic() {
        let builder = RequestDataBuilder::new()
            .set_contract_index(16)
            .set_input_type(1);
        
        assert_eq!(builder.contract_index, 16);
        assert_eq!(builder.input_type, 1);
    }

    #[test]
    fn test_add_uint64() {
        let builder = RequestDataBuilder::new()
            .add_uint64(1000);
        
        let bytes = builder.to_bytes();
        // Verify correct encoding
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_add_multiple_values() {
        let builder = RequestDataBuilder::new()
            .add_uint8(10)
            .add_uint16(1000)
            .add_uint32(100000);
        
        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 1 + 2 + 4); // 7 bytes
    }

    #[test]
    fn test_endianness() {
        let le = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint16(0x1234)
            .to_bytes();
        
        // Little Endian: [0x34, 0x12]
        assert_eq!(le[0], 0x34);
        assert_eq!(le[1], 0x12);
        
        let be = RequestDataBuilder::new()
            .with_endianness(Endianness::Big)
            .add_uint16(0x1234)
            .to_bytes();
        
        // Big Endian: [0x12, 0x34]
        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
    }
}
```

#### Testing ResponseDecoder

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_u8() {
        let data = vec![0x42];
        let result = ResponseDecoder::new(&data)
            .u8("value")
            .unwrap()
            .to_value();
        
        assert_eq!(result["value"], 0x42);
    }

    #[test]
    fn test_decode_u64() {
        // Little Endian: 1000 = 0x03E8
        let data = vec![0xE8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = ResponseDecoder::new(&data)
            .u64("balance")
            .unwrap()
            .to_value();
        
        assert_eq!(result["balance"], 1000);
    }

    #[test]
    fn test_decode_multiple_fields() {
        let data = vec![
            0x0A,       // u8: 10
            0x00, 0x64, // u16: 100 (LE)
            0x00, 0x00, 0x03, 0xE8, // u32: 1000 (LE)
        ];
        
        let result = ResponseDecoder::new(&data)
            .u8("field1")
            .unwrap()
            .u16("field2")
            .unwrap()
            .u32("field3")
            .unwrap()
            .to_value();
        
        assert_eq!(result["field1"], 10);
        assert_eq!(result["field2"], 100);
        assert_eq!(result["field3"], 1000);
    }

    #[test]
    fn test_decode_array() {
        let data = vec![
            0x01, 0x02, 0x03, 0x04, 0x05
        ];
        
        let result = ResponseDecoder::new(&data)
            .array_u8("values", 5)
            .unwrap()
            .to_value();
        
        let array = result["values"].as_array().unwrap();
        assert_eq!(array.len(), 5);
        assert_eq!(array[0], 1);
        assert_eq!(array[4], 5);
    }

    #[test]
    fn test_decode_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let result = ResponseDecoder::new(&data)
            .bytes("data", 4)
            .unwrap()
            .to_value();
        
        // Check that bytes are returned as hex string
        assert!(result["data"].is_string());
    }

    #[test]
    #[should_panic(expected = "Not enough bytes")]
    fn test_not_enough_bytes() {
        let data = vec![0x01, 0x02];
        ResponseDecoder::new(&data)
            .u64("value")
            .unwrap();
    }

    #[test]
    fn test_remaining_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let result = ResponseDecoder::new(&data)
            .u8("first")
            .unwrap()
            .remaining_bytes("rest")
            .unwrap()
            .to_value();
        
        assert_eq!(result["first"], 1);
        // rest should have 4 bytes
    }
}
```

#### Testing BinaryReader

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_reader_position() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(&data);
        
        assert_eq!(reader.position, 0);
        reader.read_u8().unwrap();
        assert_eq!(reader.position, 1);
        reader.read_u8().unwrap();
        assert_eq!(reader.position, 2);
    }

    #[test]
    fn test_ensure_available() {
        let data = vec![0x01, 0x02];
        let reader = BinaryReader::new(&data);
        
        assert!(reader.ensure_available(2).is_ok());
        assert!(reader.ensure_available(3).is_err());
    }

    #[test]
    fn test_read_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(&data);
        
        let bytes = reader.read_bytes(2).unwrap();
        assert_eq!(bytes, &[0x01, 0x02]);
        assert_eq!(reader.position, 2);
    }
}
```

## 🌐 WASM Testing

### Testing in Node.js

Create `test.mjs`:

```javascript
import init, { RequestDataBuilder, ResponseDecoder } from './pkg/scapi.js';

async function test() {
    // Initialize WASM
    await init();
    
    // Test RequestDataBuilder
    const builder = new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(1);
    
    console.log('✓ RequestDataBuilder created');
    
    // Test encoding
    builder.add_uint64(BigInt(1000));
    const bytes = builder.to_bytes();
    console.log('✓ Encoded bytes:', bytes.length);
    
    // Test ResponseDecoder
    const testData = new Uint8Array([0x0A, 0x14, 0x1E, 0x28]);
    const decoder = new ResponseDecoder(testData)
        .u8("field1")
        .u8("field2")
        .u8("field3")
        .u8("field4");
    
    const result = decoder.to_value();
    console.log('✓ Decoded result:', result);
    
    console.log('All tests passed! ✅');
}

test().catch(console.error);
```

Run:
```bash
node test.mjs
```

### Testing in Browser

Create `test.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <title>SCAPI Test</title>
</head>
<body>
    <h1>SCAPI Test</h1>
    <div id="output"></div>
    
    <script type="module">
        import init, { RequestDataBuilder, ResponseDecoder } from './pkg/scapi.js';
        
        async function runTests() {
            await init();
            
            const output = document.getElementById('output');
            
            try {
                // Test 1
                const builder = new RequestDataBuilder()
                    .set_contract_index(16)
                    .set_input_type(1);
                output.innerHTML += '<p>✓ RequestDataBuilder works</p>';
                
                // Test 2
                const testData = new Uint8Array([0x0A, 0x14]);
                const decoder = new ResponseDecoder(testData)
                    .u8("a")
                    .u8("b");
                const result = decoder.to_value();
                output.innerHTML += `<p>✓ ResponseDecoder works: ${JSON.stringify(result)}</p>`;
                
                // Test 3: Real query (if RPC is available)
                const response = await builder.send();
                output.innerHTML += `<p>✓ Query works: ${response.length} bytes</p>`;
                
                output.innerHTML += '<p><strong>All tests passed! ✅</strong></p>';
            } catch (error) {
                output.innerHTML += `<p style="color: red">❌ Error: ${error}</p>`;
            }
        }
        
        runTests();
    </script>
</body>
</html>
```

## 🔗 Integration Testing

### Testing with Real Contracts

```rust
#[tokio::test]
#[ignore] // Run with --ignored flag
async fn test_real_contract_query() {
    let response = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(1) // GetFees
        .send()
        .await
        .expect("Query failed");
    
    let result = ResponseDecoder::new(&response)
        .u8("teamFeePercent")
        .unwrap()
        .u8("distributionFeePercent")
        .unwrap()
        .to_value();
    
    // Verify reasonable values
    let team_fee = result["teamFeePercent"].as_u64().unwrap();
    assert!(team_fee <= 100, "Fee should be percentage");
}
```

Run ignored tests:
```bash
cargo test -- --ignored
```

## 🖐️ Manual Testing

### CLI Testing

```bash
# Build CLI
cargo build --bin scapi-cli

# Run CLI
cargo run --bin scapi-cli

# Test different contracts
# (Add interactive commands in main.rs)
```

### WASM Manual Testing

```bash
# Build WASM
wasm-pack build --target web --out-dir pkg

# Serve with local server
python -m http.server 8000
# or
npx serve .

# Open browser to http://localhost:8000/test.html
```

### Testing ResponseDecoder Patterns

```rust
// Test struct decoding
#[test]
fn test_struct_array() {
    let data = create_test_data_with_structs();
    
    let result = ResponseDecoder::new(&data)
        .array_struct_bytes("items", 10, 48)
        .unwrap()
        .to_value();
    
    let items = result["items"].as_array().unwrap();
    assert_eq!(items.len(), 10);
    
    // Decode first struct
    let first_item_bytes = items[0].as_array().unwrap();
    let first_item_data: Vec<u8> = first_item_bytes
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    
    let first_item = ResponseDecoder::new(&first_item_data)
        .bytes("id", 32)
        .unwrap()
        .u64("value")
        .unwrap()
        .to_value();
    
    assert!(first_item["id"].is_string());
    assert!(first_item["value"].is_number());
}
```

## 📊 Test Coverage Goals

- **Core API (sc_api.rs):** > 90%
- **WASM bindings (wasm.rs):** > 80%
- **Integration tests:** Key contract functions

## 🐛 Debugging Tests

### Enable Logging

```rust
#[test]
fn test_with_logging() {
    env_logger::init();
    
    // Your test code
}
```

### Print Intermediate Values

```rust
#[test]
fn test_debug() {
    let data = vec![0x01, 0x02, 0x03];
    println!("Input data: {:?}", data);
    
    let result = ResponseDecoder::new(&data)
        .u8("value")
        .unwrap()
        .to_value();
    
    println!("Result: {:?}", result);
}
```

## ✅ Continuous Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Check formatting
        run: cargo fmt --check
  
  wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
      - name: Build WASM
        run: wasm-pack build --target web
```

---

For more information, see:
- [Contributing Guide](CONTRIBUTING.md)
- [Architecture](ARCHITECTURE.md)
- [Examples](EXAMPLES.md)

---

Made with ❤️ for Qubic ecosystem
