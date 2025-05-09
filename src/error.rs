use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum PlayerWalletError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    
    #[error("Name too long")]
    NameTooLong,
    
    #[error("Invalid name format")]
    InvalidNameFormat,
    
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
    
    #[error("Account not initialized")]
    AccountNotInitialized,
    
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<PlayerWalletError> for ProgramError {
    fn from(e: PlayerWalletError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PlayerWalletError {
    fn type_of() -> &'static str {
        "PlayerWalletError"
    }
}
