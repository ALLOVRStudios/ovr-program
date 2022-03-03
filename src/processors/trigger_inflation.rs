use crate::state::AllovrTokenState;
use crate::INFLATION_INTERVAL_IN_SECONDS;
use crate::{error::AllovrError, utils::*, ALLOVR_MINT_SEED};
use borsh::BorshSerialize;
use solana_program::sysvar::clock::Clock;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

struct Accounts<'a, 'b: 'a> {
    state: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    mint_authority: &'a AccountInfo<'b>,
    treasury_token: &'a AccountInfo<'b>,
    founder_1_token: &'a AccountInfo<'b>,
    founder_2_token: &'a AccountInfo<'b>,
    founder_3_token: &'a AccountInfo<'b>,
    founder_4_token: &'a AccountInfo<'b>,
    _token_program: &'a AccountInfo<'b>,
    clock_sysvar: &'a AccountInfo<'b>,
}

pub fn execute(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let rent = Rent::get()?;

    let a = parse_accounts(program_id, accounts, &rent)?;

    let clock = Clock::from_account_info(a.clock_sysvar)?;

    let mut state: AllovrTokenState = try_from_slice_unchecked(&a.state.data.borrow())?;

    if !state.minted {
        return Err(AllovrError::AlreadyMinted.into());
    }

    if clock.unix_timestamp < state.next_inflation_due {
        msg!(
            "clock.unix_timestamp: {} state.next_inflation_due: {}",
            clock.unix_timestamp,
            state.next_inflation_due
        );

        return Err(AllovrError::InflationNotDue.into());
    }

    state.next_inflation_due = state.next_inflation_due + INFLATION_INTERVAL_IN_SECONDS;
    state.inflation_run_count += 1;
    state.serialize(&mut &mut a.state.data.borrow_mut()[..])?;

    // Check founder addresses correct

    if a.founder_1_token.key.ne(&state.founder_1) {
        return Err(AllovrError::IncorrectFounderAddress.into());
    }
    if a.founder_2_token.key.ne(&state.founder_2) {
        return Err(AllovrError::IncorrectFounderAddress.into());
    }
    if a.founder_3_token.key.ne(&state.founder_3) {
        return Err(AllovrError::IncorrectFounderAddress.into());
    }
    if a.founder_4_token.key.ne(&state.founder_4) {
        return Err(AllovrError::IncorrectFounderAddress.into());
    }

    let total_supply = get_token_supply(&a.mint);
    let annual_inflation = total_supply * 5 / 100;
    let weekly_inflation = annual_inflation / 52;

    let founder_share = weekly_inflation / 8;
    let treaasury_share = weekly_inflation / 2;
    let recipients: Vec<(&AccountInfo, u64)> = vec![
        (a.treasury_token, treaasury_share),
        (a.founder_1_token, founder_share),
        (a.founder_2_token, founder_share),
        (a.founder_3_token, founder_share),
        (a.founder_4_token, founder_share),
    ];

    let (_mint_auth_pda, mint_auth_pda_bump) = assert_pda(
        &a.mint_authority,
        &program_id,
        &[ALLOVR_MINT_SEED.as_bytes()],
    )?;

    let signer_seeds = &[ALLOVR_MINT_SEED.as_bytes(), &[mint_auth_pda_bump]];

    for r in recipients {
        mint_tokens_to(
            &a.mint,
            &a.mint_authority,
            &r.0,
            &[signer_seeds],
            r.1,
            false,
        )?;
    }

    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
    rent: &Rent,
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        state: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        mint: next_account_info(account_iter)?,
        mint_authority: next_account_info(account_iter)?,
        treasury_token: next_account_info(account_iter)?,
        founder_1_token: next_account_info(account_iter)?,
        founder_2_token: next_account_info(account_iter)?,
        founder_3_token: next_account_info(account_iter)?,
        founder_4_token: next_account_info(account_iter)?,
        _token_program: next_account_info(account_iter)?,
        clock_sysvar: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_clock(a.clock_sysvar)?;
    assert_token_program_matches_package(&a._token_program)?;
    assert_rent_exempt(rent, a.treasury_token)?;
    assert_rent_exempt(rent, a.founder_1_token)?;
    assert_rent_exempt(rent, a.founder_2_token)?;
    assert_rent_exempt(rent, a.founder_3_token)?;
    assert_rent_exempt(rent, a.founder_4_token)?;

    assert_signer(a.payer)?;

    Ok(a)
}
