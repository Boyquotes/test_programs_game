use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

/// Initialize the program
/// This is called once when the program is first deployed
pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get the program authority account
    let authority_info = next_account_info(account_info_iter)?;
    
    // Ensure the authority is a signer
    if !authority_info.is_signer {
        msg!("Authority must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    msg!("Program initialized successfully");
    Ok(())
}
