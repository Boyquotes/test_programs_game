// Test module for the Solana program
#[cfg(test)]
mod tests {
    use solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    };
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use std::mem::size_of;
    
    use crate::{
        instructions::{
            process_initialize,
            process_create_player_wallet,
            process_update_player_wallet,
        },
        state::PlayerWallet,
        constants::MAX_NAME_LENGTH,
    };

    // Helper function to create a program test environment
    fn program_test() -> ProgramTest {
        ProgramTest::new(
            "solana_player_wallet",
            crate::id(),
            processor!(crate::process_instruction),
        )
    }

    #[tokio::test]
    async fn test_initialize_program() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
        
        // Create program authority
        let program_authority = Keypair::new();
        
        // Create instruction data
        let instruction_data = [0]; // 0 = Initialize instruction
        
        // Create transaction
        let mut transaction = Transaction::new_with_payer(
            &[solana_program::instruction::Instruction::new_with_bincode(
                crate::id(),
                &instruction_data,
                vec![
                    solana_program::instruction::AccountMeta::new(program_authority.pubkey(), true),
                ],
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &program_authority], recent_blockhash);
        
        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();
        
        // No need to check anything, if it doesn't error, it's successful
    }
    
    #[tokio::test]
    async fn test_create_player_wallet() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
        
        // Create player wallet account
        let player_wallet_keypair = Keypair::new();
        let player_name = "TestPlayer123".to_string();
        
        // Calculate account size
        let account_size = PlayerWallet::get_account_size(player_name.len());
        
        // Get minimum rent
        let rent = banks_client.get_rent().await.unwrap();
        let rent_lamports = rent.minimum_balance(account_size);
        
        // Create instruction data
        // 1 = CreatePlayerWallet instruction
        let mut instruction_data = vec![1];
        instruction_data.extend_from_slice(&(player_name.try_to_vec().unwrap()));
        
        // Create transaction
        let mut transaction = Transaction::new_with_payer(
            &[
                // Create account
                solana_program::system_instruction::create_account(
                    &payer.pubkey(),
                    &player_wallet_keypair.pubkey(),
                    rent_lamports,
                    account_size as u64,
                    &crate::id(),
                ),
                // Initialize player wallet
                solana_program::instruction::Instruction::new_with_bincode(
                    crate::id(),
                    &instruction_data,
                    vec![
                        solana_program::instruction::AccountMeta::new(payer.pubkey(), true),
                        solana_program::instruction::AccountMeta::new(player_wallet_keypair.pubkey(), false),
                        solana_program::instruction::AccountMeta::new_readonly(solana_program::system_program::id(), false),
                    ],
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &player_wallet_keypair], recent_blockhash);
        
        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Fetch the account to verify it was created correctly
        let player_wallet_account = banks_client
            .get_account(player_wallet_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        
        // Deserialize the account data
        let player_wallet = PlayerWallet::try_from_slice(&player_wallet_account.data).unwrap();
        
        // Verify the account data
        assert_eq!(player_wallet.is_initialized, true);
        assert_eq!(player_wallet.name, player_name);
        assert_eq!(player_wallet.wallet_address, payer.pubkey());
        assert_eq!(player_wallet.nb_tokens, 0);
        assert_eq!(player_wallet.nb_transactions, 0);
        assert_eq!(player_wallet.amount_total_tokens, 0);
        assert_eq!(player_wallet.amount_total_value_stablecoin, 0);
        assert_eq!(player_wallet.date_portfolio, 0);
    }
    
    #[tokio::test]
    async fn test_update_player_wallet() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
        
        // Create player wallet account
        let player_wallet_keypair = Keypair::new();
        let player_name = "TestPlayer123".to_string();
        
        // Calculate account size
        let account_size = PlayerWallet::get_account_size(player_name.len());
        
        // Get minimum rent
        let rent = banks_client.get_rent().await.unwrap();
        let rent_lamports = rent.minimum_balance(account_size);
        
        // Create player wallet
        let player_wallet = PlayerWallet {
            is_initialized: true,
            name: player_name.clone(),
            wallet_address: payer.pubkey(),
            nb_tokens: 0,
            nb_transactions: 0,
            amount_total_tokens: 0,
            amount_total_value_stablecoin: 0,
            date_portfolio: 0,
        };
        
        // Create account with initial data
        let mut transaction = Transaction::new_with_payer(
            &[
                solana_program::system_instruction::create_account(
                    &payer.pubkey(),
                    &player_wallet_keypair.pubkey(),
                    rent_lamports,
                    account_size as u64,
                    &crate::id(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &player_wallet_keypair], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Write initial data to the account
        let mut player_wallet_account = banks_client
            .get_account(player_wallet_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        player_wallet_account.data = player_wallet.try_to_vec().unwrap();
        banks_client.set_account(&player_wallet_keypair.pubkey(), &player_wallet_account);
        
        // Update values
        let new_nb_tokens = 100;
        let new_amount_total_tokens = 500;
        let new_amount_total_value_stablecoin = 1000;
        let new_date_portfolio = 1620000000;
        
        // Create instruction data for update
        // 2 = UpdatePlayerWallet instruction
        let mut instruction_data = vec![2];
        
        // Create update payload
        #[derive(BorshSerialize)]
        struct UpdatePayload {
            nb_tokens: u32,
            amount_total_tokens: u32,
            amount_total_value_stablecoin: u32,
            date_portfolio: u64,
        }
        
        let update_payload = UpdatePayload {
            nb_tokens: new_nb_tokens,
            amount_total_tokens: new_amount_total_tokens,
            amount_total_value_stablecoin: new_amount_total_value_stablecoin,
            date_portfolio: new_date_portfolio,
        };
        
        instruction_data.extend_from_slice(&update_payload.try_to_vec().unwrap());
        
        // Create transaction for update
        let mut transaction = Transaction::new_with_payer(
            &[
                solana_program::instruction::Instruction::new_with_bincode(
                    crate::id(),
                    &instruction_data,
                    vec![
                        solana_program::instruction::AccountMeta::new(payer.pubkey(), true),
                        solana_program::instruction::AccountMeta::new(player_wallet_keypair.pubkey(), false),
                    ],
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        
        // Process transaction
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Fetch the account to verify it was updated correctly
        let updated_account = banks_client
            .get_account(player_wallet_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        
        // Deserialize the account data
        let updated_wallet = PlayerWallet::try_from_slice(&updated_account.data).unwrap();
        
        // Verify the account data was updated
        assert_eq!(updated_wallet.is_initialized, true);
        assert_eq!(updated_wallet.name, player_name);
        assert_eq!(updated_wallet.wallet_address, payer.pubkey());
        assert_eq!(updated_wallet.nb_tokens, new_nb_tokens);
        assert_eq!(updated_wallet.nb_transactions, 1); // Should be incremented
        assert_eq!(updated_wallet.amount_total_tokens, new_amount_total_tokens);
        assert_eq!(updated_wallet.amount_total_value_stablecoin, new_amount_total_value_stablecoin);
        assert_eq!(updated_wallet.date_portfolio, new_date_portfolio);
    }
    
    #[test]
    fn test_player_wallet_validation() {
        // Test name length validation
        let wallet_address = Pubkey::new_unique();
        
        // Test valid name
        let valid_name = "ValidPlayerName123".to_string();
        let valid_result = PlayerWallet::new(valid_name.clone(), wallet_address);
        assert!(valid_result.is_ok());
        
        // Test name too long
        let long_name = "a".repeat(MAX_NAME_LENGTH + 1);
        let long_result = PlayerWallet::new(long_name, wallet_address);
        assert!(long_result.is_err());
        
        // Test invalid characters
        let invalid_name = "Invalid-Name!".to_string();
        let invalid_result = PlayerWallet::new(invalid_name, wallet_address);
        assert!(invalid_result.is_err());
    }
}
