use anchor_lang::prelude::*;
use anchor_spl::token::{SetAuthority, set_authority};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use std::borrow::BorrowMut;
use std::mem::size_of;
use std::str::FromStr;
use crate::constants::{ALLOVR_MINT_SEED_PREFIX, ALLOVR_AOVR_DECIMAL_PLACES, INFLATION_INTERVAL_IN_SECONDS};
use crate::errors::AllovrError;
use crate::known_addresses::{ALLOVR_STATE_ID, ALLOVR_MINT_ID, KnownAddress};
use crate::state::{ InitAovrArgs, AllovrTokenState, StakePoolRegistry };
use anchor_spl::token::{Token, Mint, MintTo, TokenAccount};

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
    #[account(mut, token::mint = KnownAddress::allovr_mint())]
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

pub fn handle_aovr_inflation_run(
    ctx: Context<AovrInflationRun>    
) -> Result<()> {

    let aovr_state = ctx.accounts.aovr_state.borrow_mut();

    require!(!aovr_state.minted, AllovrError::AovrNotMinted);
    require!(ctx.accounts.clock.unix_timestamp < aovr_state.next_inflation_due, AllovrError::AovrInflationNotDue);    

    aovr_state.next_inflation_due = aovr_state.next_inflation_due + INFLATION_INTERVAL_IN_SECONDS;
    aovr_state.inflation_run_count += 1;    

    let total_supply = ctx.accounts.aovr_mint.supply;
    let annual_inflation = total_supply * 5 / 100;
    let weekly_inflation = annual_inflation / 52;    

    let mut recipients: Vec<(&AccountInfo, u64)> = vec![];

    // distribute the weekly inflation amount
    // if stakes exist, mint to staking treasury
    let total_staked = ctx.accounts.stake_pool_registry.total_staked;
    if total_staked > 0 {
        recipients.push((ctx.accounts.stake_pool_registry.to_account_info(), weekly_inflation/2));      
        recipients.push((ctx.accounts.aovr_treasury.to_account_info(), weekly_inflation/2));      

        let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_mut()?;
        stake_pool_registry.total_owed += weekly_inflation/2;

    } else {
        recipients.push((ctx.accounts.aovr_treasury.to_account_info(), weekly_inflation));      
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

    // the rest goes to DAO treasury    

    // let founder_share = weekly_inflation / 8;
    // let treaasury_share = weekly_inflation / 2;
    // let recipients: Vec<(&AccountInfo, u64)> = vec![
    //     (a.treasury_token, treaasury_share),
    //     (a.founder_1_token, founder_share),
    //     (a.founder_2_token, founder_share),
    //     (a.founder_3_token, founder_share),
    //     (a.founder_4_token, founder_share),
    // ];

    Ok(())
}