use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::constants::{PROGRAM_ID, TREASURY_PUBKEY, VAULT_SEED};
use crate::services::rpc::get_latest_blockhash;
use crate::types::instruction::build_batch_clean_data;
use crate::types::token_account::TokenAccountInfo;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = zeraSignAndSend, catch)]
    async fn zera_sign_and_send(
        instruction_bytes: &[u8],
        account_metas: JsValue,
        blockhash: &str,
        rpc_url: &str,
    ) -> Result<JsValue, JsValue>;
}

const MAX_ACCOUNTS_PER_TX: usize = 25;

pub fn execute_batch_clean(
    wallet: ReadSignal<String>,
    accounts: ReadSignal<Vec<TokenAccountInfo>>,
    selected: ReadSignal<Vec<usize>>,
    set_processing: WriteSignal<bool>,
    set_tx_sigs: WriteSignal<Vec<(String, String)>>,
    set_selected: WriteSignal<Vec<usize>>,
    set_accounts: WriteSignal<Vec<TokenAccountInfo>>,
) {
    let user_pubkey = wallet.get();
    let all_accounts = accounts.get();
    let sel = selected.get();

    if user_pubkey.is_empty() || sel.is_empty() {
        return;
    }

    let selected_accounts: Vec<TokenAccountInfo> = sel
        .iter()
        .filter_map(|&i| all_accounts.get(i).cloned())
        .collect();

    set_processing.set(true);

    spawn_local(async move {
        let program_id = bs58::encode(&PROGRAM_ID).into_string();
        let treasury = bs58::encode(&TREASURY_PUBKEY).into_string();
        let vault_pubkey = derive_vault_pda_bs58();
        let rpc_url = crate::constants::RPC_URL;

        let chunks: Vec<Vec<TokenAccountInfo>> = selected_accounts
            .chunks(MAX_ACCOUNTS_PER_TX)
            .map(|c| c.to_vec())
            .collect();

        let mut sigs = Vec::new();

        for chunk in &chunks {
            let num = chunk.len() as u8;
            let ix_data = build_batch_clean_data(num);

            let mut metas = Vec::new();
            metas.push(account_meta(&user_pubkey, true, true, &program_id));
            metas.push(account_meta(&vault_pubkey, false, true, &program_id));
            metas.push(account_meta(&treasury, false, true, &program_id));
            let spl_token_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
            metas.push(account_meta(spl_token_id, false, false, &program_id));
            for acc in chunk {
                metas.push(account_meta(&acc.address, false, true, &program_id));
            }

            let metas_js = serde_wasm_bindgen::to_value(&metas).unwrap_or(JsValue::NULL);

            match get_latest_blockhash().await {
                Ok(blockhash) => {
                    match zera_sign_and_send(&ix_data, metas_js, &blockhash, rpc_url).await {
                        Ok(sig_val) => {
                            let sig = sig_val.as_string().unwrap_or_default();
                            log::info!("Transaction sent: {}", sig);
                            sigs.push((sig, "confirmed".to_string()));
                        }
                        Err(e) => {
                            let err_msg = format!("{:?}", e);
                            log::error!("Transaction failed: {}", err_msg);
                            sigs.push((err_msg, "error".to_string()));
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to get blockhash: {}", e);
                    sigs.push((e, "error".to_string()));
                }
            }
        }

        set_tx_sigs.update(|existing| existing.extend(sigs));
        set_processing.set(false);
        set_selected.set(vec![]);
        let remaining: Vec<TokenAccountInfo> = accounts
            .get()
            .into_iter()
            .filter(|a| !selected_accounts.iter().any(|s| s.address == a.address))
            .collect();
        set_accounts.set(remaining);
    });
}

#[derive(serde::Serialize)]
struct AccountMeta {
    pubkey: String,
    #[serde(rename = "isSigner")]
    is_signer: bool,
    #[serde(rename = "isWritable")]
    is_writable: bool,
    #[serde(rename = "programId")]
    program_id: String,
}

fn account_meta(pubkey: &str, is_signer: bool, is_writable: bool, program_id: &str) -> AccountMeta {
    AccountMeta {
        pubkey: pubkey.to_string(),
        is_signer,
        is_writable,
        program_id: program_id.to_string(),
    }
}

fn derive_vault_pda_bs58() -> String {
    let program_id = bs58::encode(&PROGRAM_ID).into_string();
    let seed_str = std::str::from_utf8(VAULT_SEED).unwrap_or("zera-vault");
    log::info!(
        "Vault PDA derivation pending — program: {}, seed: {}",
        program_id,
        seed_str
    );
    String::from("11111111111111111111111111111111")
}
