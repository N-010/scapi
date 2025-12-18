use crate::wallet_connect::{
    events::{EventHandler, WalletConnectEvent},
    qubic_namespace::{QubicMethod, QubicNamespace, QUBIC_NAMESPACE},
    session::Session,
    types::*,
};
use hex;
use rand::Rng;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use uuid::Uuid;
use walletconnect_sdk::{
    connection::Connection as WcConnection,
    message::{Message as WcRawMessage, TYPE_0},
    types::{
        EncryptedMessage as WcEncryptedMessage, Id as WcId, Metadata as WcMetadata,
        Namespace as WcNamespace, Participant as WcParticipant, Relay, SessionProposeParams,
    },
    utils::{derive_sym_key, random_bytes32, sha256 as wc_sha256},
    wc_message::{WcData, WcMessage},
    Error as WcSdkError,
};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519StaticSecret};

#[derive(Clone)]
struct ClientState {
    connection: WcConnection,
    client_seed: [u8; 32],
    sym_key: [u8; 32],
    topic: String,
    private_key: [u8; 32],
    public_key: [u8; 32],
    derived_sym_key: Option<[u8; 32]>,
    derived_topic: Option<String>,
    session_established: bool,
    optional_namespaces: HashMap<String, WcNamespace>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TopicKind {
    Initial,
    Derived,
}

/// WalletConnect client for Qubic
#[derive(Clone)]
pub struct WalletConnectClient {
    config: WalletConnectConfig,
    session: Arc<Mutex<Option<Session>>>,
    connection_url: Arc<Mutex<String>>,
    event_handler: EventHandler,
    pairing_topic: Arc<Mutex<String>>,
    state: Arc<Mutex<Option<ClientState>>>,
    listener_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    initialized: Arc<Mutex<bool>>,
    connection_receiver: Arc<Mutex<Option<tokio::sync::oneshot::Receiver<bool>>>>,
    connection_sender: Arc<Mutex<Option<tokio::sync::oneshot::Sender<bool>>>>,
}

impl WalletConnectClient {
    /// Create a new WalletConnect client
    pub fn new(config: WalletConnectConfig) -> Self {
        Self {
            config,
            session: Arc::new(Mutex::new(None)),
            connection_url: Arc::new(Mutex::new(String::new())),
            event_handler: EventHandler::new(),
            pairing_topic: Arc::new(Mutex::new(String::new())),
            state: Arc::new(Mutex::new(None)),
            listener_handle: Arc::new(Mutex::new(None)),
            initialized: Arc::new(Mutex::new(false)),
            connection_receiver: Arc::new(Mutex::new(None)),
            connection_sender: Arc::new(Mutex::new(None)),
        }
    }

    /// Initialize the WalletConnect client
    pub async fn init(&mut self) -> WalletConnectResult<()> {
        tracing::info!("[WalletConnect] Initializing client...");

        if self.config.project_id.is_empty() {
            return Err(WalletConnectError::Other(
                "Project ID is required".to_string(),
            ));
        }

        *self.initialized.lock().unwrap() = true;

        tracing::info!("[WalletConnect] Client initialized successfully");
        Ok(())
    }

