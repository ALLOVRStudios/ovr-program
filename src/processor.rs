use crate::{error::AllovrError, instruction::AllovrInstruction, processors::*};
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
            AllovrInstruction::RegisterArtist(args) => {
                msg!("Register Artist Instruction");
                register_artist::execute(accounts, program_id, args)
            }
            AllovrInstruction::UpdateArtist(args) => {
                msg!("Update Artist Instruction");
                update_artist::execute(accounts, program_id, args)
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
            // Artist
            AllovrError::InvalidArtistTokenAccount => msg!("Error: Invalid Artist Token Account"),
            AllovrError::InvalidArtistName => msg!("Error: Invalid Artist Name"),
            AllovrError::InvalidArtistDescription => msg!("Error: Invalid Artist Description"),
            AllovrError::InvalidArtistSymbol => msg!("Error: Invalid Artist Symbol"),
            AllovrError::InvalidArtistImageUrl => msg!("Error: Invalid Artist Image Url"),
            AllovrError::InvalidArtworkSymbol => msg!("Error: Invalid Artwork Symbol"),
            AllovrError::ArtworkDescriptionMaxLengthExceeded => {
                msg!("Error: Artwork Description Max Length Exceeded")
            }
            AllovrError::InvalidArtworkTokenAccount => msg!("Error: Invalid Artwork Token Account"),
            AllovrError::InvalidSystemProgramId => msg!("Error: Invalid System Program Id"),
            AllovrError::ArtworkUnderOffer => msg!("Error: Artwork Under Offer"),
            AllovrError::IncorrectArtworkBuyer => msg!("Error: Incorrect Artwork Buyer"),
            AllovrError::IncorrectArtworkHolder => msg!("Error: Incorrect Artwork Holder"),
            AllovrError::IncorrectPaymentAccount => msg!("Error: Incorrect Payment Account"),
            AllovrError::InvalidOffer => msg!("Error: Invalid Offer"),
            AllovrError::InflationNotDue => msg!("Error: Inflation Not Due"),
            AllovrError::IncorrectFounderAddress => msg!("Error: Incorrect founder address"),
            AllovrError::ManualFail => msg!("MANUAL FAIL"),
        }
    }
}
