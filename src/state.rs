// Struct for data field of PDA

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub message: String  
}

// https://stackoverflow.com/questions/76213582/errore0277-the-trait-bound-movieaccountstate-borshdeborshdeserialize-i
impl IntroAccountState {
    pub fn try_from_slice_unchecked(data: &[u8]) -> Result<IntroAccountState, ProgramError> {
        let mut data_mut = data;
        match IntroAccountState::deserialize(&mut data_mut) {
            Ok(result) => Ok(result),
            Err(_) => Err(ProgramError::InvalidInstructionData)
        }
    }
}