use anchor_lang::prelude::*;
use crate::constants::ALLOVR_AOVR_STAKE_PREFIX;
use crate::state::{StakeMetadata, StakePool};

#[derive(Accounts)]
pub struct CancelStakeWithdrawal<'info> {
    #[account(        
        seeds = [ALLOVR_AOVR_STAKE_PREFIX.as_ref(), stake_holder.key().as_ref()],
        bump,        
        owner = *program_id)]
    stake: AccountLoader<'info, StakeMetadata>,    
    #[account(mut, owner = *program_id)]
    stake_pool: AccountLoader<'info, StakePool>,
    #[account()]
    stake_holder: Signer<'info>,    
    system_program: Program<'info, System>,
}

pub fn handle_cancel_stake_withdrawal(
    ctx: Context<CancelStakeWithdrawal>    
) -> Result<()> {
    let stake  = &mut ctx.accounts.stake.load_mut()?;
    stake.cancel_withdrawal()
}