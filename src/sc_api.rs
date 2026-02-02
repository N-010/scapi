#![allow(dead_code)]

use crate::rpc::post;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;

/// Sends raw request bytes to Qubic RPC `querySmartContract` endpoint.
///
/// The bytes are base64-encoded into JSON field `requestData`.
/// The function tries to decode base64 from `responseData` first, then `data` if present.
/// If neither base64 field exists, returns the raw JSON bytes of the response.
pub async fn query_smart_contract_with_meta(
    contract_index: u32,
    input_type: u32,
    request_bytes: &[u8],
) -> Result<Vec<u8>> {
    post::query_smart_contract_with_meta(contract_index, input_type, request_bytes).await
}

/// Backward-compatible helper that sends without metadata.
/// Note: the API expects contractIndex/inputType/inputSize; this uses zeros which may be rejected.
pub async fn query_smart_contract(request_bytes: &[u8]) -> Result<Vec<u8>> {
    post::query_smart_contract(request_bytes).await
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

#[derive(Debug, Default, Clone)]
pub struct RequestDataBuilder {
    payload: PayloadBuilder,
    endianness: Endianness,
    contract_index: u32,
    input_type: u32,
}

impl RequestDataBuilder {
    pub fn new() -> Self {
        Self {
            payload: PayloadBuilder::new(),
            endianness: Endianness::Little,
            contract_index: 0,
            input_type: 1,
        }
    }

    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }

    pub fn add_bytes(mut self, bytes: &[u8]) -> Self {
        self.payload = self.payload.add_bytes(bytes);
        self
    }

    pub fn set_contract_index(mut self, index: u32) -> Self {
        self.contract_index = index;
        self
    }

    pub fn set_input_type(mut self, input_type: u32) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn add_bool(mut self, value: bool) -> Self {
        self.payload = self.payload.add_bool(value);
        self
    }

    pub fn add_int8(mut self, value: i8) -> Self {
        self.payload = self.payload.add_int8(value);
        self
    }

    pub fn add_uint8(mut self, value: u8) -> Self {
        self.payload = self.payload.add_uint8(value);
        self
    }

    pub fn add_int16(mut self, value: i16) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_uint16(mut self, value: u16) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_int32(mut self, value: i32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_uint32_legacy(self, value: u32) -> Self {
        // Backward compatibility for earlier camelCase variant
        self.add_uint32(value)
    }

    pub fn add_uint32(mut self, value: u32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_int64(mut self, value: i64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_uint64(mut self, value: u64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_float(mut self, value: f32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_double(mut self, value: f64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.payload = self.payload.add_bytes(&bytes);
        self
    }

    pub fn add_char(mut self, value: u8) -> Self {
        // C++ char as single byte
        self.payload = self.payload.add_char(value);
        self
    }

    /// Append 256-bit integer (m256i) from raw 32 bytes as-is
    pub fn add_m256i_bytes(mut self, bytes32: [u8; 32]) -> Self {
        self.payload = self.payload.add_m256i_bytes(bytes32);
        self
    }

    /// Append 256-bit integer assembled from four u64 words.
    /// Word order respects current endianness:
    /// - Little endian: least-significant word first
    /// - Big endian: most-significant word first
    pub fn add_m256i_from_u64x4(mut self, w0: u64, w1: u64, w2: u64, w3: u64) -> Self {
        let words = match self.endianness {
            Endianness::Little => [w0, w1, w2, w3],
            Endianness::Big => [w3, w2, w1, w0],
        };
        for w in words {
            let bytes = match self.endianness {
                Endianness::Little => w.to_le_bytes(),
                Endianness::Big => w.to_be_bytes(),
            };
            self.payload = self.payload.add_bytes(&bytes);
        }
        self
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.payload.to_bytes()
    }

    pub fn to_base64(&self) -> String {
        self.payload.to_base64()
    }

    pub async fn send(self) -> Result<Vec<u8>> {
        let payload = self.payload.to_bytes();
        post::query_smart_contract_with_meta(self.contract_index, self.input_type, &payload).await
    }
}

// ------------------------
// PayloadBuilder (Little Endian)
// ------------------------

#[derive(Debug, Default, Clone)]
pub struct PayloadBuilder {
    buffer: Vec<u8>,
    endianness: Endianness,
}

impl PayloadBuilder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            endianness: Endianness::Little,
        }
    }

    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }

    pub fn add_bytes(mut self, bytes: &[u8]) -> Self {
        self.buffer.extend_from_slice(bytes);
        self
    }

    pub fn add_bool(mut self, value: bool) -> Self {
        self.buffer.push(if value { 1 } else { 0 });
        self
    }

    pub fn add_int8(mut self, value: i8) -> Self {
        self.buffer.push(value as u8);
        self
    }

    pub fn add_uint8(mut self, value: u8) -> Self {
        self.buffer.push(value);
        self
    }

    pub fn add_int16(mut self, value: i16) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_uint16(mut self, value: u16) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_int32(mut self, value: i32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_uint32(mut self, value: u32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_int64(mut self, value: i64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_uint64(mut self, value: u64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_float(mut self, value: f32) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_double(mut self, value: f64) -> Self {
        let bytes = match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    pub fn add_char(mut self, value: u8) -> Self {
        self.buffer.push(value);
        self
    }

    /// Append 256-bit integer (m256i) from raw 32 bytes as-is
    pub fn add_m256i_bytes(mut self, bytes32: [u8; 32]) -> Self {
        self.buffer.extend_from_slice(&bytes32);
        self
    }

    /// Append 256-bit integer assembled from four u64 words.
    pub fn add_m256i_from_u64x4(mut self, w0: u64, w1: u64, w2: u64, w3: u64) -> Self {
        let words = match self.endianness {
            Endianness::Little => [w0, w1, w2, w3],
            Endianness::Big => [w3, w2, w1, w0],
        };
        for w in words {
            let bytes = match self.endianness {
                Endianness::Little => w.to_le_bytes(),
                Endianness::Big => w.to_be_bytes(),
            };
            self.buffer.extend_from_slice(&bytes);
        }
        self
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.buffer)
    }
}

// ------------------------
// Response decoding helpers
// ------------------------

#[derive(Debug, Clone)]
pub struct BinaryReader<'a> {
    buffer: &'a [u8],
    position: usize,
    endianness: Endianness,
}

impl<'a> BinaryReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
            endianness: Endianness::Little,
        }
    }

    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }

    fn ensure_available(&self, needed: usize) -> Result<()> {
        if self.position + needed > self.buffer.len() {
            return Err(anyhow!(
                "Not enough bytes: need {}, have {} from pos {}",
                needed,
                self.buffer.len() - self.position,
                self.position
            ));
        }
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.position
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.ensure_available(1)?;
        let v = self.buffer[self.position];
        self.position += 1;
        Ok(v)
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.ensure_available(2)?;
        let bytes = &self.buffer[self.position..self.position + 2];
        self.position += 2;
        Ok(match self.endianness {
            Endianness::Little => u16::from_le_bytes([bytes[0], bytes[1]]),
            Endianness::Big => u16::from_be_bytes([bytes[0], bytes[1]]),
        })
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.ensure_available(4)?;
        let bytes = &self.buffer[self.position..self.position + 4];
        self.position += 4;
        Ok(match self.endianness {
            Endianness::Little => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            Endianness::Big => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        })
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        self.ensure_available(8)?;
        let bytes = &self.buffer[self.position..self.position + 8];
        self.position += 8;
        Ok(match self.endianness {
            Endianness::Little => u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            Endianness::Big => u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
        })
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        self.ensure_available(len)?;
        let start = self.position;
        self.position += len;
        Ok(&self.buffer[start..start + len])
    }

    /// Read 256-bit value (32 bytes) as raw bytes
    pub fn read_m256i_bytes(&mut self) -> Result<[u8; 32]> {
        let b = self.read_bytes(32)?;
        let mut out = [0u8; 32];
        out.copy_from_slice(b);
        Ok(out)
    }
}

