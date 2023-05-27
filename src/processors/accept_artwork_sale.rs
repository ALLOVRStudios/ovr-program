use crate::state::ArtworkMetadata;
use crate::state::Currency;
use crate::ALLOVR_MINT_ID;
use crate::ARTWORK_ESCROW_SEED_PREFIX;
use crate::{error::AllovrError, utils::*, ARTWORK_SEED_PREFIX};
use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::program::invoke_signed;
use std::str::FromStr;

use solana_program::borsh::try_from_slice_unchecked;
use solana_program::program::invoke;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

struct Accounts<'a, 'b: 'a> {
    buyer_wallet: &'a AccountInfo<'b>,
    buyer_artwork_ata: &'a AccountInfo<'b>,
    buyer_payment: &'a AccountInfo<'b>, //  could be base address when paying with SOL or a token address like AOV
    artwork_holder: &'a AccountInfo<'b>,
    payment_account: &'a AccountInfo<'b>,
    artwork_meta: &'a AccountInfo<'b>,
    escrow: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
}

pub fn execute(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let a = parse_accounts(program_id, accounts)?;
    let mut metadata: ArtworkMetadata =
        try_from_slice_unchecked(&a.artwork_meta.data.borrow_mut())?;

    if *a.buyer_wallet.key != metadata.offered_to.unwrap() {
        return Err(AllovrError::IncorrectArtworkBuyer.into());
    }

    if *a.artwork_holder.key != metadata.holder {
        return Err(AllovrError::IncorrectArtworkHolder.into());
    }

    if *a.payment_account.key != metadata.payment_account.unwrap() {
        return Err(AllovrError::IncorrectPaymentAccount.into());
    }

    if metadata.offer_currency == None
        || metadata.offer_price == None
        || metadata.payment_account == None
    {
        return Err(AllovrError::InvalidOffer.into());
    }

    // Check the buyer artwork ATA belong to the buyer wallet and the correct artwork token
    let (artwork_mint_pda, _artwork_mint_bump) = Pubkey::find_program_address(
        &[
            ARTWORK_SEED_PREFIX.as_bytes(),
            metadata.symbol.as_bytes(),
            metadata.artist.as_ref(),
        ],
        &program_id,
    );

    msg!("Checking buyer artwork ATA has correct artwork mint and is owned by buyer account...");
    assert_ata_mint_and_owner(&a.buyer_artwork_ata, artwork_mint_pda, *a.buyer_wallet.key)?;

    // Take payment from buyer
    match metadata.offer_currency.unwrap() {
        Currency::SOL => {
            msg!("Checking buyer payment account is SOL account...");
            assert_owned_by(&a.buyer_payment, a.system_program.key)?;
            msg!("Checking seller payment account is SOL account...");
            assert_owned_by(&a.payment_account, a.system_program.key)?;
            invoke(
                &solana_program::system_instruction::transfer(
                    &a.buyer_payment.key,
                    a.payment_account.key,
                    metadata.offer_price.unwrap(),
                ),
                &[
                    a.buyer_payment.clone(),
                    a.payment_account.clone(),
                    a.system_program.clone(),
                ],
            )?;
        }
        Currency::AOVR => {
            msg!("Checking buyer payment account is AOVR account owned by buyer wallet account...");
            assert_ata_mint_and_owner(
                &a.buyer_payment,
                Pubkey::from_str(ALLOVR_MINT_ID).unwrap(),
                *a.buyer_wallet.key,
            )?;
            msg!(
                "Checking seller payment account is AOVR account owned by seller wallet account..."
            );
            assert_ata_mint_and_owner(
                &a.payment_account,
                Pubkey::from_str(ALLOVR_MINT_ID).unwrap(),
                *a.artwork_holder.key,
            )?;

            transfer_token(
                &a.token_program,
                &a.buyer_payment,
                &a.payment_account,
                &a.buyer_wallet,
                metadata.offer_price.unwrap(),
            )?;
        }
    }

    let offer_count_string = metadata.offer_count.to_string();

    msg!("Checking escrow PDA is correct...");
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

    // send artwork to buyer ATA
    msg!("Sending artwork to buyer...");
    let ix = spl_token::instruction::transfer(
        &a.token_program.key,
        &a.escrow.key,
        &a.buyer_artwork_ata.key,
        &escrow_pda,
        &[],
        1,
    )?;

    invoke_signed(
        &ix,
        &[
            a.escrow.clone(),
            a.buyer_artwork_ata.clone(),
            a.token_program.clone(),
        ],
        &[escrow_signer_seeds],
    )?;

    // close escrow account and send back SOL to payer
    msg!("Closing escrow account...");
    let close_escrow_ix = spl_token::instruction::close_account(
        &a.token_program.key,
        &a.escrow.key,
        &a.artwork_holder.key,
        &escrow_pda,
        &[],
    )?;

    invoke_signed(
        &close_escrow_ix,
        &[
            a.escrow.clone(),
            a.artwork_holder.clone(),
            a.token_program.clone(),
        ],
        &[escrow_signer_seeds],
    )?;

    // Clear metadata ready for next sale and set new holder
    metadata.offered_to = None;
    metadata.offer_price = None;
    metadata.offer_currency = None;
    metadata.payment_account = None;
    metadata.holder = *a.buyer_wallet.key;

    msg!("Setting artwork metadata...");
    metadata.serialize(&mut &mut a.artwork_meta.data.borrow_mut()[..])?;

    Ok(())
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let a = Accounts {
        buyer_wallet: next_account_info(account_iter)?,
        buyer_artwork_ata: next_account_info(account_iter)?,
        buyer_payment: next_account_info(account_iter)?,
        artwork_holder: next_account_info(account_iter)?,
        payment_account: next_account_info(account_iter)?,
        artwork_meta: next_account_info(account_iter)?,
        escrow: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
        system_program: next_account_info(account_iter)?,
    };

    assert_program_id(&program_id)?;
    assert_system(&a.system_program)?;
    assert_signer(&a.buyer_wallet)?;
    assert_token_program_matches_package(&a.token_program)?;

    // TODO: More checks!

    Ok(a)
}
