use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::rpc::get::{Hits, Pagination, Range, Transaction};
use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_TRANSACTIONS_FOR_IDENTITY: &str = "getTransactionsForIdentity";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GetTransactionsForIdentityRequest {
    pub identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<HashMap<String, Range>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTransactionsForIdentityResponse {
    #[serde(rename = "validForTick")]
    pub valid_for_tick: u32,
    pub hits: Hits,
    pub transactions: Vec<Transaction>,
}

pub async fn get_transactions_for_identity(
    request: &GetTransactionsForIdentityRequest,
) -> Result<GetTransactionsForIdentityResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_transactions_for_identity_with(&client, request).await
}

pub async fn get_transactions_for_identity_with(
    client: &RpcClient,
    request: &GetTransactionsForIdentityRequest,
) -> Result<GetTransactionsForIdentityResponse> {
    client
        .post_json(PATH_GET_TRANSACTIONS_FOR_IDENTITY, request)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_get_transactions_for_identity_request() {
        let mut filters = HashMap::new();
        filters.insert(
            "destination".to_string(),
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFXIB".to_string(),
        );
        filters.insert("inputType".to_string(), "0".to_string());

        let mut ranges = HashMap::new();
        ranges.insert(
            "amount".to_string(),
            Range {
                gte: Some("1000000000".to_string()),
                ..Range::default()
            },
        );

        let request = GetTransactionsForIdentityRequest {
            identity: "AFZPUAIYVPNUYGJRQVLUKOPPVLHAZQTGLYAAUUNBXFTVTAMSBKQBLEIEPCVJ".to_string(),
            filters: Some(filters),
            ranges: Some(ranges),
            pagination: Some(Pagination {
                offset: Some(0),
                size: Some(10),
            }),
        };

        let json = serde_json::to_string(&request).expect("serialize identity request");
        assert!(json.contains(
            "\"identity\":\"AFZPUAIYVPNUYGJRQVLUKOPPVLHAZQTGLYAAUUNBXFTVTAMSBKQBLEIEPCVJ\""
        ));
        assert!(json.contains("\"filters\""));
        assert!(json.contains("\"ranges\""));
        assert!(json.contains("\"pagination\""));
    }
}
