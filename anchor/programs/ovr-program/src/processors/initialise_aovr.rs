use anchor_lang::prelude::*;
use anchor_spl::token::{SetAuthority, set_authority};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use std::borrow::BorrowMut;
use std::mem::size_of;
use std::str::FromStr;
use crate::constants::{ALLOVR_MINT_SEED_PREFIX, ALLOVR_AOVR_DECIMAL_PLACES};
use crate::errors::AllovrError;
use crate::known_addresses::{ALLOVR_STATE_ID, ALLOVR_MINT_ID, KnownAddress};
use crate::state::{ InitAovrArgs, AllovrTokenState};
use anchor_spl::token::{ Token, Mint };

#[account]
pub struct Auth {}

#[derive(Accounts)]
pub struct InitialiseAovr<'info> {
    #[account(
        init,
        address = KnownAddress::allovr_state(),
        payer = initialiser, 
        owner = *program_id, 
        space = size_of::<AllovrTokenState>() + 8)]
    aovr_state: Account<'info, AllovrTokenState>,
    #[account(
        init,
        address = KnownAddress::allovr_mint(),
        payer = initialiser,
        mint::decimals = ALLOVR_AOVR_DECIMAL_PLACES,
        mint::authority = initialiser,
    )]
    aovr_mint: Account<'info, Mint>,
    #[account(
        init,
        space = 8,
        payer = initialiser,
        seeds = [ALLOVR_MINT_SEED_PREFIX.as_ref()],
        bump,
    )]
    mint_authority: Account<'info, Auth>,
    #[account(mut)]
    initialiser: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle_initialise_aovr(
    ctx: Context<InitialiseAovr>,
    founders: InitAovrArgs
) -> Result<()> {

    let aovr_state = ctx.accounts.aovr_state.borrow_mut();

    aovr_state.minted = false;
    aovr_state.next_inflation_due = 0;
    aovr_state.inflation_run_count = 0;
    aovr_state.founder_1 = founders.founder_1;
    aovr_state.founder_2 = founders.founder_2;
    aovr_state.founder_3 = founders.founder_3;
    aovr_state.founder_4 = founders.founder_4;
    aovr_state.founder_5 = founders.founder_5;
    aovr_state.founder_6 = founders.founder_6;
    aovr_state.founder_7 = founders.founder_7;
    aovr_state.founder_8 = founders.founder_8;

    // set mint authority
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.aovr_mint.to_account_info(),
        current_authority: ctx.accounts.initialiser.to_account_info(),
    };

    let seeds = vec![ALLOVR_MINT_SEED_PREFIX.as_bytes()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    set_authority(cpi_ctx, AuthorityType::MintTokens, Some(ctx.accounts.mint_authority.key()))?;

    Ok(())
}