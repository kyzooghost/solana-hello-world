use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    borsh0_10::try_from_slice_unchecked,
};
use std::convert::TryInto;
use borsh::BorshSerialize;
use crate::state::{IntroAccountState, IntroReplyCounter};
use crate::error::IntroError;

pub fn add_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String
) -> ProgramResult {
    msg!("Adding introduction...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);
    
    // Get Account iterator
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_intro = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Check that `initializer` is signer
    // No msg.sender global variable available in Solana, and fee-payer is not available here either
    // So convention is to send initiating address as the first provided account => Check this is signer
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::get()?;

    /*** CREATE INTRO PDA ***/
    {
        // Get PDA
        let pda_seed = &[initializer.key.as_ref()];
        let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

        // Validate pda_intro is expected
        if pda != *pda_intro.key {
            msg!("Invalid seeds for Intro PDA");
            return Err(IntroError::InvalidPDA.into())
        }

        // Ensure data length <1000 bytes
        let account_len = 1000;
        if IntroAccountState::get_account_size(name.clone(), message.clone()) > account_len {
            msg!("Intro Account data larger than 1000 bytes");
            return Err(IntroError::InvalidDataLength.into())  
        }

        // Compute rent
        let rent_lamports = rent.minimum_balance(account_len);

        msg!("Creating Intro PDA");
        // Create new account
        let cpi_instruction = &system_instruction::create_account(
            initializer.key, 
            pda_intro.key, 
            rent_lamports, 
            account_len.try_into().unwrap(), 
            program_id
        );
    
        let cpi_accounts = &[initializer.clone(), pda_intro.clone(), system_program.clone()];
        let initializer_seed = initializer.key.as_ref();
        let bump_seed_casted: &[u8] = &[bump_seed];
        let signed_seeds:  &[&[&[u8]]] = &[&[initializer_seed, bump_seed_casted]];
    
        invoke_signed(cpi_instruction, cpi_accounts, signed_seeds)?;
        msg!("Intro PDA created: {}", pda);
    
        // Update account data
        msg!("unpacking Intro PDA");
        let mut account_data = try_from_slice_unchecked::<IntroAccountState>(&pda_intro.data.borrow()).unwrap();
        msg!("borrowed Intro PDA data");
    
        // Account is_initialized validation
        if account_data.is_initialized() {
            msg!("Intro PDA already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    
        account_data.discriminator = IntroAccountState::DISCRIMINATOR.to_string();
        account_data.is_initialized = true;
        account_data.introducer = *initializer.key;
        account_data.name = name;
        account_data.message = message;
    
        // Take 'account_data' object, serialize it, set it to writer object
        account_data.serialize(&mut &mut pda_intro.data.borrow_mut()[..])?;
        msg!("Intro PDA initialized");
    }

    /*** CREATE REPLY COUNTER PDA ***/
    {
        // Get PDA
        let reply_counter_seed = "reply";
        let pda_seed = &[initializer.key.as_ref(), reply_counter_seed.as_ref()];
        let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

        // Validate pda_counter is expected
        if pda != *pda_counter.key {
            msg!("Invalid seeds for Reply Counter PDA");
            return Err(IntroError::InvalidPDA.into())
        }

        // Ensure data length <1000 bytes
        let account_len = IntroReplyCounter::SIZE;

        // Compute rent
        let rent_lamports = rent.minimum_balance(account_len);

        msg!("Creating Intro PDA");
        // Create new account
        let cpi_instruction = &system_instruction::create_account(
            initializer.key, 
            pda_counter.key, 
            rent_lamports, 
            account_len.try_into().unwrap(), 
            program_id
        );
    
        let cpi_accounts = &[initializer.clone(), pda_counter.clone(), system_program.clone()];

        let initializer_seed: &[u8] = initializer.key.as_ref();
        let reply_counter_seed_casted: &[u8] = reply_counter_seed.as_ref();
        let bump_seed_casted: &[u8] = &[bump_seed];
        let signed_seeds:  &[&[&[u8]]] = &[&[initializer_seed, reply_counter_seed_casted, bump_seed_casted]];
    
        invoke_signed(cpi_instruction, cpi_accounts, signed_seeds)?;
        msg!("Reply Counter PDA created: {}", pda);
    
        // Update account data
        msg!("unpacking Reply Counter PDA");
        let mut account_data = try_from_slice_unchecked::<IntroReplyCounter>(&pda_counter.data.borrow()).unwrap();

        // let mut account_data = IntroReplyCounter::try_from_slice_unchecked(&pda_counter.data.borrow()).unwrap();
        msg!("borrowed Reply Counter PDA data");
    
        // Account is_initialized validation
        if account_data.is_initialized() {
            msg!("Reply Counter PDA already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }
    
        account_data.discriminator = IntroReplyCounter::DISCRIMINATOR.to_string();
        account_data.is_initialized = true;
        account_data.counter = 0;

        // Take 'account_data' object, serialize it, set it to writer object
        account_data.serialize(&mut &mut pda_counter.data.borrow_mut()[..])?;
        msg!("Reply Counter PDA initialized");
    }


    Ok(())
}
