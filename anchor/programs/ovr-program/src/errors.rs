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
    #[msg("No stake")]
    NoStakeExists,
    #[msg("Minimum stake amount required")]
    MinimumStakeAmountRequried,
    #[msg("Minimum stake withdrawal amount must be more than 0")]
    MinimumStakeWithdrawalAmountMoreThanZero,
    #[msg("Withdrawal Amount Exceeds Staked Amount")]
    WithdrawalAmountExceedsStakedAmount,
    #[msg("Slot index in invalid")]
    InvalidSlotIndex,
    #[msg("Stake Withdrawal Request Invalid")]
    StakeWithdrawalRequestInvalid,
    #[msg("Stake pool registry rebalance is required")]
    StakePoolRegistryRebalanceRequired,
    #[msg("Stake pool rebalance is required")]
    StakePoolRebalanceRequired,
    #[msg("Stake already initialised")]
    StakeAlreadyInitialised,
    #[msg("AOVR not minted")]
    AovrNotMinted,
    #[msg("AOVR already minted")]
    AovrAlreadyMinted,
    #[msg("AOVR inflation not due")]
    AovrInflationNotDue,
    #[msg("Insufficient AOVR balance")]
    InsufficientAovrBalance,
    // This is reserved for situations that should never happen, like finding that the head of the pool registry no longer points to the correct pool index
    #[msg("Fatal Error")]
    FatalError,
}
