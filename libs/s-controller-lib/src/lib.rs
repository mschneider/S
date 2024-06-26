use s_controller_interface::{
    LstState, PoolState, RebalanceRecord, SWAP_EXACT_IN_IX_ACCOUNTS_LEN,
    SWAP_EXACT_OUT_IX_ACCOUNTS_LEN,
};
use static_assertions::const_assert_eq;

mod accounts_resolvers;
mod accounts_serde;
mod calc;
mod consts;
mod disable_pool_authority_list;
mod instructions;
mod lst_indexes;
mod lst_state_list;
mod pda;
mod state;
mod u8bool;

pub use accounts_resolvers::*;
pub use accounts_serde::*;
pub use calc::*;
pub use consts::*;
pub use disable_pool_authority_list::*;
pub use instructions::*;
pub use lst_indexes::*;
pub use lst_state_list::*;
pub use pda::*;
pub use state::*;
pub use u8bool::*;

// std::mem::size_of and std::mem::align_of are const fns so we dont technically need these
// but the const asserts helps guard against unexpected size changes

pub const POOL_STATE_SIZE: usize = 176;
const_assert_eq!(std::mem::size_of::<PoolState>(), POOL_STATE_SIZE);
pub const POOL_STATE_ALIGN: usize = 8;
const_assert_eq!(std::mem::align_of::<PoolState>(), POOL_STATE_ALIGN);

pub const LST_STATE_SIZE: usize = 80;
const_assert_eq!(std::mem::size_of::<LstState>(), LST_STATE_SIZE);
pub const LST_STATE_ALIGN: usize = 8;
const_assert_eq!(std::mem::align_of::<LstState>(), LST_STATE_ALIGN);

const_assert_eq!(
    SWAP_EXACT_IN_IX_ACCOUNTS_LEN,
    SWAP_EXACT_OUT_IX_ACCOUNTS_LEN
);

pub const REBALANCE_RECORD_SIZE: usize = 16;
const_assert_eq!(
    std::mem::size_of::<RebalanceRecord>(),
    REBALANCE_RECORD_SIZE
);
pub const REBALANCE_RECORD_ALIGN: usize = 8;
const_assert_eq!(
    std::mem::align_of::<RebalanceRecord>(),
    REBALANCE_RECORD_ALIGN
);

// putting these consts here instead of in consts.rs
// so that we dont forget to update the declare_program_keys!()
// macro below if we change them
// TODO: update declare_program_keys!() to take exprs as seeds
// so that consts can be used

pub const POOL_STATE_PDA_SEED: &[u8] = b"state";
pub const LST_STATE_LIST_PDA_SEED: &[u8] = b"lst-state-list";
pub const DISABLE_POOL_AUTHORITY_LIST_PDA_SEED: &[u8] = b"disable-pool-authority-list";
pub const REBALANCE_RECORD_PDA_SEED: &[u8] = b"rebalance-record";
pub const PROTOCOL_FEE_PDA_SEED: &[u8] = b"protocol-fee";

pub mod program {
    sanctum_macros::declare_program_keys!(
        "5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx",
        [
            ("pool-state", b"state"),
            ("lst-state-list", b"lst-state-list"),
            (
                "disable-pool-authority-list",
                b"disable-pool-authority-list"
            ),
            ("rebalance-record", b"rebalance-record"),
            ("protocol-fee", b"protocol-fee"),
        ]
    );
}
