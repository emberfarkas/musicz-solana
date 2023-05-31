//! Program state
//!

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{account_info::Account, pubkey::Pubkey};
use std::collections::HashMap;

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Stake {
    stake: HashMap<Pubkey, HashMap<Pubkey, u64>>,
}
