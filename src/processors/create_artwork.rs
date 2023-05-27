use crate::instruction::CreateArtworkArgs;
use crate::state::ArtworkMetadata;
use crate::{
    error::AllovrError, utils::*, ARTWORK_METADATA_DESCRIPTION_SIZE, ARTWORK_METADATA_SEED_PREFIX,
    ARTWORK_METADATA_SIZE, ARTWORK_METADATA_SYMBOL_SIZE, ARTWORK_SEED_PREFIX, MINT_SIZE,
};
use borsh::BorshSerialize;

use solana_program::borsh::try_from_slice_unchecked;
use solana_program::msg;
use solana_program::program::invoke;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

struct Accounts<'a, 'b: 'a> {
    artist_wallet: &'a AccountInfo<'b>,
    artist_aov_token: &'a AccountInfo<'b>,
    artist_artwork_token: &'a AccountInfo<'b>,
    artwork_mint: &'a AccountInfo<'b>,
    artwork_meta: &'a AccountInfo<'b>,
    treasury_aov_token: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    rent_sysvar: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
    _apa_program: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: CreateArtworkArgs,
) -> ProgramResult {
    let rent = Rent::get()?;

    let artwork_token_symbol = args.symbol.trim().to_string();
    if artwork_token_symbol.len() != ARTWORK_METADATA_SYMBOL_SIZE {
        return Err(AllovrError::InvalidArtworkSymbol.into());
    }

    let artwork_description = args.description.trim().to_string();
    if artwork_description.len() == 0
        || artwork_description.len() > ARTWORK_METADATA_DESCRIPTION_SIZE
    {
        return Err(AllovrError::ArtworkDescriptionMaxLengthExceeded.into());
    }

    let a = parse_accounts(program_id, accounts)?;

    // Artist must pay 1000 AOV to create artwork
    msg!("Creating 10K AOVR transfer instruction...");
    let transfer_aov_ix = spl_token::instruction::transfer(
        &a.token_program.key,
        &a.artist_aov_token.key,
        &a.treasury_aov_token.key,
        &a.artist_wallet.key,
        &[],
        ui_amount_to_amount(1000.0),
    )?;

    msg!("Transferring 10K AOVR to treasury...");
    invoke(
        &transfer_aov_ix,
        &[
            a.artist_aov_token.clone(),
            a.treasury_aov_token.clone(),
            a.artist_wallet.clone(),
            a.token_program.clone(),
        ],
    )?;

    let symbol_bytes = artwork_token_symbol.as_bytes();

    let (_artwork_token_mint_pda, artwork_token_mint_bump) = assert_pda(
        &a.artwork_mint,
        program_id,
        &[
            ARTWORK_SEED_PREFIX.as_bytes(),
            symbol_bytes,
            a.artist_wallet.key.as_ref(),
        ],
    )?;

    let artwork_mint_authority_signer_seeds = &[
        ARTWORK_SEED_PREFIX.as_bytes(),
        symbol_bytes,
        a.artist_wallet.key.as_ref(),
        &[artwork_token_mint_bump],
    ];

    msg!("Creating artwork token mint...");
    create_raw(
        spl_token::id(),
        a.artwork_mint,
        &rent,
        &a.system,
        &a.artist_wallet,
        MINT_SIZE,
        artwork_mint_authority_signer_seeds,
    )?;

    initalise_pda_mint_account(
        &a.artwork_mint,
        &a.artist_wallet,
        &a.rent_sysvar,
        artwork_mint_authority_signer_seeds,
        0,
    )?;

    create_ata(
        &a.artist_wallet,
        &a.artist_wallet,
        &a.artist_artwork_token,
        &a.artwork_mint,
        a.rent_sysvar,
        AllovrError::InvalidArtworkTokenAccount,
    )?;

    // Mint the artwork to the artist
    mint_tokens_to(
        a.artwork_mint,         // mint account (PDA)
        a.artwork_mint,         // mint authority (same PDA)
        a.artist_artwork_token, // artist assoicated token account
        &[artwork_mint_authority_signer_seeds],
        1,
        true,
    )?;

    // The PDA of the meta data accountshould be unique per Artist pubkey and Symbol
    // (artist must create unique symbol per artpiece)
    let (_metadata_key, metadata_bump_seed) = assert_pda(
        a.artwork_meta,
        program_id,
        &[
            ARTWORK_METADATA_SEED_PREFIX.as_bytes(),
            a.artwork_mint.key.as_ref(),
        ],
    )?;
    let metadata_authority_signer_seeds = &[
        ARTWORK_METADATA_SEED_PREFIX.as_bytes(),
        a.artwork_mint.key.as_ref(),
        &[metadata_bump_seed],
    ];

    // create Metadata account
    create_raw(
        *program_id,
        a.artwork_meta,
        &rent,
        &a.system,
        &a.artist_wallet,
        ARTWORK_METADATA_SIZE,
        metadata_authority_signer_seeds,
    )?;

    // Save meta data
    let mut metadata: ArtworkMetadata =
        try_from_slice_unchecked(&a.artwork_meta.data.borrow_mut())?;

    metadata.artist = *a.artist_wallet.key;
    metadata.symbol = artwork_token_symbol;
    metadata.description = artwork_description;
    metadata.holder = *a.artist_wallet.key;
    metadata.offer_count = 0;
    metadata.offered_to = None;
    metadata.offer_price = None;
    metadata.offer_currency = None;

    metadata.serialize(&mut &mut a.artwork_meta.data.borrow_mut()[..])?;

    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        artist_wallet: next_account_info(account_iter)?,
        artist_aov_token: next_account_info(account_iter)?,
        artist_artwork_token: next_account_info(account_iter)?,
        artwork_mint: next_account_info(account_iter)?,
        artwork_meta: next_account_info(account_iter)?,
        treasury_aov_token: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        rent_sysvar: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
        _apa_program: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_token_program_matches_package(a.token_program)?;
    assert_signer(&a.artist_wallet)?;
    assert_owned_by(&a.artist_aov_token, &spl_token::id())?;

    // TODO: More checks!

    Ok(a)
}
