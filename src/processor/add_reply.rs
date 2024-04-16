use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::IsInitialized,
    borsh0_10::try_from_slice_unchecked,
    native_token::LAMPORTS_PER_SOL,
};
use spl_associated_token_account::{instruction::create_associated_token_account, get_associated_token_address};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::convert::TryInto;
use borsh::BorshSerialize;
use crate::state::{IntroReplyCounter, IntroReply};
use crate::error::IntroError;
use crate::utils::is_account_initialized::is_account_initialized;

pub fn add_reply(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    reply: String,
) -> ProgramResult {
    msg!("Adding reply...");
    msg!("Reply: {}", reply);
    
    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let replier = next_account_info(account_info_iter)?;
    let pda_intro = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let pda_reply = next_account_info(account_info_iter)?;
    let token_mint = next_account_info(account_info_iter)?;
    let mint_auth = next_account_info(account_info_iter)?;
    let user_ata = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;

    // Check that `replier` is signer
    if !replier.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate that program owns PDAs
    if pda_intro.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    if pda_counter.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    // Validate token-related accounts
    msg!("deriving mint authority");
    let (mint_pda, _mint_bump) = Pubkey::find_program_address(&[b"token_mint"], program_id);
    let (mint_auth_pda, mint_auth_bump) = Pubkey::find_program_address(&[b"token_auth"], program_id);

    if *token_mint.key != mint_pda {
        msg!("Incorrect token mint");
        return Err(IntroError::IncorrectAccountError.into());
    }
    
    if *mint_auth.key != mint_auth_pda {
        msg!("Mint passed in and mint derived do not match");
        return Err(IntroError::InvalidPDA.into());
    }
    
    if *user_ata.key != get_associated_token_address(replier.key, token_mint.key) {
        msg!("Incorrect token mint");
        return Err(IntroError::IncorrectAccountError.into());
    }
    
    if *token_program.key != TOKEN_PROGRAM_ID {
        msg!("Incorrect token program");
        return Err(IntroError::IncorrectAccountError.into());
    }

    // Get counter_data
    let mut counter_data = try_from_slice_unchecked::<IntroReplyCounter>(&pda_counter.data.borrow()).unwrap();
    let counter_value = counter_data.counter.to_be_bytes();

    // Create Reply PDA
    let account_len = IntroReply::get_account_size(reply.clone());
    if account_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into());
    };

    // Get PDA
    let pda_seed = &[pda_intro.key.as_ref(), counter_value.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

    // Validate pda_reply is expected
    if pda != *pda_reply.key {
        msg!("Invalid seeds for Reply PDA");
        return Err(IntroError::InvalidPDA.into())
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    msg!("Creating Reply PDA");
    // Create new account
    let cpi_instruction = &system_instruction::create_account(
        replier.key, 
        pda_reply.key, 
        rent_lamports,
        account_len.try_into().unwrap(), 
        program_id
    );

    let cpi_accounts = &[replier.clone(), pda_reply.clone(), system_program.clone()];

    let intro_seed: &[u8] = pda_intro.key.as_ref();
    let counter_seed_casted: &[u8] = counter_value.as_ref();
    let bump_seed_casted: &[u8] = &[bump_seed];
    let signed_seeds:  &[&[&[u8]]] = &[&[intro_seed, counter_seed_casted, bump_seed_casted]];

    invoke_signed(cpi_instruction, cpi_accounts, signed_seeds)?;
    msg!("Reply Counter PDA created: {}", pda);

    msg!("Adding comment");
    let mut reply_data = try_from_slice_unchecked::<IntroReply>(&pda_reply.data.borrow()).unwrap();

    if reply_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    reply_data.discriminator = IntroReply::DISCRIMINATOR.to_string();
    reply_data.is_initialized = true;
    reply_data.intro = *pda_intro.key;
    reply_data.replier = *replier.key;
    reply_data.reply = reply;
    reply_data.count = counter_data.counter + 1;

    reply_data.serialize(&mut &mut pda_reply.data.borrow_mut()[..])?;

    msg!("Comment {} added!", counter_data.counter + 1);

    counter_data.counter += 1;
    counter_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;
    msg!("Counter updated to {}!", counter_data.counter);

    // Mint tokens

    if is_account_initialized(user_ata) == false {
        msg!("ATA not created, creating...");
        let instruction = create_associated_token_account(replier.key, replier.key, token_mint.key, token_program.key);
        // https://docs.rs/spl-associated-token-account/latest/spl_associated_token_account/fn.create_associated_token_account.html
        invoke(
            &instruction,
            &[
                replier.clone(),
                user_ata.clone(),
                token_mint.clone(),
                system_program.clone(),
                token_program.clone(),
            ]
        )?;
        msg!("Created user ATA");
    }

    msg!("Minting 5 tokens to User associated token account");
    invoke_signed(
        // Instruction
        &spl_token::instruction::mint_to(
            token_program.key,
            token_mint.key,
            user_ata.key,
            mint_auth.key,
            &[],
            5 * LAMPORTS_PER_SOL,
        )?,
        // Account_infos
        &[token_mint.clone(), user_ata.clone(), mint_auth.clone()],
        // Seeds
        &[&[b"token_auth", &[mint_auth_bump]]],
    )?;
    msg!("Minted 5 tokens to User associated token account");

    Ok(())
}
