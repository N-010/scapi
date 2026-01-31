mod sc_api;
pub mod rpc;
#[cfg(not(target_arch = "wasm32"))]
pub mod wallet_connect;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use crate::rpc::*;
pub use crate::sc_api::*;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::wallet_connect::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;