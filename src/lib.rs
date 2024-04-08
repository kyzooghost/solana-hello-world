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
    msg!("Hello world!");
    // Expected success return value
    Ok(())
}
