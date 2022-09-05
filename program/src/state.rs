//! Program state
//! 

use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Stake {
}
