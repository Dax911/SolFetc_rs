#![allow(unexpected_cfgs)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

use solana_program::entrypoint;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    instruction_data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    processor::process(program_id, accounts, instruction_data)
}
