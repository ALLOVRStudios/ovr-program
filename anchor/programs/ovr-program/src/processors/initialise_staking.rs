use anchor_lang::prelude::*;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AUTHORITY_PREFIX, ALLOVR_AOVR_STAKE_TREASURY_PREFIX};
use crate::state::StakePoolRegistry;
use crate::known_addresses::KnownAddress;
use anchor_spl::token::{TokenAccount, Mint, Token};
use std::mem::size_of;

#[account]
pub struct Auth {}
#[derive(Accounts)]
pub struct InitialiseStakingRegistry<'info> {
    #[account(
        init, 
        seeds = [ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX.as_ref()],
        bump,
        payer = initialiser, 
        owner = *program_id, 
        space = size_of::<StakePoolRegistry>() + 16)]
    stake_pool_registry: AccountLoader<'info, StakePoolRegistry>,

    #[account( address = KnownAddress::allovr_mint())]
    aovr_mint: Account<'info, Mint>,

    #[account(
        init,
        space = 8,
        payer = initialiser,
        seeds = [ALLOVR_AOVR_STAKE_TREASURY_PREFIX.as_ref(), ALLOVR_AUTHORITY_PREFIX.as_ref()],
        bump,
        owner = *program_id
    )]
    aovr_staking_treasury_authority: Account<'info, Auth>,

    #[account(
        init,
        payer = initialiser,
        seeds = [ALLOVR_AOVR_STAKE_TREASURY_PREFIX.as_ref()],
        bump,
        token::mint = aovr_mint,
        token::authority = aovr_staking_treasury_authority,
    )]
    aovr_staking_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    initialiser: Signer<'info>,

    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handle_initialise_staking(
    ctx: Context<InitialiseStakingRegistry>
) -> Result<()> {
    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_init()?;
    stake_pool_registry.init(ctx.accounts.aovr_staking_treasury.key());
    Ok(())
}