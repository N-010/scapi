//! Service-oriented client generated from the four vendored Qubic specifications.
use reqwest::{Client, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

pub const LIVE_URL: &str = "https://rpc.qubic.org/live/v1";
pub const QUERY_URL: &str = "https://rpc.qubic.org/query/v1";
pub const ARCHIVE_URL: &str = "https://rpc.qubic.org";
pub const STATS_URL: &str = "https://rpc.qubic.org";

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("invalid request URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("request construction failed: {0}")]
    Request(#[source] reqwest::Error),
    #[error("transport error: {0}")]
    Transport(#[source] reqwest::Error),
    #[error("HTTP {status}: {body}")]
    Http {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("response decoding failed: {source}; body: {body}")]
    Decode {
        source: serde_json::Error,
        body: String,
    },
}

#[derive(Clone, Debug)]
pub struct QubicClientConfig {
    pub live_url: String,
    pub query_url: String,
    pub archive_url: String,
    pub stats_url: String,
}

impl Default for QubicClientConfig {
    fn default() -> Self {
        Self {
            live_url: LIVE_URL.into(),
            query_url: QUERY_URL.into(),
            archive_url: ARCHIVE_URL.into(),
            stats_url: STATS_URL.into(),
        }
    }
}

impl QubicClientConfig {
    pub fn live_url(mut self, value: impl Into<String>) -> Self {
        self.live_url = value.into();
        self
    }
    pub fn query_url(mut self, value: impl Into<String>) -> Self {
        self.query_url = value.into();
        self
    }
    pub fn archive_url(mut self, value: impl Into<String>) -> Self {
        self.archive_url = value.into();
        self
    }
    pub fn stats_url(mut self, value: impl Into<String>) -> Self {
        self.stats_url = value.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct QubicClient {
    pub(crate) http: Client,
    pub(crate) config: QubicClientConfig,
}
impl Default for QubicClient {
    fn default() -> Self {
        Self::new()
    }
}
impl QubicClient {
    pub fn new() -> Self {
        Self::with_config(QubicClientConfig::default())
    }
    pub fn with_config(config: QubicClientConfig) -> Self {
        Self {
            http: Client::new(),
            config,
        }
    }
    pub fn with_http_client(config: QubicClientConfig, http: Client) -> Self {
        Self { http, config }
    }
    pub fn config(&self) -> &QubicClientConfig {
        &self.config
    }
    pub fn live(&self) -> LiveApi<'_> {
        LiveApi(self)
    }
    pub fn query(&self) -> QueryApi<'_> {
        QueryApi(self)
    }
    pub fn archive(&self) -> ArchiveApi<'_> {
        ArchiveApi(self)
    }
    pub fn stats(&self) -> StatsApi<'_> {
        StatsApi(self)
    }

    async fn request<T: DeserializeOwned, Q: Serialize + ?Sized, B: Serialize + ?Sized>(
        &self,
        base: &str,
        method: Method,
        segments: &[String],
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, ClientError> {
        let mut url = Url::parse(base.trim_end_matches('/'))?;
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| url::ParseError::RelativeUrlWithoutBase)?;
            for segment in segments {
                path.push(segment);
            }
        }
        let mut request = self.http.request(method, url);
        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        let request = request.build().map_err(ClientError::Request)?;
        let response = self
            .http
            .execute(request)
            .await
            .map_err(ClientError::Transport)?;
        let status = response.status();
        let bytes = response.bytes().await.map_err(ClientError::Transport)?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        if !status.is_success() {
            return Err(ClientError::Http { status, body: text });
        }
        serde_json::from_slice(&bytes).map_err(|source| ClientError::Decode { source, body: text })
    }
}

pub struct LiveApi<'a>(pub(crate) &'a QubicClient);
pub struct QueryApi<'a>(pub(crate) &'a QubicClient);
pub struct ArchiveApi<'a>(pub(crate) &'a QubicClient);
pub struct StatsApi<'a>(pub(crate) &'a QubicClient);

mod openapi_api;
pub use openapi_api::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    fn server(status: &str, body: &str) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();
        let status = status.to_owned();
        let body = body.to_owned();
        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = [0; 8192];
            let size = stream.read(&mut bytes).unwrap();
            let _ = tx.send(String::from_utf8_lossy(&bytes[..size]).into_owned());
            write!(stream, "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        });
        (format!("http://{address}/base/"), rx)
    }

    #[tokio::test]
    async fn encodes_path_and_omits_none_query_fields() {
        let (base, request) = server("200 OK", "{}");
        let client = QubicClient::with_config(QubicClientConfig::default().stats_url(base));
        let query = GetAssetOwnersQuery {
            page: Some(2),
            page_size: None,
        };
        client
            .stats()
            .get_asset_owners("issuer /?".into(), "QX&".into(), &query)
            .await
            .unwrap();
        let request = request.recv().unwrap();
        assert!(request
            .starts_with("GET /base/v1/issuers/issuer%20%2F%3F/assets/QX&/owners?page=2 HTTP/1.1"));
        assert!(!request.contains("pageSize"));
    }

    #[tokio::test]
    async fn preserves_http_error_body() {
        let (base, _) = server("429 Too Many Requests", "quota exceeded");
        let client = QubicClient::with_config(QubicClientConfig::default().live_url(base));
        match client.live().get_tick_info().await.unwrap_err() {
            ClientError::Http { status, body } => {
                assert_eq!(status, 429);
                assert_eq!(body, "quota exceeded");
            }
            error => panic!("unexpected error: {error}"),
        }
    }

    #[tokio::test]
    async fn reports_malformed_success_json_with_body() {
        let (base, _) = server("200 OK", "not-json");
        let client = QubicClient::with_config(QubicClientConfig::default().live_url(base));
        match client.live().get_tick_info().await.unwrap_err() {
            ClientError::Decode { body, .. } => assert_eq!(body, "not-json"),
            error => panic!("unexpected error: {error}"),
        }
    }
}
