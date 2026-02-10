use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum JanitorInstruction {
    /// Close `num_accounts` empty token accounts, collecting rent into the vault,
    /// then splitting: 5% fee to treasury, 95% back to user.
    ///
    /// Accounts expected:
    /// 0. `[signer]`   User wallet
    /// 1. `[writable]` Vault PDA (program-owned)
    /// 2. `[]`          Treasury wallet
    /// 3. `[]`          SPL Token program
    /// 4..4+N `[writable]` Token accounts to close
    BatchClean { num_accounts: u8 },
}
