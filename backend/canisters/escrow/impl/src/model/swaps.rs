use escrow_canister::{SwapStatus, SwapStatusAccepted, SwapStatusCancelled, SwapStatusCompleted};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::{icrc1::CompletedCryptoTransaction, CanisterId, TimestampMillis, TokenInfo, UserId};

#[derive(Serialize, Deserialize, Default)]
pub struct Swaps {
    map: BTreeMap<u32, Swap>,
}

impl Swaps {
    pub fn push(&mut self, caller: UserId, args: escrow_canister::create_swap::Args, now: TimestampMillis) -> u32 {
        let id = self.map.last_key_value().map(|(k, _)| *k).unwrap_or_default();
        self.map.insert(id, Swap::new(id, caller, args, now));
        id
    }

    pub fn get(&self, id: u32) -> Option<&Swap> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Swap> {
        self.map.get_mut(&id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    pub id: u32,
    pub created_at: TimestampMillis,
    pub created_by: UserId,
    pub token0: TokenInfo,
    pub amount0: u128,
    pub token1: TokenInfo,
    pub amount1: u128,
    pub expires_at: TimestampMillis,
    pub cancelled_at: Option<TimestampMillis>,
    pub accepted_by: Option<(UserId, TimestampMillis)>,
    pub token0_received: bool,
    pub token1_received: bool,
    pub token0_transfer_out: Option<CompletedCryptoTransaction>,
    pub token1_transfer_out: Option<CompletedCryptoTransaction>,
    pub refunds: Vec<CompletedCryptoTransaction>,
    pub canister_to_notify: Option<CanisterId>,
}

impl Swap {
    pub fn new(id: u32, caller: UserId, args: escrow_canister::create_swap::Args, now: TimestampMillis) -> Swap {
        Swap {
            id,
            created_at: now,
            created_by: caller,
            token0: args.token0,
            amount0: args.token0_amount,
            token1: args.token1,
            amount1: args.token1_amount,
            expires_at: args.expires_at,
            cancelled_at: None,
            accepted_by: None,
            token0_received: false,
            token1_received: false,
            token0_transfer_out: None,
            token1_transfer_out: None,
            refunds: Vec::new(),
            canister_to_notify: args.canister_to_notify,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.token0_transfer_out.is_some() && self.token1_transfer_out.is_some()
    }

    pub fn status(&self, now: TimestampMillis) -> SwapStatus {
        if let Some((accepted_by, accepted_at)) = self.accepted_by {
            if let (Some(token0_transfer_out), Some(token1_transfer_out)) =
                (self.token0_transfer_out.clone(), self.token1_transfer_out.clone())
            {
                SwapStatus::Completed(Box::new(SwapStatusCompleted {
                    accepted_by,
                    accepted_at,
                    token0_transfer_out,
                    token1_transfer_out,
                }))
            } else {
                SwapStatus::Accepted(Box::new(SwapStatusAccepted {
                    accepted_by,
                    accepted_at,
                }))
            }
        } else if let Some(cancelled_at) = self.cancelled_at {
            SwapStatus::Cancelled(Box::new(SwapStatusCancelled { cancelled_at }))
        } else if self.expires_at < now {
            SwapStatus::Expired
        } else {
            SwapStatus::Open
        }
    }
}
