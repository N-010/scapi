pub mod errors;
mod impls;
pub mod traits;

/// 64 byte SchnorrQ signature type
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signature(pub [u8; 64]);

impl Default for Signature {
    fn default() -> Self {
        Self([0; 64])
    }
}

/// Represents a Qubic ID containing only the decoded public key
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct QubicId(pub [u8; 32]);

/// Represents a Qubic wallet containing private key, subseed and public key of the corresponding wallet
#[derive(Debug, Clone, Copy, Default)]
pub struct QubicWallet {
    private_key: [u8; 32],
    subseed: [u8; 32],
    pub public_key: QubicId,
}

pub use impls::*;
