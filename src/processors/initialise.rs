use crate::instruction::InitialisaAllovrArgs;
use crate::state::AllovrTokenState;
use crate::{
    error::AllovrError, utils::*, ALLOVR_MINT_SEED_PREFIX, ALLOVR_STATE_SEED_PREFIX,
    ALL_DECIMAL_PLACES, MINT_SIZE, STATE_SIZE,
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
    system: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: InitialisaAllovrArgs,
) -> ProgramResult {
    let rent = Rent::get()?;

    let a = parse_accounts(program_id, accounts)?;
    
    // Create State account with known address. Can only be crated once, hence init can only be called once
    create_account(
        &rent,
        STATE_SIZE,
        &a.payer,
        &a.state,
        *program_id, // owner is program
    )?;

    let state_data = AllovrTokenState {
        minted: false,
        next_inflation_due: 0,  // will be used later
        inflation_run_count: 0, // will be used later
        founder_1: args.founder_1,
        founder_2: args.founder_2,
        founder_3: args.founder_3,
        founder_4: args.founder_4,
    };
    state_data.serialize(&mut &mut a.state.data.borrow_mut()[..])?;

    // Create Mint Account. Mint address is known ALLOVR_MINT_ID
    create_account(
        &rent,
        MINT_SIZE,
        &a.payer,
        &a.mint,
        spl_token::id(),
    )?;

    // Check mint authority account matches expected mint authority PDA
    let (_mint_auth_pda, mint_auth_pda_bump) = assert_pda(
        &a.mint_authority,
        &program_id,
        &[ALLOVR_MINT_SEED_PREFIX.as_bytes()],
    )?;

    initalise_mint_account(
        &a.mint,
        &a.mint_authority, // authority is PDA
        &a.rent_sysvar,
        ALLOVR_MINT_SEED_PREFIX,
        mint_auth_pda_bump,
        ALL_DECIMAL_PLACES,
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
        system: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_system(&a.system)?;
    assert_state(&a.state.key)?;    
    assert_token_program_matches_package(&a.token_program)?;

    assert_owned_by(a.payer, &solana_program::system_program::id())?; // standard SOL account
    assert_signer(a.payer)?;

    // keys used to delpoy contract must be present as signer
    assert_signer(a.initiator)?;
    if a.initiator.key != program_id {
        return Err(AllovrError::InvalidInitialiser.into());
    }

    Ok(a)    
}
