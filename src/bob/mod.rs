use crate::rpc::RpcClient;
use anyhow::Result;
use serde_json::{json, Number, Value};
use std::sync::atomic::{AtomicU64, Ordering};

pub const DEFAULT_BOB_RPC_ENDPOINT: &str = "http://localhost:40420/qubic";

#[derive(Debug)]
pub struct BobRpcClient {
    endpoint_url: String,
    rpc: RpcClient,
    next_id: AtomicU64,
}

impl BobRpcClient {
    pub fn new(endpoint_url: impl Into<String>) -> Self {
        Self {
            endpoint_url: endpoint_url.into(),
            rpc: RpcClient::new(),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn with_base_url(base_url: impl AsRef<str>) -> Self {
        let mut base = base_url.as_ref().trim_end_matches('/').to_string();
        base.push_str("/qubic");
        Self::new(base)
    }

    pub fn endpoint_url(&self) -> &str {
        &self.endpoint_url
    }

    pub fn set_endpoint_url(&mut self, endpoint_url: impl Into<String>) {
        self.endpoint_url = endpoint_url.into();
    }

    pub async fn call(&self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });
        self.rpc
            .post_json_value_url(&self.endpoint_url, &payload)
            .await
    }

    pub async fn qubic_chain_id(&self) -> Result<Value> {
        self.call("qubic_chainId", Value::Array(vec![])).await
    }

    pub async fn qubic_client_version(&self) -> Result<Value> {
        self.call("qubic_clientVersion", Value::Array(vec![])).await
    }

    pub async fn qubic_syncing(&self) -> Result<Value> {
        self.call("qubic_syncing", Value::Array(vec![])).await
    }

    pub async fn qubic_status(&self) -> Result<Value> {
        self.call("qubic_status", Value::Array(vec![])).await
    }

    pub async fn qubic_get_current_epoch(&self) -> Result<Value> {
        self.call("qubic_getCurrentEpoch", Value::Array(vec![]))
            .await
    }

    pub async fn qubic_get_tick_number(&self) -> Result<Value> {
        self.call("qubic_getTickNumber", Value::Array(vec![])).await
    }

    pub async fn qubic_get_tick_by_number(
        &self,
        tick_number_or_tag: impl Into<String>,
        include_transactions: bool,
    ) -> Result<Value> {
        self.call(
            "qubic_getTickByNumber",
            Value::Array(vec![
                Value::String(tick_number_or_tag.into()),
                Value::Bool(include_transactions),
            ]),
        )
        .await
    }

    pub async fn qubic_get_transaction_by_hash(&self, tx_hash: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_getTransactionByHash",
            Value::Array(vec![Value::String(tx_hash.into())]),
        )
        .await
    }

    pub async fn qubic_get_transaction_receipt(&self, tx_hash: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_getTransactionReceipt",
            Value::Array(vec![Value::String(tx_hash.into())]),
        )
        .await
    }

    pub async fn qubic_broadcast_transaction(&self, signed_tx: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_broadcastTransaction",
            Value::Array(vec![Value::String(signed_tx.into())]),
        )
        .await
    }

    pub async fn qubic_send_raw_transaction(&self, signed_tx: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_sendRawTransaction",
            Value::Array(vec![Value::String(signed_tx.into())]),
        )
        .await
    }

    pub async fn qubic_get_balance(&self, identity: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_getBalance",
            Value::Array(vec![Value::String(identity.into())]),
        )
        .await
    }

    pub async fn qubic_get_transfers(&self, filter: Value) -> Result<Value> {
        self.call("qubic_getTransfers", Value::Array(vec![filter]))
            .await
    }

    pub async fn qubic_get_asset_balance(
        &self,
        identity: impl Into<String>,
        issuer: impl Into<String>,
        asset_name: impl Into<String>,
    ) -> Result<Value> {
        self.call(
            "qubic_getAssetBalance",
            Value::Array(vec![
                Value::String(identity.into()),
                Value::String(issuer.into()),
                Value::String(asset_name.into()),
            ]),
        )
        .await
    }

    pub async fn qubic_get_assets(&self, identity: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_getAssets",
            Value::Array(vec![Value::String(identity.into())]),
        )
        .await
    }

    pub async fn qubic_get_epoch_info(&self, epoch: u32) -> Result<Value> {
        self.call(
            "qubic_getEpochInfo",
            Value::Array(vec![Value::Number(Number::from(epoch))]),
        )
        .await
    }

    pub async fn qubic_get_end_epoch_logs(&self, epoch: u32) -> Result<Value> {
        self.call(
            "qubic_getEndEpochLogs",
            Value::Array(vec![Value::Number(Number::from(epoch))]),
        )
        .await
    }

    pub async fn qubic_get_logs(&self, filter: Value) -> Result<Value> {
        self.call("qubic_getLogs", Value::Array(vec![filter])).await
    }

    pub async fn qubic_find_log_ids(&self, filter: Value) -> Result<Value> {
        self.call("qubic_findLogIds", Value::Array(vec![filter]))
            .await
    }

    pub async fn qubic_get_logs_by_id_range(
        &self,
        epoch: u32,
        from_id: u32,
        to_id: u32,
    ) -> Result<Value> {
        self.call(
            "qubic_getLogsByIdRange",
            Value::Array(vec![
                Value::Number(Number::from(epoch)),
                Value::Number(Number::from(from_id)),
                Value::Number(Number::from(to_id)),
            ]),
        )
        .await
    }

    pub async fn qubic_get_qu_transfers(&self, filter: Value) -> Result<Value> {
        self.call("qubic_getQuTransfers", Value::Array(vec![filter]))
            .await
    }

    pub async fn qubic_get_asset_transfers(&self, filter: Value) -> Result<Value> {
        self.call("qubic_getAssetTransfers", Value::Array(vec![filter]))
            .await
    }

    pub async fn qubic_get_all_asset_transfers(&self, filter: Value) -> Result<Value> {
        self.call("qubic_getAllAssetTransfers", Value::Array(vec![filter]))
            .await
    }

    pub async fn qubic_subscribe(
        &self,
        subscription_type: impl Into<String>,
        filter_params: Value,
    ) -> Result<Value> {
        self.call(
            "qubic_subscribe",
            Value::Array(vec![Value::String(subscription_type.into()), filter_params]),
        )
        .await
    }

    pub async fn qubic_unsubscribe(&self, subscription_id: impl Into<String>) -> Result<Value> {
        self.call(
            "qubic_unsubscribe",
            Value::Array(vec![Value::String(subscription_id.into())]),
        )
        .await
    }
}
