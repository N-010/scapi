use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_PROCESSED_TICK_INTERVALS: &str = "getProcessedTickIntervals";

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ProcessedTickInterval {
    pub epoch: u32,
    #[serde(rename = "firstTick")]
    pub first_tick: u32,
    #[serde(rename = "lastTick")]
    pub last_tick: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetProcessedTickIntervalsResponse {
    #[serde(rename = "processedTickIntervals")]
    pub processed_tick_intervals: Vec<ProcessedTickInterval>,
}

pub async fn get_processed_tick_intervals() -> Result<GetProcessedTickIntervalsResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_processed_tick_intervals_with(&client).await
}

pub async fn get_processed_tick_intervals_with(
    client: &RpcClient,
) -> Result<GetProcessedTickIntervalsResponse> {
    client.get_json(PATH_GET_PROCESSED_TICK_INTERVALS).await
}
