use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
};
use crate::instruction::IntroInstruction;

mod add_intro;
mod update_intro;
mod add_reply;

use add_intro::add_intro;
use update_intro::update_intro;
use add_reply::add_reply;

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
        },
        IntroInstruction::AddReply { reply } => {
            add_reply(program_id, accounts, reply)
        },
    }
}
