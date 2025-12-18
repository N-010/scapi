use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WalletConnect connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletConnectionStatus {
    Idle,
    Connecting,
    RequestingAccounts,
    Connected,
    ProposalExpired,
    UserRejectedConnection,
    Error,
}

/// Wallet account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAccount {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(flatten)]
    pub additional_data: HashMap<String, serde_json::Value>,
}

/// Metadata for WalletConnect client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMetadata {
    pub name: String,
    pub description: String,
    pub url: String,
    pub icons: Vec<String>,
}

impl Default for ClientMetadata {
    fn default() -> Self {
        Self {
            name: "RandomLottery".to_string(),
            description: "Lottery".to_string(),
            url: "https://qubicrl.org".to_string(),
            icons: vec!["https://qx.qubic.org/assets/icons/favicon.ico".to_string()],
        }
    }
}

/// Configuration for WalletConnect client
#[derive(Debug, Clone)]
pub struct WalletConnectConfig {
    pub project_id: String,
    pub qubic_chain_id: String,
    pub metadata: ClientMetadata,
    pub relay_url: Option<String>,
}

impl WalletConnectConfig {
    pub fn new(project_id: String, qubic_chain_id: String) -> Self {
        Self {
            project_id,
            qubic_chain_id,
            metadata: ClientMetadata::default(),
            relay_url: Some("wss://relay.walletconnect.org".to_string()),
        }
    }

    pub fn with_metadata(mut self, metadata: ClientMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_relay_url(mut self, url: String) -> Self {
        self.relay_url = Some(url);
        self
    }
}

/// Transaction parameters for Qubic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QubicTransactionParams {
    pub from: String,
    pub to: String,
    pub amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

/// Response from signing operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_transaction: Option<String>,
}

/// Error types for WalletConnect operations
#[derive(Debug, thiserror::Error)]
pub enum WalletConnectError {
    #[error("WalletConnect client not initialized")]
    NotInitialized,

    #[error("No active session")]
    NoActiveSession,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Proposal expired")]
    ProposalExpired,

    #[error("User rejected connection")]
    UserRejected,

    #[error("Invalid chain ID")]
    InvalidChainId,

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Relay error: {0}")]
    RelayError(String),

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Timeout")]
    Timeout,

    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for WalletConnectError {
    fn from(err: serde_json::Error) -> Self {
        WalletConnectError::SerializationError(err.to_string())
    }
}

pub type WalletConnectResult<T> = Result<T, WalletConnectError>;
