//! Program entrypoint

#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use crate::{error::StakePoolError, processor::Processor};
use solana_program::{ msg,
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("-----process_instruction");

    Ok(())
}
