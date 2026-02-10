use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
};

use crate::error::JanitorError;
use crate::instruction::JanitorInstruction;
use crate::state::{find_vault_pda, BPS_DENOMINATOR, FEE_BPS, VAULT_SEED};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = JanitorInstruction::try_from_slice(instruction_data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;

    match instruction {
        JanitorInstruction::BatchClean { num_accounts } => {
            process_batch_clean(program_id, accounts, num_accounts)
        }
    }
}

fn process_batch_clean(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    num_accounts: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let user = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let treasury = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // 1. Validate user is signer
    if !user.is_signer {
        return Err(JanitorError::MissingSigner.into());
    }

    // 2. Validate vault PDA
    let (expected_vault, vault_bump) = find_vault_pda(program_id);
    if vault.key != &expected_vault {
        return Err(JanitorError::InvalidVaultPda.into());
    }

    // 3. Record vault lamports before closing accounts
    let lamports_before = vault.lamports();

    // 4. CPI close each token account — rent goes to vault
    let vault_seeds: &[&[u8]] = &[VAULT_SEED, &[vault_bump]];

    for _ in 0..num_accounts {
        let token_account = next_account_info(accounts_iter)?;

        // Build CloseAccount CPI: authority=user, destination=vault
        let close_ix = spl_token::instruction::close_account(
            token_program.key,
            token_account.key,
            vault.key,
            user.key,
            &[],
        )?;

        invoke_signed(
            &close_ix,
            &[
                token_account.clone(),
                vault.clone(),
                user.clone(),
                token_program.clone(),
            ],
            &[vault_seeds],
        )?;
    }

    // 5. Calculate rent collected
    let lamports_after = vault.lamports();
    let rent_collected = lamports_after
        .checked_sub(lamports_before)
        .ok_or(JanitorError::Overflow)?;

    msg!("Rent collected: {} lamports", rent_collected);

    // 6. Split: fee to treasury, remainder to user
    let fee = rent_collected
        .checked_mul(FEE_BPS)
        .ok_or(JanitorError::Overflow)?
        .checked_div(BPS_DENOMINATOR)
        .ok_or(JanitorError::Overflow)?;

    let user_payout = rent_collected
        .checked_sub(fee)
        .ok_or(JanitorError::Overflow)?;

    msg!("Fee: {} lamports, User payout: {} lamports", fee, user_payout);

    // 7. Direct lamport transfer (vault is program-owned PDA)
    **vault.try_borrow_mut_lamports()? -= fee + user_payout;
    **treasury.try_borrow_mut_lamports()? += fee;
    **user.try_borrow_mut_lamports()? += user_payout;

    msg!("Batch clean complete: {} accounts closed", num_accounts);

    Ok(())
}
