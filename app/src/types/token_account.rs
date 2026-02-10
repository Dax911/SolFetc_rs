use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenAccountInfo {
    /// Token account address (base58)
    pub address: String,
    /// Mint address (base58)
    pub mint: String,
    /// Token balance (raw u64 as string from RPC)
    pub amount: u64,
    /// Lamports held by the account (rent)
    pub lamports: u64,
}

impl TokenAccountInfo {
    /// Estimated SOL reclaimable from closing this account.
    pub fn rent_sol(&self) -> f64 {
        self.lamports as f64 / 1_000_000_000.0
    }

    /// Truncated mint for display.
    pub fn mint_short(&self) -> String {
        if self.mint.len() > 8 {
            format!("{}...{}", &self.mint[..4], &self.mint[self.mint.len() - 4..])
        } else {
            self.mint.clone()
        }
    }
}
