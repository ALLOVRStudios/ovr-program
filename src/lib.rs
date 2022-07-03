use core::mem::size_of;
use solana_program::clock::UnixTimestamp;
use solana_program::pubkey::Pubkey;

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod processors;
pub mod state;
pub mod utils;

// PDA Seed Prefixes
pub const ALLOVR_MINT_SEED_PREFIX: &'static str = "ALLOVRMINT";
pub const ARTIST_SEED_PREFIX: &'static str = "ALLOVRARTIST";
pub const ARTIST_METADATA_SEED_PREFIX: &'static str = "ALLOVRARTISTMETA";
pub const ARTWORK_SEED_PREFIX: &'static str = "ALLOVRARTWORK";
pub const ARTWORK_METADATA_SEED_PREFIX: &'static str = "ALLOVRARTWORKMETA";
pub const ARTWORK_ESCROW_SEED_PREFIX: &'static str = "ALLOVRARTWORKESCROW";

// Known Addresses
pub const ALLOVR_PROGRAM_ID: &'static str = "9Le3FfZ7g6c11EzTtSWie45XLVp2La9AcFhFqWbBWMvs";
pub const ALLOVR_MINT_ID: &'static str = "97hKUPwp1iG3NqKNko8yagkBH75voGptj4BBvj9Ccjdu";
pub const ALLOVR_STATE_ID: &'static str = "AsLGvz3VzoTRFTa83sJD6xG57M8pEoP1sV4MsYhdyvsc";
pub const ALLOVR_AOVR_TREASURY_ID: &'static str = "HBuFrepbLok3Q6UMiqbGanWed8YUgMNKGYrvSQ64n6jt";

pub const ALL_DECIMAL_PLACES: u8 = 9;
pub const MINT_SIZE: usize = 82;
pub const STATE_SIZE: usize = size_of::<bool>() + // minted
    size_of::<UnixTimestamp>() + // Next Inflation Due UnixTimestamp
    size_of::<u32>() + // Inflation Run Count
    (8 * size_of::<Pubkey>()); // Founder Pubkey * 8
pub const TOKEN_ACCOUNT_SIZE: usize = 165;
pub const INFLATION_INTERVAL_IN_SECONDS: i64 = 604800; // seconds in a week (60 * 60 * 24 * 7);

// Artist Metadata
pub const ARTIST_METADATA_NAME_SIZE: usize = 50;
pub const ARTIST_METADATA_DESCRIPTION_SIZE: usize = 100;
pub const ARTIST_METADATA_SYMBOL_SIZE: usize = 4;
pub const ARTIST_METADATA_IMAGE_URL_SIZE: usize = 100;
pub const ARTIST_METADATA_SIZE: usize = ARTIST_METADATA_NAME_SIZE
    + ARTIST_METADATA_DESCRIPTION_SIZE
    + ARTIST_METADATA_SYMBOL_SIZE
    + ARTIST_METADATA_IMAGE_URL_SIZE;

// Artwork Metadata
pub const ARTWORK_METADATA_SYMBOL_SIZE: usize = 3;
pub const ARTWORK_METADATA_DESCRIPTION_SIZE: usize = 256;
pub const ARTWORK_METADATA_SIZE: usize = ARTWORK_METADATA_SYMBOL_SIZE + // Symbol (ABC)
    ARTWORK_METADATA_DESCRIPTION_SIZE + // Description
    size_of::<Pubkey>() + // Holder Pubkey
    size_of::<bool>() + size_of::<Pubkey>() + // Option (Offered to Pubkey)     
    size_of::<bool>() + size_of::<u64>() + // Option (Offer Price)    
    size_of::<bool>() + 1; //size_of::<Currency>(); // Option (Offer Currency);

solana_program::declare_id!("4ujXmUcCa8upcfy9u8CJsxoSfGRuTMw7eZvTxkPEH4Ae");
