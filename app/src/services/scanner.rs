use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::services::rpc::get_token_accounts_by_owner;
use crate::types::token_account::TokenAccountInfo;

/// Dust threshold: accounts with balance below this are considered closeable.
const DUST_THRESHOLD: u64 = 0;

pub fn scan_token_accounts(
    owner_pubkey: String,
    set_accounts: WriteSignal<Vec<TokenAccountInfo>>,
    set_scanning: WriteSignal<bool>,
) {
    spawn_local(async move {
        match get_token_accounts_by_owner(&owner_pubkey).await {
            Ok(result) => {
                let mut closeable = Vec::new();

                if let Some(accounts) = result["value"].as_array() {
                    for acc in accounts {
                        let address = acc["pubkey"].as_str().unwrap_or_default().to_string();
                        let info = &acc["account"]["data"]["parsed"]["info"];

                        let amount = info["tokenAmount"]["amount"]
                            .as_str()
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(0);

                        let mint = info["mint"].as_str().unwrap_or_default().to_string();

                        let lamports = acc["account"]["lamports"].as_u64().unwrap_or(0);

                        if amount <= DUST_THRESHOLD {
                            closeable.push(TokenAccountInfo {
                                address,
                                mint,
                                amount,
                                lamports,
                            });
                        }
                    }
                }

                log::info!("Found {} closeable accounts", closeable.len());
                set_accounts.set(closeable);
            }
            Err(e) => {
                log::error!("Scan failed: {}", e);
                set_accounts.set(vec![]);
            }
        }
        set_scanning.set(false);
    });
}
