use anchor_lang::prelude::*;
use crate::constants::{ALLOVR_AOVR_STAKE_MINIMUM_STAKE, ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_PREFIX};
use crate::errors::AllovrError;
use crate::state::{StakePoolRegistry, StakeMetadata, StakePool};
use crate::utils;
use crate::known_addresses::KnownAddress;
use anchor_spl::token::{TokenAccount, Transfer, Token};
use std::mem::size_of;

/// Handles initialising the stake data account if it is not there, and staking the first and all subsequent times 
#[derive(Accounts)]
#[instruction(amount: u64, pool_index: u8, slot_index: u8, rebalance: bool)]
pub struct Stake<'info> {
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account(
        init_if_needed, 
        seeds = [ALLOVR_AOVR_STAKE_PREFIX.as_ref(), stake_holder.key().as_ref()],
        bump,
        payer = stake_holder, 
        owner = *program_id, 
        space = size_of::<StakeMetadata>() + 16)]
    stake: AccountLoader<'info, StakeMetadata>,
    #[account(mut)]
    stake_holder: Signer<'info>,
    #[account(mut, token::mint = KnownAddress::allovr_mint())]
    stake_holder_aovr: Account<'info, TokenAccount>,
    #[account(mut, token::mint = KnownAddress::allovr_mint())]
    aovr_treasury: Account<'info, TokenAccount>,
    clock: Sysvar<'info, Clock>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handle_stake(
    ctx: Context<Stake>,
    amount: u64,
    pool_index: u8,
    slot_index: u8,
    rebalance: bool
) -> Result<()> {

    require_gt!(amount, ALLOVR_AOVR_STAKE_MINIMUM_STAKE, AllovrError::MinimumStakeAmountRequried);

    // check balance of AOVR
    require_gte!(ctx.accounts.stake_holder_aovr.amount, amount, AllovrError::InsufficientAovrBalance);

    // transfer AOVR to treasury
    let transfer_instruction = Transfer{
        from: ctx.accounts.stake_holder_aovr.to_account_info(),
        to: ctx.accounts.aovr_treasury.to_account_info(),
        authority: ctx.accounts.stake_holder.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );
    
    anchor_spl::token::transfer(cpi_ctx, amount)?;

    let stake  = &mut ctx.accounts.stake.load_init()?;

    if stake.initialised_date == 0 {
        // first stake
        require!(stake.init(pool_index, slot_index, ctx.accounts.clock.unix_timestamp).is_ok(), AllovrError::StakeAlreadyInitialised);
    } else {
        require_eq!(stake.pool_index, pool_index, AllovrError::StakeAlreadyInitialised);
        require_eq!(stake.slot_index, slot_index, AllovrError::StakeAlreadyInitialised);
    }

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

    // check that pool_index points a registered pool
    let registered_pool_option = stake_pool_registry.pools[usize::from(pool_index)];
    require!(registered_pool_option.is_some(), AllovrError::InvalidPoolIndex);

    // check that pool exists in pool registry and matched passed in address
    let mut registered_pool = registered_pool_option.unwrap();

    // check that the slot is not occupied
    require_eq!(stake_pool.stakes[usize::from(slot_index)], 0, AllovrError::SlotIndexOccupied);

    if rebalance {
        require!(utils::rebalance(stake_pool_registry, stake_pool, pool_index).is_ok(), AllovrError::StakePoolRegistryRebalanceRequired);
    }

    // update pool
    stake_pool.total_staked += amount;
    stake_pool.stakes[usize::from(slot_index)] += amount;

    stake_pool_registry.total_staked += amount;
    registered_pool.total_staked += amount;
    stake_pool_registry.pools[usize::from(pool_index)] = Some(registered_pool);

    Ok(())
}