use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitialisaAllovrArgs {
    pub founder_1: Pubkey,
    pub founder_2: Pubkey,
    pub founder_3: Pubkey,
    pub founder_4: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, BorshSchema)]
pub enum AllovrInstruction {
    IntialiseAllovr(InitialisaAllovrArgs),
    /// Mint ALLOVR Token
    ///
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer, writable]` Initator (program_id)
    /// 1. `[writable, signer]` Payer
    /// 2. `[writable]` The Program State account with PDA
    /// ... TODO    
    ///     
    MintAllovr,
    /// TODO
    TriggerInflation,
}
