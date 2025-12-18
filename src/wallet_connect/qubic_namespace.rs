use serde::{Deserialize, Serialize};

/// Qubic namespace methods
pub const QUBIC_NAMESPACE: &str = "qubic";

/// Qubic WalletConnect methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QubicMethod {
    RequestAccounts,
    SendQubic,
    SignTransaction,
    SendTransaction,
    Sign,
}

impl QubicMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            QubicMethod::RequestAccounts => "qubic_requestAccounts",
            QubicMethod::SendQubic => "qubic_sendQubic",
            QubicMethod::SignTransaction => "qubic_signTransaction",
            QubicMethod::SendTransaction => "qubic_sendTransaction",
            QubicMethod::Sign => "qubic_sign",
        }
    }

    pub fn all_methods() -> Vec<&'static str> {
        vec![
            Self::RequestAccounts.as_str(),
            Self::SendQubic.as_str(),
            Self::SignTransaction.as_str(),
            Self::SendTransaction.as_str(),
            Self::Sign.as_str(),
        ]
    }
}

/// Qubic wallet events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QubicEvent {
    AmountChanged,
    AssetAmountChanged,
    AccountsChanged,
}

impl QubicEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            QubicEvent::AmountChanged => "amountChanged",
            QubicEvent::AssetAmountChanged => "assetAmountChanged",
            QubicEvent::AccountsChanged => "accountsChanged",
        }
    }

    pub fn all_events() -> Vec<&'static str> {
        vec![
            Self::AmountChanged.as_str(),
            Self::AssetAmountChanged.as_str(),
            Self::AccountsChanged.as_str(),
        ]
    }
}

/// Namespace configuration for Qubic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QubicNamespace {
    pub chains: Vec<String>,
    pub methods: Vec<String>,
    pub events: Vec<String>,
}

impl QubicNamespace {
    pub fn new(chain_id: String) -> Self {
        Self {
            chains: vec![chain_id],
            methods: QubicMethod::all_methods()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            events: QubicEvent::all_events()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
