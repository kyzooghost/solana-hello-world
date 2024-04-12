use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized
};
use std::convert::TryInto;
use borsh::BorshSerialize;
use crate::instruction::IntroInstruction;
use crate::state::IntroAccountState;
use crate::error::IntroError;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = IntroInstruction::unpack(instruction_data)?;
    match instruction {
        IntroInstruction::AddIntro { name, message } => {
            add_intro(program_id, accounts, name, message)
        },
        IntroInstruction::UpdateIntro { name, message } => {
            update_intro(program_id, accounts, name, message)
        }
    }
}

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
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Check that `initializer` is signer
    // No msg.sender global variable available in Solana, and fee-payer is not available here either
    // So convention is to send initiating address as the first provided account => Check this is signer
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Get PDA
    let pda_seed = &[initializer.key.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

    // Validate pda_account is expected
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(IntroError::InvalidPDA.into())
    }

    // Ensure data length <1000 bytes
    let data_len: usize = 1 + (4 + name.len()) + (4 + message.len());
    if data_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into())  
    }
    let account_len = 1000;

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    msg!("Creating PDA");
    // Create new account
    let cpi_instruction = &system_instruction::create_account(
        initializer.key, 
        pda_account.key, 
        rent_lamports, 
        account_len.try_into().unwrap(), 
        program_id
    );

    let cpi_accounts = &[initializer.clone(), pda_account.clone(), system_program.clone()];
    let initializer_seed = initializer.key.as_ref();
    let bump_seed_casted: &[u8] = &[bump_seed];
    let signed_seeds:  &[&[&[u8]]] = &[&[initializer_seed, bump_seed_casted]];

    invoke_signed(cpi_instruction, cpi_accounts, signed_seeds)?;
    msg!("PDA created: {}", pda);

    // Update account data
    msg!("unpacking state account");
    let mut account_data = IntroAccountState::try_from_slice_unchecked(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    // Account is_initialized validation
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.is_initialized = true;
    account_data.name = name;
    account_data.message = message;

    // Take 'account_data' object, serialize it, set it to writer object
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("PDA updated");

    Ok(())
}

pub fn update_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String
) -> ProgramResult {
    msg!("Updating introduction...");
    msg!("Name: {}", name);
    msg!("Message: {}", message);
    
    // Get Account iterator
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    // Validate that program owns PDA
    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    // Check that `initializer` is signer
    // No msg.sender global variable available in Solana, and fee-payer is not available here either
    // So convention is to send initiating address as the first provided account => Check this is signer
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Get PDA
    let pda_seed = &[initializer.key.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

    // Validate pda_account is expected
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(IntroError::InvalidPDA.into())
    }

    // Ensure new_account_len <= 1000 bytes
    let new_account_len: usize = 1 + (4 + name.len()) + (4 + message.len());
    if new_account_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into())  
    };

    // Update account data
    msg!("unpacking state account");
    let mut account_data = IntroAccountState::try_from_slice_unchecked(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    // Account is_initialized validation
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(IntroError::UninitializedAccount.into());
    }

    msg!("Intro before update:");
    msg!("Name: {}", account_data.name);
    msg!("Message: {}", account_data.message);

    account_data.name = name;
    account_data.message = message;

    msg!("Intro after update:");
    msg!("Name: {}", account_data.name);
    msg!("Message: {}", account_data.message);

    msg!("serializing account");
    // Take 'account_data' object, serialize it, set it to writer object
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("PDA updated");

    Ok(())
}
