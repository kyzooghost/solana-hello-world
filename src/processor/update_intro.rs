use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    program_pack::IsInitialized,
    borsh0_10::try_from_slice_unchecked,
};
use borsh::BorshSerialize;
use crate::state::IntroAccountState;
use crate::error::IntroError;



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
    let pda_intro = next_account_info(account_info_iter)?;

    // Validate that program owns PDA
    if pda_intro.owner != program_id {
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
    let (pda, _bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

    // Validate pda_intro is expected
    if pda != *pda_intro.key {
        msg!("Invalid seeds for PDA");
        return Err(IntroError::InvalidPDA.into())
    }

    // Ensure new_account_len <= 1000 bytes
    let new_account_len: usize = 1 + (4 + name.len()) + (4 + message.len());
    if new_account_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into())  
    };

    // Ensure data length <1000 bytes
    let account_len = 1000;
    if IntroAccountState::get_account_size(name.clone(), message.clone()) > account_len {
        msg!("Data length is larger than 1000 bytes");
        return Err(IntroError::InvalidDataLength.into())  
    }

    // Update account data
    msg!("unpacking state account");
    let mut account_data = try_from_slice_unchecked::<IntroAccountState>(&pda_intro.data.borrow()).unwrap();
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
    account_data.serialize(&mut &mut pda_intro.data.borrow_mut()[..])?;
    msg!("PDA updated");

    Ok(())
}
