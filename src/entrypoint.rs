#![cfg(not(feature = "no-entrypoint"))]

use solana_program::program_error::PrintProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = crate::processor::Processor::process(program_id, accounts, instruction_data)
    {
        error.print::<crate::error::AllovrError>();
        return Err(error);
    }
    Ok(())
}
