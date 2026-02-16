use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::RpcClient;

const PATH_BALANCES: &str = "balances";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BalanceInfo {
    pub id: String,
    pub balance: String,
    #[serde(rename = "validForTick")]
    pub valid_for_tick: u64,
    #[serde(rename = "latestIncomingTransferTick")]
    pub latest_incoming_transfer_tick: u64,
    #[serde(rename = "latestOutgoingTransferTick")]
    pub latest_outgoing_transfer_tick: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BalanceResponse {
    pub balance: BalanceInfo,
}

pub async fn get_balance(identity: &str) -> Result<BalanceResponse> {
    let client = RpcClient::new();
    get_balance_with(&client, identity).await
}

pub async fn get_balance_with(client: &RpcClient, identity: &str) -> Result<BalanceResponse> {
    let path = format!("{}/{}", PATH_BALANCES, identity);
    client.get_json(&path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_balance_response() {
        let json = r#"
        {
          "balance": {
            "id": "QUBICIDENTITYEXAMPLE",
            "balance": "1000000",
            "validForTick": 123456,
            "latestIncomingTransferTick": 123450,
            "latestOutgoingTransferTick": 123440
          }
        }
        "#;

        let parsed: BalanceResponse = serde_json::from_str(json).expect("deserialize balances");
        assert_eq!(parsed.balance.id, "QUBICIDENTITYEXAMPLE");
        assert_eq!(parsed.balance.balance, "1000000");
        assert_eq!(parsed.balance.valid_for_tick, 123456);
        assert_eq!(parsed.balance.latest_incoming_transfer_tick, 123450);
        assert_eq!(parsed.balance.latest_outgoing_transfer_tick, 123440);
    }
}
