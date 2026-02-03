use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::{query_base_url, RpcClient};

const PATH_TICK_DATA: &str = "getTickData";

#[derive(Debug, Serialize, Clone, Copy)]
pub struct GetTickDataRequest {
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TickData {
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
    pub epoch: u32,
    #[serde(rename = "computorIndex")]
    pub computor_index: u32,
    pub timestamp: String,
    #[serde(rename = "varStruct")]
    pub var_struct: String,
    #[serde(rename = "timeLock")]
    pub time_lock: String,
    #[serde(rename = "transactionHashes")]
    pub transaction_hashes: Vec<String>,
    #[serde(rename = "contractFees")]
    pub contract_fees: Vec<String>,
    pub signature: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetTickDataResponse {
    #[serde(rename = "tickData")]
    pub tick_data: Option<TickData>,
}

pub async fn get_tick_data(tick: u32) -> Result<GetTickDataResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_tick_data_with(&client, tick).await
}

pub async fn get_tick_data_with(client: &RpcClient, tick: u32) -> Result<GetTickDataResponse> {
    let payload = GetTickDataRequest { tick_number: tick };
    client.post_json(PATH_TICK_DATA, &payload).await
}
