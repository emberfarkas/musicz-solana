//! Program instruction

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

/// Instructions supported by the generic Name Registry program
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum MusicZInstruction {
    /// SetOperator an empty name record
    SetOperator {
        /// SHA256 of the (HASH_PREFIX + Name) of the record to create, hashing is done off-chain
        operator: Pubkey,

        /// Number of bytes of memory to allocate in addition to the `NameRecordHeader`
        index: u32,
    },

    /// Almagate 质押
    Almagate,

    /// Update the data in a name record
    ///
    /// Accounts expected by this instruction:
    ///   * If account class is `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Account owner
    ///
    ///   * If account class is not `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Account class
    ///
    ///   * If the signer is the parent name account owner
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Parent name account owner
    ///   2. `[]` Parent name record
    Update {
        /// 偏移值
        offset: u32,
        /// 数据
        data: Vec<u8>,
    },

    /// Transfer ownership of a name record
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * If account class is `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///
    ///   * If account class is not `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///   2. `[signer]` Account class
    ///
    ///    * If the signer is the parent name account owner
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///   2. `[signer]` Account class
    ///   3. `[]` Parent name record
    Transfer {
        /// 所有者
        new_owner: Pubkey,
    },

    /// Delete a name record.
    ///
    /// Any lamports remaining in the name record will be transferred to the refund account (#2)
    ///
    /// Accounts expected by this instruction:
    ///   0. `[writeable]` Name record to be deleted
    ///   1. `[signer]` Account owner
    ///   2. `[writeable]` Refund account
    ///
    Delete,
}
