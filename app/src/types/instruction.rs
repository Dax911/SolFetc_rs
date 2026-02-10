use borsh::BorshSerialize;

#[derive(BorshSerialize)]
pub enum JanitorInstruction {
    BatchClean { num_accounts: u8 },
}

/// Serialize BatchClean instruction to bytes for the on-chain program.
pub fn build_batch_clean_data(num_accounts: u8) -> Vec<u8> {
    let ix = JanitorInstruction::BatchClean { num_accounts };
    borsh::to_vec(&ix).expect("Failed to serialize instruction")
}