    /// Generate connection URI for QR code (equivalent to client.connect() in JavaScript SDK)
    ///
    /// This creates a unique pairing URI that can be encoded into a QR code for wallet scanning.
    /// Similar to JavaScript SDK's `client.connect({ requiredNamespaces: {...} })`.
    ///
    /// # Important
    /// - Each call creates a completely NEW and UNIQUE connection URI
    /// - Old sessions are automatically cleaned up before creating new one
    /// - This prevents "connection already established" errors
    ///
    /// # Flow
    /// 1. Cleans up any previous connection state
    /// 2. Generates new symmetric key and topic
    /// 3. Creates SessionPropose message
    /// 4. Publishes to relay server
    /// 5. Returns URI for QR code
    ///
    /// # Returns
    /// URI string in format: `wc:<topic>@2?expiryTimestamp=...&relay-protocol=irn&symKey=...`
    ///
    /// # Example
    /// ```no_run
    /// # use scapi::wallet_connect::*;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = WalletConnectClient::new(/* config */);
    /// client.init().await?;
    /// 
    /// // This is like JS: const { uri, approval } = await client.connect(...)
    /// let uri = client.connect().await?;
    /// 
    /// // Display QR code or deep link
    /// println!("Scan this: {}", uri);
    /// 
    /// // Then wait for approval (see wait_for_connection)
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&mut self) -> WalletConnectResult<String> {
        tracing::info!("[WalletConnect] Generating connection URI...");

        if !*self.initialized.lock().unwrap() {
            return Err(WalletConnectError::NotInitialized);
        }

        // Clean up any previous connection state first (like JS SDK does on init)
        self.cleanup_old_state().await?;

        let (sender, receiver) = tokio::sync::oneshot::channel();
        *self.connection_sender.lock().unwrap() = Some(sender);
        *self.connection_receiver.lock().unwrap() = Some(receiver);

        let relay_base = self
            .config
            .relay_url
            .clone()
            .unwrap_or_else(|| "wss://relay.walletconnect.org".to_string());
        let (relay_rpc, relay_auth) = build_relay_endpoints(&relay_base);

        let metadata = self.sdk_metadata();
        let client_seed = random_bytes32();
        let connection = WcConnection::new(
            &relay_rpc,
            &relay_auth,
            &self.config.project_id,
            client_seed,
            metadata.clone(),
        );

        // Generate truly unique pairing topic using UUID + timestamp to avoid collisions
        let sym_key_bytes = random_bytes32();
        let sym_key_hex = hex::encode(sym_key_bytes);
        let topic = hex::encode(wc_sha256(sym_key_bytes));

        tracing::debug!("[WalletConnect] Generated symKey: {}", sym_key_hex);
        tracing::debug!(
            "[WalletConnect] Calculated topic (SHA256 of symKey): {}",
            topic
        );
        tracing::info!("[WalletConnect] Creating NEW pairing session (unique per connection)");

        *self.pairing_topic.lock().unwrap() = topic.clone();

        let expiry_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 300;
        tracing::debug!(
            "[WalletConnect] Expiry timestamp: {} (expires in 5 minutes)",
            expiry_timestamp
        );

        let namespace = QubicNamespace::new(self.config.qubic_chain_id.clone());

        // IMPORTANT: In JavaScript SDK v2.0, requiredNamespaces are DEPRECATED
        // The JS SDK automatically merges required into optional (engine.ts:233-239):
        //   connectParams.optionalNamespaces = mergeRequiredAndOptionalNamespaces(...)
        //   connectParams.requiredNamespaces = {}
        // So we MUST use optional_namespaces for compatibility!
        let mut optional_namespaces = HashMap::new();
        optional_namespaces.insert(
            QUBIC_NAMESPACE.to_string(),
            WcNamespace {
                accounts: None,
                chains: namespace.chains.clone(),
                events: namespace.events.clone(),
                methods: namespace.methods.clone(),
            },
        );

        let private_key = random_bytes32();
        let static_secret = X25519StaticSecret::from(private_key);
        let public_key = X25519PublicKey::from(&static_secret);
        let public_key_bytes = public_key.to_bytes();
        let public_key_hex = hex::encode(public_key_bytes);

        tracing::info!("[WalletConnect] Using OPTIONAL namespaces (matching JS SDK v2.0)");
        tracing::debug!("[WalletConnect] Chains: {:?}", namespace.chains);
        tracing::debug!("[WalletConnect] Methods: {:?}", namespace.methods);

        let session_propose = SessionProposeParams {
            required_namespaces: HashMap::new(),
            optional_namespaces: optional_namespaces.clone(),
            relays: vec![Relay {
                protocol: "irn".to_string(),
            }],
            pairing_topic: topic.clone(),
            proposer: WcParticipant {
                public_key: public_key_hex.clone(),
                metadata: metadata.clone(),
            },
            expiry_timestamp,
        };

        // Log SessionPropose before encryption for debugging
        println!("\n=== [WalletConnect] SessionPropose DETAILS (BEFORE encryption) ===\n");
        println!("[WalletConnect] Pairing topic: {}", topic);
        println!("[WalletConnect] Proposer public key: {}", public_key_hex);
        println!("[WalletConnect] Expiry timestamp: {}", expiry_timestamp);
        println!("[WalletConnect] Optional namespaces:");
        for (ns, ns_params) in &optional_namespaces {
            println!("  - {}: chains={:?}, methods={:?}, events={:?}", 
                ns, ns_params.chains, ns_params.methods, ns_params.events);
        }
        println!("[WalletConnect] Metadata: name={}, url={}", metadata.name, metadata.url);
        
        // Serialize and log full SessionPropose JSON
        match serde_json::to_string_pretty(&session_propose) {
            Ok(json) => {
                tracing::debug!("[WalletConnect] SessionPropose JSON:\n{}", json);
                println!("[WalletConnect] Full SessionPropose JSON:\n{}", json);
            }
            Err(e) => {
                tracing::warn!("[WalletConnect] Failed to serialize SessionPropose: {}", e);
            }
        }

        let wc_message = WcMessage {
            data: WcData::SessionPropose(session_propose),
            id: self.generate_message_id(),
            irn_tag_override: None,
        };

        let irn_tag = wc_message.irn_tag();
        let ttl = wc_message.ttl();
        let raw_message = wc_message.into_raw().map_err(map_sdk_error)?;

        // Log message before encryption
        match serde_json::to_string_pretty(&raw_message) {
            Ok(json) => {
                tracing::debug!("[WalletConnect] Message before encryption:\n{}", json);
            }
            Err(e) => {
                tracing::warn!("[WalletConnect] Failed to serialize message: {}", e);
            }
        }

        let encrypted = raw_message
            .encrypt(sym_key_bytes, Some(TYPE_0), None, None)
            .map_err(map_sdk_error)?;
        tracing::debug!(
            "[WalletConnect] Encrypted SessionPropose (base64): {}",
            encrypted
        );

        tracing::info!(
            "[WalletConnect] 📤 Subscribing to pairing topic: {}",
            topic
        );
        connection
            .irn_subscribe(&topic)
            .await
            .map_err(map_sdk_error)?;

        tracing::info!(
            "[WalletConnect] 📤 Publishing SessionPropose to topic: {} (irn_tag: {:?}, ttl: {})",
            topic,
            irn_tag,
            ttl
        );
        tracing::debug!(
            "[WalletConnect] SessionPropose details: pairing_topic={}, proposer_pk={}, expiry={}",
            topic,
            public_key_hex,
            expiry_timestamp
        );
        println!("\n=== [WalletConnect] SENDING SessionPropose ===\n");
        println!("[WalletConnect] 📤 Sending SessionPropose on topic: {}", topic);
        println!("[WalletConnect] 📤 Params: irn_tag={:?}, ttl={}", irn_tag, ttl);
        println!("[WalletConnect] 📤 Proposer public key: {}", public_key_hex);
        println!("[WalletConnect] 📤 Expiry timestamp: {}", expiry_timestamp);
        
        eprintln!("[WalletConnect DEBUG] Publishing to topic: {}", topic);
        
        let mut encrypted_message =
            WcEncryptedMessage::new(topic.clone(), encrypted, irn_tag.clone(), ttl);
        if irn_tag == walletconnect_sdk::types::IrnTag::SessionPropose {
            encrypted_message.prompt = Some(true);
        }
        tracing::debug!(
            "[WalletConnect] IRN publish options: prompt={:?}",
            encrypted_message.prompt
        );

        match connection
            .irn_publish(encrypted_message)
            .await
        {
            Ok(_) => {
                println!("[WalletConnect] ✅ SessionPropose sent to relay server!");
                println!("[WalletConnect] ⏳ Waiting for wallet response on topic: {}", topic);
                eprintln!("[WalletConnect DEBUG] Published successfully to relay");
            }
            Err(e) => {
                println!("[WalletConnect] ❌ ERROR sending SessionPropose: {:?}", e);
                eprintln!("[WalletConnect ERROR] Failed to publish: {:?}", e);
                return Err(map_sdk_error(e));
            }
        }
        tracing::info!(
            "[WalletConnect] ✅ SessionPropose sent successfully! Waiting for wallet response..."
        );

        {
            let mut state_guard = self.state.lock().unwrap();
            *state_guard = Some(ClientState {
                connection: connection.clone(),
                client_seed,
                sym_key: sym_key_bytes,
                topic: topic.clone(),
                private_key,
                public_key: public_key_bytes,
                derived_sym_key: None,
                derived_topic: None,
                session_established: false,
                optional_namespaces: optional_namespaces.clone(),
            });
        }

        self.spawn_listener();

        let uri = format!(
            "wc:{}@2?expiryTimestamp={}&relay-protocol=irn&symKey={}",
            topic, expiry_timestamp, sym_key_hex
        );

        *self.connection_url.lock().unwrap() = uri.clone();

        println!("\n=== [WalletConnect] URI GENERATED ===\n");
        println!("[WalletConnect] ✅ Connection URI generated successfully");
        println!("[WalletConnect] 🔑 Pairing topic: {}", topic);
        println!("[WalletConnect] ⏰ Expiry: {} (expires in 5 minutes)", expiry_timestamp);
        println!("[WalletConnect] 🔐 SymKey (hex): {}", sym_key_hex);
        println!("[WalletConnect] 📋 Full URI:");
        println!("   {}", uri);
        println!("\n=== [WalletConnect] SESSION PROPOSAL SENT ===\n");
        tracing::info!("[WalletConnect] ✅ Connection URI generated successfully");
        tracing::info!("[WalletConnect] 🔑 Pairing topic: {}", topic);

        Ok(uri)
    }

