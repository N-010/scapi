use crate::bob::DEFAULT_BOB_RPC_ENDPOINT;
use crate::rpc::{set_default_qubic_rpc_query, set_default_qubic_rpc_query_services, RpcClient};
use crate::{
    query_smart_contract, query_smart_contract_with_meta, Endianness, RequestDataBuilder,
    ResponseDecoder,
};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use js_sys::{Promise, Uint8Array};
use qrcode::{render::svg, QrCode};
use serde_json::{json, Number, Value};
use serde_wasm_bindgen::Serializer;
use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

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

fn js_to_value(value: JsValue) -> Result<Value, JsValue> {
    serde_wasm_bindgen::from_value(value).map_err(|err| JsValue::from_str(&err.to_string()))
}

fn params_to_js(params: Vec<Value>) -> JsValue {
    into_js_value(params).unwrap_or_else(|_| js_sys::Array::new().into())
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
    let svg = code.render::<svg::Color>().min_dimensions(256, 256).build();
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

#[wasm_bindgen(js_name = setDefaultRpcBaseUrl)]
pub fn set_default_rpc_base_url(url: String) {
    set_default_qubic_rpc_query(url);
}

#[wasm_bindgen(js_name = setDefaultRpcQueryBaseUrl)]
pub fn set_default_rpc_query_base_url(url: String) {
    set_default_qubic_rpc_query_services(url);
}

#[wasm_bindgen(js_name = RpcHttpClient)]
pub struct RpcHttpClient {
    base_url: String,
    client: RpcClient,
}

#[wasm_bindgen(js_name = getTickInfo)]
pub fn get_tick_info_async() -> Promise {
    future_to_promise(async move {
        let value = crate::rpc::get::get_tick_info()
            .await
            .map_err(anyhow_to_js)?;
        into_js_value(value)
    })
}

#[wasm_bindgen(js_name = getBalance)]
pub fn get_balance_async(identity: String) -> Promise {
    future_to_promise(async move {
        let value = crate::rpc::get::get_balance(&identity)
            .await
            .map_err(anyhow_to_js)?;
        into_js_value(value)
    })
}

#[wasm_bindgen(js_name = getTickData)]
pub fn get_tick_data_async(tick: u32) -> Promise {
    future_to_promise(async move {
        let value = crate::rpc::get::get_tick_data(tick)
            .await
            .map_err(anyhow_to_js)?;
        into_js_value(value)
    })
}

#[wasm_bindgen(js_name = broadcastTransaction)]
pub fn broadcast_transaction_async(encoded_transaction: String) -> Promise {
    future_to_promise(async move {
        let value = crate::rpc::post::broadcast_transaction(encoded_transaction)
            .await
            .map_err(anyhow_to_js)?;
        into_js_value(value)
    })
}

#[wasm_bindgen(js_name = broadcastTransactionBytes)]
pub fn broadcast_transaction_bytes_async(tx_bytes: Vec<u8>) -> Promise {
    future_to_promise(async move {
        let value = crate::rpc::post::broadcast_transaction_bytes(&tx_bytes)
            .await
            .map_err(anyhow_to_js)?;
        into_js_value(value)
    })
}

#[wasm_bindgen(js_name = BobRpcClient)]
pub struct BobRpcClientHandle {
    endpoint_url: String,
    rpc: RpcClient,
    next_id: Arc<AtomicU64>,
}

#[wasm_bindgen(js_class = BobRpcClient)]
impl BobRpcClientHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(endpoint_url: Option<String>) -> BobRpcClientHandle {
        let endpoint_url = endpoint_url.unwrap_or_else(|| DEFAULT_BOB_RPC_ENDPOINT.to_string());
        BobRpcClientHandle {
            endpoint_url,
            rpc: RpcClient::new(),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    #[wasm_bindgen(js_name = withBaseUrl)]
    pub fn with_base_url(base_url: String) -> BobRpcClientHandle {
        let endpoint_url = format!("{}/qubic", base_url.trim_end_matches('/'));
        BobRpcClientHandle::new(Some(endpoint_url))
    }

    #[wasm_bindgen(js_name = endpointUrl)]
    pub fn endpoint_url(&self) -> String {
        self.endpoint_url.clone()
    }

    #[wasm_bindgen(js_name = setEndpointUrl)]
    pub fn set_endpoint_url(&mut self, endpoint_url: String) {
        self.endpoint_url = endpoint_url;
    }

    pub fn call(&self, method: String, params: JsValue) -> Promise {
        let payload = js_to_value(params);
        let endpoint_url = self.endpoint_url.clone();
        let rpc = self.rpc.clone();
        let next_id = Arc::clone(&self.next_id);
        future_to_promise(async move {
            let params = payload?;
            let id = next_id.fetch_add(1, Ordering::Relaxed);
            let request = json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
                "id": id
            });
            let value = rpc
                .post_json_value_url(&endpoint_url, &request)
                .await
                .map_err(anyhow_to_js)?;
            into_js_value(value)
        })
    }

    #[wasm_bindgen(js_name = qubicChainId)]
    pub fn qubic_chain_id(&self) -> Promise {
        self.call("qubic_chainId".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicClientVersion)]
    pub fn qubic_client_version(&self) -> Promise {
        self.call("qubic_clientVersion".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicSyncing)]
    pub fn qubic_syncing(&self) -> Promise {
        self.call("qubic_syncing".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicStatus)]
    pub fn qubic_status(&self) -> Promise {
        self.call("qubic_status".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicGetCurrentEpoch)]
    pub fn qubic_get_current_epoch(&self) -> Promise {
        self.call("qubic_getCurrentEpoch".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicGetTickNumber)]
    pub fn qubic_get_tick_number(&self) -> Promise {
        self.call("qubic_getTickNumber".to_string(), params_to_js(vec![]))
    }

    #[wasm_bindgen(js_name = qubicGetTickByNumber)]
    pub fn qubic_get_tick_by_number(
        &self,
        tick_number_or_tag: String,
        include_transactions: bool,
    ) -> Promise {
        self.call(
            "qubic_getTickByNumber".to_string(),
            into_js_value(vec![
                Value::String(tick_number_or_tag),
                Value::Bool(include_transactions),
            ])
            .unwrap_or_else(|_| js_sys::Array::new().into()),
        )
    }

    #[wasm_bindgen(js_name = qubicGetTransactionByHash)]
    pub fn qubic_get_transaction_by_hash(&self, tx_hash: String) -> Promise {
        self.call(
            "qubic_getTransactionByHash".to_string(),
            params_to_js(vec![Value::String(tx_hash)]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetTransactionReceipt)]
    pub fn qubic_get_transaction_receipt(&self, tx_hash: String) -> Promise {
        self.call(
            "qubic_getTransactionReceipt".to_string(),
            params_to_js(vec![Value::String(tx_hash)]),
        )
    }

    #[wasm_bindgen(js_name = qubicBroadcastTransaction)]
    pub fn qubic_broadcast_transaction(&self, signed_tx: String) -> Promise {
        self.call(
            "qubic_broadcastTransaction".to_string(),
            params_to_js(vec![Value::String(signed_tx)]),
        )
    }

    #[wasm_bindgen(js_name = qubicSendRawTransaction)]
    pub fn qubic_send_raw_transaction(&self, signed_tx: String) -> Promise {
        self.call(
            "qubic_sendRawTransaction".to_string(),
            params_to_js(vec![Value::String(signed_tx)]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetBalance)]
    pub fn qubic_get_balance(&self, identity: String) -> Promise {
        self.call(
            "qubic_getBalance".to_string(),
            params_to_js(vec![Value::String(identity)]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetTransfers)]
    pub fn qubic_get_transfers(&self, filter: JsValue) -> Promise {
        self.call("qubic_getTransfers".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicGetAssetBalance)]
    pub fn qubic_get_asset_balance(
        &self,
        identity: String,
        issuer: String,
        asset_name: String,
    ) -> Promise {
        self.call(
            "qubic_getAssetBalance".to_string(),
            into_js_value(vec![
                Value::String(identity),
                Value::String(issuer),
                Value::String(asset_name),
            ])
            .unwrap_or_else(|_| js_sys::Array::new().into()),
        )
    }

    #[wasm_bindgen(js_name = qubicGetAssets)]
    pub fn qubic_get_assets(&self, identity: String) -> Promise {
        self.call(
            "qubic_getAssets".to_string(),
            params_to_js(vec![Value::String(identity)]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetEpochInfo)]
    pub fn qubic_get_epoch_info(&self, epoch: u32) -> Promise {
        self.call(
            "qubic_getEpochInfo".to_string(),
            params_to_js(vec![Value::Number(Number::from(epoch))]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetEndEpochLogs)]
    pub fn qubic_get_end_epoch_logs(&self, epoch: u32) -> Promise {
        self.call(
            "qubic_getEndEpochLogs".to_string(),
            params_to_js(vec![Value::Number(Number::from(epoch))]),
        )
    }

    #[wasm_bindgen(js_name = qubicGetLogs)]
    pub fn qubic_get_logs(&self, filter: JsValue) -> Promise {
        self.call("qubic_getLogs".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicFindLogIds)]
    pub fn qubic_find_log_ids(&self, filter: JsValue) -> Promise {
        self.call("qubic_findLogIds".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicGetLogsByIdRange)]
    pub fn qubic_get_logs_by_id_range(&self, epoch: u32, from_id: u32, to_id: u32) -> Promise {
        self.call(
            "qubic_getLogsByIdRange".to_string(),
            into_js_value(vec![
                Value::Number(Number::from(epoch)),
                Value::Number(Number::from(from_id)),
                Value::Number(Number::from(to_id)),
            ])
            .unwrap_or_else(|_| js_sys::Array::new().into()),
        )
    }

    #[wasm_bindgen(js_name = qubicGetQuTransfers)]
    pub fn qubic_get_qu_transfers(&self, filter: JsValue) -> Promise {
        self.call("qubic_getQuTransfers".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicGetAssetTransfers)]
    pub fn qubic_get_asset_transfers(&self, filter: JsValue) -> Promise {
        self.call("qubic_getAssetTransfers".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicGetAllAssetTransfers)]
    pub fn qubic_get_all_asset_transfers(&self, filter: JsValue) -> Promise {
        self.call("qubic_getAllAssetTransfers".to_string(), filter)
    }

    #[wasm_bindgen(js_name = qubicSubscribe)]
    pub fn qubic_subscribe(&self, subscription_type: String, filter_params: JsValue) -> Promise {
        let params = js_sys::Array::new();
        params.push(&JsValue::from_str(&subscription_type));
        params.push(&filter_params);
        self.call("qubic_subscribe".to_string(), params.into())
    }

    #[wasm_bindgen(js_name = qubicUnsubscribe)]
    pub fn qubic_unsubscribe(&self, subscription_id: String) -> Promise {
        self.call(
            "qubic_unsubscribe".to_string(),
            params_to_js(vec![Value::String(subscription_id)]),
        )
    }
}

#[wasm_bindgen(js_class = RpcHttpClient)]
impl RpcHttpClient {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String) -> RpcHttpClient {
        let client = RpcClient::with_base_url(Cow::Owned(base_url.clone()));
        RpcHttpClient { base_url, client }
    }

    #[wasm_bindgen(js_name = baseUrl)]
    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    #[wasm_bindgen(js_name = setBaseUrl)]
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url.clone();
        self.client = RpcClient::with_base_url(Cow::Owned(base_url));
    }

    #[wasm_bindgen(js_name = getJson)]
    pub fn get_json(&self, path: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let value = client.get_json_value(&path).await.map_err(anyhow_to_js)?;
            into_js_value(value)
        })
    }

    #[wasm_bindgen(js_name = getJsonUrl)]
    pub fn get_json_url(&self, url: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let value = client
                .get_json_value_url(&url)
                .await
                .map_err(anyhow_to_js)?;
            into_js_value(value)
        })
    }

    #[wasm_bindgen(js_name = postJson)]
    pub fn post_json(&self, path: String, payload: JsValue) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let payload_value = js_to_value(payload)?;
            let value = client
                .post_json_value(&path, &payload_value)
                .await
                .map_err(anyhow_to_js)?;
            into_js_value(value)
        })
    }

    #[wasm_bindgen(js_name = postJsonUrl)]
    pub fn post_json_url(&self, url: String, payload: JsValue) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let payload_value = js_to_value(payload)?;
            let value = client
                .post_json_value_url(&url, &payload_value)
                .await
                .map_err(anyhow_to_js)?;
            into_js_value(value)
        })
    }

    #[wasm_bindgen(js_name = postJsonBytes)]
    pub fn post_json_bytes(&self, path: String, payload: JsValue) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let payload_value = js_to_value(payload)?;
            let bytes = client
                .post_json_bytes(&path, &payload_value)
                .await
                .map_err(anyhow_to_js)?;
            Ok(vec_to_uint8array(bytes))
        })
    }

    #[wasm_bindgen(js_name = postJsonBytesUrl)]
    pub fn post_json_bytes_url(&self, url: String, payload: JsValue) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let payload_value = js_to_value(payload)?;
            let bytes = client
                .post_json_bytes_url(&url, &payload_value)
                .await
                .map_err(anyhow_to_js)?;
            Ok(vec_to_uint8array(bytes))
        })
    }
}
