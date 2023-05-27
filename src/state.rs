use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::clock::UnixTimestamp;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AllovrTokenState {
    pub minted: bool,
    pub next_inflation_due: UnixTimestamp,
    pub inflation_run_count: u32,
    pub founder_1: Pubkey,
    pub founder_2: Pubkey,
    pub founder_3: Pubkey,
    pub founder_4: Pubkey,
    pub founder_5: Pubkey,
    pub founder_6: Pubkey,
    pub founder_7: Pubkey,
    pub founder_8: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ArtistMetadata {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub uri: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ArtworkMetadata {
    pub artist: Pubkey,
    pub symbol: String,
    pub description: String,
    pub holder: Pubkey,
    pub offer_count: u32,
    pub offered_to: Option<Pubkey>,
    pub offer_price: Option<u64>,
    pub offer_currency: Option<Currency>,
    pub payment_account: Option<Pubkey>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub enum Currency {
    SOL,
    AOVR,
}
