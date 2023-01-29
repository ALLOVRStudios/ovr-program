use crate::constants::{ALLOVR_MINT_SEED_PREFIX, INFLATION_INTERVAL_IN_SECONDS};
use crate::errors::AllovrError;
use crate::known_addresses::KnownAddress;
use crate::state::AllovrTokenState;
use crate::utils::ui_amount_to_amount;
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, TokenAccount};
use anchor_spl::token::{Mint, Token};
use std::borrow::BorrowMut;

#[account]
pub struct Auth {}

#[derive(Accounts)]
pub struct MintAovr<'info> {
    #[account(mut, address = KnownAddress::allovr_state(), constraint = aovr_state.to_account_info().owner == program_id)]
    aovr_state: Account<'info, AllovrTokenState>,

    #[account(mut, address = KnownAddress::allovr_mint(), mint::authority = mint_authority)]
    aovr_mint: Account<'info, Mint>,

    #[account(mut, seeds = [ALLOVR_MINT_SEED_PREFIX.as_ref()], bump)]
    mint_authority: Account<'info, Auth>,

    #[account(mut, token::mint = KnownAddress::allovr_mint(), address = KnownAddress::allovr_dao_aovr_treasury())]
    aovr_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    initialiser: Signer<'info>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}

pub fn handle_mint_aovr(ctx: Context<MintAovr>) -> Result<()> {
    let aovr_state = ctx.accounts.aovr_state.borrow_mut();

    require!(aovr_state.minted == false, AllovrError::AovrAlreadyMinted);

    aovr_state.minted = true;
    aovr_state.next_inflation_due =
        ctx.accounts.clock.unix_timestamp + INFLATION_INTERVAL_IN_SECONDS;

    let cpi_accounts = MintTo {
        mint: ctx.accounts.aovr_mint.to_account_info(),
        to: ctx.accounts.aovr_treasury.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let (_mint_authority, mint_authority_bump) =
        Pubkey::find_program_address(&[ALLOVR_MINT_SEED_PREFIX.as_bytes()], ctx.program_id);

    let mint_authority_bump_bytes = mint_authority_bump.to_le_bytes();
    let seeds = vec![
        ALLOVR_MINT_SEED_PREFIX.as_bytes(),
        mint_authority_bump_bytes.as_ref(),
    ];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    mint_to(cpi_ctx, ui_amount_to_amount(100_000_000.0))?;

    Ok(())
}
