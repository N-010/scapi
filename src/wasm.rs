use crate::{
    query_smart_contract, query_smart_contract_with_meta, Endianness, RequestDataBuilder,
    ResponseDecoder,
};
use js_sys::{Promise, Uint8Array};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use qrcode::{render::svg, QrCode};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;

fn anyhow_to_js(err: anyhow::Error) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn into_js_value<T: serde::Serialize>(value: T) -> std::result::Result<JsValue, JsValue> {
    value
        .serialize(&Serializer::json_compatible())
        .map_err(|err| JsValue::from_str(&err.to_string()))
}

fn vec_to_uint8array(bytes: Vec<u8>) -> JsValue {
    Uint8Array::from(bytes.as_slice()).into()
}

#[wasm_bindgen(js_name = RequestDataBuilder)]
pub struct RequestDataBuilderHandle {
    inner: RequestDataBuilder,
}

#[wasm_bindgen(js_class = RequestDataBuilder)]
impl RequestDataBuilderHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RequestDataBuilderHandle {
        Self {
            inner: RequestDataBuilder::new(),
        }
    }

    pub fn with_endianness(self, little_endian: bool) -> RequestDataBuilderHandle {
        let endianness = if little_endian {
            Endianness::Little
        } else {
            Endianness::Big
        };
        Self {
            inner: self.inner.with_endianness(endianness),
        }
    }

    pub fn add_bytes(mut self, bytes: Vec<u8>) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_bytes(&bytes);
        self
    }

    pub fn set_contract_index(mut self, index: u32) -> RequestDataBuilderHandle {
        self.inner = self.inner.set_contract_index(index);
        self
    }

    pub fn set_input_type(mut self, input_type: u32) -> RequestDataBuilderHandle {
        self.inner = self.inner.set_input_type(input_type);
        self
    }

    pub fn add_bool(mut self, value: bool) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_bool(value);
        self
    }

    pub fn add_int8(mut self, value: i8) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_int8(value);
        self
    }

    pub fn add_uint8(mut self, value: u8) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_uint8(value);
        self
    }

    pub fn add_int16(mut self, value: i16) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_int16(value);
        self
    }

    pub fn add_uint16(mut self, value: u16) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_uint16(value);
        self
    }

    pub fn add_int32(mut self, value: i32) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_int32(value);
        self
    }

    pub fn add_uint32(mut self, value: u32) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_uint32(value);
        self
    }

    pub fn add_int64(mut self, value: i64) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_int64(value);
        self
    }

    pub fn add_uint64(mut self, value: u64) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_uint64(value);
        self
    }

    pub fn add_float(mut self, value: f32) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_float(value);
        self
    }

    pub fn add_double(mut self, value: f64) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_double(value);
        self
    }

    pub fn add_char(mut self, value: u8) -> RequestDataBuilderHandle {
        self.inner = self.inner.add_char(value);
        self
    }

    pub fn add_m256i_bytes(self, bytes32: Vec<u8>) -> Result<RequestDataBuilderHandle, JsValue> {
        if bytes32.len() != 32 {
            return Err(JsValue::from_str(
                "add_m256i_bytes expects exactly 32 bytes",
            ));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes32);
        Ok(RequestDataBuilderHandle {
            inner: self.inner.add_m256i_bytes(arr),
        })
    }

    pub fn add_m256i_from_u64x4(
        self,
        w0: u64,
        w1: u64,
        w2: u64,
        w3: u64,
    ) -> RequestDataBuilderHandle {
        RequestDataBuilderHandle {
            inner: self.inner.add_m256i_from_u64x4(w0, w1, w2, w3),
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.inner.clone().to_bytes()
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }

    pub fn send(self) -> Promise {
        let builder = self.inner;
        future_to_promise(async move {
            match builder.send().await {
                Ok(bytes) => Ok(vec_to_uint8array(bytes)),
                Err(err) => Err(anyhow_to_js(err)),
            }
        })
    }
}

#[wasm_bindgen]
pub fn query_smart_contract_with_meta_async(
    contract_index: u32,
    input_type: u32,
    request_bytes: Vec<u8>,
) -> Promise {
    future_to_promise(async move {
        match query_smart_contract_with_meta(contract_index, input_type, &request_bytes).await {
            Ok(bytes) => Ok(vec_to_uint8array(bytes)),
            Err(err) => Err(anyhow_to_js(err)),
        }
    })
}

