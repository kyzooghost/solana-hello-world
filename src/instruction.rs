// Define and deserialize instructions

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum IntroInstruction {
    AddIntro {
        name: String,
        message: String,
    },
    UpdateIntro {
        name: String,
        message: String,
    }
}

#[derive(BorshDeserialize)]
struct IntroPayload {
    name: String,
    message: String
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = IntroPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddIntro {
                name: payload.name,
                message: payload.message
            },
            1 => Self::UpdateIntro {
                name: payload.name,
                message: payload.message
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}