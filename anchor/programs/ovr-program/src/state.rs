use crate::{
    constants::{ALLOVR_AOVR_STAKE_NUM_POOLS, ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL},
    errors::AllovrError,
};
use anchor_lang::prelude::*;

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\
/// Stake Pool Registry
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\

#[account(zero_copy)]
#[derive(Debug)]
pub struct StakePoolRegistry {
    pub total_staked: u64,
    pub pool_head: u8,
    pub pools: [Option<StakePoolInfo>; ALLOVR_AOVR_STAKE_NUM_POOLS],
}

impl StakePoolRegistry {
    pub fn init(&mut self) {
        self.total_staked = 0;
        self.pool_head = 0;
    }

    pub fn require_stake_pool_address_at_index(
        &self,
        pool_index: usize,
        address: Pubkey,
    ) -> Result<()> {
        let registered_pool_option = self.pools[pool_index];
        require!(
            registered_pool_option.is_some(),
            AllovrError::InvalidPoolIndex
        );

        // check that pool exists in pool registry and matches passed in address
        require_keys_eq!(
            registered_pool_option.unwrap().pool_address,
            address,
            AllovrError::InvalidPoolAddress
        );
        Ok(())
    }
}

#[zero_copy]
#[repr(packed)]
#[derive(Debug, Default)]
pub struct StakePoolInfo {
    pub total_staked: u64,
    pub total_owed: u64,
    pub pool_address: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct RpcStakePoolInfo {
    total_staked: u64,
    total_owed: u64,
    pool_address: Pubkey,
}

impl From<RpcStakePoolInfo> for StakePoolInfo {
    fn from(e: RpcStakePoolInfo) -> StakePoolInfo {
        StakePoolInfo {
            total_staked: e.total_staked,
            total_owed: e.total_owed,
            pool_address: e.pool_address,
        }
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\
/// Stake Pool
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\

#[account(zero_copy)]
pub struct StakePool {
    pub total_staked: u64,
    pub stakes: [u64; ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL],
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\
/// Stake
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\

#[account(zero_copy)]
#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct StakeMetadata {
    pub initialised_date: i64, // (seconds since the Unix epoch)
    pub pool_index: u8,
    pub slot_index: u8,
    pub withdrawal_request: u64,
    pub withdrawal_request_date: Option<i64>, // (seconds since the Unix epoch)
}

impl StakeMetadata {
    pub fn init(&mut self, pool_index: u8, slot_index: u8, timestamp: i64) -> Result<()> {
        require_eq!(
            self.initialised_date,
            0,
            AllovrError::StakeAlreadyInitialised
        );
        self.pool_index = pool_index;
        self.slot_index = slot_index;
        self.withdrawal_request = 0;
        self.initialised_date = timestamp;
        self.withdrawal_request_date = None;
        Ok(())
    }

    pub fn request_withdrawal(&mut self, amount: u64, timestamp: i64) -> Result<()> {
        require_neq!(self.initialised_date, 0, AllovrError::NoStakeExists);

        // don't care if there was already a withdrawal request, overwrite
        self.withdrawal_request = amount;
        self.withdrawal_request_date = Some(timestamp);
        Ok(())
    }

    pub fn cancel_withdrawal(&mut self) -> Result<()> {
        self.withdrawal_request = 0;
        self.withdrawal_request_date = None;
        Ok(())
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\
/// AOVR Token
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ \\\

#[account]
pub struct AllovrTokenState {
    pub minted: bool,
    pub next_inflation_due: i64,
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

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct InitAovrArgs {
    pub founder_1: Pubkey,
    pub founder_2: Pubkey,
    pub founder_3: Pubkey,
    pub founder_4: Pubkey,
    pub founder_5: Pubkey,
    pub founder_6: Pubkey,
    pub founder_7: Pubkey,
    pub founder_8: Pubkey,
}
