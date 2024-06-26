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
    },
    AddReply {
        reply: String
    },
    InitializeMint
}

#[derive(BorshDeserialize)]
struct IntroPayload {
    name: String,
    message: String
}

#[derive(BorshDeserialize)]
struct ReplyPayload {
    reply: String
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = IntroPayload::try_from_slice(rest).unwrap();
                Self::AddIntro {
                    name: payload.name,
                    message: payload.message
                }
            },
            1 => {
                let payload = IntroPayload::try_from_slice(rest).unwrap();
                Self::UpdateIntro {
                    name: payload.name,
                    message: payload.message
                }
            },
            2 => {
                let payload = ReplyPayload::try_from_slice(rest).unwrap();
                Self::AddReply {
                    reply: payload.reply                
                }
            },
            3 => Self::InitializeMint,
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}