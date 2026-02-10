use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum JanitorError {
    #[error("Invalid vault PDA derivation")]
    InvalidVaultPda,

    #[error("Missing required signer")]
    MissingSigner,

    #[error("Token account has non-zero balance")]
    NonZeroBalance,

    #[error("Arithmetic overflow")]
    Overflow,
}

impl From<JanitorError> for ProgramError {
    fn from(e: JanitorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
