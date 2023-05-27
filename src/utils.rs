use crate::error::AllovrError;
use crate::instruction::RegisterArtistArgs;
use crate::ALL_DECIMAL_PLACES;
use crate::ARTIST_METADATA_DESCRIPTION_SIZE;
use crate::ARTIST_METADATA_NAME_SIZE;
use crate::ARTIST_METADATA_URI_SIZE;
use mpl_token_metadata::state::DataV2;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program::sysvar::rent::Rent;
use std::convert::TryInto;
use std::str::FromStr;

use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::instruction::update_metadata_accounts_v2;

pub fn ui_amount_to_amount(aov: f64) -> u64 {
    spl_token::ui_amount_to_amount(aov, ALL_DECIMAL_PLACES)
}

pub fn assert_rent_exempt(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        Err(AllovrError::NotRentExempt.into())
    } else {
        Ok(())
    }
}

pub fn assert_pda(
    account_info: &AccountInfo,
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> Result<(Pubkey, u8), AllovrError> {
    let (derived_pub_key, derived_seed_bump) = Pubkey::find_program_address(seeds, &program_id);

    if derived_pub_key != *account_info.key {
        msg!(
            "assert_pda failed - DERIVED. {} - PROVIDED. {}",
            &derived_pub_key,
            account_info.key
        );
        return Err(AllovrError::InvalidPda.into());
    }

    Ok((derived_pub_key, derived_seed_bump))
}

pub fn assert_token_program_matches_package(token_program_info: &AccountInfo) -> ProgramResult {
    if *token_program_info.key != spl_token::id() {
        return Err(AllovrError::InvalidTokenProgram.into());
    } else {
        Ok(())
    }
}

pub fn assert_ata_program_matches_package(ata_program_info: &AccountInfo) -> ProgramResult {
    if *ata_program_info.key != spl_associated_token_account::id() {
        return Err(AllovrError::InvalidAssociatedTokenAccountProgram.into());
    } else {
        Ok(())
    }
}

pub fn assert_system(system_program_info: &AccountInfo) -> ProgramResult {
    if *system_program_info.key != solana_program::system_program::id() {
        return Err(AllovrError::InvalidSystemProgramId.into());
    } else {
        Ok(())
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        msg!(
            "{} Owner Invalid, Expected {}, Got {}",
            account.key,
            owner,
            account.owner
        );
        Err(AllovrError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_ata_mint_and_owner(
    account: &AccountInfo,
    mint: Pubkey,
    owner: Pubkey,
) -> ProgramResult {
    assert_owned_by(account, &spl_token::id())?;
    let token_data: spl_token::state::Account =
        spl_token::state::Account::unpack_from_slice(&account.try_borrow_mut_data()?).unwrap();

    if token_data.owner != owner || token_data.mint != mint {
        msg!(
            "{} Token Owner or Mint Invalid, Expected Owner {}, Got Owner {}, Expected Mint {}, Got Mint {}",
            account.key,
            owner,
            token_data.owner,
            mint,
            token_data.mint,
        );
        Err(AllovrError::IncorrectTokenOwnerOrMint.into())
    } else {
        Ok(())
    }
}

pub fn get_token_supply(mint: &AccountInfo) -> u64 {
    let mint_data: spl_token::state::Mint =
        spl_token::state::Mint::unpack_from_slice(&mint.try_borrow_mut_data().unwrap()).unwrap();
    return mint_data.supply;
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(AllovrError::MissingRequiredSignature.into())
    } else {
        Ok(())
    }
}

pub fn assert_metaplex_program(account_info: &AccountInfo) -> ProgramResult {
    if *account_info.key != mpl_token_metadata::ID {
        Err(AllovrError::InvalidMetaplexMetadataProgramId.into())
    } else {
        Ok(())
    }
}

pub fn assert_program_id(program_id: &Pubkey) -> ProgramResult {
    if *program_id != Pubkey::from_str(crate::ALLOVR_PROGRAM_ID).unwrap() {
        Err(AllovrError::InvalidProgramId.into())
    } else {
        Ok(())
    }
}

pub fn assert_state(state: &Pubkey) -> ProgramResult {
    if *state != Pubkey::from_str(crate::ALLOVR_STATE_ID).unwrap() {
        Err(AllovrError::InvalidStateAccount.into())
    } else {
        Ok(())
    }
}

pub fn assert_aovr_treasury(treasury: &Pubkey) -> ProgramResult {
    if *treasury != Pubkey::from_str(crate::ALLOVR_AOVR_TREASURY_ID).unwrap() {
        Err(AllovrError::InvalidAllovrTreasury.into())
    } else {
        Ok(())
    }
}

pub fn assert_clock(clock: &AccountInfo) -> ProgramResult {
    if *clock.key != solana_program::sysvar::clock::id() {
        Err(AllovrError::InvalidClockSysvarId.into())
    } else {
        Ok(())
    }
}

pub fn create_account<'a>(
    rent: &Rent,
    account_size: usize,
    payer_account: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    owner_id: Pubkey,
) -> ProgramResult {
    let lamports_required = rent.minimum_balance(account_size);
    let create_account_ix = system_instruction::create_account(
        &payer_account.key,
        &new_account.key,
        lamports_required,
        account_size.try_into().unwrap(),
        &owner_id,
    );
    invoke(
        &create_account_ix,
        &[payer_account.clone(), new_account.clone()],
    )
}

pub fn create_raw<'a>(
    program_id: Pubkey,
    new_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account.key, required_lamports),
            &[
                payer_info.clone(),
                new_account.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    let accounts = &[new_account.clone(), system_program_info.clone()];

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account.key, size.try_into().unwrap()),
        accounts,
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account.key, &program_id),
        accounts,
        &[&signer_seeds],
    )?;

    Ok(())
}

