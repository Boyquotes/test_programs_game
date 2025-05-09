mod initialize;
mod create_player_wallet;
mod update_player_wallet;

pub use initialize::*;
pub use create_player_wallet::*;
pub use update_player_wallet::*;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use crate::error::PlayerWalletError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum PlayerWalletInstruction {
    /// Initialize the program
    /// Accounts expected:
    /// 0. `[signer]` The program authority
    Initialize,
    
    /// Create a new player wallet
    /// Accounts expected:
    /// 0. `[signer]` The account owner
    /// 1. `[writable]` The player wallet account to create
    /// 2. `[]` The system program
    CreatePlayerWallet {
        /// Player name (max 50 characters, letters and numbers only)
        name: String,
    },
    
    /// Update player wallet data
    /// Accounts expected:
    /// 0. `[signer]` The account owner
    /// 1. `[writable]` The player wallet account to update
    UpdatePlayerWallet {
        /// Number of tokens
        nb_tokens: u32,
        /// Total amount of tokens
        amount_total_tokens: u32,
        /// Total value in stablecoin
        amount_total_value_stablecoin: u32,
        /// Portfolio date (timestamp)
        date_portfolio: u64,
    },
}

impl PlayerWalletInstruction {
    /// Unpacks a byte buffer into a PlayerWalletInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(PlayerWalletError::InvalidInstruction)?;
        
        Ok(match tag {
            0 => Self::Initialize,
            1 => {
                let name: String = BorshDeserialize::deserialize(&mut &rest[..])
                    .map_err(|_| PlayerWalletError::InvalidInstruction)?;
                Self::CreatePlayerWallet { name }
            },
            2 => {
                let payload: UpdatePlayerWalletPayload = BorshDeserialize::deserialize(&mut &rest[..])
                    .map_err(|_| PlayerWalletError::InvalidInstruction)?;
                Self::UpdatePlayerWallet {
                    nb_tokens: payload.nb_tokens,
                    amount_total_tokens: payload.amount_total_tokens,
                    amount_total_value_stablecoin: payload.amount_total_value_stablecoin,
                    date_portfolio: payload.date_portfolio,
                }
            },
            _ => return Err(PlayerWalletError::InvalidInstruction.into()),
        })
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct UpdatePlayerWalletPayload {
    nb_tokens: u32,
    amount_total_tokens: u32,
    amount_total_value_stablecoin: u32,
    date_portfolio: u64,
}
