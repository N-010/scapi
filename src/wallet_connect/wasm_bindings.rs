#[cfg(target_arch = "wasm32")]
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
#[cfg(target_arch = "wasm32")]
use base64::Engine as _;
#[cfg(target_arch = "wasm32")]
use qrcode::{render::svg, QrCode};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::wallet_connect::{client::WalletConnectClient as NativeClient, types::*};

/// WASM-compatible WalletConnect client
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WalletConnectClient {
    inner: Option<NativeClient>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WalletConnectClient {
    /// Create a new WalletConnect client
    #[wasm_bindgen(constructor)]
    pub fn new(project_id: String, qubic_chain_id: String) -> Self {
        // Initialize tracing for WASM
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let config = WalletConnectConfig::new(project_id, qubic_chain_id);
        Self {
            inner: Some(NativeClient::new(config)),
        }
    }

    /// Create client with custom metadata
    #[wasm_bindgen(js_name = newWithMetadata)]
    pub fn new_with_metadata(
        project_id: String,
        qubic_chain_id: String,
        metadata_json: JsValue,
    ) -> Result<WalletConnectClient, JsValue> {
        let metadata: ClientMetadata = serde_wasm_bindgen::from_value(metadata_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid metadata: {}", e)))?;

        let config = WalletConnectConfig::new(project_id, qubic_chain_id).with_metadata(metadata);

        Ok(Self {
            inner: Some(NativeClient::new(config)),
        })
    }

    /// Initialize the client
    #[wasm_bindgen(js_name = initClient)]
    pub async fn init_client(&mut self) -> Result<(), JsValue> {
        let client = self
            .inner
            .as_mut()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        client
            .init()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Generate connection URI for QR code
    #[wasm_bindgen(js_name = genConnectUrl)]
    pub async fn gen_connect_url(&mut self) -> Result<String, JsValue> {
        let client = self
            .inner
            .as_mut()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        client
            .connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get the current connection URL
    #[wasm_bindgen(js_name = getConnectionUrl)]
    pub fn get_connection_url(&self) -> String {
        match &self.inner {
            Some(client) => client.get_connection_url(),
            None => String::new(),
        }
    }

    /// Check if session is active
    #[wasm_bindgen(js_name = isSessionActive)]
    pub fn is_session_active(&self) -> bool {
        match &self.inner {
            Some(client) => client.is_session_active(),
            None => false,
        }
    }

    /// Request accounts from wallet
    #[wasm_bindgen(js_name = requestAccounts)]
    pub async fn request_accounts(&self) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        let accounts = client
            .request_accounts()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&accounts)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Send Qubic (simple transfer)
    #[wasm_bindgen(js_name = sendQubic)]
    pub async fn send_qubic(
        &self,
        from: String,
        to: String,
        amount: String,
    ) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        let amount_u64 = amount
            .parse::<u64>()
            .map_err(|_| JsValue::from_str("Invalid amount"))?;

        let result = client
            .send_qubic(&from, &to, amount_u64)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Sign a transaction
    #[wasm_bindgen(js_name = signTransaction)]
    pub async fn sign_transaction(&self, params_json: JsValue) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        let params: QubicTransactionParams = serde_wasm_bindgen::from_value(params_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid params: {}", e)))?;

        let result = client
            .sign_transaction(params)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Send a transaction (sign and broadcast)
    #[wasm_bindgen(js_name = sendTransaction)]
    pub async fn send_transaction(&self, params_json: JsValue) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        let params: QubicTransactionParams = serde_wasm_bindgen::from_value(params_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid params: {}", e)))?;

        let result = client
            .send_transaction(params)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Sign a message
    #[wasm_bindgen(js_name = signMessage)]
    pub async fn sign_message(&self, from: String, message: String) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        let result = client
            .sign_message(&from, &message)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Disconnect from wallet
    #[wasm_bindgen]
    pub async fn disconnect(&mut self) -> Result<(), JsValue> {
        let client = self
            .inner
            .as_mut()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        client
            .disconnect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get current session info
    #[wasm_bindgen(js_name = getSession)]
    pub fn get_session(&self) -> Result<JsValue, JsValue> {
        let client = self
            .inner
            .as_ref()
            .ok_or_else(|| JsValue::from_str("Client not initialized"))?;

        match client.get_session() {
            Some(session) => serde_wasm_bindgen::to_value(&session)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
            None => Ok(JsValue::NULL),
        }
    }
}

/// Helper function to create QR code data URI from WalletConnect URI
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = generateQRCode)]
pub fn generate_qr_code(uri: String) -> Result<String, JsValue> {
    let code = QrCode::new(uri.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("QR encode error: {}", e)))?;
    let svg = code.render::<svg::Color>().min_dimensions(256, 256).build();
    let encoded = BASE64_STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", encoded))
}
