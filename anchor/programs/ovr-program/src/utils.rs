use crate::constants::{ALLOVR_AOVR_DECIMAL_PLACES, ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL};
use crate::errors::AllovrError;
use crate::state::{StakePool, StakePoolRegistry};
use anchor_lang::prelude::*;
use std::cell::RefMut;

pub fn rebalance(
    stake_pool_registry: &mut RefMut<StakePoolRegistry>,
    stake_pool: &mut RefMut<StakePool>,
    pool_index: u8,
) -> Result<()> {
    // check that pool_index points a registered pool
    let registered_pool_option = stake_pool_registry.pools[usize::from(pool_index)];
    require!(
        registered_pool_option.is_some(),
        AllovrError::InvalidPoolIndex
    );

    // check that pool exists in pool registry and matched passed in address
    let mut registered_pool = registered_pool_option.unwrap();

    if registered_pool.total_owed == 0 {
        return Ok(());
    }

    for slot_index in 0..ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL {
        // increase each slot based on their share of the pie
        stake_pool.stakes[slot_index] = stake_pool.stakes[slot_index]
            / registered_pool.total_staked
            * registered_pool.total_owed;
    }

    // update pool
    stake_pool.total_staked += registered_pool.total_owed;

    stake_pool_registry.total_staked += registered_pool.total_owed;

    registered_pool.total_staked += registered_pool.total_owed;
    registered_pool.total_owed = 0;

    Ok(())
}

pub fn ui_amount_to_amount(aovr: f64) -> u64 {
    spl_token::ui_amount_to_amount(aovr, ALLOVR_AOVR_DECIMAL_PLACES)
}
