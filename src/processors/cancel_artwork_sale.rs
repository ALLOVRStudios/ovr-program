use crate::state::ArtworkMetadata;
use crate::ARTWORK_ESCROW_SEED_PREFIX;
use crate::{error::AllovrError, utils::*, ARTWORK_SEED_PREFIX};
use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::program::invoke_signed;

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
    escrow: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
}

pub fn execute(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let a = parse_accounts(program_id, accounts)?;
    let mut metadata: ArtworkMetadata =
        try_from_slice_unchecked(&a.artwork_meta.data.borrow_mut())?;

    msg!(
        "Checking escrow PDA symbol {:?} artist {:?}",
        metadata.symbol,
        metadata.artist
    );

    let (artwork_mint_pda, _artwork_mint_bump) = Pubkey::find_program_address(
        &[
            ARTWORK_SEED_PREFIX.as_bytes(),
            metadata.symbol.as_bytes(),
            metadata.artist.as_ref(),
        ],
        &program_id,
    );

    assert_ata_mint_and_owner(
        a.artwork_holder_ata,
        artwork_mint_pda,
        *a.artwork_holder_wallet.key,
    )?;

    if *a.artwork_holder_wallet.key != metadata.holder {
        return Err(AllovrError::MissingRequiredSignature.into());
    }

    metadata.offered_to = None;
    metadata.offer_price = None;
    metadata.offer_currency = None;
    metadata.payment_account = None;

    metadata.serialize(&mut &mut a.artwork_meta.data.borrow_mut()[..])?;

    let offer_count_string = metadata.offer_count.to_string();

    let (escrow_pda, escrow_bump) = assert_pda(
        &a.escrow,
        program_id,
        &[
            ARTWORK_ESCROW_SEED_PREFIX.as_bytes(),
            metadata.symbol.as_bytes(),
            offer_count_string.as_bytes(),
            metadata.artist.as_ref(),
        ],
    )?;

    let escrow_signer_seeds = &[
        ARTWORK_ESCROW_SEED_PREFIX.as_bytes(),
        metadata.symbol.as_bytes(),
        offer_count_string.as_bytes(),
        metadata.artist.as_ref(),
        &[escrow_bump],
    ];

    // return artwork from escrow account
    let transfer_artwork_ix = spl_token::instruction::transfer(
        &a.token_program.key,
        &a.escrow.key,
        &a.artwork_holder_ata.key,
        &escrow_pda,
        &[],
        1,
    )?;

    invoke_signed(
        &transfer_artwork_ix,
        &[
            a.escrow.clone(),
            a.artwork_holder_ata.clone(),
            a.token_program.clone(),
        ],
        &[escrow_signer_seeds],
    )?;

    // close escrow account and send back SOL to payer

    let close_escrow_ix = spl_token::instruction::close_account(
        &a.token_program.key,
        &a.escrow.key,
        &a.artwork_holder_wallet.key,
        &escrow_pda,
        &[],
    )?;

    invoke_signed(
        &close_escrow_ix,
        &[
            a.escrow.clone(),
            a.artwork_holder_wallet.clone(),
            a.token_program.clone(),
        ],
        &[escrow_signer_seeds],
    )?;

    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        artwork_holder_wallet: next_account_info(account_iter)?,
        artwork_holder_ata: next_account_info(account_iter)?,
        artwork_meta: next_account_info(account_iter)?,
        escrow: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
    };

    assert_signer(&a.artwork_holder_wallet)?;
    Ok(a)
}
