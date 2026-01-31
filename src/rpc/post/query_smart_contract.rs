use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::env;

use crate::rpc::{base_url, join_url, QueryResponsePossible, RpcClient};

const PATH_QUERY_SMART_CONTRACT: &str = "querySmartContract";

#[derive(Debug, Serialize)]
struct QueryRequest<'a> {
    #[serde(rename = "contractIndex")]
    contract_index: u32,
    #[serde(rename = "inputType")]
    input_type: u32,
    #[serde(rename = "inputSize")]
    input_size: u32,
    #[serde(rename = "requestData")]
    request_data: &'a str,
}

fn query_smart_contract_url() -> Cow<'static, str> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env::var("QUBIC_RPC_QUERY_SMART_CONTRACT")
            .map(Cow::Owned)
            .unwrap_or_else(|_| Cow::Owned(join_url(base_url().as_ref(), PATH_QUERY_SMART_CONTRACT)))
    }
    #[cfg(target_arch = "wasm32")]
    {
        option_env!("QUBIC_RPC_QUERY_SMART_CONTRACT").map(Cow::Borrowed).unwrap_or_else(|| {
            Cow::Owned(join_url(base_url().as_ref(), PATH_QUERY_SMART_CONTRACT))
        })
    }
}

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
    let encoded = STANDARD.encode(request_bytes);
    let payload = QueryRequest {
        contract_index,
        input_type,
        input_size: request_bytes.len() as u32,
        request_data: &encoded,
    };

    let client = RpcClient::new();
    let bytes = client
        .post_json_bytes_url(query_smart_contract_url().as_ref(), &payload)
        .await?;

    let parsed: Result<QueryResponsePossible> = serde_json::from_slice(&bytes).map_err(Into::into);
    if let Ok(body) = parsed {
        if let Some(b64) = body.response_data.or(body.data) {
            let decoded = STANDARD
                .decode(b64.as_bytes())
                .map_err(|e| anyhow!("Failed to decode base64 response: {}", e))?;
            return Ok(decoded);
        }
        if let Some(Value::String(s)) = body.result {
            if let Ok(decoded) = STANDARD.decode(s.as_bytes()) {
                return Ok(decoded);
            }
        }
        return Ok(bytes);
    }

    Ok(bytes)
}

/// Backward-compatible helper that sends without metadata.
/// Note: the API expects contractIndex/inputType/inputSize; this uses zeros which may be rejected.
pub async fn query_smart_contract(request_bytes: &[u8]) -> Result<Vec<u8>> {
    query_smart_contract_with_meta(0, 0, request_bytes).await
}

#[cfg(test)]
mod tests {
    use super::QueryRequest;

    #[test]
    fn serialize_query_request() {
        let req = QueryRequest {
            contract_index: 1,
            input_type: 2,
            input_size: 3,
            request_data: "AQID",
        };

        let json = serde_json::to_string(&req).expect("serialize query request");
        assert!(json.contains("\"contractIndex\":1"));
        assert!(json.contains("\"inputType\":2"));
        assert!(json.contains("\"inputSize\":3"));
        assert!(json.contains("\"requestData\":\"AQID\""));
    }
}
