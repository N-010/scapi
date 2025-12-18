use serde::{Deserialize, Serialize};

/// WalletConnect event types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletConnectEvent {
    SessionProposal,
    SessionRequest,
    SessionDelete,
    SessionExpire,
    ProposalExpire,
    SessionEvent,
    SessionUpdate,
    SessionExtend,
    SessionPing,
}

impl WalletConnectEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletConnectEvent::SessionProposal => "session_proposal",
            WalletConnectEvent::SessionRequest => "session_request",
            WalletConnectEvent::SessionDelete => "session_delete",
            WalletConnectEvent::SessionExpire => "session_expire",
            WalletConnectEvent::ProposalExpire => "proposal_expire",
            WalletConnectEvent::SessionEvent => "session_event",
            WalletConnectEvent::SessionUpdate => "session_update",
            WalletConnectEvent::SessionExtend => "session_extend",
            WalletConnectEvent::SessionPing => "session_ping",
        }
    }
}

/// Event payload for session events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEventPayload {
    pub topic: String,
    pub params: SessionEventParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEventParams {
    pub event: EventData,
    pub chain_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub name: String,
    pub data: serde_json::Value,
}

/// Event callback trait
pub trait EventCallback: Send + Sync {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value);
}

/// Event handler
#[derive(Clone)]
pub struct EventHandler {
    callbacks: std::sync::Arc<std::sync::Mutex<Vec<Box<dyn EventCallback>>>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            callbacks: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn register(&self, callback: Box<dyn EventCallback>) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(callback);
    }

    pub fn emit(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback.on_event(event.clone(), payload.clone());
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple logging callback for debugging
pub struct LoggingCallback;

impl EventCallback for LoggingCallback {
    fn on_event(&self, event: WalletConnectEvent, payload: serde_json::Value) {
        tracing::info!("[WalletConnect Event] {:?}: {:?}", event, payload);
    }
}
