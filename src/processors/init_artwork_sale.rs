use crate::instruction::InitArtworkSaleArgs;
use crate::state::ArtworkMetadata;
use crate::ARTWORK_SEED_PREFIX;
use crate::TOKEN_ACCOUNT_SIZE;
use crate::{error::AllovrError, utils::*, ARTWORK_ESCROW_SEED_PREFIX};
use borsh::BorshSerialize;
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;

use solana_program::borsh::try_from_slice_unchecked;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

struct Accounts<'a, 'b: 'a> {
    artwork_holder_wallet: &'a AccountInfo<'b>,
    artwork_holder_ata: &'a AccountInfo<'b>,
    artwork_meta: &'a AccountInfo<'b>,
    artwork_mint: &'a AccountInfo<'b>,
    escrow: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    rent_sysvar: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
}

pub fn execute(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    args: InitArtworkSaleArgs,
) -> ProgramResult {
    let a = parse_accounts(program_id, accounts)?;
    let rent = Rent::get()?;

    let mut metadata: ArtworkMetadata =
        try_from_slice_unchecked(&a.artwork_meta.data.borrow_mut())?;

    if *a.artwork_holder_wallet.key != metadata.holder {
        return Err(AllovrError::MissingRequiredSignature.into());
    }

    if metadata.offered_to != None
        || metadata.offer_currency != None
        || metadata.offer_price != None
    {
        return Err(AllovrError::ArtworkUnderOffer.into());
    }

    metadata.offered_to = Some(args.buyer);
    metadata.offer_price = Some(args.amount);
    metadata.offer_currency = Some(args.currency);
    metadata.payment_account = Some(args.payment_account);
    metadata.offer_count += 1;

    metadata.serialize(&mut &mut a.artwork_meta.data.borrow_mut()[..])?;

    let symbol_bytes = metadata.symbol.as_bytes();
    let offer_count = metadata.offer_count.to_string();

    let (mint_pub_key, _mint_seed_bump) = assert_pda(
        &a.artwork_mint,
        program_id,
        &[
            ARTWORK_SEED_PREFIX.as_bytes(),
            symbol_bytes,
            metadata.artist.as_ref(),
        ],
    )?;

    // check that the artwork ATA really belongs to the seller wallet
    assert_ata_mint_and_owner(
        &a.artwork_holder_ata,
        mint_pub_key,
        *a.artwork_holder_wallet.key,
    )?;

    let (escrow_pda, escrow_bump) = assert_pda(
        &a.escrow,
        program_id,
        &[
            ARTWORK_ESCROW_SEED_PREFIX.as_bytes(),
            symbol_bytes,
            offer_count.as_bytes(),
            metadata.artist.as_ref(),
        ],
    )?;

    create_raw(
        spl_token::id(),
        &a.escrow,
        &rent,
        &a.system,
        &a.artwork_holder_wallet,
        TOKEN_ACCOUNT_SIZE,
        &[
            ARTWORK_ESCROW_SEED_PREFIX.as_bytes(),
            symbol_bytes,
            offer_count.as_bytes(),
            metadata.artist.as_ref(),
            &[escrow_bump],
        ],
    )?;

    initialise_token_account(&a.escrow, &a.artwork_mint, escrow_pda, &a.rent_sysvar)?;

    transfer_token(
        &a.token_program,
        &a.artwork_holder_ata,
        &a.escrow,
        &a.artwork_holder_wallet,
        1,
    )?;

    // return Err(AllovrError::ManualFail.into());
    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        artwork_holder_wallet: next_account_info(account_iter)?,
        artwork_holder_ata: next_account_info(account_iter)?,
        artwork_meta: next_account_info(account_iter)?,
        artwork_mint: next_account_info(account_iter)?,
        escrow: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        rent_sysvar: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
    };

    assert_program_id(program_id)?;
    assert_token_program_matches_package(a.token_program)?;
    assert_signer(&a.artwork_holder_wallet)?;
    assert_owned_by(&a.artwork_holder_ata, &spl_token::id())?;
    assert_owned_by(&a.artwork_meta, &program_id)?;

    assert_ata_mint_and_owner(
        a.artwork_holder_ata,
        *a.artwork_mint.key,
        *a.artwork_holder_wallet.key,
    )?;

    Ok(a)
}