    /// Wait for wallet connection with timeout (equivalent to approval() in JavaScript SDK)
    ///
    /// This method waits for the wallet to scan the QR code and approve the connection.
    /// It's similar to `await approval()` in the JavaScript WalletConnect SDK.
    ///
    /// # Flow
    /// 1. User scans QR code with wallet app
    /// 2. Wallet sends SessionSettle message
    /// 3. This method receives the approval and returns
    ///
    /// # Arguments
    /// * `timeout_secs` - Maximum time to wait for connection (in seconds)
    ///
    /// # Returns
    /// * `Ok(true)` - Successfully connected and session established
    /// * `Ok(false)` - Connection was rejected by wallet
    /// * `Err(Timeout)` - User didn't scan QR code within timeout
    /// * `Err(ConnectionFailed)` - Connection failed for other reasons
    ///
    /// # Example
    /// ```no_run
    /// # use scapi::wallet_connect::*;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = WalletConnectClient::new(/* config */);
    /// client.init().await?;
    /// 
    /// // Generate QR code URI
    /// let uri = client.connect().await?;
    /// println!("Scan this QR: {}", uri);
    /// 
    /// // Wait for user to scan and approve (like JS: await approval())
    /// match client.wait_for_connection(120).await {
    ///     Ok(true) => println!("Connected!"),
    ///     Ok(false) => println!("User rejected"),
    ///     Err(e) => println!("Timeout or error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_connection(&mut self, timeout_secs: u64) -> WalletConnectResult<bool> {
        tracing::info!(
            "[WalletConnect] Waiting for wallet approval (timeout: {}s)...",
            timeout_secs
        );
        tracing::debug!("[WalletConnect] This is equivalent to 'await approval()' in JS SDK");

