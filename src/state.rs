// Struct PDA data fields

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    pubkey::Pubkey,
    program_pack::{IsInitialized, Sealed}
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroAccountState {
    // account_type
    pub discriminator: String,
    pub is_initialized: bool,
    pub introducer: Pubkey,
    pub name: String,
    pub message: String  
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroReplyCounter {
    pub discriminator: String,
    pub is_initialized: bool,
    pub counter: u64
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntroReply {
    pub discriminator: String,
    pub is_initialized: bool,
    pub intro: Pubkey,
    pub replier: Pubkey,
    pub reply: String,
    pub count: u64
}

// https://stackoverflow.com/questions/76213582/errore0277-the-trait-bound-movieaccountstate-borshdeborshdeserialize-i
impl IntroAccountState {
    pub const DISCRIMINATOR: &'static str = "intro";

    pub fn get_account_size(name: String, message: String) -> usize {
        return (4 + IntroAccountState::DISCRIMINATOR.len())
            + 1
            + 32
            + (4 + name.len())
            + (4 + message.len());
    }
}

impl IntroReplyCounter {
    pub const DISCRIMINATOR: &'static str = "counter";
    pub const SIZE: usize = (4 + IntroReplyCounter::DISCRIMINATOR.len()) + 1 + 8;
}

impl IntroReply {
    pub const DISCRIMINATOR: &'static str = "reply";

    pub fn get_account_size(reply: String) -> usize {
        return (4 + IntroReply::DISCRIMINATOR.len()) 
            + 1 
            + 32 
            + 32 
            + (4 + reply.len()) 
            + 8;
    }
}

impl Sealed for IntroAccountState {}

impl IsInitialized for IntroAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for IntroReplyCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for IntroReply {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}