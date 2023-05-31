//! Program instruction processor

use borsh::BorshDeserialize;
use {
    crate::instruction::MusicZInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        sysvar::{clock::Clock, rent::Rent, Sysvar},
    },
};

/// Processor document
pub struct Processor {}

impl Processor {
    /// set operator
    fn process_set_operator(accounts: &[AccountInfo], token: &Pubkey, index: u32) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let stack_info = next_account_info(account_iter);
        Ok(())
    }

    /// pause almagate
    fn process_almagate(accounts: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }

    /// update token
    fn process_update(accounts: &[AccountInfo], offset: u32, data: Vec<u8>) -> ProgramResult {
        Ok(())
    }

    /// pause transfer
    fn process_transfer(accounts: &[AccountInfo], owner: &Pubkey) -> ProgramResult {
        Ok(())
    }

    /// ceate
    fn process_create(accounts: &[AccountInfo]) -> ProgramResult {
        // Create in iterator to safety reference accounts in the slice
        let account_info_iter = &mut accounts.iter();

        // Get the clock sysvar via syscall
        let clock_via_sysvar = Clock::get()?;
        // Or deserialize the account into a clock struct
        let clock_sysvar_info = next_account_info(account_info_iter)?;
        let clock_via_account = Clock::from_account_info(clock_sysvar_info)?;
        // Both produce the same sysvar
        assert_eq!(clock_via_sysvar, clock_via_account);
        // Note: `format!` can be very expensive, use cautiously
        msg!("{:?}", clock_via_sysvar);

        // Get the rent sysvar via syscall
        let rent_via_sysvar = Rent::get()?;
        // Or deserialize the account into a rent struct
        let rent_sysvar_info = next_account_info(account_info_iter)?;
        let rent_via_account = Rent::from_account_info(rent_sysvar_info)?;
        // Both produce the same sysvar
        assert_eq!(rent_via_sysvar, rent_via_account);
        // Can't print `exemption_threshold` because BPF does not support printing floats
        msg!(
            "Rent: lamports_per_byte_year: {:?}, burn_percent: {:?}",
            rent_via_sysvar.lamports_per_byte_year,
            rent_via_sysvar.burn_percent
        );
        Ok(())
    }

    /// Instruction processor
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = MusicZInstruction::try_from_slice(instruction_data)?;
        msg!("Instruction unpacked");
        let account_info_iter = &mut accounts.iter();
        match instruction {
            MusicZInstruction::SetOperator { operator, index } => {
                Self::process_set_operator(accounts, &operator, index)
            }
            MusicZInstruction::Almagate => Self::process_almagate(accounts),
            MusicZInstruction::Update { offset, data } => {
                Self::process_update(accounts, offset, data)
            }
            MusicZInstruction::Transfer { new_owner } => {
                Self::process_transfer(accounts, &new_owner)
            }
            MusicZInstruction::Delete => Ok(()),
        }
    }
}