        let receiver = {
            let mut receiver_guard = self.connection_receiver.lock().unwrap();
            receiver_guard.take()
        };

        if let Some(receiver) = receiver {
            // Wait for connection with timeout
            match tokio::time::timeout(Duration::from_secs(timeout_secs), receiver).await {
                Ok(Ok(connected)) => {
                    if connected {
                        tracing::info!("[WalletConnect] ✅ Connection approved and established!");
                        Ok(true)
                    } else {
                        tracing::warn!("[WalletConnect] ❌ Connection rejected by wallet");
                        Err(WalletConnectError::ConnectionFailed(
                            "Connection rejected by wallet".to_string(),
                        ))
                    }
                }
                Ok(Err(_)) => {
                    tracing::error!("[WalletConnect] Connection channel closed unexpectedly");
                    Err(WalletConnectError::Other(
                        "Connection channel closed".to_string(),
                    ))
                }
                Err(_) => {
                    tracing::warn!("[WalletConnect] ⏰ Connection timeout - QR code not scanned");
                    Err(WalletConnectError::Timeout)
                }
            }
        } else {
            Err(WalletConnectError::Other(
                "No pending connection (did you call connect() first?)".to_string(),
            ))
        }
    }

    /// Simulate wallet connection (for testing)
    /// In real implementation, this would be called when wallet scans QR and approves
    pub fn simulate_connection(&self, success: bool) -> WalletConnectResult<()> {
        let sender = {
            let mut sender_guard = self.connection_sender.lock().unwrap();
            sender_guard.take()
        };

        if let Some(sender) = sender {
            if success {
                // Create a mock session
                let session = Session {
                    topic: self.pairing_topic.lock().unwrap().clone(),
                    expiry: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        + 86400, // 24 hours
                    acknowledged: true,
                    controller: "wallet".to_string(),
                    namespaces: json!({}),
                    relay_protocol: "irn".to_string(),
                };
                *self.session.lock().unwrap() = Some(session);
            }

            sender.send(success).map_err(|_| {
                WalletConnectError::Other("Failed to send connection notification".to_string())
            })?;

            tracing::info!("[WalletConnect] Connection notification sent: {}", success);
            Ok(())
        } else {
            Err(WalletConnectError::Other(
                "No pending connection to simulate".to_string(),
            ))
        }
    }

    /// Approve the pending connection and establish session
    pub async fn approve(&mut self) -> WalletConnectResult<Session> {
        tracing::info!("[WalletConnect] Waiting for session approval...");

        // In a real implementation, this would wait for the wallet to scan QR and approve
        // For now, we'll simulate the approval process

        // This is a placeholder - actual implementation would listen to relay messages
        // and wait for wallet approval

        Err(WalletConnectError::Other(
            "Approval waiting not yet fully implemented - requires relay message handling"
                .to_string(),
        ))
    }

    /// Check if there's an active session
    pub fn is_session_active(&self) -> bool {
        let session_guard = self.session.lock().unwrap();
        match &*session_guard {
            Some(session) => session.is_active(),
            None => false,
        }
    }

    /// Get current session
    pub fn get_session(&self) -> Option<Session> {
        self.session.lock().unwrap().clone()
    }

    /// Set session (used after approval)
    pub fn set_session(&self, session: Session) {
        *self.session.lock().unwrap() = Some(session);
    }

    /// Clear current session
    pub fn clear_session(&self) {
        *self.session.lock().unwrap() = None;
    }

    /// Make a request to the wallet
    async fn make_request<T: serde::de::DeserializeOwned>(
        &self,
        method: QubicMethod,
        params: serde_json::Value,
    ) -> WalletConnectResult<T> {
        tracing::info!("[WalletConnect] Making request: {}", method.as_str());

        if !self.is_session_active() {
            return Err(WalletConnectError::NoActiveSession);
        }

        let session_guard = self.session.lock().unwrap();
        let session = session_guard
            .as_ref()
            .ok_or(WalletConnectError::NoActiveSession)?;
        let _topic = session.topic.clone();
        drop(session_guard);

        let request_id = Uuid::new_v4().to_string();

        let _request_payload = json!({
            "id": request_id,
            "jsonrpc": "2.0",
            "method": method.as_str(),
            "params": params,
        });

        // In a real implementation, this would:
        // 1. Send the request through the relay
        // 2. Wait for response from wallet
        // 3. Return the result

        Err(WalletConnectError::Other(
            "Request handling not yet fully implemented - requires relay message handling"
                .to_string(),
        ))
    }

    /// Request accounts from wallet
    pub async fn request_accounts(&self) -> WalletConnectResult<Vec<WalletAccount>> {
        self.make_request(QubicMethod::RequestAccounts, json!({}))
            .await
    }

    /// Send Qubic (simple transfer)
    pub async fn send_qubic(
        &self,
        from: &str,
        to: &str,
        amount: u64,
    ) -> WalletConnectResult<serde_json::Value> {
        let params = json!({
            "from": from,
            "to": to,
            "amount": amount,
        });
        self.make_request(QubicMethod::SendQubic, params).await
    }

    /// Sign a transaction
    pub async fn sign_transaction(
        &self,
        params: QubicTransactionParams,
    ) -> WalletConnectResult<SignatureResponse> {
        let mut request_params = json!({
            "from": params.from,
            "to": params.to,
            "amount": params.amount,
            "inputType": params.input_type.unwrap_or(0),
            "payload": params.payload.as_ref().unwrap_or(&"".to_string()),
        });

        if let Some(tick) = params.tick {
            request_params["tick"] = json!(tick);
        }

        self.make_request(QubicMethod::SignTransaction, request_params)
            .await
    }

    /// Send a transaction (sign and broadcast)
    pub async fn send_transaction(
        &self,
        params: QubicTransactionParams,
    ) -> WalletConnectResult<serde_json::Value> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut request_params = json!({
            "from": params.from,
            "to": params.to,
            "amount": params.amount,
            "inputType": params.input_type.unwrap_or(0),
            "payload": params.payload.as_ref().unwrap_or(&"".to_string()),
            "nonce": now.to_string(),
        });

        if let Some(tick) = params.tick {
            request_params["tick"] = json!(tick);
        }

        self.make_request(QubicMethod::SendTransaction, request_params)
            .await
    }

    /// Sign a message
    pub async fn sign_message(
        &self,
        from: &str,
        message: &str,
    ) -> WalletConnectResult<SignatureResponse> {
        let params = json!({
            "from": from,
            "message": message,
        });
        self.make_request(QubicMethod::Sign, params).await
    }

    /// Disconnect from wallet
    pub async fn disconnect(&mut self) -> WalletConnectResult<()> {
        tracing::info!("[WalletConnect] Disconnecting...");

        if let Some(session) = self.get_session() {
            // In a real implementation, send disconnect message through relay
            self.event_handler.emit(
                WalletConnectEvent::SessionDelete,
                json!({ "topic": session.topic }),
            );
        }

        self.clear_session();
        *self.connection_url.lock().unwrap() = String::new();

        tracing::info!("[WalletConnect] Disconnected successfully");
        Ok(())
    }

    /// Get the event handler for registering callbacks
    pub fn event_handler(&self) -> &EventHandler {
        &self.event_handler
    }

    /// Get connection URL
    pub fn get_connection_url(&self) -> String {
        self.connection_url.lock().unwrap().clone()
    }
}

