use solana_program::pubkey::Pubkey;

pub const VAULT_SEED: &[u8] = zera_shared::VAULT_SEED;
pub const FEE_BPS: u64 = zera_shared::FEE_BPS;
pub const BPS_DENOMINATOR: u64 = zera_shared::BPS_DENOMINATOR;

/// Derive the vault PDA and its bump seed.
pub fn find_vault_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[VAULT_SEED], program_id)
}
