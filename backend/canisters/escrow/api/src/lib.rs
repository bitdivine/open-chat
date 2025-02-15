use candid::{Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::Serialize;
use sha256::sha256;
use types::icrc1::CompletedCryptoTransaction;
use types::{TimestampMillis, UserId};

mod lifecycle;
mod updates;

pub use lifecycle::*;
pub use updates::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum SwapStatus {
    Open,
    Cancelled(Box<SwapStatusCancelled>),
    Expired,
    Accepted(Box<SwapStatusAccepted>),
    Completed(Box<SwapStatusCompleted>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapStatusCancelled {
    pub cancelled_at: TimestampMillis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapStatusAccepted {
    pub accepted_by: UserId,
    pub accepted_at: TimestampMillis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapStatusCompleted {
    pub accepted_by: UserId,
    pub accepted_at: TimestampMillis,
    pub token0_transfer_out: CompletedCryptoTransaction,
    pub token1_transfer_out: CompletedCryptoTransaction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapStatusChange {
    pub swap_id: u32,
    pub status: SwapStatus,
}

pub fn deposit_subaccount(user_id: UserId, swap_id: u32) -> Subaccount {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(Principal::from(user_id).as_slice());
    bytes.extend_from_slice(&swap_id.to_be_bytes());
    sha256(&bytes)
}
