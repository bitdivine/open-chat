use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::{AccessGate, BuildVersion, CanisterId, CommunityPermissions, Document, Milliseconds, Rules, SourceGroup, UserId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub rules: Rules,
    pub avatar: Option<Document>,
    pub banner: Option<Document>,
    pub permissions: CommunityPermissions,
    pub primary_language: String,
    pub created_by_principal: Principal,
    pub created_by_user_id: UserId,
    pub mark_active_duration: Milliseconds,
    pub user_index_canister_id: CanisterId,
    pub local_user_index_canister_id: CanisterId,
    pub group_index_canister_id: CanisterId,
    pub local_group_index_canister_id: CanisterId,
    pub notifications_canister_id: CanisterId,
    pub proposals_bot_user_id: UserId,
    pub escrow_canister_id: CanisterId,
    pub gate: Option<AccessGate>,
    pub default_channels: Vec<String>,
    pub default_channel_rules: Option<Rules>,
    pub source_group: Option<SourceGroup>,
    pub wasm_version: BuildVersion,
    pub test_mode: bool,
}
