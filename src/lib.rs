pub mod instruction;
use instruction::IntroInstruction;

use solana_program::{
    account_info::AccountInfo,
    // Macro that defines entry point
    entrypoint,
    // Result of ProgramError
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    // Macro to log messages
    msg
};

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
    Ok(())
}
