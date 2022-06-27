use crate::instruction::RegisterArtistArgs;
use crate::state::ArtistMetadata;
use crate::{
    error::AllovrError, utils::*, ALLOVR_MINT_ID, ARTIST_METADATA_DESCRIPTION_SIZE,
    ARTIST_METADATA_IMAGE_URL_SIZE, ARTIST_METADATA_NAME_SIZE, ARTIST_METADATA_SEED_PREFIX,
    ARTIST_METADATA_SIZE, ARTIST_SEED_PREFIX, MINT_SIZE,
};

use borsh::BorshSerialize;
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::program::invoke;
use std::str::FromStr;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

struct Accounts<'a, 'b: 'a> {
    artist_wallet: &'a AccountInfo<'b>,
    artist_aovr_token: &'a AccountInfo<'b>,
    artist_artist_token: &'a AccountInfo<'b>,
    artist_token_mint: &'a AccountInfo<'b>,
    artist_token_meta: &'a AccountInfo<'b>,
    treasury_aovr_token: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    _associated_token_account_program: &'a AccountInfo<'b>,
    rent_sysvar: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: RegisterArtistArgs,
) -> ProgramResult {    
    let artist_data = santitise_artist_data(args)?;
    let rent = Rent::get()?;

    // Artist pays 10 000 AOVR to register
    let a = parse_accounts(program_id, accounts)?;
    let transfer_aovr_ix = spl_token::instruction::transfer(
        &a.token_program.key,
        &a.artist_aovr_token.key,
        &a.treasury_aovr_token.key,
        &a.artist_wallet.key,
        &[],
        ui_amount_to_amount(10000.0),
    )?;

    invoke(
        &transfer_aovr_ix,
        &[
            a.artist_aovr_token.clone(),
            a.treasury_aovr_token.clone(),
            a.artist_wallet.clone(),
            a.token_program.clone(),
        ],
    )?;

    // check the artist token mint PDA passed in is correct
    let (_artist_token_mint_pda, artist_token_mint_bump) = assert_pda(
        &a.artist_token_mint,
        program_id,
        &[
            ARTIST_SEED_PREFIX.as_bytes(),
            &a.artist_wallet.key.to_bytes(),
        ],
    )?;

    // create artist token mint account
    create_pda_account(
        &rent,
        MINT_SIZE,
        &a.artist_wallet,
        &a.artist_token_mint,
        spl_token::id(),
        ARTIST_SEED_PREFIX,
        artist_token_mint_bump,
    )?;

    let signers_seeds = &[
        ARTIST_SEED_PREFIX.as_bytes(),
        &a.artist_wallet.key.to_bytes(),
        &[artist_token_mint_bump],
    ];

    // inti artist token mint account
    initalise_pda_mint_account(
        &a.artist_token_mint,
        &a.artist_wallet,
        &a.rent_sysvar,
        signers_seeds,
        2,
    )?;

    //create ATA for new artist token for the artist
    create_ata(
        &a.artist_wallet,
        &a.artist_wallet,
        &a.artist_artist_token,
        &a.artist_token_mint,
        a.rent_sysvar,
        AllovrError::InvalidArtistTokenAccount,
    )?;

    let mint_signer_seeds = &[
        ARTIST_SEED_PREFIX.as_bytes(),
        &a.artist_wallet.key.to_bytes(),
        &[artist_token_mint_bump],
    ];

    // TODO: Remove locking the mint account and see if more can be minted
    // mint 10 000 000 artist tokens to artist
    mint_tokens_to(
        a.artist_token_mint,   // mint account (PDA)
        a.artist_token_mint,   // mint authority (same PDA)
        a.artist_artist_token, // artist assoicated token account
        &[mint_signer_seeds],
        10000000,
        true,
    )?;

    // Create artist Metadata account
    let (_metadata_key, metadata_bump_seed) = assert_pda(
        a.artist_token_meta,
        program_id,
        &[
            ARTIST_METADATA_SEED_PREFIX.as_bytes(),
            a.artist_token_mint.key.as_ref(),
        ],
    )?;
    let metadata_authority_signer_seeds = &[
        ARTIST_METADATA_SEED_PREFIX.as_bytes(),
        a.artist_token_mint.key.as_ref(),
        &[metadata_bump_seed],
    ];

    create_raw(
        *program_id,
        a.artist_token_meta,
        &rent,
        &a.system,
        &a.artist_wallet,
        ARTIST_METADATA_SIZE,
        metadata_authority_signer_seeds,
    )?;

    // Save meta data
    let mut metadata: ArtistMetadata =
        try_from_slice_unchecked(&a.artist_token_meta.data.borrow_mut())?;

    metadata.name = artist_data.name;
    metadata.description = artist_data.description;
    metadata.symbol = artist_data.token_symbol;

    if artist_data.image_url.len() == 0 {
        metadata.image_url = None;
    } else {
        metadata.image_url = Some(artist_data.image_url);
    }

    metadata.serialize(&mut &mut a.artist_token_meta.data.borrow_mut()[..])?;
    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        artist_wallet: next_account_info(account_iter)?,
        artist_aovr_token: next_account_info(account_iter)?,
        artist_artist_token: next_account_info(account_iter)?,
        artist_token_mint: next_account_info(account_iter)?,
        artist_token_meta: next_account_info(account_iter)?,
        treasury_aovr_token: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        _associated_token_account_program: next_account_info(account_iter)?,
        rent_sysvar: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    assert_system(accounts.system)?;
    assert_token_program_matches_package(accounts.token_program)?;
    assert_program_id(program_id)?;
    assert_signer(accounts.artist_wallet)?;
    assert_aovr_treasury(accounts.treasury_aovr_token.key)?;

    let mint_key = Pubkey::from_str(ALLOVR_MINT_ID).unwrap();

    assert_ata_mint_and_owner(
        accounts.artist_aovr_token,
        mint_key,
        *accounts.artist_wallet.key,
    )?;

    if *accounts._associated_token_account_program.key != spl_associated_token_account::id() {
        return Err(AllovrError::InvalidAssociatedTokenAccountProgram.into());
    }

    Ok(accounts)
}
