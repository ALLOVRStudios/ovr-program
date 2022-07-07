use crate::instruction::RegisterArtistArgs;
use crate::state::ArtistMetadata;
use crate::{utils::*, ARTIST_METADATA_SEED_PREFIX, ARTIST_SEED_PREFIX};

use borsh::BorshSerialize;
use solana_program::borsh::try_from_slice_unchecked;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

struct Accounts<'a, 'b: 'a> {
    artist_wallet: &'a AccountInfo<'b>,
    artist_token_mint: &'a AccountInfo<'b>,
    artist_token_meta: &'a AccountInfo<'b>,
    artist_token_metaplex_meta: &'a AccountInfo<'b>,
    metaplex_meta_program_account: &'a AccountInfo<'b>,
    _system: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: RegisterArtistArgs,
) -> ProgramResult {
    let artist_data = santitise_artist_data(args)?;
    let a = parse_accounts(program_id, accounts)?;

    // derive artist token mint for signer account
    let (artist_token_mint_key, _) = Pubkey::find_program_address(
        &[
            ARTIST_SEED_PREFIX.as_bytes(),
            &a.artist_wallet.key.to_bytes(),
        ],
        &program_id,
    );

    // Check artist Metadata account passed in matches signer artist token
    let (_metadata_key, _) = assert_pda(
        a.artist_token_meta,
        program_id,
        &[
            ARTIST_METADATA_SEED_PREFIX.as_bytes(),
            artist_token_mint_key.as_ref(),
        ],
    )?;

    // Save meta data
    let mut metadata: ArtistMetadata =
        try_from_slice_unchecked(&a.artist_token_meta.data.borrow_mut())?;

    metadata.name = artist_data.name;
    metadata.description = artist_data.description;
    metadata.symbol = artist_data.token_symbol;
    metadata.uri = artist_data.uri.clone();

    metadata.serialize(&mut &mut a.artist_token_meta.data.borrow_mut()[..])?;

    let mut uri = String::new();
    if !artist_data.uri.is_none() {
        uri = artist_data.uri.unwrap();
    }

    // check the artist token mint PDA passed in is correct
    let (_artist_token_mint_pda, artist_token_mint_bump) = assert_pda(
        &a.artist_token_mint,
        program_id,
        &[
            ARTIST_SEED_PREFIX.as_bytes(),
            &a.artist_wallet.key.to_bytes(),
        ],
    )?;

    let signers_seeds = &[
        ARTIST_SEED_PREFIX.as_bytes(),
        &a.artist_wallet.key.to_bytes(),
        &[artist_token_mint_bump],
    ];

    // Update the Metaplex metadata for artist token so that wallets can show the token name, symbol and image

    update_metaplex_metadata_account(
        &a.artist_token_metaplex_meta,
        &a.artist_token_mint,
        &a.metaplex_meta_program_account,
        String::from(&metadata.name),
        String::from(&metadata.symbol),
        uri,
        signers_seeds,
    )?;

    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        artist_wallet: next_account_info(account_iter)?,
        artist_token_mint: next_account_info(account_iter)?,
        artist_token_meta: next_account_info(account_iter)?,
        artist_token_metaplex_meta: next_account_info(account_iter)?,
        metaplex_meta_program_account: next_account_info(account_iter)?,
        _system: next_account_info(account_iter)?,
    };
    assert_program_id(program_id)?;
    assert_signer(accounts.artist_wallet)?;
    assert_metaplex_program(accounts.metaplex_meta_program_account)?;

    Ok(accounts)
}
