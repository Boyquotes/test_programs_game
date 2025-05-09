use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::PlayerWallet;
use crate::error::PlayerWalletError;

/// Update an existing player wallet account
pub fn process_update_player_wallet(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    nb_tokens: u32,
    amount_total_tokens: u32,
    amount_total_value_stablecoin: u32,
    date_portfolio: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get the required accounts
    let owner_info = next_account_info(account_info_iter)?;
    let player_wallet_info = next_account_info(account_info_iter)?;
    
    // Ensure the owner is a signer
    if !owner_info.is_signer {
        msg!("Owner must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Ensure the player wallet account belongs to this program
    if player_wallet_info.owner != program_id {
        msg!("Player wallet account does not belong to this program");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize the player wallet data
    let mut player_wallet = PlayerWallet::try_from_slice(&player_wallet_info.data.borrow())
        .map_err(|_| PlayerWalletError::AccountNotInitialized)?;
    
    // Ensure the account is initialized
    if !player_wallet.is_initialized {
        return Err(PlayerWalletError::AccountNotInitialized.into());
    }
    
    // Ensure the owner is authorized to update this wallet
    if player_wallet.wallet_address != *owner_info.key {
        return Err(PlayerWalletError::Unauthorized.into());
    }
    
    // Update the player wallet data
    player_wallet.nb_tokens = nb_tokens;
    player_wallet.amount_total_tokens = amount_total_tokens;
    player_wallet.amount_total_value_stablecoin = amount_total_value_stablecoin;
    player_wallet.date_portfolio = date_portfolio;
    player_wallet.nb_transactions += 1;
    
    // Serialize the updated player wallet data to the account
    player_wallet.serialize(&mut *player_wallet_info.data.borrow_mut())?;
    
    msg!("Player wallet updated successfully");
    Ok(())
}
