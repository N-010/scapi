use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::env;
use std::sync::{OnceLock, RwLock};

pub mod get;
pub mod post;

pub use get::*;
pub use post::*;

pub const DEFAULT_QUBIC_RPC_QUERY: &str = "https://rpc.qubic.org/live/v1/";

static DEFAULT_RPC_BASE_URL_OVERRIDE: OnceLock<RwLock<Option<String>>> = OnceLock::new();

pub fn set_default_qubic_rpc_query<S: Into<String>>(base_url: S) {
    let storage = DEFAULT_RPC_BASE_URL_OVERRIDE.get_or_init(|| RwLock::new(None));
    let mut guard = storage.write().expect("default RPC base URL lock poisoned");
    *guard = Some(base_url.into());
}

fn default_qubic_rpc_query_override() -> Option<Cow<'static, str>> {
    DEFAULT_RPC_BASE_URL_OVERRIDE
        .get()
        .and_then(|lock| lock.read().ok().and_then(|guard| guard.clone()))
        .map(Cow::Owned)
}

fn qubic_rpc_base_url() -> Cow<'static, str> {
    if let Some(override_url) = default_qubic_rpc_query_override() {
        return override_url;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env::var("QUBIC_RPC_BASE_URL")
            .map(Cow::Owned)
            .unwrap_or_else(|_| Cow::Borrowed(DEFAULT_QUBIC_RPC_QUERY))
    }
    #[cfg(target_arch = "wasm32")]
    {
        option_env!("QUBIC_RPC_BASE_URL")
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Borrowed(DEFAULT_QUBIC_RPC_QUERY))
    }
}

pub(crate) fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

pub(crate) fn base_url() -> Cow<'static, str> {
    qubic_rpc_base_url()
}

#[derive(Clone, Debug)]
pub struct RpcClient {
    base_url: Cow<'static, str>,
    http: Client,
}

impl RpcClient {
    pub fn new() -> Self {
        Self::with_base_url(base_url())
    }

    pub fn with_base_url(base_url: Cow<'static, str>) -> Self {
        Self {
            base_url,
            http: Client::new(),
        }
    }

    fn url_for(&self, path: &str) -> String {
        join_url(self.base_url.as_ref(), path)
    }

    pub async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.url_for(path);
        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("RPC HTTP error: {} {}", status, body));
        }
        Ok(resp.json::<T>().await?)
    }

    pub async fn post_json<T, P>(&self, path: &str, payload: &P) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.url_for(path);
        let resp = self.http.post(url).json(payload).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("RPC HTTP error: {} {}", status, body));
        }
        Ok(resp.json::<T>().await?)
    }

    pub async fn post_json_bytes<P>(&self, path: &str, payload: &P) -> Result<Vec<u8>>
    where
        P: Serialize + ?Sized,
    {
        let url = self.url_for(path);
        self.post_json_bytes_url(&url, payload).await
    }

    pub async fn post_json_bytes_url<P>(&self, url: &str, payload: &P) -> Result<Vec<u8>>
    where
        P: Serialize + ?Sized,
    {
        let resp = self.http.post(url).json(payload).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("RPC HTTP error: {} {}", status, body));
        }
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryResponsePossible {
    #[serde(rename = "responseData")]
    pub response_data: Option<String>,
    pub data: Option<String>,
    pub result: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::join_url;

    #[test]
    fn join_url_normalizes_slashes() {
        assert_eq!(
            join_url("https://rpc.qubic.org/live/v1/", "tick-info"),
            "https://rpc.qubic.org/live/v1/tick-info"
        );
        assert_eq!(
            join_url("https://rpc.qubic.org/live/v1", "/tick-info"),
            "https://rpc.qubic.org/live/v1/tick-info"
        );
        assert_eq!(
            join_url("https://rpc.qubic.org/live/v1/", "/tick-info"),
            "https://rpc.qubic.org/live/v1/tick-info"
        );
    }
}
