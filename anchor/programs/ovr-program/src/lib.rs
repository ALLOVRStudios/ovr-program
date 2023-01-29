mod constants;
mod errors;
mod known_addresses;
mod processors;
mod state;
mod utils;

use anchor_lang::prelude::*;
use processors::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ovr_program {
    use super::*;

    pub fn initialise_aovr(ctx: Context<InitialiseAovr>, founders: InitAovrArgs) -> Result<()> {
        handle_initialise_aovr(ctx, founders)
    }

    pub fn mint_aovr(ctx: Context<MintAovr>) -> Result<()> {
        handle_mint_aovr(ctx)
    }

    pub fn aovr_inflation_run(ctx: Context<AovrInflationRun>) -> Result<()> {
        handle_aovr_inflation_run(ctx)
    }

    pub fn initialise_staking(ctx: Context<InitialiseStakingRegistry>) -> Result<()> {
        handle_initialise_staking(ctx)
    }

    pub fn register_staking_pool(ctx: Context<RegisterStakingPool>, pool_index: u8) -> Result<()> {
        handle_register_staking_pool(ctx, pool_index)
    }

    pub fn stake(
        ctx: Context<Stake>,
        pool_index: u8,
        slot_index: u8,
        amount: u64,
        rebalance_pool_if_needed: bool,
    ) -> Result<()> {
        handle_stake(
            ctx,
            amount,
            pool_index,
            slot_index,
            rebalance_pool_if_needed,
        )
    }

    pub fn rebalance_staking_pool(
        ctx: Context<RebalanceStakingPool>,
        pool_index: u8,
    ) -> Result<()> {
        handle_rebalance_staking_pool(ctx, pool_index)
    }

    pub fn request_stake_withdrawal(
        ctx: Context<RequestStakeWithdrawal>,
        amount: u64,
    ) -> Result<()> {
        handle_request_stake_withdrawal(ctx, amount)
    }

    pub fn cancel_stake_withdrawal(ctx: Context<CancelStakeWithdrawal>) -> Result<()> {
        handle_cancel_stake_withdrawal(ctx)
    }

    /// TEST SECTION - Remove Before Deploy
    ///
    pub fn test_update_inflation_run(ctx: Context<TestUpdateInflationRun>) -> Result<()> {
        handle_test_update_inflation_run(ctx)
    }
}
