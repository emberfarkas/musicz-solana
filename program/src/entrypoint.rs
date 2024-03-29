//! Program entrypoint

#![cfg(not(feature = "no-entrypoint"))]

use {
    crate::processor::Processor,
    solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print i
        return Err(error);
    }
    Ok(())
}
