use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::error::PlayerWalletError;
use crate::constants::MAX_NAME_LENGTH;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PlayerWallet {
    // Flag to check if the account is initialized
    pub is_initialized: bool,
    
    // Player name (max 50 characters, letters and numbers only)
    pub name: String,
    
    // Player wallet address (Solana public key)
    pub wallet_address: Pubkey,
    
    // Number of tokens owned
    pub nb_tokens: u32,
    
    // Number of transactions made
    pub nb_transactions: u32,
    
    // Total amount of tokens
    pub amount_total_tokens: u32,
    
    // Total value in stablecoin
    pub amount_total_value_stablecoin: u32,
    
    // Portfolio date (timestamp)
    pub date_portfolio: u64,
}

impl PlayerWallet {
    pub fn new(name: String, wallet_address: Pubkey) -> Result<Self, ProgramError> {
        // Validate name length
        if name.len() > MAX_NAME_LENGTH {
            return Err(PlayerWalletError::NameTooLong.into());
        }
        
        // Validate name format (letters and numbers only)
        if !name.chars().all(|c| c.is_alphanumeric()) {
            return Err(PlayerWalletError::InvalidNameFormat.into());
        }
        
        Ok(Self {
            is_initialized: true,
            name,
            wallet_address,
            nb_tokens: 0,
            nb_transactions: 0,
            amount_total_tokens: 0,
            amount_total_value_stablecoin: 0,
            date_portfolio: 0,
        })
    }
    
    // Get the size of the PlayerWallet struct for account allocation
    pub fn get_account_size(name_len: usize) -> usize {
        // Size calculation:
        // - is_initialized: 1 byte (bool)
        // - name: variable length string
        // - wallet_address: 32 bytes (Pubkey)
        // - nb_tokens: 4 bytes (u32)
        // - nb_transactions: 4 bytes (u32)
        // - amount_total_tokens: 4 bytes (u32)
        // - amount_total_value_stablecoin: 4 bytes (u32)
        // - date_portfolio: 8 bytes (u64)
        
        // Note: Borsh serialization adds 4 bytes for String length
        1 + 4 + name_len + 32 + 4 + 4 + 4 + 4 + 8
    }
}
