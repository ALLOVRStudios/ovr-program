use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_PREFIX, ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_NUM_POOLS};
use crate::errors::AllovrError;
use crate::state::{StakePool, StakePoolRegistry, StakePoolInfo};

#[derive(Accounts)]
#[instruction(pool_index: u8)]
pub struct RegisterStakingPool<'info> {
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(
        init, 
        seeds = [ALLOVR_AOVR_STAKE_POOL_PREFIX.as_ref(), payer.key().as_ref(), &[pool_index.try_into().unwrap()]],
        bump,
        payer = payer, 
        owner = *program_id, 
        space = size_of::<StakePool>() + 16)]
    stake_pool: AccountLoader<'info, StakePool>,    
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_register_staking_pool(
    ctx: Context<RegisterStakingPool>,
    pool_index: u8
) -> Result<()> {

    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;

    require!(stake_pool_registry.pool_head == pool_index, AllovrError::PoolIndexDoesNotMatchHead);
    require!(stake_pool_registry.pools[usize::from(pool_index)].is_none(), AllovrError::PoolAlreadyExists);
    require!(usize::from(pool_index) < ALLOVR_AOVR_STAKE_NUM_POOLS, AllovrError::InvalidPoolIndex);

    let stake_pool = &mut ctx.accounts.stake_pool.load_init()?;
    stake_pool.total_staked = 0;

    let stake_pool_info = StakePoolInfo {
        total_owed: 0,
        total_staked: 0,
        pool_address: ctx.accounts.stake_pool.key()
    };

    stake_pool_registry.pools[usize::from(pool_index)] = Some(stake_pool_info);
    stake_pool_registry.pool_head = pool_index + 1;
    Ok(())
}