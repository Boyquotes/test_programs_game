pub mod instructions;
pub mod state;
pub mod error;
pub mod constants;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

use instructions::*;

// Program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Solana Player Wallet program entrypoint");
    
    // Decode and dispatch instructions
    let instruction = PlayerWalletInstruction::unpack(instruction_data)?;
    
    match instruction {
        PlayerWalletInstruction::Initialize => {
            msg!("Instruction: Initialize");
            process_initialize(program_id, accounts)
        },
        PlayerWalletInstruction::CreatePlayerWallet { name } => {
            msg!("Instruction: CreatePlayerWallet");
            process_create_player_wallet(program_id, accounts, name)
        },
        PlayerWalletInstruction::UpdatePlayerWallet { 
            nb_tokens, 
            amount_total_tokens,
            amount_total_value_stablecoin,
            date_portfolio,
        } => {
            msg!("Instruction: UpdatePlayerWallet");
            process_update_player_wallet(
                program_id, 
                accounts, 
                nb_tokens,
                amount_total_tokens,
                amount_total_value_stablecoin,
                date_portfolio,
            )
        },
    }
}
