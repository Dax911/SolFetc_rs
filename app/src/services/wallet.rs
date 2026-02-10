use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = connect)]
    async fn phantom_connect() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = disconnect)]
    async fn phantom_disconnect();

    #[wasm_bindgen(js_namespace = ["window", "solana"], getter, js_name = isPhantom)]
    fn is_phantom() -> bool;

    #[wasm_bindgen(js_namespace = ["window", "solana"], getter, js_name = publicKey)]
    fn phantom_public_key() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "backpack", "solana"], js_name = connect)]
    async fn backpack_connect() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "backpack"], getter, js_name = isBackpack)]
    fn is_backpack() -> bool;
}

fn get_pubkey_string(pk: JsValue) -> Option<String> {
    let to_string = js_sys::Reflect::get(&pk, &JsValue::from_str("toString")).ok()?;
    let func = to_string.dyn_ref::<js_sys::Function>()?;
    let result = func.call0(&pk).ok()?;
    result.as_string()
}

pub fn connect_wallet() {
    let set_wallet = expect_context::<WriteSignal<String>>();

    spawn_local(async move {
        // Try Phantom first, then Backpack
        let result = phantom_connect().await;

        if let Some(pk_val) = js_sys::Reflect::get(&result, &JsValue::from_str("publicKey")).ok() {
            if let Some(addr) = get_pubkey_string(pk_val) {
                set_wallet.set(addr);
                return;
            }
        }

        // Fallback: check if publicKey is set on window.solana
        let pk = phantom_public_key();
        if !pk.is_undefined() && !pk.is_null() {
            if let Some(addr) = get_pubkey_string(pk) {
                set_wallet.set(addr);
                return;
            }
        }

        log::error!("Failed to connect wallet");
    });
}

pub fn disconnect_wallet() {
    let set_wallet = expect_context::<WriteSignal<String>>();
    let set_accounts = expect_context::<WriteSignal<Vec<crate::types::token_account::TokenAccountInfo>>>();
    let set_selected = expect_context::<WriteSignal<Vec<usize>>>();
    let set_tx_sigs = expect_context::<WriteSignal<Vec<(String, String)>>>();

    spawn_local(async move {
        phantom_disconnect().await;
        set_wallet.set(String::new());
        set_accounts.set(vec![]);
        set_selected.set(vec![]);
        set_tx_sigs.set(vec![]);
    });
}
