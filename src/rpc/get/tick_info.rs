use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::RpcClient;

const PATH_TICK_INFO: &str = "tick-info";

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct TickInfo {
    pub tick: u32,
    pub duration: u32,
    pub epoch: u32,
    #[serde(rename = "initialTick")]
    pub initial_tick: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct TickInfoResponse {
    #[serde(rename = "tickInfo")]
    pub tick_info: TickInfo,
}

pub async fn get_tick_info() -> Result<TickInfoResponse> {
    let client = RpcClient::new();
    get_tick_info_with(&client).await
}

pub async fn get_tick_info_with(client: &RpcClient) -> Result<TickInfoResponse> {
    client.get_json(PATH_TICK_INFO).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tick_info_response() {
        let json = r#"
        {
          "tickInfo": {
            "tick": 123,
            "duration": 10,
            "epoch": 42,
            "initialTick": 7
          }
        }
        "#;

        let parsed: TickInfoResponse = serde_json::from_str(json).expect("deserialize tick-info");
        assert_eq!(parsed.tick_info.tick, 123);
        assert_eq!(parsed.tick_info.duration, 10);
        assert_eq!(parsed.tick_info.epoch, 42);
        assert_eq!(parsed.tick_info.initial_tick, 7);
    }
}
