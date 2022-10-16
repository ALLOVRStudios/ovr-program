use crate::constants::{ALLOVR_AOVR_STAKE_NUM_POOLS, ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL};
use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Debug)]
pub struct StakePoolRegistry {
    pub total_staked: u64,
    pub total_owed: u64,
    pub pool_head: u8,
    pub pools: [Option<StakePoolInfo>; ALLOVR_AOVR_STAKE_NUM_POOLS],
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

#[account(zero_copy)]
pub struct StakePool {
    pub staked: u64,
    pub owed: u64,
    pub stakes: [u64; ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL],
}
