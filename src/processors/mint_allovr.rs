use crate::state::AllovrTokenState;
use crate::INFLATION_INTERVAL_IN_SECONDS;
use crate::{error::AllovrError, utils::*, ALLOVR_MINT_SEED_PREFIX};
use borsh::BorshSerialize;
use solana_program::sysvar::clock::Clock;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

struct Accounts<'a, 'b: 'a> {
    initiator: &'a AccountInfo<'b>,
    state: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    mint_authority: &'a AccountInfo<'b>,
    treasury_token: &'a AccountInfo<'b>,    
    founder_1_token: &'a AccountInfo<'b>,
    founder_2_token: &'a AccountInfo<'b>,
    founder_3_token: &'a AccountInfo<'b>,
    founder_4_token: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    _rent_sysvar: &'a AccountInfo<'b>,
    clock_sysvar: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
}

pub fn execute(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let rent = Rent::get()?;

    let a = parse_accounts(program_id, accounts, &rent)?;

    // Initial token mint and split
    // Supply: 100 000 000
    // Founders: 30%    
    // Treasury: 70%

    let clock = Clock::from_account_info(a.clock_sysvar)?;

    let mut state: AllovrTokenState = try_from_slice_unchecked(&a.state.data.borrow())?;

    if state.minted {
        return Err(AllovrError::AlreadyMinted.into());
    }

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

    state.minted = true;
    state.next_inflation_due = clock.unix_timestamp + INFLATION_INTERVAL_IN_SECONDS;
    state.serialize(&mut &mut a.state.data.borrow_mut()[..])?;

    let total = 100000000.0;
    let founder_share = ui_amount_to_amount(total * 0.075);
    let treasury_share = ui_amount_to_amount(total * 0.7);
    let recipients: Vec<(&AccountInfo, u64)> = vec![
        (
            a.treasury_token,
            treasury_share,
        ),
        (a.founder_1_token, founder_share),
        (a.founder_2_token, founder_share),
        (a.founder_3_token, founder_share),
        (a.founder_4_token, founder_share),
    ];

    let (_mint_auth_pda, mint_auth_pda_bump) = assert_pda(
        &a.mint_authority,
        &program_id,
        &[ALLOVR_MINT_SEED_PREFIX.as_bytes()],
    )?;

    let signer_seeds = &[ALLOVR_MINT_SEED_PREFIX.as_bytes(), &[mint_auth_pda_bump]];

    for r in recipients {
        assert_rent_exempt(&rent, &r.0)?;
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
        initiator: next_account_info(account_iter)?,
        state: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        mint: next_account_info(account_iter)?,
        mint_authority: next_account_info(account_iter)?,
        treasury_token: next_account_info(account_iter)?,
        founder_1_token: next_account_info(account_iter)?,
        founder_2_token: next_account_info(account_iter)?,
        founder_3_token: next_account_info(account_iter)?,
        founder_4_token: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        _rent_sysvar: next_account_info(account_iter)?,
        clock_sysvar: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_system(&a.system)?;
    assert_state(&a.state.key)?;
    assert_token_program_matches_package(&a.token_program)?;

    assert_owned_by(a.payer, &solana_program::system_program::id())?;
    assert_owned_by(a.treasury_token, &spl_token::id())?;
    assert_owned_by(a.founder_1_token, &spl_token::id())?;
    assert_owned_by(a.founder_2_token, &spl_token::id())?;
    assert_owned_by(a.founder_3_token, &spl_token::id())?;
    assert_owned_by(a.founder_4_token, &spl_token::id())?;

    assert_rent_exempt(rent, a.treasury_token)?;
    assert_rent_exempt(rent, a.founder_1_token)?;
    assert_rent_exempt(rent, a.founder_2_token)?;
    assert_rent_exempt(rent, a.founder_3_token)?;
    assert_rent_exempt(rent, a.founder_4_token)?;

    assert_signer(a.payer)?;
    assert_signer(a.initiator)?;    

    if a.initiator.key != program_id {
        return Err(AllovrError::InvalidInitialiser.into());
    }

    Ok(a)
}