#[wasm_bindgen]
pub fn query_smart_contract_async(request_bytes: Vec<u8>) -> Promise {
    future_to_promise(async move {
        match query_smart_contract(&request_bytes).await {
            Ok(bytes) => Ok(vec_to_uint8array(bytes)),
            Err(err) => Err(anyhow_to_js(err)),
        }
    })
}

#[wasm_bindgen(js_name = generateQRCode)]
pub fn generate_qr_code(uri: String) -> Result<String, JsValue> {
    let code = QrCode::new(uri.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("QR encode error: {}", e)))?;
    let svg = code
        .render::<svg::Color>()
        .min_dimensions(256, 256)
        .build();
    let encoded = BASE64_STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", encoded))
}

#[wasm_bindgen(js_name = ResponseDecoder)]
pub struct ResponseDecoderHandle {
    data: Box<[u8]>,
    decoder: ResponseDecoder<'static>,
}

impl ResponseDecoderHandle {
    fn new_internal(bytes: Vec<u8>) -> ResponseDecoderHandle {
        let data = bytes.into_boxed_slice();
        let slice: &'static [u8] = unsafe { std::slice::from_raw_parts(data.as_ptr(), data.len()) };
        let decoder = ResponseDecoder::new(slice);
        ResponseDecoderHandle { data, decoder }
    }

    fn map_result(
        data: Box<[u8]>,
        result: anyhow::Result<ResponseDecoder<'static>>,
    ) -> Result<ResponseDecoderHandle, JsValue> {
        result
            .map(|decoder| ResponseDecoderHandle { data, decoder })
            .map_err(anyhow_to_js)
    }
}

#[wasm_bindgen(js_class = ResponseDecoder)]
impl ResponseDecoderHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> ResponseDecoderHandle {
        ResponseDecoderHandle::new_internal(bytes)
    }

    pub fn with_endianness(self, little_endian: bool) -> ResponseDecoderHandle {
        let endianness = if little_endian {
            Endianness::Little
        } else {
            Endianness::Big
        };
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle {
            data,
            decoder: decoder.with_endianness(endianness),
        }
    }

    pub fn u8(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.u8(field))
    }

    pub fn array_u8(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_u8(field, count))
    }

    pub fn i8(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.i8(field))
    }

    pub fn array_i8(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_i8(field, count))
    }

    pub fn u16(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.u16(field))
    }

    pub fn array_u16(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_u16(field, count))
    }

    pub fn i16(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.i16(field))
    }

    pub fn array_i16(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_i16(field, count))
    }

    pub fn u32(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.u32(field))
    }

    pub fn array_u32(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_u32(field, count))
    }

    pub fn i32(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.i32(field))
    }

    pub fn array_i32(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_i32(field, count))
    }

    pub fn u64(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.u64(field))
    }

    pub fn array_u64(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_u64(field, count))
    }

    pub fn uint64(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        self.u64(field)
    }

    pub fn i64(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.i64(field))
    }

    pub fn array_i64(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_i64(field, count))
    }

    pub fn f32(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.f32(field))
    }

    pub fn array_f32(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_f32(field, count))
    }

    pub fn f64(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.f64(field))
    }

    pub fn array_f64(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_f64(field, count))
    }

    pub fn bytes(self, field: &str, len: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.bytes(field, len))
    }

    pub fn m256i(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.m256i(field))
    }

    pub fn array_m256i(self, field: &str, count: usize) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.array_m256i(field, count))
    }

    pub fn remaining_bytes(self, field: &str) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(data, decoder.remaining_bytes(field))
    }

    pub fn array_struct_bytes(
        self,
        field: &str,
        count: usize,
        struct_size: usize,
    ) -> Result<ResponseDecoderHandle, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        ResponseDecoderHandle::map_result(
            data,
            decoder.array_struct_bytes(field, count, struct_size),
        )
    }

    pub fn to_value(self) -> Result<JsValue, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        into_js_value(decoder.to_value()).map(|js| {
            drop(data);
            js
        })
    }

    pub fn to_json_string(self) -> Result<String, JsValue> {
        let ResponseDecoderHandle { data, decoder } = self;
        let result = decoder.to_json_string().map_err(anyhow_to_js)?;
        drop(data);
        Ok(result)
    }
}

#[wasm_bindgen]
pub fn decode_response_to_js(bytes: Vec<u8>) -> Result<JsValue, JsValue> {
    ResponseDecoderHandle::new(bytes).to_value()
}
