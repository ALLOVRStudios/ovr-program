use crate::constants::ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX;
use crate::errors::AllovrError;
use crate::state::{StakePool, StakePoolRegistry};
use crate::utils;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(pool_index: u8)]
pub struct RebalanceStakingPool<'info> {
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_rebalance_staking_pool(
    ctx: Context<RebalanceStakingPool>,
    pool_index: u8,
) -> Result<()> {
    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;
    let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;
    require!(
        stake_pool_registry
            .require_stake_pool_address_at_index(
                usize::from(pool_index),
                ctx.accounts.stake_pool.key()
            )
            .is_ok(),
        AllovrError::InvalidPoolAddress
    );

    utils::rebalance(stake_pool_registry, stake_pool, pool_index)
}
