use crate::known_addresses::KnownAddress;
use crate::state::AllovrTokenState;
use anchor_lang::prelude::*;
use std::borrow::BorrowMut;

/// This function here only for running integration tests, it should not be deployed
#[derive(Accounts)]
pub struct TestUpdateInflationRun<'info> {
    #[account(mut, address = KnownAddress::allovr_state(), constraint = aovr_state.to_account_info().owner == program_id)]
    aovr_state: Account<'info, AllovrTokenState>,
    system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}

pub fn handle_test_update_inflation_run(ctx: Context<TestUpdateInflationRun>) -> Result<()> {
    let aovr_state = ctx.accounts.aovr_state.borrow_mut();
    aovr_state.next_inflation_due = ctx.accounts.clock.unix_timestamp;
    Ok(())
}
