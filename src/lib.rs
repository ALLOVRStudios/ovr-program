pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod processors;
pub mod state;
pub mod utils;

// PDA Seed Prefixes
pub const ALLOVR_MINT_SEED: &'static str = "ALLOVRMINT";
pub const ARTIST_SEED_PREFIX: &'static str = "ALLOVRARTIST";
pub const ARTIST_METADATA_SEED_PREFIX: &'static str = "ALLOVRARTISTMETA";
pub const ARTWORK_SEED_PREFIX: &'static str = "ALLOVRARTWORK";
pub const ARTWORK_METADATA_SEED_PREFIX: &'static str = "ALLOVRARTWORKMETA";
pub const ARTWORK_ESCROW_SEED_PREFIX: &'static str = "ALLOVRARTWORKESCROW";

// Known Addresses
pub const ALLOVR_PROGRAM_ID: &'static str = "B6w8UQGNEbujVCvtdMhsEPfjnxd3w8MgiMTx6syAu123";
pub const ALLOVR_MINT_ID: &'static str = "ALLMusFNnKAjg5QdbmcSxPsseERfUb4WFvvxU6zaR337";
pub const ALLOVR_TREASURY_ID: &'static str = "ALLTebv8dpcbxiBdxKmSJ88GDKxhTXVvD9ihsvvnsB55";

pub const AOV_DECIMAL_PLACES: u8 = 9;
pub const MINT_SIZE: usize = 82;
pub const STATE_SIZE: usize = 1 + // minted
    64 + // Next Inflation Due UnixTimestamp
    32 + // Founder 1 Pubkey
    32 + // Founder 2 Pubkey
    32 + // Founder 3 Pubkey
    32; // Founder 4 Pubkey

pub const TOKEN_ACCOUNT_SIZE: usize = 165;
pub const INFLATION_INTERVAL_IN_SECONDS: i64 = 60 * 60; // 604800; // seconds in a week (60 * 60 * 24 * 7);

// Artist Metadata
pub const ARTIST_METADATA_NAME_SIZE: usize = 50;
pub const ARTIST_METADATA_COUNTRY_SIZE: usize = 50;
pub const ARTIST_METADATA_SIZE: usize = ARTIST_METADATA_NAME_SIZE + ARTIST_METADATA_COUNTRY_SIZE;

// Artwork Metadata
pub const ARTWORK_METADATA_SYMBOL_SIZE: usize = 3;
pub const ARTWORK_METADATA_DESCRIPTION_SIZE: usize = 256;
pub const ARTWORK_METADATA_SIZE: usize = ARTWORK_METADATA_SYMBOL_SIZE + // Symbol (ABC)
    ARTWORK_METADATA_DESCRIPTION_SIZE + // Description
    32 + // Holder Pubkey
    1 + 32 + // Option (Offered to Pubkey)     
    1 + 8 + // Option (Offer Price)    
    1 + 1; // Option (Offer Currency);

solana_program::declare_id!("ALLu9MhcZ8WbN86fBjgsyhrqZpWTb1Y3TpByf3tFCjG7");
