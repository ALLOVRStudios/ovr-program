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
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct RegisterArtistArgs {
    pub name: String,
    pub description: String,
    pub token_symbol: String,
    pub uri: Option<String>,
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
    /// Register Artist
    ///
    /// Summary: Any account can register as artist exactly once.
    /// 1. 10 000 AOVR is transferred to ALLOVR DAO Treasury
    /// 2. Artist token mint account is created with PDA seeds [ALLOVRARTIST, artist account address] and initialised
    /// 3. 10 000 000 artist tokens are minted to artist account
    /// 4. Artist metadata account is created with PDA seeds [ALLOVRARTISTMETA, artist token mint account address]
    /// 5. Meta data args (see RegisterArtistArgs) are saved to metadata data account
    ///
    /// Accounts expected:
    ///
    /// `[signer,writable]` Artist Account, payer
    /// `[writable]` Artist AOVR Token ATA
    /// `[writable]` Artist's Artist Token ATA (based on Artist Token Mint PDA below)
    /// `[writable]` Artist Token Mint PDA with seeds [ALLOVRARTIST, artist account address]
    /// `[writable]` Artist Token Metadata PDA with seeds [ALLOVRARTISTMETA, artist token mint address]        
    /// `[writable]` Treasury AOVR Token Account (destincation for 10K AOVR)
    /// `[]` Token Program
    /// `[]` ATA Program
    /// `[]` Rent Sysvar    
    /// `[]` System
    RegisterArtist(RegisterArtistArgs),
    /// Update Artist Info
    ///
    /// Accounts expected:
    ///
    /// `[signer]` Artist Wallet
    /// `[writable]` Artist Metadata Account to be updated (PDA with seed prefix ALLOVRARTISTMETA and Artist Token Mint address)   
    UpdateArtist(RegisterArtistArgs),
}
