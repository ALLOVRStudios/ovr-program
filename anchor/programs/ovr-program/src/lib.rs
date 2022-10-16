mod constants;
mod errors;
mod processors;
mod state;

use anchor_lang::prelude::*;
use processors::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ovr_program {
    use super::*;

    pub fn initialise_staking(ctx: Context<InitialiseStakingRegistry>) -> Result<()> {
        handle_initialise_staking(ctx)
    }

    pub fn register_staking_pool(ctx: Context<RegisterStakingPool>, pool_index: u8) -> Result<()> {
        handle_register_staking_pool(ctx, pool_index)
    }
}
