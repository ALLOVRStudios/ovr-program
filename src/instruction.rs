use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitialisaAllovrArgs {
    pub founder_1: Pubkey,
    pub founder_2: Pubkey,
    pub founder_3: Pubkey,
    pub founder_4: Pubkey,
    pub founder_5: Pubkey,
    pub founder_6: Pubkey,
    pub founder_7: Pubkey,
    pub founder_8: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, BorshSchema)]
pub enum AllovrInstruction {
    /// Initialise ALLOVR Token
    ///
    /// Accounts expected:
    ///
    /// `[writable, signer]` Initator (program_id)
    /// `[writable, signer]` State (PDA with seed prefix ALLOVRSTATE)
    /// `[writable, signer]` Payer
    /// `[writable, signer]` Mint (ALLOVR Mint account with known address ALLM...)
    /// `[]` Mint Authority (PDA with seed prefix ALLOVRMINT)
    /// `[]` Token Program
    /// `[]` Rent Sysvar
    /// `[]` System    
    IntialiseAllovr(InitialisaAllovrArgs),
    /// Mint ALLOVR Token
    ///
    /// Accounts expected:
    ///
    /// `[signer]` Initator (program_id)
    /// `[writable]` State (PDA with seed prefix ALLOVRSTATE)
    /// `[writable, signer]` Payer
    /// `[writable]` Mint (ALLOVR Mint account with known address ALLM...)
    /// `[]` Mint Authority (PDA with seed prefix ALLOVRMINT)
    /// `[writable]` Treasury Token Account
    /// `[writable]` Founder 1 Token Account
    /// `[writable]` Founder 2 Token Account
    /// `[writable]` Founder 3 Token Account
    /// `[writable]` Founder 4 Token Account
    /// `[writable]` Founder 5 Token Account
    /// `[writable]` Founder 6 Token Account
    /// `[writable]` Founder 7 Token Account
    /// `[writable]` Founder 8 Token Account
    /// `[]` Token Program
    /// `[]` Rent Sysvar
    /// `[]` Clock Sysvar
    /// `[]` System    
    MintAllovr,
}
