use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_TREASURY_PREFIX};
use crate::state::StakePoolRegistry;

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
    #[account(
        init,
        payer = initialiser,
        seeds = [ALLOVR_AOVR_STAKE_TREASURY_PREFIX.as_ref()],
        bump = wallet_bump,
        token::mint = aovr_mint,
        token::authority = *program_id,
    )]    
    aovr_staking_treasury: Account<'info, TokenAccount>,
    #[account(mut, address = KnownAddress::allovr_mint(), mint::authority = mint_authority)]
    aovr_mint: Account<'info, Mint>,
    #[account(mut)]
    initialiser: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_initialise_staking(
    ctx: Context<InitialiseStakingRegistry>
) -> Result<()> {
    let stake_pool_registry = &mut ctx.accounts.stake_pool_registry.load_init()?;
    stake_pool_registry.init();
    Ok(())
}