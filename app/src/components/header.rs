use leptos::prelude::*;

use crate::services::wallet::{connect_wallet, disconnect_wallet};
use crate::types::token_account::TokenAccountInfo;

#[component]
pub fn Header() -> impl IntoView {
    let wallet = expect_context::<ReadSignal<String>>();
    let set_wallet = expect_context::<WriteSignal<String>>();
    let set_accounts = expect_context::<WriteSignal<Vec<TokenAccountInfo>>>();
    let set_selected = expect_context::<WriteSignal<Vec<usize>>>();
    let set_tx_sigs = expect_context::<WriteSignal<Vec<(String, String)>>>();

    let connected = move || !wallet.get().is_empty();

    let display_addr = move || {
        let w = wallet.get();
        if w.len() > 8 {
            format!("{}...{}", &w[..4], &w[w.len() - 4..])
        } else {
            w
        }
    };

    view! {
        <header class="glass-panel mx-4 mt-4 px-6 py-4 flex items-center justify-between">
            <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-lg bg-cyber-cyan/20 border border-cyber-cyan/40 flex items-center justify-center">
                    <span class="text-cyber-cyan font-bold text-sm font-mono">"Z"</span>
                </div>
                <h1 class="text-lg font-semibold tracking-tight">
                    "Zera "<span class="text-cyber-cyan">"Janitor"</span>
                </h1>
            </div>
            <div>
                {move || {
                    if connected() {
                        view! {
                            <div class="flex items-center gap-3">
                                <span class="text-sm font-mono text-text-muted">{display_addr}</span>
                                <button
                                    class="btn-danger text-sm px-4 py-2"
                                    on:click=move |_| disconnect_wallet(set_wallet, set_accounts, set_selected, set_tx_sigs)
                                >
                                    "Disconnect"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                class="btn-primary text-sm"
                                on:click=move |_| connect_wallet(set_wallet)
                            >
                                "Connect Wallet"
                            </button>
                        }.into_any()
                    }
                }}
            </div>
        </header>
    }
}
