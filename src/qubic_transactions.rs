use core::ptr::read_unaligned;

use tiny_keccak::{Hasher, IntoXof, KangarooTwelve, Xof};

use crate::qubic_types::{
    errors::ByteEncodingError,
    traits::{FromBytes, GetSigner, Sign, ToBytes},
    QubicId, QubicWallet, Signature,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RawTransaction {
    pub from: QubicId,
    pub to: QubicId,
    pub amount: u64,
    pub tick: u32,
    pub input_type: u16,
    pub input_size: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TransactionData {
    #[default]
    None,
    Unknown(Vec<u8>),
}

impl ToBytes for TransactionData {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            TransactionData::Unknown(d) => d.clone(),
            TransactionData::None => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TransactionWithData {
    pub raw_transaction: RawTransaction,
    pub data: TransactionData,
    pub signature: Signature,
}

impl ToBytes for TransactionWithData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data = self.raw_transaction.to_bytes();
        data.extend(self.data.to_bytes());
        data.extend(self.signature.to_bytes());
        data
    }
}

impl FromBytes for TransactionWithData {
    fn from_bytes(data: &[u8]) -> Result<Self, ByteEncodingError> {
        let min = core::mem::size_of::<RawTransaction>() + core::mem::size_of::<Signature>();
        if data.len() < min {
            return Err(ByteEncodingError::InvalidMinimumDataLength {
                expected_min: min,
                found: data.len(),
            });
        }

        let raw_tx = unsafe { read_unaligned(data.as_ptr() as *const RawTransaction) };

        let sig = unsafe {
            read_unaligned(
                &data[data.len() - core::mem::size_of::<Signature>()] as *const u8
                    as *const Signature,
            )
        };

        let tx_data = data[core::mem::size_of::<RawTransaction>()
            ..data.len() - core::mem::size_of::<Signature>()]
            .to_vec();

        let data = if tx_data.is_empty() {
            TransactionData::None
        } else {
            TransactionData::Unknown(tx_data)
        };

        Ok(Self {
            raw_transaction: raw_tx,
            data,
            signature: sig,
        })
    }
}

impl GetSigner for TransactionWithData {
    fn get_signer(&self) -> &QubicId {
        &self.raw_transaction.from
    }
}

impl From<TransactionWithData> for [u8; 32] {
    fn from(val: TransactionWithData) -> Self {
        let mut hash = [0; 32];
        let tx_bytes = val.to_bytes();
        let mut kg = KangarooTwelve::new(b"");
        kg.update(&tx_bytes);
        kg.into_xof().squeeze(&mut hash);
        hash
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransactionBuilder<'a> {
    raw_tx: RawTransaction,
    data: TransactionData,
    signer: Option<&'a QubicWallet>,
}

impl<'a> TransactionBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_from_id(mut self, from: QubicId) -> Self {
        self.raw_tx.from = from;
        self
    }

    pub fn with_to_id(mut self, to: QubicId) -> Self {
        self.raw_tx.to = to;
        self
    }

    pub fn with_amount(mut self, amount: u64) -> Self {
        self.raw_tx.amount = amount;
        self
    }

    pub fn with_tx_data(mut self, data: TransactionData) -> Self {
        self.data = data;
        self
    }

    pub fn with_tick(mut self, tick: u32) -> Self {
        self.raw_tx.tick = tick;
        self
    }

    pub fn with_input_type_and_size(mut self, input_type: u16, input_size: u16) -> Self {
        self.raw_tx.input_type = input_type;
        self.raw_tx.input_size = input_size;
        self
    }

    pub fn with_signing_wallet(mut self, wallet: &'a QubicWallet) -> Self {
        self.signer = Some(wallet);
        self
    }

    pub fn build(mut self) -> TransactionWithData {
        if let Some(signer) = self.signer {
            self.raw_tx.from = signer.public_key;
            let mut tx = TransactionWithData {
                raw_transaction: self.raw_tx,
                data: self.data,
                signature: Signature::default(),
            };
            tx.sign(signer).unwrap();
            tx
        } else {
            TransactionWithData {
                raw_transaction: self.raw_tx,
                data: self.data,
                signature: Signature::default(),
            }
        }
    }
}