impl Drop for WalletConnectClient {
    fn drop(&mut self) {
        // Cleanup on drop
        self.clear_session();
    }
}

impl WalletConnectClient {
    /// Clean up any previous connection state before creating a new connection
    /// This prevents "connection already established" errors
    async fn cleanup_old_state(&mut self) -> WalletConnectResult<()> {
        tracing::debug!("[WalletConnect] Cleaning up old state...");
        
        // Stop old listener if exists
        {
            let mut handle_guard = self.listener_handle.lock().unwrap();
            if let Some(handle) = handle_guard.take() {
                handle.abort();
                tracing::debug!("[WalletConnect] Aborted old listener");
            }
        }

        // Clear old state (connection will be dropped, automatically closing subscriptions)
        {
            let mut state_guard = self.state.lock().unwrap();
            if state_guard.is_some() {
                *state_guard = None;
                tracing::debug!("[WalletConnect] Cleared old connection state");
            }
        }

        // Clear old connection channels to prevent "already connected" errors
        {
            let mut sender_guard = self.connection_sender.lock().unwrap();
            if sender_guard.take().is_some() {
                tracing::debug!("[WalletConnect] Cleared old connection sender");
            }
        }
        {
            let mut receiver_guard = self.connection_receiver.lock().unwrap();
            if receiver_guard.take().is_some() {
                tracing::debug!("[WalletConnect] Cleared old connection receiver");
            }
        }

        // Clear session and connection info
        self.clear_session();
        *self.pairing_topic.lock().unwrap() = String::new();
        *self.connection_url.lock().unwrap() = String::new();

        tracing::info!("[WalletConnect] Old state cleaned up - ready for new connection");
        Ok(())
    }

