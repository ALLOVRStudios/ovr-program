use anchor_lang::prelude::*;

#[error_code]
pub enum AllovrError {
    #[msg("Invalid ALLOVR State Address")]
    InvalidAllovrStateAddress,
    #[msg("Invalid ALLOVR Mint Address")]
    InvalidAllovrMintAddress,
    #[msg("Pool already exists")]
    PoolAlreadyExists,
    #[msg("Pool index does not match head")]
    PoolIndexDoesNotMatchHead,
    #[msg("Pool index is invalid (allowed: 0 - 99)")]
    InvalidPoolIndex,
    #[msg("Pool address is invalid")]
    InvalidPoolAddress,
    #[msg("Slot index is occupied")]
    SlotIndexOccupied,
    #[msg("Minimum stake amount required")]
    MinimumStakeAmountRequried,
    #[msg("Slot index in invalid")]
    InvalidSlotIndex,
    #[msg("Stake pool registry rebalance is required")]
    StakePoolRegistryRebalanceRequired,
    #[msg("Stake pool rebalance is required")]
    StakePoolRebalanceRequired,
    #[msg("Stake already initialised")]
    StakeAlreadyInitialised,
}
