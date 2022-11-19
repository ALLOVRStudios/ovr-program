use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_PREFIX, ALLOVR_AOVR_STAKE_MINIMUM_STAKE};
use crate::errors::AllovrError;
use crate::state::{StakePoolRegistry, StakeMetadata, StakePool};
use crate::utils;

#[derive(Accounts)]
#[instruction(rebalance: bool)]
pub struct WithdrawStake<'info> {
    #[account(        
        seeds = [ALLOVR_AOVR_STAKE_PREFIX.as_ref(), stake_holder.key().as_ref()],
        bump,        
        owner = *program_id)]
    stake: AccountLoader<'info, StakeMetadata>,
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account]
    stake_holder: Signer<'info>,
    clock: Sysvar<'info, Clock>,
    system_program: Program<'info, System>,
}

pub fn handle_withdraw_stake(
    ctx: Context<WithdrawStake>,
    rebalance: bool
) -> Result<()> {



    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;
    let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;
    let stake  = &mut ctx.accounts.stake.load_init()?;

    // check that pool_index on stake matches registered pool
    let registered_pool_option = stake_pool_registry.pools[usize::from(stake.pool_index)];
    require!(registered_pool_option.is_some(), AllovrError::InvalidPoolIndex);
    require_keys_eq!(registered_pool_option.unwrap(), stake_pool.key, AllovrError::InvalidPoolIndex);
    
    // get stake holder balance to see max that can be withdrawn
    let mut staked = stake_pool.stakes[stake.stake_index];
    let pool_staked = stake_pool.total_staked;
    if stake_pool.total_owed > 0 {
        // need to see how much stake_holder is owed
        let share_of_total_percentage = staked / pool_staked;
        let owed_to_stake_holder = share_of_total_percentage * stake_pool.total_owed;        
        staked += owed_to_stake_holder;
    }

    require_lte!(amount, staked, AllovrError::WithdrawalAmountExceedsStakedAmount);

    // record withdrawal request data
    require!(stake::request_withdrawal(amount, ctx.accounts.clock.unix_timestamp).is_ok(), AllovrError::StakeWithdrawalRequestInvalid);

    Ok(())
}