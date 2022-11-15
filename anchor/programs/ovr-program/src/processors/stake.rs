use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_PREFIX, ALLOVR_AOVR_STAKE_MINIMUM_STAKE};
use crate::errors::AllovrError;
use crate::state::{StakePoolRegistry, StakeMetadata, StakePool};
use crate::utils;

#[derive(Accounts)]
#[instruction(pool_index: u8, slot_index: u8, amount: u64, rebalance: bool)]
pub struct Stake<'info> {
    #[account(
        init_if_needed, 
        seeds = [ALLOVR_AOVR_STAKE_PREFIX.as_ref(), initialiser.key().as_ref()],
        bump,
        payer = initialiser, 
        owner = *program_id, 
        space = size_of::<Stake>() + 16)]
    stake: AccountLoader<'info, StakeMetadata>,
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account(mut)]
    initialiser: Signer<'info>,
    clock: Sysvar<'info, Clock>,
    system_program: Program<'info, System>,
}

pub fn handle_stake(
    ctx: Context<Stake>,
    pool_index: u8, slot_index: u8, amount: u64, rebalance: bool
) -> Result<()> {

    require_gte!(amount, ALLOVR_AOVR_STAKE_MINIMUM_STAKE, AllovrError::MinimumStakeAmountRequried);

    // check balance of AOVR
    // transfer AOVR to treasury

    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;
    // require_eq!(stake_pool_registry.total_owed, 0, AllovrError::StakePoolRegistryRebalanceRequired);

    require!(
        stake_pool_registry
            .require_stake_pool_address_at_index(
                usize::from(pool_index),
                ctx.accounts.stake_pool.key()
            )
            .is_ok(),
        AllovrError::InvalidPoolAddress
    );

    // check that pool_index points a registered pool
    let registered_pool_option = stake_pool_registry.pools[usize::from(pool_index)];
    require!(registered_pool_option.is_some(), AllovrError::InvalidPoolIndex);

    // check that pool exists in pool registry and matched passed in address
    let mut registered_pool = registered_pool_option.unwrap();

    // check that the slot is not occupied
    let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;
    require_eq!(stake_pool.stakes[usize::from(slot_index)], 0, AllovrError::SlotIndexOccupied);

    if rebalance {
        require!(utils::rebalance(stake_pool_registry, stake_pool, pool_index).is_ok(), AllovrError::StakePoolRegistryRebalanceRequired);
    }

    require_eq!(registered_pool.total_owed, 0, AllovrError::StakePoolRebalanceRequired);

    let stake  = &mut ctx.accounts.stake.load_init()?;

    if stake.initialised_date == 0 {
        // first stake
        require!(stake.init(pool_index, slot_index, ctx.accounts.clock.unix_timestamp).is_ok(), AllovrError::StakeAlreadyInitialised);
    } else {
        // ensure correct caller
        require_eq!(stake.pool_index, pool_index, AllovrError::InvalidPoolIndex);
        require_eq!(stake.slot_index, slot_index, AllovrError::InvalidSlotIndex);
    }

    // update pool
    stake_pool.staked += amount;
    stake_pool.stakes[usize::from(slot_index)] += amount;

    stake_pool_registry.total_staked += amount;
    registered_pool.total_staked += amount;
    stake_pool_registry.pools[usize::from(pool_index)] = Some(registered_pool);

    Ok(())
}