# SCAPI Quick Start Guide

Get started with SCAPI in 5 minutes.

## 🚀 For JavaScript Developers

### 1. Get WASM Module

```bash
# Copy ready-made files from pkg/ directory
cp -r pkg/ your-project/public/wasm/
```

Or build yourself:
```bash
wasm-pack build --target web --out-dir pkg
```

### 2. Initialize SCAPI

```javascript
// scapi.js - create initialization wrapper
import init, { RequestDataBuilder, ResponseDecoder } from './wasm/pkg/scapi.js';

let scapiInitialized = false;

export async function getScapi() {
    if (!scapiInitialized) {
        await init({ module_or_path: './wasm/pkg/scapi_bg.wasm' });
        scapiInitialized = true;
    }
    return { RequestDataBuilder, ResponseDecoder };
}
```

### 3. Use in Code

```javascript
import { getScapi } from './scapi.js';

async function queryContract() {
    // Get SCAPI
    const { RequestDataBuilder, ResponseDecoder } = await getScapi();
    
    // Make request
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(1)
        .send();
    
    // Decode response
    const result = new ResponseDecoder(response)
        .u8("teamFeePercent")
        .u8("distributionFeePercent")
        .to_value();
    
    console.log('Team Fee:', result.teamFeePercent, '%');
}

queryContract();
```

### 4. In React Component

```javascript
import { useEffect, useState } from 'react';
import { getScapi } from './scapi.js';

function ContractData() {
    const [fees, setFees] = useState(null);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        async function fetchFees() {
            try {
                const { RequestDataBuilder, ResponseDecoder } = await getScapi();
                
                const response = await new RequestDataBuilder()
                    .set_contract_index(16)
                    .set_input_type(1)
                    .send();
                
                const result = new ResponseDecoder(response)
                    .u8("teamFeePercent")
                    .u8("distributionFeePercent")
                    .to_value();
                
                setFees(result);
            } catch (error) {
                console.error('Error:', error);
            } finally {
                setLoading(false);
            }
        }
        
        fetchFees();
    }, []);
    
    if (loading) return <div>Loading...</div>;
    
    return (
        <div>
            <p>Team Fee: {fees?.teamFeePercent}%</p>
            <p>Distribution: {fees?.distributionFeePercent}%</p>
        </div>
    );
}
```

---

## 🦀 For Rust Developers

### 1. Add Dependency

```toml
[dependencies]
scapi = { git = "https://github.com/your-repo/scapi" }
tokio = { version = "1", features = ["full"] }
```

### 2. Write Code

```rust
use scapi::{RequestDataBuilder, ResponseDecoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build request
    let response = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(1)
        .send()
        .await?;
    
    // Decode response
    let result = ResponseDecoder::new(&response)
        .u8("teamFeePercent")
        .u8("distributionFeePercent")
        .to_value();
    
    println!("Fees: {}", result);
    Ok(())
}
```

### 3. Run

```bash
cargo run
```

---

## 📚 Common Tasks Examples

### Get Contract Balance

```javascript
async function getBalance() {
    const { RequestDataBuilder, ResponseDecoder } = await getScapi();
    
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(10)
        .send();
    
    const result = new ResponseDecoder(response)
        .u64("balance")
        .to_value();
    
    return result.balance;
}
```

### Get List of Items

```javascript
async function getPlayers() {
    const { RequestDataBuilder, ResponseDecoder } = await getScapi();
    
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(2)
        .send();
    
    const result = new ResponseDecoder(response)
        .array_m256i("players", 1024)
        .u64("playerCounter")
        .to_value();
    
    const count = Number(result.playerCounter);
    return result.players.slice(0, count);
}
```

### Decode Array of Structs

```javascript
async function getWinners() {
    const { RequestDataBuilder, ResponseDecoder } = await getScapi();
    
    const BYTES_PER_WINNER = 48;
    const MAX_WINNERS = 1024;
    
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(3)
        .send();
    
    const result = new ResponseDecoder(response)
        .array_struct_bytes("winners", MAX_WINNERS, BYTES_PER_WINNER)
        .u64("winnersCounter")
        .to_value();
    
    const count = Number(result.winnersCounter);
    const winners = [];
    
    for (let i = 0; i < count; i++) {
        const winnerBytes = result.winners[i];
        const winner = new ResponseDecoder(new Uint8Array(winnerBytes))
            .bytes("address", 32)
            .u64("revenue")
            .u16("epoch")
            .u32("tick")
            .to_value();
        
        winners.push(winner);
    }
    
    return winners;
}
```

---

## 🔧 Common Issues

### WASM Not Initializing

```javascript
// ❌ Wrong (old format)
await init('./wasm/pkg/scapi_bg.wasm');

// ✅ Correct (new format)
await init({ module_or_path: './wasm/pkg/scapi_bg.wasm' });
```

### "Not enough bytes" Error

```javascript
// ❌ Wrong - view functions DON'T return returnCode
const result = new ResponseDecoder(response)
    .u8("returnCode")  // This field doesn't exist!
    .u64("balance")
    .to_value();

// ✅ Correct
const result = new ResponseDecoder(response)
    .u64("balance")
    .to_value();
```

### Wrong Field Order

```javascript
// Field order must match the contract struct!

// Struct: struct { u8 a, u16 b, u32 c }
const result = new ResponseDecoder(response)
    .u8("a")
    .u16("b")
    .u32("c")
    .to_value();
```

---

## 📖 Next Steps

- Read [full README](README.md) for detailed documentation
- Study [examples](EXAMPLES.md) for advanced cases
- See [API Reference](README.md#-api-reference) for all methods
- Join [development](CONTRIBUTING.md)

---

## 🆘 Need Help?

- GitHub Issues: [create an issue](https://github.com/your-repo/scapi/issues)
- Documentation: [README.md](README.md)
- Examples: [EXAMPLES.md](EXAMPLES.md)

---

Made with ❤️ for Qubic ecosystem