    fn sdk_metadata(&self) -> WcMetadata {
        let metadata = &self.config.metadata;
        // Add unique identifier to metadata name to avoid wallet caching issues
        // Each connection should have unique metadata to prevent "already connected" errors
        let unique_id = Uuid::new_v4().to_string()[..8].to_string();
        let unique_name = format!("{} ({}@{})", metadata.name, unique_id, 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        tracing::debug!("[WalletConnect] Using unique metadata name: {}", unique_name);
        
        WcMetadata {
            name: unique_name,
            description: metadata.description.clone(),
            url: metadata.url.clone(),
            icons: metadata.icons.clone(),
        }
    }

    fn generate_message_id(&self) -> WcId {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut rng = rand::rng();
        let extra: u64 = rng.random_range(0..1_000);
        let id_u64 = (now_ms as u64).saturating_mul(1_000).saturating_add(extra);
        WcId::Number(serde_json::Number::from(id_u64))
    }

    fn spawn_listener(&self) {
        let mut guard = self.listener_handle.lock().unwrap();
        if guard.is_some() {
            return;
        }

        let state = Arc::clone(&self.state);
        let event_handler = self.event_handler.clone();
        let session_store = Arc::clone(&self.session);
        let connection_sender = Arc::clone(&self.connection_sender);

        let handle = tokio::spawn(async move {
            listener_loop(state, event_handler, session_store, connection_sender).await;
        });

        *guard = Some(handle);
    }
}

fn build_relay_endpoints(relay_url: &str) -> (String, String) {
    let mut base = relay_url.trim().to_string();
    if base.starts_with("wss://") {
        base = base.replacen("wss://", "https://", 1);
    } else if base.starts_with("ws://") {
        base = base.replacen("ws://", "http://", 1);
    } else if !base.starts_with("https://") && !base.starts_with("http://") {
        base = format!("https://{}", base.trim_start_matches("://"));
    }
    let auth = base.trim_end_matches('/').to_string();
    let rpc = format!("{}/rpc", auth);
    (rpc, auth)
}

fn map_sdk_error(err: WcSdkError) -> WalletConnectError {
    WalletConnectError::RelayError(format!("{err:?}"))
}

async fn listener_loop(
    state: Arc<Mutex<Option<ClientState>>>,
    event_handler: EventHandler,
    session_store: Arc<Mutex<Option<Session>>>,
    connection_sender: Arc<Mutex<Option<tokio::sync::oneshot::Sender<bool>>>>,
) {
    loop {
        if let Err(err) = process_topic(
            &state,
            TopicKind::Initial,
            &event_handler,
            &session_store,
            &connection_sender,
        )
        .await
        {
            tracing::error!("[WalletConnect] Error processing pairing topic: {}", err);
        }

        if let Err(err) = process_topic(
            &state,
            TopicKind::Derived,
            &event_handler,
            &session_store,
            &connection_sender,
        )
        .await
        {
            tracing::error!("[WalletConnect] Error processing derived topic: {}", err);
        }

        sleep(Duration::from_millis(500)).await;
    }
}

async fn process_topic(
    state: &Arc<Mutex<Option<ClientState>>>,
    kind: TopicKind,
    event_handler: &EventHandler,
    session_store: &Arc<Mutex<Option<Session>>>,
    connection_sender: &Arc<Mutex<Option<tokio::sync::oneshot::Sender<bool>>>>,
) -> WalletConnectResult<()> {
    let state_snapshot_opt = { state.lock().unwrap().clone() };
    let Some(state_snapshot) = state_snapshot_opt else {
        return Ok(());
    };

    let (connection, sym_key, topic) = match kind {
        TopicKind::Initial => (
            state_snapshot.connection.clone(),
            state_snapshot.sym_key,
            state_snapshot.topic.clone(),
        ),
        TopicKind::Derived => {
            if let (Some(derived_sym), Some(derived_topic)) = (
                state_snapshot.derived_sym_key,
                state_snapshot.derived_topic.clone(),
            ) {
                (
                    state_snapshot.connection.clone(),
                    derived_sym,
                    derived_topic,
                )
            } else {
                return Ok(());
            }
        }
    };

    let messages = connection
        .irn_fetch_messages(&topic)
        .await
        .map_err(map_sdk_error)?;

    if !messages.is_empty() {
        println!(
            "[WalletConnect] 📬 Received {} messages on topic {} (kind: {:?})",
            messages.len(),
            topic,
            kind
        );
    } else if kind == TopicKind::Initial {
        // Log only for the initial topic to avoid spamming
        tracing::debug!("[WalletConnect] No messages on topic {} (kind: {:?})", topic, kind);
    }

    for encrypted in messages {
        if let Err(err) = handle_message(
            state,
            &state_snapshot,
            encrypted,
            sym_key,
            kind,
            event_handler,
            session_store,
            connection_sender,
        )
        .await
        {
            tracing::error!(
                "[WalletConnect] Failed to handle message on topic {}: {}",
                topic,
                err
            );
        }
    }

    Ok(())
}

async fn handle_message(
    state: &Arc<Mutex<Option<ClientState>>>,
    state_snapshot: &ClientState,
    encrypted: WcEncryptedMessage,
    sym_key: [u8; 32],
    kind: TopicKind,
    event_handler: &EventHandler,
    session_store: &Arc<Mutex<Option<Session>>>,
    connection_sender: &Arc<Mutex<Option<tokio::sync::oneshot::Sender<bool>>>>,
) -> WalletConnectResult<()> {
    let topic = &encrypted.topic; // Get topic from encrypted message
    let decrypted =
        WcRawMessage::decrypt(&encrypted.message, sym_key, None).map_err(map_sdk_error)?;
    let decoded = decrypted.clone().decode().map_err(map_sdk_error)?;

    let method_name = decoded.data.method();
    println!(
        "[WalletConnect] 📨 Received message from wallet: {:?} on topic {} (kind: {:?})",
        method_name, topic, kind
    );
    tracing::info!(
        "[WalletConnect] 📨 Received message: {:?} on topic {} (kind: {:?})",
        method_name,
        topic,
        kind
    );

    let message_data = decoded.data.clone();
    match message_data {
        WcData::SessionProposeResponse(resp) => {
            if kind != TopicKind::Initial {
                tracing::debug!(
                    "[WalletConnect] SessionProposeResponse received on derived topic, ignoring"
                );
                return Ok(());
            }

            tracing::info!(
                "[WalletConnect] ✅ Received SessionProposeResponse from wallet!"
            );
            tracing::debug!(
                "[WalletConnect] Responder public key: {}",
                resp.responder_public_key
            );

            let already_has_derived = {
                let guard = state.lock().unwrap();
                guard
                    .as_ref()
                    .and_then(|s| s.derived_topic.clone())
                    .is_some()
            };
            if already_has_derived {
                tracing::warn!(
                    "[WalletConnect] ⚠️ Derived topic already set, wallet sent duplicate response"
                );
                return Ok(());
            }

            let responder_pk_vec = hex::decode(resp.responder_public_key).map_err(|e| {
                WalletConnectError::Other(format!("Invalid responder public key: {e}"))
            })?;
            if responder_pk_vec.len() != 32 {
                return Err(WalletConnectError::Other(
                    "Responder public key must contain 32 bytes".to_string(),
                ));
            }
            let mut responder_pk = [0u8; 32];
            responder_pk.copy_from_slice(&responder_pk_vec);

            let derived_sym_key = derive_sym_key(state_snapshot.private_key, responder_pk);
            let derived_topic = hex::encode(wc_sha256(derived_sym_key));

            tracing::info!(
                "[WalletConnect] ✅ Wallet approved! Creating derived topic: {}",
                derived_topic
            );

            tracing::info!(
                "[WalletConnect] 📤 Subscribing to derived topic: {}",
                derived_topic
            );
            
            state_snapshot
                .connection
                .irn_subscribe(&derived_topic)
                .await
                .map_err(map_sdk_error)?;

            tracing::info!(
                "[WalletConnect] ✅ Successfully subscribed to derived topic: {}",
                derived_topic
            );
            println!(
                "[WalletConnect] ✅ Subscribed to derived topic: {}",
                derived_topic
            );
            println!(
                "[WalletConnect] ⏳ Waiting for SessionSettle from wallet on derived topic..."
            );

            {
                let mut guard = state.lock().unwrap();
                if let Some(state_mut) = guard.as_mut() {
                    state_mut.derived_sym_key = Some(derived_sym_key);
                    state_mut.derived_topic = Some(derived_topic);
                }
            }
            
            tracing::info!(
                "[WalletConnect] ✅ State updated with derived topic. Listener will check for messages on next poll."
            );
        }
        WcData::SessionSettle(params) => {
            if kind != TopicKind::Derived {
                tracing::debug!(
                    "[WalletConnect] SessionSettle received on pairing topic; waiting for derived topic"
                );
                return Ok(());
            }

            tracing::info!("[WalletConnect] 🎉 SessionSettle received! Session is being established...");

            let topic = state_snapshot
                .derived_topic
                .clone()
                .unwrap_or_else(|| state_snapshot.topic.clone());

            let session = Session {
                topic: topic.clone(),
                expiry: params.expiry,
                acknowledged: true,
                controller: params.controller.public_key.clone(),
                namespaces: serde_json::to_value(&params.namespaces)?,
                relay_protocol: params.relay.protocol.clone(),
            };
            *session_store.lock().unwrap() = Some(session.clone());

            {
                let mut guard = state.lock().unwrap();
                if let Some(state_mut) = guard.as_mut() {
                    state_mut.session_established = true;
                }
            }

            tracing::info!("[WalletConnect] ✅ Session established successfully! Topic: {}", topic);

            if let Some(sender) = connection_sender.lock().unwrap().take() {
                let _ = sender.send(true);
                tracing::info!("[WalletConnect] ✅ Connection notification sent to wait_for_connection()");
            }

            event_handler.emit(
                WalletConnectEvent::SessionUpdate,
                serde_json::json!({ "session": session }),
            );

            let response = decoded.create_response(WcData::SessionSettleResult(true), None);
            let response_raw = response.into_raw().map_err(map_sdk_error)?;
            let encrypted_ack = response_raw
                .encrypt(sym_key, Some(TYPE_0), None, None)
                .map_err(map_sdk_error)?;
            state_snapshot
                .connection
                .irn_publish(WcEncryptedMessage::new(
                    topic,
                    encrypted_ack,
                    response.irn_tag(),
                    response.ttl(),
                ))
                .await
                .map_err(map_sdk_error)?;
        }
        WcData::SessionSettleResult(result) => {
            tracing::debug!("[WalletConnect] Received session settle result: {}", result);
        }
        WcData::SessionRequest(params) => {
            event_handler.emit(
                WalletConnectEvent::SessionRequest,
                serde_json::json!({
                    "topic": if kind == TopicKind::Derived {
                        state_snapshot
                            .derived_topic
                            .clone()
                            .unwrap_or_else(|| state_snapshot.topic.clone())
                    } else {
                        state_snapshot.topic.clone()
                    },
                    "request": params
                }),
            );
        }
        WcData::SessionDelete(payload) => {
            tracing::warn!(
                "[WalletConnect] Wallet requested session delete: {:?}",
                payload
            );
            if let Some(sender) = connection_sender.lock().unwrap().take() {
                let _ = sender.send(false);
            }
            event_handler.emit(
                WalletConnectEvent::SessionDelete,
                serde_json::json!({ "payload": payload }),
            );
        }
        WcData::Error { message, code, .. } => {
            tracing::error!(
                "[WalletConnect] ❌ Received ERROR from wallet - code: {}, message: '{}'",
                code,
                message
            );
            tracing::error!(
                "[WalletConnect] This error message suggests: {}",
                if message.contains("already")
                    || message.contains("established")
                    || message.contains("via this URL")
                {
                    "Wallet sees an existing connection. Check if old pairing/session needs to be deleted."
                } else {
                    "Wallet rejected the connection request for unknown reason."
                }
            );
            if let Some(sender) = connection_sender.lock().unwrap().take() {
                let _ = sender.send(false);
            }
            return Err(WalletConnectError::ConnectionFailed(format!(
                "Wallet error (code {}): {}",
                code, message
            )));
        }
        other => {
            tracing::debug!(
                "[WalletConnect] Ignoring message on topic {:?}: {:?}",
                kind,
                other
            );
        }
    }

    Ok(())
}
