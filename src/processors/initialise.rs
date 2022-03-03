use crate::instruction::InitialisaAllovrArgs;
use crate::state::AllovrTokenState;
use crate::{
    error::AllovrError, utils::*, ALLOVR_MINT_SEED, AOV_DECIMAL_PLACES, MINT_SIZE, STATE_SIZE,
};
use borsh::BorshSerialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
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
    token_program: &'a AccountInfo<'b>,
    rent_sysvar: &'a AccountInfo<'b>,
    _system: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: InitialisaAllovrArgs,
) -> ProgramResult {
    let rent = Rent::get()?;

    let accounts = parse_accounts(program_id, accounts)?;

    // Create State account. Can only be crated once, hence init can only be called once
    create_account(
        &rent,
        STATE_SIZE,
        &accounts.payer,
        &accounts.state,
        *program_id,
    )?;

    let state_data = AllovrTokenState {
        minted: false,
        next_inflation_due: 0,
        inflation_run_count: 0,
        founder_1: args.founder_1,
        founder_2: args.founder_2,
        founder_3: args.founder_3,
        founder_4: args.founder_4,
    };
    state_data.serialize(&mut &mut accounts.state.data.borrow_mut()[..])?;

    // Create Mint Account
    create_account(
        &rent,
        MINT_SIZE,
        &accounts.payer,
        &accounts.mint,
        spl_token::id(),
    )?;

    let (_mint_auth_pda, mint_auth_pda_bump) = assert_pda(
        &accounts.mint_authority,
        &program_id,
        &[ALLOVR_MINT_SEED.as_bytes()],
    )?;

    initalise_mint_account(
        &accounts.mint,
        &accounts.mint_authority,
        &accounts.rent_sysvar,
        ALLOVR_MINT_SEED,
        mint_auth_pda_bump,
        AOV_DECIMAL_PLACES,
    )?;
    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        initiator: next_account_info(account_iter)?,
        state: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        mint: next_account_info(account_iter)?,
        mint_authority: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        rent_sysvar: next_account_info(account_iter)?,
        _system: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_token_program_matches_package(&a.token_program)?;

    assert_owned_by(a.payer, &solana_program::system_program::id())?;

    assert_signer(a.payer)?;
    assert_signer(a.initiator)?;
    assert_signer(a.mint)?;

    if a.initiator.key != program_id {
        return Err(AllovrError::InvalidInitialiser.into());
    }

    Ok(a)
}
