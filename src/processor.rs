use crate::{
    error::AllovrError, instruction::AllovrInstruction, processors::mint_allovr, processors::*,
};
use solana_program::borsh::try_from_slice_unchecked;

use num_traits::FromPrimitive;

use solana_program::decode_error::DecodeError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::PrintProgramError,
    pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction: AllovrInstruction = try_from_slice_unchecked(input)?;
        match instruction {
            AllovrInstruction::IntialiseAllovr(args) => {
                msg!("Intialise ALLOVR Instruction");
                initialise::execute(accounts, program_id, args)
            }
            AllovrInstruction::MintAllovr => {
                msg!("Mint ALLOVR Instruction");
                mint_allovr::execute(accounts, program_id)
            }
            AllovrInstruction::TriggerInflation => {
                msg!("Trigger Inflation Instruction");
                trigger_inflation::execute(accounts, program_id)
            }
        }
    }
}

impl PrintProgramError for AllovrError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            AllovrError::InvalidInstruction => msg!("Error: Invalid instruction"),
            AllovrError::MissingRequiredSignature => msg!("Error: Missing Required Signature"),
            AllovrError::AlreadyMinted => msg!("Error: ALLOVR Already Minted"),
            AllovrError::InvalidInitialiser => msg!("Error: Invalid Initialiser"),
            AllovrError::InvalidStateAccount => msg!("Error: Invalid State Account"),
            AllovrError::NotRentExempt => msg!("Error: Not Rent Exempt"),
            AllovrError::InvalidTokenProgram => msg!("Error: Invalid Token Program"),
            AllovrError::IncorrectOwner => msg!("Error: Incorrect Owner"),
            AllovrError::InvalidPda => msg!("Error: Invalid PDA"),
            AllovrError::InvalidProgramId => msg!("Error: Invalid Program ID"),
            AllovrError::InvalidAllovrMint => msg!("Error: Invalid ALLOVR Mint"),
            AllovrError::InvalidAllovrTreasury => msg!("Error: Invalid ALLOVR Treasury"),
            AllovrError::InvalidAssociatedTokenAccountProgram => msg!("Error: Invalid ATA Program"),
            AllovrError::InvalidAssociatedTokenAccount => msg!("Error: Invalid ATA"),
            AllovrError::InvalidClockSysvarId => msg!("Error: Invalid Clock Sysvar Id"),
            AllovrError::IncorrectTokenOwnerOrMint => msg!("Error: Incorrect Token Owner or Mint"),
            AllovrError::InvalidSystemProgramId => msg!("Error: Invalid System Program Id"),
            AllovrError::InflationNotDue => msg!("Error: Inflation Not Due"),
            AllovrError::IncorrectFounderAddress => msg!("Error: Incorrect founder address"),
            AllovrError::ManualFail => msg!("MANUAL FAIL"),
        }
    }
}