pub fn transfer_token<'a>(
    token_program: &AccountInfo<'a>,
    holder: &AccountInfo<'a>,
    recipient: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    let ix = spl_token::instruction::transfer(
        &token_program.key,
        &holder.key,
        &recipient.key,
        &authority.key,
        &[],
        amount,
    )?;

    invoke(
        &ix,
        &[
            holder.clone(),
            recipient.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}

pub fn create_pda_account<'a>(
    rent: &Rent,
    account_size: usize,
    payer_account: &AccountInfo<'a>,
    pda_account: &AccountInfo<'a>,
    program_id: Pubkey,
    pda_seed: &str,
    pda_seed_bump: u8,
) -> ProgramResult {
    let lamports_required = rent.minimum_balance(account_size);

    let signers_seeds = &[
        pda_seed.as_bytes(),
        &payer_account.key.to_bytes(),
        &[pda_seed_bump],
    ];

    let create_pda_account_ix = system_instruction::create_account(
        &payer_account.key,
        &pda_account.key,
        lamports_required,
        account_size.try_into().unwrap(),
        &program_id,
    );
    invoke_signed(
        &create_pda_account_ix,
        &[payer_account.clone(), pda_account.clone()],
        &[signers_seeds],
    )
}

pub fn initialise_token_account<'a>(
    account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: Pubkey,
    rent_sysvar: &AccountInfo<'a>,
) -> ProgramResult {
    let ix = spl_token::instruction::initialize_account2(
        &spl_token::id(),
        &account.key,
        &mint.key,
        &owner,
    )?;

    invoke(&ix, &[account.clone(), mint.clone(), rent_sysvar.clone()])?;
    Ok(())
}

pub fn initalise_mint_account<'a>(
    mint_account: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    seed_prefix: &str,
    seed_bump: u8,
    decimals: u8,
) -> ProgramResult {
    let signers_seeds = &[seed_prefix.as_bytes(), &[seed_bump]];

    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        mint_account.key,
        mint_authority.key,
        None,
        decimals,
    )?;

    invoke_signed(
        &initialize_mint_ix,
        &[
            mint_account.clone(),
            rent_account.clone(),
            mint_authority.clone(),
        ],
        &[signers_seeds],
    )?;

    Ok(())
}

pub fn initalise_pda_mint_account<'a>(
    mint_account: &AccountInfo<'a>,
    payer_account: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    signers_seeds: &[&[u8]],
    decimals: u8,
) -> ProgramResult {
    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        mint_account.key,
        mint_account.key,
        None,
        decimals,
    )?;

    invoke_signed(
        &initialize_mint_ix,
        &[
            mint_account.clone(),
            rent_account.clone(),
            payer_account.clone(),
        ],
        &[signers_seeds],
    )?;

    Ok(())
}

pub fn created_associated_token_account<'a>(
    payer_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    owner_wallet_account: &AccountInfo<'a>,
    owner_token_account: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    system_account: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
) -> ProgramResult {
    let associated_token_account_address =
        spl_associated_token_account::get_associated_token_address(
            owner_wallet_account.key,
            mint_account.key,
        );
    if *owner_token_account.key != associated_token_account_address {
        return Err(AllovrError::InvalidAssociatedTokenAccount.into());
    }

    let create_associated_token_account_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            payer_account.key,
            owner_wallet_account.key,
            mint_account.key,
        );

    invoke(
        &create_associated_token_account_ix,
        &[
            payer_account.clone(),
            owner_token_account.clone(),
            owner_wallet_account.clone(),
            mint_account.clone(),
            system_account.clone(),
            token_account.clone(),
            rent_account.clone(),
        ],
    )?;

    Ok(())
}

