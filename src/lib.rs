use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
};
use std::convert::TryInto;

pub mod instruction;
pub mod state;

use instruction::IntroInstruction;
use state::IntroAccountState;
use borsh::BorshSerialize;

// Declare entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = IntroInstruction::unpack(instruction_data)?;
    match instruction {
        IntroInstruction::AddIntro { name, message } => {
            add_intro(program_id, accounts, name, message)
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

    // Get PDA
    let pda_seed = &[initializer.key.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(pda_seed, program_id);

    // Calculate account size required
    let account_len: usize = 1 + (4 + name.len()) + (4 + message.len());

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
    let mut account_data = IntroAccountState::try_from_slice_unchecked(&pda_account.data.borrow()).unwrap();

    account_data.is_initialized = true;
    account_data.name = name;
    account_data.message = message;

    // Take 'account_data' object, serialize it, set it to writer object
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("PDA updated");

    Ok(())
}
