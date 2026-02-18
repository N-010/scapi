use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::rpc::get::{Range, Transaction};
use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_TRANSACTIONS_FOR_TICK: &str = "getTransactionsForTick";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GetTransactionsForTickRequest {
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<HashMap<String, Range>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTransactionsForTickResponse {
    pub transactions: Vec<Transaction>,
}

pub async fn get_transactions_for_tick(
    request: &GetTransactionsForTickRequest,
) -> Result<GetTransactionsForTickResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_transactions_for_tick_with(&client, request).await
}

pub async fn get_transactions_for_tick_with(
    client: &RpcClient,
    request: &GetTransactionsForTickRequest,
) -> Result<GetTransactionsForTickResponse> {
    client
        .post_json(PATH_GET_TRANSACTIONS_FOR_TICK, request)
        .await
}