pub fn mint_tokens_to<'a>(
    mint_account: &AccountInfo<'a>,
    mint_authority_account: &AccountInfo<'a>,
    recipient_token_account: &AccountInfo<'a>,
    signers_seeds: &[&[&[u8]]],
    amount: u64,
    close_after_mint: bool,
) -> ProgramResult {
    let mint_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_account.key,
        &recipient_token_account.key,
        &mint_authority_account.key,
        &[],
        amount,
    )?;

    invoke_signed(
        &mint_ix,
        &[
            mint_account.clone(),
            recipient_token_account.clone(),
            mint_authority_account.clone(),
        ],
        signers_seeds,
    )?;

    if close_after_mint {
        let close_mint_ix = spl_token::instruction::set_authority(
            &spl_token::id(),
            mint_account.key,
            None,
            spl_token::instruction::AuthorityType::MintTokens,
            mint_account.key,
            &[],
        )?;

        invoke_signed(&close_mint_ix, &[mint_account.clone()], signers_seeds)?;
    }

    Ok(())
}

pub fn create_ata<'a>(
    funding_account: &AccountInfo<'a>,
    wallet_account: &AccountInfo<'a>,
    ata_account: &AccountInfo<'a>,
    spl_token_mint_account: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
    error: AllovrError,
) -> ProgramResult {
    let ata_address = spl_associated_token_account::get_associated_token_address(
        wallet_account.key,
        spl_token_mint_account.key,
    );

    if ata_account.key.ne(&ata_address) {
        return Err(error.into());
    }

    let ix = spl_associated_token_account::instruction::create_associated_token_account(
        funding_account.key,
        wallet_account.key,
        spl_token_mint_account.key,
    );

    invoke(
        &ix,
        &[
            funding_account.clone(),
            wallet_account.clone(),
            rent_sysvar.clone(),
            ata_account.clone(),
            spl_token_mint_account.clone(),
        ],
    )?;

    Ok(())
}

pub fn santitise_artist_data(args: RegisterArtistArgs) -> Result<RegisterArtistArgs, ProgramError> {
    let artist_name = args.name.trim().to_string();
    if artist_name.len() == 0 || artist_name.len() > ARTIST_METADATA_NAME_SIZE {
        return Err(AllovrError::InvalidArtistName.into());
    }

    let artist_description = args.description.trim().to_string();
    if artist_description.len() == 0 || artist_description.len() > ARTIST_METADATA_DESCRIPTION_SIZE
    {
        return Err(AllovrError::InvalidArtistDescription.into());
    }

    let artist_token_symbol = args.token_symbol.trim().to_string();
    if artist_token_symbol.len() != 3 && artist_token_symbol.len() != 4 {
        return Err(AllovrError::InvalidArtistSymbol.into());
    }

    let mut artist_uri_option = None;
    if args.uri.is_some() {
        let artist_uri = args.uri.unwrap().trim().to_string();
        if artist_uri.len() > ARTIST_METADATA_URI_SIZE {
            return Err(AllovrError::InvalidArtistUri.into());
        }

        artist_uri_option = Some(artist_uri);
    }

    let mut create_metaplex_metadata_option: Option<bool> = None;
    if args.create_metaplex_metadata.is_some() {
        let create_metaplex_metadata = args.create_metaplex_metadata.unwrap();
        create_metaplex_metadata_option = Some(create_metaplex_metadata);
    }

    let response = RegisterArtistArgs {
        name: artist_name,
        description: artist_description,
        token_symbol: artist_token_symbol,
        uri: artist_uri_option,
        create_metaplex_metadata: create_metaplex_metadata_option,
    };

    Ok(response)
}

pub fn create_metaplex_metadata_account<'a>(
    metadata_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    artist_account: &AccountInfo<'a>,
    meta_program_account: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    name: String,
    symbol: String,
    uri: String,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let cmda_instruction = create_metadata_accounts_v3(
        mpl_token_metadata::ID,
        *metadata_account.key,
        *mint_account.key,
        *mint_account.key,
        *artist_account.key,
        *mint_account.key,
        name,
        symbol,
        uri,
        None,
        0,
        true,
        true,
        None,
        None,
        None,
    );

    let metadata_infos = vec![
        metadata_account.clone(),
        mint_account.clone(),
        artist_account.clone(),
        meta_program_account.clone(),
        rent_account.clone(),
    ];

    // create meta data accounts
    invoke_signed(
        &cmda_instruction,
        metadata_infos.as_slice(),
        &[signer_seeds],
    )?;

    Ok(())
}

pub fn update_metaplex_metadata_account<'a>(
    metadata_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    meta_program_account: &AccountInfo<'a>,
    name: String,
    symbol: String,
    uri: String,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let data = DataV2 {
        name: name,
        symbol: symbol,
        uri: uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let cmda_instruction = update_metadata_accounts_v2(
        mpl_token_metadata::ID,
        *metadata_account.key,
        *mint_account.key,
        Some(*mint_account.key),
        Some(data),
        None,
        Some(true),
    );

    let metadata_infos = vec![
        metadata_account.clone(),
        mint_account.clone(),
        meta_program_account.clone(),
        // rent_account.clone(),
    ];
    invoke_signed(
        &cmda_instruction,
        metadata_infos.as_slice(),
        &[signer_seeds],
    )?;

    Ok(())
}
