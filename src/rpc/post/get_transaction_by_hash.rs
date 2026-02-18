use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::rpc::get::Transaction;
use crate::rpc::{query_base_url, RpcClient};

const PATH_GET_TRANSACTION_BY_HASH: &str = "getTransactionByHash";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTransactionByHashRequest {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetTransactionByHashResponse {
    pub transaction: Option<Transaction>,
}

pub async fn get_transaction_by_hash(hash: &str) -> Result<GetTransactionByHashResponse> {
    let client = RpcClient::with_base_url(query_base_url());
    get_transaction_by_hash_with(&client, hash).await
}

pub async fn get_transaction_by_hash_with(
    client: &RpcClient,
    hash: &str,
) -> Result<GetTransactionByHashResponse> {
    let payload = GetTransactionByHashRequest {
        hash: hash.to_string(),
    };
    client
        .post_json(PATH_GET_TRANSACTION_BY_HASH, &payload)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_get_transaction_by_hash_response() {
        let json = r#"
        {
          "transaction": {
            "hash": "abc123",
            "amount": "1000",
            "source": "SRC",
            "destination": "DST",
            "tickNumber": 42,
            "timestamp": "1757376000000",
            "inputType": 0,
            "inputSize": 4,
            "inputData": "AQIDBA==",
            "signature": "SGVsbG8=",
            "moneyFlew": true
          }
        }
        "#;

        let parsed: GetTransactionByHashResponse =
            serde_json::from_str(json).expect("deserialize getTransactionByHash response");
        let transaction = parsed.transaction.expect("transaction");
        assert_eq!(transaction.hash, "abc123");
        assert_eq!(transaction.tick_number, 42);
        assert_eq!(transaction.input_type, 0);
        assert_eq!(transaction.money_flew, Some(true));
    }
}
