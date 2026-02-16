use anyhow::Result;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::rpc::RpcClient;

const PATH_BROADCAST_TRANSACTION: &str = "broadcast-transaction";

#[derive(Debug, Serialize)]
pub struct BroadcastTransactionRequest {
    #[serde(rename = "encodedTransaction")]
    pub encoded_transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BroadcastTransactionResponse {
    #[serde(rename = "peersBroadcasted")]
    pub peers_broadcasted: i32,
    #[serde(rename = "encodedTransaction")]
    pub encoded_transaction: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

pub async fn broadcast_transaction(
    encoded_transaction: String,
) -> Result<BroadcastTransactionResponse> {
    let client = RpcClient::new();
    broadcast_transaction_with(&client, encoded_transaction).await
}

pub async fn broadcast_transaction_with(
    client: &RpcClient,
    encoded_transaction: String,
) -> Result<BroadcastTransactionResponse> {
    let payload = BroadcastTransactionRequest {
        encoded_transaction,
    };
    client.post_json(PATH_BROADCAST_TRANSACTION, &payload).await
}

pub async fn broadcast_transaction_bytes(tx_bytes: &[u8]) -> Result<BroadcastTransactionResponse> {
    let encoded = BASE64_STANDARD.encode(tx_bytes);
    broadcast_transaction(encoded).await
}
