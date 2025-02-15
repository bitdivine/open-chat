use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{CanisterId, TimestampMillis, TokenInfo};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token0: TokenInfo,
    pub token0_amount: u128,
    pub token1: TokenInfo,
    pub token1_amount: u128,
    pub expires_at: TimestampMillis,
    pub canister_to_notify: Option<CanisterId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    InvalidSwap(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub id: u32,
}
