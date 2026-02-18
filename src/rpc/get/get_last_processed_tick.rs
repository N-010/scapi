use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_LAST_PROCESSED_TICK: &str = "getLastProcessedTick";

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GetLastProcessedTickResponse {
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
    pub epoch: u32,
    #[serde(rename = "intervalInitialTick")]
    pub interval_initial_tick: u32,
}

pub async fn get_last_processed_tick() -> Result<GetLastProcessedTickResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_last_processed_tick_with(&client).await
}

pub async fn get_last_processed_tick_with(
    client: &RpcClient,
) -> Result<GetLastProcessedTickResponse> {
    client.get_json(PATH_GET_LAST_PROCESSED_TICK).await
}
