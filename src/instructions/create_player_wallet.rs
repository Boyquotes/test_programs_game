use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke_signed,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    msg,
};
use borsh::BorshSerialize;

use crate::state::PlayerWallet;
use crate::error::PlayerWalletError;

/// Create a new player wallet account
pub fn process_create_player_wallet(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get the required accounts
    let owner_info = next_account_info(account_info_iter)?;
    let player_wallet_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    
    // Ensure the owner is a signer
    if !owner_info.is_signer {
        msg!("Owner must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Validate the player wallet account
    if player_wallet_info.owner != program_id {
        // Account doesn't belong to this program, create it
        let rent = Rent::get()?;
        let account_size = PlayerWallet::get_account_size(name.len());
        let rent_lamports = rent.minimum_balance(account_size);
        
        // Create the account
        invoke_signed(
            &system_instruction::create_account(
                owner_info.key,
                player_wallet_info.key,
                rent_lamports,
                account_size as u64,
                program_id,
            ),
            &[
                owner_info.clone(),
                player_wallet_info.clone(),
                system_program_info.clone(),
            ],
            &[],
        )?;
    }
    
    // Create the player wallet
    let player_wallet = PlayerWallet::new(name, *owner_info.key)?;
    
    // Serialize the player wallet data to the account
    player_wallet.serialize(&mut *player_wallet_info.data.borrow_mut())?;
    
    msg!("Player wallet created successfully");
    Ok(())
}