// Example: decoder for the provided C++ struct
#[derive(Debug, Clone, Copy)]
pub struct GetFeesOutput {
    pub team_fee_percent: u8,
    pub distribution_fee_percent: u8,
    pub winner_fee_percent: u8,
    pub burn_percent: u8,
    pub return_code: u8,
}

impl GetFeesOutput {
    pub fn decode(mut reader: BinaryReader<'_>) -> Result<Self> {
        // Layout: 5 x u8 in order
        let team_fee_percent = reader.read_u8()?;
        let distribution_fee_percent = reader.read_u8()?;
        let winner_fee_percent = reader.read_u8()?;
        let burn_percent = reader.read_u8()?;
        let return_code = reader.read_u8()?;
        Ok(Self {
            team_fee_percent,
            distribution_fee_percent,
            winner_fee_percent,
            burn_percent,
            return_code,
        })
    }
}

// --------------------------------------
// Fluent ResponseDecoder -> serde_json
// --------------------------------------
#[derive(Debug, Clone)]
pub struct ResponseDecoder<'a> {
    reader: BinaryReader<'a>,
    obj: serde_json::Map<String, Value>,
}

impl<'a> ResponseDecoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            reader: BinaryReader::new(bytes),
            obj: serde_json::Map::new(),
        }
    }

    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.reader = self.reader.with_endianness(endianness);
        self
    }

    pub fn u8(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_u8()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_u8(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_u8()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn i8(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_i8()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_i8(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_i8()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn u16(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_u16()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_u16(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_u16()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn i16(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_i16()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_i16(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_i16()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn u32(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_u32()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_u32(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_u32()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn i32(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_i32()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_i32(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_i32()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn u64(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_u64()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_u64(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_u64()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn i64(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_i64()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_i64(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_i64()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn f32(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_f32()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_f32(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_f32()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn f64(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_f64()?;
        self.obj.insert(field.to_string(), Value::from(v));
        Ok(self)
    }

    pub fn array_f64(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr = Vec::with_capacity(count);
        for _ in 0..count {
            arr.push(Value::from(self.reader.read_f64()?));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn bytes(mut self, field: &str, len: usize) -> Result<Self> {
        let v = self.reader.read_bytes(len)?;
        self.obj.insert(
            field.to_string(),
            Value::from(format!("0x{}", hex::encode(v))),
        );
        Ok(self)
    }

    /// Read 256-bit integer (m256i) and output as 0x-prefixed hex string (32 bytes)
    pub fn m256i(mut self, field: &str) -> Result<Self> {
        let v = self.reader.read_m256i_bytes()?;
        self.obj.insert(
            field.to_string(),
            Value::from(format!("0x{}", hex::encode(v))),
        );
        Ok(self)
    }

    /// Read an array of m256i values by element count (each is 32 bytes)
    pub fn array_m256i(mut self, field: &str, count: usize) -> Result<Self> {
        let mut arr: Vec<Value> = Vec::with_capacity(count);
        for _ in 0..count {
            let v = self.reader.read_m256i_bytes()?;
            arr.push(Value::from(format!("0x{}", hex::encode(v))));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn remaining_bytes(mut self, field: &str) -> Result<Self> {
        let rem = self.reader.remaining();
        let v = self.reader.read_bytes(rem)?;
        self.obj.insert(
            field.to_string(),
            Value::from(format!("0x{}", hex::encode(v))),
        );
        Ok(self)
    }

    /// Advance the reader by `len` bytes without storing the data.
    pub fn skip_bytes(mut self, len: usize) -> Result<Self> {
        self.reader.read_bytes(len)?;
        Ok(self)
    }

    /// Decode an array of structures. Each structure is decoded by the provided closure.
    /// The closure receives a ResponseDecoder positioned at the start of each struct.
    pub fn array_struct<F>(
        mut self,
        field: &str,
        count: usize,
        struct_size: usize,
        decode_fn: F,
    ) -> Result<Self>
    where
        F: Fn(ResponseDecoder) -> Result<Value>,
    {
        let mut arr: Vec<Value> = Vec::with_capacity(count);
        let endianness = self.reader.endianness;
        for _ in 0..count {
            let struct_bytes = self.reader.read_bytes(struct_size)?;
            let struct_decoder = ResponseDecoder::new(struct_bytes).with_endianness(endianness);
            let decoded_struct = decode_fn(struct_decoder)?;
            arr.push(decoded_struct);
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    /// Read raw bytes for an array of structures (for manual decoding in WASM)
    /// Returns an array of byte slices, one per structure.
    pub fn array_struct_bytes(
        mut self,
        field: &str,
        count: usize,
        struct_size: usize,
    ) -> Result<Self> {
        let mut arr: Vec<Value> = Vec::with_capacity(count);
        for _ in 0..count {
            let struct_bytes = self.reader.read_bytes(struct_size)?;
            arr.push(Value::Array(
                struct_bytes.iter().map(|&b| Value::from(b)).collect(),
            ));
        }
        self.obj.insert(field.to_string(), Value::Array(arr));
        Ok(self)
    }

    pub fn to_value(self) -> Value {
        Value::Object(self.obj)
    }

    pub fn to_json_string(self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.to_value())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BinaryReader tests
    #[test]
    fn test_binary_reader_u8() {
        let data = vec![0x42, 0xFF, 0x00];
        let mut reader = BinaryReader::new(&data);

        assert_eq!(reader.read_u8().unwrap(), 0x42);
        assert_eq!(reader.read_u8().unwrap(), 0xFF);
        assert_eq!(reader.read_u8().unwrap(), 0x00);
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn test_binary_reader_u16_little_endian() {
        let data = vec![0x34, 0x12]; // 0x1234 in LE
        let mut reader = BinaryReader::new(&data);
        reader.endianness = Endianness::Little;

        assert_eq!(reader.read_u16().unwrap(), 0x1234);
    }

    #[test]
    fn test_binary_reader_u16_big_endian() {
        let data = vec![0x12, 0x34]; // 0x1234 in BE
        let mut reader = BinaryReader::new(&data);
        reader.endianness = Endianness::Big;

        assert_eq!(reader.read_u16().unwrap(), 0x1234);
    }

    #[test]
    fn test_binary_reader_u32_little_endian() {
        let data = vec![0x78, 0x56, 0x34, 0x12]; // 0x12345678 in LE
        let mut reader = BinaryReader::new(&data);
        reader.endianness = Endianness::Little;

        assert_eq!(reader.read_u32().unwrap(), 0x12345678);
    }

    #[test]
    fn test_binary_reader_u64_little_endian() {
        let data = vec![0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12];
        let mut reader = BinaryReader::new(&data);
        reader.endianness = Endianness::Little;

        assert_eq!(reader.read_u64().unwrap(), 0x1234567890ABCDEF);
    }

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
    fn test_binary_reader_read_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(&data);

        let bytes = reader.read_bytes(2).unwrap();
        assert_eq!(bytes, &[0x01, 0x02]);
        assert_eq!(reader.position, 2);

        let bytes2 = reader.read_bytes(2).unwrap();
        assert_eq!(bytes2, &[0x03, 0x04]);
    }

    #[test]
    fn test_binary_reader_not_enough_bytes() {
        let data = vec![0x01, 0x02];
        let mut reader = BinaryReader::new(&data);

        assert!(reader.ensure_available(2).is_ok());
        assert!(reader.ensure_available(3).is_err());

        reader.read_u8().unwrap();
        assert!(reader.ensure_available(2).is_err());
    }

    // RequestDataBuilder tests
    #[test]
    fn test_request_builder_basic() {
        let builder = RequestDataBuilder::new()
            .set_contract_index(16)
            .set_input_type(1);

        assert_eq!(builder.contract_index, 16);
        assert_eq!(builder.input_type, 1);
    }

    #[test]
    fn test_request_builder_add_uint8() {
        let builder = RequestDataBuilder::new().add_uint8(10);

        assert_eq!(builder.to_bytes(), vec![10]);
    }

    #[test]
    fn test_request_builder_add_uint16() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint16(0x1234);

        assert_eq!(builder.to_bytes(), vec![0x34, 0x12]);
    }

    #[test]
    fn test_request_builder_add_uint32() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint32(0x12345678);

        assert_eq!(builder.to_bytes(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_request_builder_add_uint64() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint64(1000);

        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0], 0xE8); // 1000 = 0x03E8
        assert_eq!(bytes[1], 0x03);
    }

    #[test]
    fn test_request_builder_add_multiple() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint8(10)
            .add_uint16(1000)
            .add_uint32(100000);

        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 1 + 2 + 4); // 7 bytes total
    }

    #[test]
    fn test_request_builder_endianness() {
        let le = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint16(0x1234)
            .to_bytes();

        assert_eq!(le[0], 0x34);
        assert_eq!(le[1], 0x12);

        let be = RequestDataBuilder::new()
            .with_endianness(Endianness::Big)
            .add_uint16(0x1234)
            .to_bytes();

        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
    }

    #[test]
    fn test_request_builder_to_base64() {
        let builder = RequestDataBuilder::new()
            .add_uint8(0x01)
            .add_uint8(0x02)
            .add_uint8(0x03);

        let base64 = builder.to_base64();
        assert!(!base64.is_empty());

        // Verify it can be decoded
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let decoded = STANDARD.decode(&base64).unwrap();
        assert_eq!(decoded, vec![0x01, 0x02, 0x03]);
    }

    // ResponseDecoder tests
    #[test]
    fn test_decoder_u8() {
        let data = vec![0x42];
        let result = ResponseDecoder::new(&data).u8("value").unwrap().to_value();

        assert_eq!(result["value"], 0x42);
    }

    #[test]
    fn test_decoder_u16() {
        let data = vec![0x34, 0x12]; // 0x1234 in LE
        let result = ResponseDecoder::new(&data).u16("value").unwrap().to_value();

        assert_eq!(result["value"], 0x1234);
    }

    #[test]
    fn test_decoder_u32() {
        let data = vec![0x78, 0x56, 0x34, 0x12]; // 0x12345678 in LE
        let result = ResponseDecoder::new(&data).u32("value").unwrap().to_value();

        assert_eq!(result["value"], 0x12345678u32);
    }

    #[test]
    fn test_decoder_u64() {
        let data = vec![0xE8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // 1000 in LE
        let result = ResponseDecoder::new(&data)
            .u64("balance")
            .unwrap()
            .to_value();

        assert_eq!(result["balance"], 1000u64);
    }

    #[test]
    fn test_decoder_multiple_fields() {
        let data = vec![
            0x0A, // u8: 10
            0x64, 0x00, // u16: 100 (LE)
            0xE8, 0x03, 0x00, 0x00, // u32: 1000 (LE)
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
    fn test_decoder_array_u8() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];

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
    fn test_decoder_array_u16() {
        let data = vec![
            0x01, 0x00, // 1
            0x02, 0x00, // 2
            0x03, 0x00, // 3
        ];

        let result = ResponseDecoder::new(&data)
            .array_u16("values", 3)
            .unwrap()
            .to_value();

        let array = result["values"].as_array().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
    }

    #[test]
    fn test_decoder_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let result = ResponseDecoder::new(&data)
            .bytes("data", 4)
            .unwrap()
            .to_value();

        // bytes() returns hex string
        assert!(result["data"].is_string());
        let hex = result["data"].as_str().unwrap();
        assert!(hex.starts_with("0x"));
    }

    #[test]
    fn test_decoder_remaining_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let result = ResponseDecoder::new(&data)
            .u8("first")
            .unwrap()
            .remaining_bytes("rest")
            .unwrap()
            .to_value();

        assert_eq!(result["first"], 1);
        let rest = result["rest"].as_str().unwrap();
        assert!(rest.contains("02")); // Should contain the rest of bytes
    }

    #[test]
    fn test_decoder_not_enough_bytes() {
        let data = vec![0x01, 0x02];
        let result = ResponseDecoder::new(&data).u64("value");

        assert!(result.is_err());
    }

    #[test]
    fn test_decoder_endianness() {
        let data = vec![0x12, 0x34];

        // Little Endian (default)
        let result_le = ResponseDecoder::new(&data).u16("value").unwrap().to_value();
        assert_eq!(result_le["value"], 0x3412);

        // Big Endian
        let result_be = ResponseDecoder::new(&data)
            .with_endianness(Endianness::Big)
            .u16("value")
            .unwrap()
            .to_value();
        assert_eq!(result_be["value"], 0x1234);
    }

    #[test]
    fn test_decoder_to_json_string() {
        let data = vec![0x0A, 0x14];
        let json_str = ResponseDecoder::new(&data)
            .u8("a")
            .unwrap()
            .u8("b")
            .unwrap()
            .to_json_string()
            .unwrap();

        assert!(json_str.contains("\"a\""));
        assert!(json_str.contains("\"b\""));
        assert!(json_str.contains("10"));
        assert!(json_str.contains("20"));
    }

    #[test]
    fn test_decoder_array_struct_bytes() {
        // Create test data: 3 structs of 4 bytes each
        let data = vec![
            0x01, 0x02, 0x03, 0x04, // struct 1
            0x05, 0x06, 0x07, 0x08, // struct 2
            0x09, 0x0A, 0x0B, 0x0C, // struct 3
        ];

        let result = ResponseDecoder::new(&data)
            .array_struct_bytes("items", 3, 4)
            .unwrap()
            .to_value();

        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 3);

        // Each item should be an array of bytes
        let first_item = items[0].as_array().unwrap();
        assert_eq!(first_item.len(), 4);
        assert_eq!(first_item[0], 1);
        assert_eq!(first_item[3], 4);
    }

    #[test]
    fn test_decoder_signed_integers() {
        let data = vec![
            0xFF, // i8: -1
            0xFF, 0xFF, // i16: -1
            0xFF, 0xFF, 0xFF, 0xFF, // i32: -1
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // i64: -1
        ];

        let result = ResponseDecoder::new(&data)
            .i8("i8_val")
            .unwrap()
            .i16("i16_val")
            .unwrap()
            .i32("i32_val")
            .unwrap()
            .i64("i64_val")
            .unwrap()
            .to_value();

        assert_eq!(result["i8_val"], -1);
        assert_eq!(result["i16_val"], -1);
        assert_eq!(result["i32_val"], -1);
        assert_eq!(result["i64_val"], -1);
    }

    #[test]
    fn test_decoder_floats() {
        let mut data = vec![0u8; 12];

        // Write f32 (1.5) and f64 (2.5) in little endian
        data[0..4].copy_from_slice(&1.5f32.to_le_bytes());
        data[4..12].copy_from_slice(&2.5f64.to_le_bytes());

        let result = ResponseDecoder::new(&data)
            .f32("float_val")
            .unwrap()
            .f64("double_val")
            .unwrap()
            .to_value();

        assert_eq!(result["float_val"].as_f64().unwrap(), 1.5);
        assert_eq!(result["double_val"].as_f64().unwrap(), 2.5);
    }

    #[test]
    fn test_request_builder_signed_integers() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_int8(-1)
            .add_int16(-1)
            .add_int32(-1)
            .add_int64(-1);

        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 1 + 2 + 4 + 8); // 15 bytes

        // All bytes should be 0xFF for -1
        for &b in &bytes {
            assert_eq!(b, 0xFF);
        }
    }

    #[test]
    fn test_request_builder_floats() {
        let builder = RequestDataBuilder::new()
            .with_endianness(Endianness::Little)
            .add_float(1.5)
            .add_double(2.5);

        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 4 + 8); // 12 bytes
    }

    // PayloadBuilder tests
    #[test]
    fn test_payload_builder_basic() {
        let builder = PayloadBuilder::new().add_uint8(10).add_uint16(1000);

        let bytes = builder.to_bytes();
        assert_eq!(bytes.len(), 1 + 2);
        assert_eq!(bytes[0], 10);
    }

    #[test]
    fn test_payload_builder_endianness() {
        let le = PayloadBuilder::new()
            .with_endianness(Endianness::Little)
            .add_uint16(0x1234)
            .to_bytes();

        assert_eq!(le[0], 0x34);
        assert_eq!(le[1], 0x12);

        let be = PayloadBuilder::new()
            .with_endianness(Endianness::Big)
            .add_uint16(0x1234)
            .to_bytes();

        assert_eq!(be[0], 0x12);
        assert_eq!(be[1], 0x34);
    }

    #[test]
    fn test_payload_builder_to_base64() {
        let builder = PayloadBuilder::new()
            .add_uint8(0x01)
            .add_uint8(0x02)
            .add_uint8(0x03);

        let base64 = builder.to_base64();
        assert!(!base64.is_empty());

        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let decoded = STANDARD.decode(&base64).unwrap();
        assert_eq!(decoded, vec![0x01, 0x02, 0x03]);
    }
}
