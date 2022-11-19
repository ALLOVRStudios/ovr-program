use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::constants::{ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX, ALLOVR_AOVR_STAKE_PREFIX, ALLOVR_AOVR_STAKE_MINIMUM_STAKE};
use crate::errors::AllovrError;
use crate::state::{StakePoolRegistry, StakeMetadata, StakePool};
use crate::utils;

#[derive(Accounts)]
pub struct CancelStakeWithdrawal<'info> {
    #[account(        
        seeds = [ALLOVR_AOVR_STAKE_PREFIX.as_ref(), stake_holder.key().as_ref()],
        bump,        
        owner = *program_id)]
    stake: AccountLoader<'info, StakeMetadata>,    
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account]
    stake_holder: Signer<'info>,    
    system_program: Program<'info, System>,
}

pub fn handle_cancel_stake_withdrawal(
    ctx: Context<Stake>    
) -> Result<()> {
    let mut stake  = &mut ctx.accounts.stake.load_mut()?;
    stake::cancel_withdrawal()    
}