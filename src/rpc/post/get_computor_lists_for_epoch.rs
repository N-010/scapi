use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_COMPUTOR_LISTS_FOR_EPOCH: &str = "getComputorListsForEpoch";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComputorList {
    pub epoch: u32,
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
    pub identities: Vec<String>,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct GetComputorListsForEpochRequest {
    pub epoch: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetComputorListsForEpochResponse {
    #[serde(rename = "computorsLists")]
    pub computors_lists: Vec<ComputorList>,
}

pub async fn get_computor_lists_for_epoch(epoch: u32) -> Result<GetComputorListsForEpochResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_computor_lists_for_epoch_with(&client, epoch).await
}

pub async fn get_computor_lists_for_epoch_with(
    client: &RpcClient,
    epoch: u32,
) -> Result<GetComputorListsForEpochResponse> {
    let payload = GetComputorListsForEpochRequest { epoch };
    client
        .post_json(PATH_GET_COMPUTOR_LISTS_FOR_EPOCH, &payload)
        .await
}
