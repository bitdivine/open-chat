use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_candid_and_msgpack;
use canister_tracing_macros::trace;
use escrow_canister::cancel_swap::{Response::*, *};

#[update_candid_and_msgpack]
#[trace]
fn cancel_swap(args: Args) -> Response {
    mutate_state(|state| cancel_swap_impl(args, state))
}

fn cancel_swap_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(swap) = state.data.swaps.get_mut(args.swap_id) {
        let user_id = state.env.caller().into();
        let now = state.env.now();
        if swap.created_by != user_id {
            NotAuthorized
        } else if swap.accepted_by.is_some() {
            SwapAlreadyAccepted
        } else if swap.expires_at < now {
            SwapExpired
        } else {
            if swap.cancelled_at.is_none() {
                swap.cancelled_at = Some(now);
            }
            Success
        }
    } else {
        SwapNotFound
    }
}
