# SCAPI - Usage Examples

Collection of practical examples for using SCAPI to interact with Qubic smart contracts.

## 📋 Table of Contents

- [Rust Examples](#rust-examples)
- [JavaScript/WASM Examples](#javascriptwasm-examples)
- [Working with RandomLottery Contract](#working-with-randomlottery-contract)
- [Decoding Complex Structures](#decoding-complex-structures)

---

## Rust Examples

### Basic Contract Query

```rust
use scapi::{RequestDataBuilder, ResponseDecoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple query without parameters
    let response = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(1)
        .send()
        .await?;
    
    println!("Raw response: {} bytes", response.len());
    Ok(())
}
```

### Query with Parameters

```rust
// Query with uint64 parameter
let response = RequestDataBuilder::new()
    .set_contract_index(16)
    .set_input_type(5)
    .add_uint64(1000000) // 1 QUBIC
    .send()
    .await?;
```

### Decoding Simple Types

```rust
let result = ResponseDecoder::new(&response)
    .u8("status")
    .u16("count")
    .u32("timestamp")
    .u64("balance")
    .to_value();

println!("Status: {}", result["status"]);
println!("Balance: {}", result["balance"]);
```

### Working with Arrays

```rust
let result = ResponseDecoder::new(&response)
    .array_u32("values", 10)
    .u64("sum")
    .to_value();

let values = result["values"].as_array().unwrap();
for (i, val) in values.iter().enumerate() {
    println!("Value[{}]: {}", i, val);
}
```

---

## JavaScript/WASM Examples

### Module Initialization

```javascript
import init, { RequestDataBuilder, ResponseDecoder } from './pkg/scapi.js';

// Option 1: Automatic WASM loading
await init();

// Option 2: Explicit path to WASM file
await init('./pkg/scapi_bg.wasm');

// Option 3: With error handling
try {
    await init();
    console.log('SCAPI initialized successfully');
} catch (error) {
    console.error('Failed to initialize SCAPI:', error);
}
```

### Simple Query

```javascript
async function getContractData() {
    try {
        const response = await new RequestDataBuilder()
            .set_contract_index(16)
            .set_input_type(1)
            .send();
        
        console.log('Response received:', response.length, 'bytes');
        return response;
    } catch (error) {
        console.error('Query failed:', error);
        throw error;
    }
}
```

### Decoding in JavaScript

```javascript
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
    
    return {
        team: fees.teamFeePercent,
        distribution: fees.distributionFeePercent,
        winner: fees.winnerFeePercent,
        burn: fees.burnPercent
    };
}

// Usage
const fees = await getFees();
console.log('Team fee:', fees.team, '%');
```

---

## Working with RandomLottery Contract

### GetFees (Getting Fees)

```javascript
async function getFees() {
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(1) // GetFees
        .send();
    
    return new ResponseDecoder(response)
        .u8("teamFeePercent")
        .u8("distributionFeePercent")
        .u8("winnerFeePercent")
        .u8("burnPercent")
        .to_value();
}
```

### GetPlayers (Getting Player List)

```javascript
async function getPlayers() {
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(2) // GetPlayers
        .send();
    
    const result = new ResponseDecoder(response)
        .array_m256i("players", 1024)
        .u64("playerCounter")
        .to_value();
    
    const playerCount = Number(result.playerCounter);
    const players = [];
    
    for (let i = 0; i < playerCount; i++) {
        players.push(result.players[i]);
    }
    
    return {
        count: playerCount,
        players: players
    };
}
```

### GetWinners (Getting Winners)

```javascript
async function getWinners() {
    const BYTES_PER_WINNER = 48; // id(32) + revenue(8) + epoch(2) + padding(2) + tick(4)
    const MAX_WINNERS = 1024;
    
    const response = await new RequestDataBuilder()
        .set_contract_index(16)
        .set_input_type(3) // GetWinners
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
            .bytes("winnerAddress", 32)
            .u64("revenue")
            .u16("epoch")
            .bytes("_padding", 2)
            .u32("tick")
            .to_value();
        
        winners.push({
            address: winner.winnerAddress,
            revenue: winner.revenue,
            epoch: winner.epoch,
            tick: winner.tick
        });
    }
    
    return { count, winners };
}
```

---

## Decoding Complex Structures

### Nested Structures

```javascript
// Structure: Array<Struct, 100> where each Struct = { id: [u8; 32], value: u64 }
const STRUCT_SIZE = 32 + 8; // 40 bytes
const ARRAY_COUNT = 100;

const result = new ResponseDecoder(response)
    .array_struct_bytes("items", ARRAY_COUNT, STRUCT_SIZE)
    .u32("totalCount")
    .to_value();

// Decode each structure
const items = [];
for (let i = 0; i < result.totalCount; i++) {
    const itemBytes = result.items[i];
    const item = new ResponseDecoder(new Uint8Array(itemBytes))
        .bytes("id", 32)
        .u64("value")
        .to_value();
    
    items.push(item);
}
```

### Working with Hex Strings

```javascript
// ResponseDecoder.bytes() returns hex string "0x..."
// Convert hex to bytes:
function hexToBytes(hexString) {
    const hex = hexString.startsWith('0x') ? hexString.slice(2) : hexString;
    const bytes = new Uint8Array(hex.length / 2);
    for (let i = 0; i < bytes.length; i++) {
        bytes[i] = parseInt(hex.substr(i * 2, 2), 16);
    }
    return bytes;
}

// Convert bytes to base64
function bytesToBase64(bytes) {
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
        binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
}

// Usage
const result = new ResponseDecoder(response)
    .bytes("address", 32)
    .to_value();

const addressBytes = hexToBytes(result.address);
const addressBase64 = bytesToBase64(addressBytes);
```

---

## React Integration

### Custom Hook for SCAPI

```javascript
import { useState, useEffect } from 'react';
import { getScapi } from './scapi';

export function useContractQuery(contractIndex, inputType, params = {}) {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    
    useEffect(() => {
        let cancelled = false;
        
        async function fetchData() {
            try {
                setLoading(true);
                const { RequestDataBuilder } = await getScapi();
                
                let builder = new RequestDataBuilder()
                    .set_contract_index(contractIndex)
                    .set_input_type(inputType);
                
                // Add parameters if present
                for (const [key, value] of Object.entries(params)) {
                    if (typeof value === 'number') {
                        builder = builder.add_uint64(value);
                    }
                }
                
                const response = await builder.send();
                
                if (!cancelled) {
                    setData(response);
                    setError(null);
                }
            } catch (err) {
                if (!cancelled) {
                    setError(err.message);
                }
            } finally {
                if (!cancelled) {
                    setLoading(false);
                }
            }
        }
        
        fetchData();
        
        return () => {
            cancelled = true;
        };
    }, [contractIndex, inputType, JSON.stringify(params)]);
    
    return { data, loading, error };
}

// Usage in component
function LotteryStats() {
    const { data, loading, error } = useContractQuery(16, 1);
    
    if (loading) return <div>Loading...</div>;
    if (error) return <div>Error: {error}</div>;
    
    // Decode data
    const fees = new ResponseDecoder(data)
        .u8("teamFeePercent")
        .u8("distributionFeePercent")
        .to_value();
    
    return (
        <div>
            <p>Team Fee: {fees.teamFeePercent}%</p>
            <p>Distribution: {fees.distributionFeePercent}%</p>
        </div>
    );
}
```

---

## Error Handling

### In Rust

```rust
use anyhow::Result;

async fn safe_query() -> Result<()> {
    let response = RequestDataBuilder::new()
        .set_contract_index(16)
        .set_input_type(1)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Query failed: {}", e))?;
    
    let result = ResponseDecoder::new(&response)
        .u8("value")
        .to_value()?;
    
    Ok(())
}
```

### In JavaScript

```javascript
async function safeQuery() {
    try {
        const response = await new RequestDataBuilder()
            .set_contract_index(16)
            .set_input_type(1)
            .send();
        
        return { success: true, data: response };
    } catch (error) {
        console.error('Query error:', error);
        return { success: false, error: error.message };
    }
}

// Usage with retry
async function queryWithRetry(maxRetries = 3) {
    for (let i = 0; i < maxRetries; i++) {
        const result = await safeQuery();
        if (result.success) {
            return result.data;
        }
        
        // Wait before retry
        await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
    }
    
    throw new Error('Max retries exceeded');
}
```

---

## Optimization and Best Practices

### Caching Results

```javascript
const cache = new Map();

async function getCachedData(contractIndex, inputType, ttl = 5000) {
    const key = `${contractIndex}-${inputType}`;
    const cached = cache.get(key);
    
    if (cached && Date.now() - cached.timestamp < ttl) {
        return cached.data;
    }
    
    const response = await new RequestDataBuilder()
        .set_contract_index(contractIndex)
        .set_input_type(inputType)
        .send();
    
    cache.set(key, {
        data: response,
        timestamp: Date.now()
    });
    
    return response;
}
```

### Batch Queries

```javascript
async function batchQuery(queries) {
    const results = await Promise.all(
        queries.map(async ({ contractIndex, inputType }) => {
            try {
                const response = await new RequestDataBuilder()
                    .set_contract_index(contractIndex)
                    .set_input_type(inputType)
                    .send();
                return { success: true, data: response };
            } catch (error) {
                return { success: false, error: error.message };
            }
        })
    );
    
    return results;
}

// Usage
const results = await batchQuery([
    { contractIndex: 16, inputType: 1 },
    { contractIndex: 16, inputType: 2 },
    { contractIndex: 16, inputType: 3 }
]);
```

---

## Additional Resources

- [Main README](README.md)
- [API Reference](README.md#-api-reference)
- [Qubic Documentation](https://docs.qubic.org)

---

Made with ❤️ for Qubic developers
