use anchor_lang::prelude::*;
use std::borrow::BorrowMut;
use std::mem::size_of;
use crate::constants::{ALLOVR_MINT_SEED_PREFIX, ALLOVR_AOVR_DECIMAL_PLACES};
use crate::known_addresses::KnownAddress;
use crate::state::{InitAovrArgs, AllovrTokenState};
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
        mint::authority = mint_authority,
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
    Ok(())
}