use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AllovrError {
    #[error("ALLOVR Already Minted")]
    AlreadyMinted,
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Missing Required Signature")]
    MissingRequiredSignature,
    #[error("Invalid Initialiser")]
    InvalidInitialiser,
    #[error("Invalid State Account")]
    InvalidStateAccount,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Invalid Token Program")]
    InvalidTokenProgram,
    #[error("Invalid Assoicated Token Account Program")]
    InvalidAssociatedTokenAccountProgram,
    #[error("Incorrect Owner")]
    IncorrectOwner,
    #[error("Incorrect Token Owner or Mint")]
    IncorrectTokenOwnerOrMint,
    #[error("Invalid PDA")]
    InvalidPda,
    #[error("Invalid Program ID")]
    InvalidProgramId,
    #[error("Invalid System Program ID")]
    InvalidSystemProgramId,
    #[error("Invalid Clock Sysvar ID")]
    InvalidClockSysvarId,
    #[error("Invalid ALLOVR Mint")]
    InvalidAllovrMint,
    #[error("Invalid ALLOVR Treasury")]
    InvalidAllovrTreasury,
    #[error("Invalid Associated Token Account")]
    InvalidAssociatedTokenAccount,
    #[error("Inflation Not Due")]
    InflationNotDue,
    #[error("Incorrect founder address")]
    IncorrectFounderAddress,
    #[error("MANUAL FAIL")]
    ManualFail,
}

impl From<AllovrError> for ProgramError {
    fn from(e: AllovrError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for AllovrError {
    fn type_of() -> &'static str {
        "AllovrError"
    }
}
