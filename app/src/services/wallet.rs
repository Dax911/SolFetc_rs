use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use leptos::prelude::*;
use crate::types::token_account::TokenAccountInfo;

/// Get the wallet provider object (`window.phantom.solana`, `window.solana`, or `window.backpack`).
fn get_provider() -> Option<js_sys::Object> {
    let window = web_sys::window()?;

    // Try window.phantom.solana first (newer Phantom injection)
    if let Ok(phantom) = js_sys::Reflect::get(&window, &JsValue::from_str("phantom")) {
        if !phantom.is_undefined() && !phantom.is_null() {
            if let Ok(sol) = js_sys::Reflect::get(&phantom, &JsValue::from_str("solana")) {
                if !sol.is_undefined() && !sol.is_null() {
                    return sol.dyn_into::<js_sys::Object>().ok();
                }
            }
        }
    }

    // Fallback: window.solana
    if let Ok(sol) = js_sys::Reflect::get(&window, &JsValue::from_str("solana")) {
        if !sol.is_undefined() && !sol.is_null() {
            return sol.dyn_into::<js_sys::Object>().ok();
        }
    }

    // Try window.backpack
    if let Ok(bp) = js_sys::Reflect::get(&window, &JsValue::from_str("backpack")) {
        if !bp.is_undefined() && !bp.is_null() {
            return bp.dyn_into::<js_sys::Object>().ok();
        }
    }

    None
}

fn pubkey_to_string(pk: &JsValue) -> Option<String> {
    if pk.is_undefined() || pk.is_null() {
        return None;
    }
    let to_string = js_sys::Reflect::get(pk, &JsValue::from_str("toString")).ok()?;
    let func = to_string.dyn_ref::<js_sys::Function>()?;
    let result = func.call0(pk).ok()?;
    result.as_string()
}

pub fn connect_wallet(set_wallet: WriteSignal<String>) {
    spawn_local(async move {
        let provider = match get_provider() {
            Some(p) => p,
            None => {
                log::error!("No Solana wallet found. Install Phantom or Backpack.");
                return;
            }
        };

        // Call provider.connect()
        let connect_fn = match js_sys::Reflect::get(&provider, &JsValue::from_str("connect")) {
            Ok(f) if f.is_function() => f.unchecked_into::<js_sys::Function>(),
            _ => {
                log::error!("Wallet provider has no connect() method");
                return;
            }
        };

        let promise = match connect_fn.call0(&provider) {
            Ok(p) => p,
            Err(e) => {
                log::error!("connect() call failed: {:?}", e);
                return;
            }
        };

        // Await the promise
        match JsFuture::from(js_sys::Promise::from(promise)).await {
            Ok(result) => {
                // Try getting publicKey from the result object
                if let Ok(pk) = js_sys::Reflect::get(&result, &JsValue::from_str("publicKey")) {
                    if let Some(addr) = pubkey_to_string(&pk) {
                        log::info!("Connected: {}", addr);
                        set_wallet.set(addr);
                        return;
                    }
                }

                // Fallback: publicKey might be on the provider itself after connect
                if let Ok(pk) = js_sys::Reflect::get(&provider, &JsValue::from_str("publicKey")) {
                    if let Some(addr) = pubkey_to_string(&pk) {
                        log::info!("Connected (fallback): {}", addr);
                        set_wallet.set(addr);
                        return;
                    }
                }

                log::error!("Connected but couldn't read publicKey");
            }
            Err(e) => {
                log::error!("Wallet connect rejected: {:?}", e);
            }
        }
    });
}

pub fn disconnect_wallet(
    set_wallet: WriteSignal<String>,
    set_accounts: WriteSignal<Vec<TokenAccountInfo>>,
    set_selected: WriteSignal<Vec<usize>>,
    set_tx_sigs: WriteSignal<Vec<(String, String)>>,
) {
    spawn_local(async move {
        if let Some(provider) = get_provider() {
            if let Ok(f) = js_sys::Reflect::get(&provider, &JsValue::from_str("disconnect")) {
                if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                    if let Ok(promise) = func.call0(&provider) {
                        let _ = JsFuture::from(js_sys::Promise::from(promise)).await;
                    }
                }
            }
        }

        set_wallet.set(String::new());
        set_accounts.set(vec![]);
        set_selected.set(vec![]);
        set_tx_sigs.set(vec![]);
    });
}
