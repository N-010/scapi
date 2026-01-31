pub use crate::qubic_transactions::{
    RawTransaction, TransactionBuilder, TransactionData, TransactionWithData,
};
pub use crate::qubic_types::errors::QubicError;
pub use crate::qubic_types::traits::{Sign, ToBytes, VerifySignature};
pub use crate::qubic_types::{QubicId, QubicWallet, Signature};

#[derive(Debug, Clone)]
pub struct TransactionParams {
    pub from: QubicId,
    pub to: QubicId,
    pub amount: u64,
    pub tick: u32,
    pub input_type: u16,
    pub payload: Vec<u8>,
}

impl TransactionParams {
    pub fn new(from: QubicId, to: QubicId, amount: u64) -> Self {
        Self {
            from,
            to,
            amount,
            tick: 0,
            input_type: 0,
            payload: Vec::new(),
        }
    }

    pub fn with_tick(mut self, tick: u32) -> Self {
        self.tick = tick;
        self
    }

    pub fn with_input_type(mut self, input_type: u16) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("payload too large (max {max}, found {found})")]
    PayloadTooLarge { max: usize, found: usize },

    #[error("signer mismatch (expected {expected}, found {found})")]
    SignerMismatch { expected: QubicId, found: QubicId },

    #[error(transparent)]
    Qubic(#[from] QubicError),
}

pub type TransactionResult<T> = Result<T, TransactionError>;

pub fn build_unsigned_transaction(
    params: &TransactionParams,
) -> TransactionResult<TransactionWithData> {
    ensure_payload_size(&params.payload)?;

    let tx = TransactionBuilder::new()
        .with_from_id(params.from)
        .with_to_id(params.to)
        .with_amount(params.amount)
        .with_tick(params.tick)
        .with_input_type_and_size(params.input_type, params.payload.len() as u16)
        .with_tx_data(TransactionData::Unknown(params.payload.clone()))
        .build();

    Ok(tx)
}

pub fn build_signed_transaction(
    params: &TransactionParams,
    wallet: &QubicWallet,
) -> TransactionResult<TransactionWithData> {
    ensure_payload_size(&params.payload)?;

    if params.from != wallet.public_key {
        return Err(TransactionError::SignerMismatch {
            expected: wallet.public_key,
            found: params.from,
        });
    }

    let tx = TransactionBuilder::new()
        .with_to_id(params.to)
        .with_amount(params.amount)
        .with_tick(params.tick)
        .with_input_type_and_size(params.input_type, params.payload.len() as u16)
        .with_tx_data(TransactionData::Unknown(params.payload.clone()))
        .with_signing_wallet(wallet)
        .build();

    Ok(tx)
}

pub fn sign_transaction(
    mut tx: TransactionWithData,
    wallet: &QubicWallet,
) -> TransactionResult<TransactionWithData> {
    tx.sign(wallet)?;
    Ok(tx)
}

pub fn transaction_bytes(tx: &TransactionWithData) -> Vec<u8> {
    tx.to_bytes()
}

pub fn build_signed_transaction_bytes(
    params: &TransactionParams,
    wallet: &QubicWallet,
) -> TransactionResult<Vec<u8>> {
    let tx = build_signed_transaction(params, wallet)?;
    Ok(transaction_bytes(&tx))
}

pub fn build_signed_transaction_bytes_from_seed(
    seed: &str,
    params: &TransactionParams,
) -> TransactionResult<Vec<u8>> {
    let wallet = QubicWallet::from_seed(seed)?;
    build_signed_transaction_bytes(params, &wallet)
}

pub fn build_ticket_tx_bytes_from_seed(
    seed: &str,
    to: QubicId,
    amount: u64,
    tick: u32,
    input_type: u16,
    payload: Vec<u8>,
) -> TransactionResult<Vec<u8>> {
    let wallet = QubicWallet::from_seed(seed)?;
    let params = TransactionParams::new(wallet.public_key, to, amount)
        .with_tick(tick)
        .with_input_type(input_type)
        .with_payload(payload);
    build_signed_transaction_bytes(&params, &wallet)
}

pub fn build_ticket_tx_bytes(
    wallet: &QubicWallet,
    to: QubicId,
    amount: u64,
    tick: u32,
    input_type: u16,
    payload: Vec<u8>,
) -> TransactionResult<Vec<u8>> {
    let params = TransactionParams::new(wallet.public_key, to, amount)
        .with_tick(tick)
        .with_input_type(input_type)
        .with_payload(payload);
    build_signed_transaction_bytes(&params, wallet)
}

fn ensure_payload_size(payload: &[u8]) -> TransactionResult<()> {
    if payload.len() > u16::MAX as usize {
        return Err(TransactionError::PayloadTooLarge {
            max: u16::MAX as usize,
            found: payload.len(),
        });
    }
    Ok(())
}
