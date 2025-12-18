use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub topic: String,
    pub expiry: u64,
    pub acknowledged: bool,
    pub controller: String,
    pub namespaces: serde_json::Value,
    pub relay_protocol: String,
}

impl Session {
    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expiry < now
    }

    /// Check if the session is active (not expired and acknowledged)
    pub fn is_active(&self) -> bool {
        self.acknowledged && !self.is_expired()
    }

    /// Get time remaining until expiry in seconds
    pub fn time_until_expiry(&self) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if self.expiry > now {
            Some(self.expiry - now)
        } else {
            None
        }
    }
}

/// Session proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProposal {
    pub id: u64,
    pub params: SessionProposalParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProposalParams {
    pub proposer: Proposer,
    pub required_namespaces: serde_json::Value,
    pub relay_protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposer {
    pub public_key: String,
    pub metadata: ProposerMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposerMetadata {
    pub name: String,
    pub description: String,
    pub url: String,
    pub icons: Vec<String>,
}

/// Session storage key constants
pub mod storage_keys {
    pub const SESSION_TOPIC: &str = "sessionTopic";
    pub const WC_ACCOUNTS: &str = "wc_accounts";
    pub const WC_SELECTED_ACCOUNT: &str = "wc_selectedAccount";
}
