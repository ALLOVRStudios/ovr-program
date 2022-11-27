use crate::constants::{
    ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_TREASURY_PREFIX,
    ALLOVR_MINT_SEED_PREFIX, INFLATION_INTERVAL_IN_SECONDS,
};
use crate::errors::AllovrError;
use crate::known_addresses::KnownAddress;
use crate::state::{AllovrTokenState, StakePoolRegistry};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use std::borrow::BorrowMut;

#[account]
pub struct Auth {}

#[derive(Accounts)]
pub struct AovrInflationRun<'info> {
    #[account(mut, address = KnownAddress::allovr_state(), constraint = aovr_state.to_account_info().owner == program_id)]
    aovr_state: Account<'info, AllovrTokenState>,
    #[account(mut, address = KnownAddress::allovr_mint(), mint::authority = mint_authority)]
    aovr_mint: Account<'info, Mint>,
    #[account(seeds = [ALLOVR_MINT_SEED_PREFIX.as_ref()], bump)]
    mint_authority: Account<'info, Auth>,
    #[account(mut, token::mint = KnownAddress::allovr_mint(), address = KnownAddress::allovr_dao_aovr_treasury())]
    aovr_treasury: Account<'info, TokenAccount>,
    #[account(mut, token::mint = KnownAddress::allovr_mint(), seeds = [ALLOVR_AOVR_STAKE_TREASURY_PREFIX.as_ref()], bump)]
    aovr_staking_treasury: Account<'info, TokenAccount>,
    #[account(mut, seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()], bump)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,
    #[account(mut)]
    payer: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}

pub fn handle_aovr_inflation_run(ctx: Context<AovrInflationRun>) -> Result<()> {
    let aovr_state = ctx.accounts.aovr_state.borrow_mut();

    require!(aovr_state.minted, AllovrError::AovrNotMinted);
    require!(
        ctx.accounts.clock.unix_timestamp < aovr_state.next_inflation_due,
        AllovrError::AovrInflationNotDue
    );

    aovr_state.next_inflation_due = aovr_state.next_inflation_due + INFLATION_INTERVAL_IN_SECONDS;
    aovr_state.inflation_run_count += 1;

    let total_supply = ctx.accounts.aovr_mint.supply;
    let annual_inflation = total_supply * 5 / 100;
    let weekly_inflation = annual_inflation / 52;

    let mut recipients: Vec<(AccountInfo, u64)> = vec![];

    // distribute the weekly inflation amount
    // if stakes exist, mint to staking treasury
    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;
    let total_staked = stake_pool_registry.total_staked;

    if total_staked > 0 {
        let third = weekly_inflation / 3;
        recipients.push((
            ctx.accounts.stake_pool_registry.to_account_info(),
            third,
        ));
        recipients.push((
            ctx.accounts.aovr_treasury.to_account_info(),
            2 * third,
        ));
        
        let total_staked = stake_pool_registry.total_staked;
        stake_pool_registry.total_staked = total_staked + third;

        let mut i = 0;
        while i < stake_pool_registry.pool_head {
            let mut pool = stake_pool_registry.pools[i];
            pool.total_owed = (pool.total_staked / total_staked) * third; 
            i = i + 1;
        }
    } else {
        recipients.push((
            ctx.accounts.aovr_treasury.to_account_info(),
            weekly_inflation,
        ));
    }

    for r in recipients {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.aovr_mint.to_account_info(),
            to: r.0,
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        mint_to(cpi_ctx, r.1)?;
    }

    Ok(())
}
