use leptos::prelude::*;

use crate::services::transaction::execute_batch_clean;
use crate::types::token_account::TokenAccountInfo;

#[component]
pub fn BatchPanel() -> impl IntoView {
    let wallet = expect_context::<ReadSignal<String>>();
    let accounts = expect_context::<ReadSignal<Vec<TokenAccountInfo>>>();
    let selected = expect_context::<ReadSignal<Vec<usize>>>();
    let set_selected = expect_context::<WriteSignal<Vec<usize>>>();
    let set_accounts = expect_context::<WriteSignal<Vec<TokenAccountInfo>>>();
    let processing = expect_context::<ReadSignal<bool>>();
    let set_processing = expect_context::<WriteSignal<bool>>();
    let set_tx_sigs = expect_context::<WriteSignal<Vec<(String, String)>>>();

    let count = move || selected.get().len();
    let has_selection = move || count() > 0;

    let total_rent = move || {
        let accs = accounts.get();
        let sel = selected.get();
        sel.iter()
            .filter_map(|&i| accs.get(i))
            .map(|a| a.lamports as f64)
            .sum::<f64>()
            / 1_000_000_000.0
    };

    let fee_estimate = move || total_rent() * 0.05;
    let user_gets = move || total_rent() - fee_estimate();

    let select_all = move |_| {
        let len = accounts.get().len();
        set_selected.set((0..len).collect());
    };

    let select_none = move |_| {
        set_selected.set(vec![]);
    };

    let on_incinerate = move |_| {
        execute_batch_clean(
            wallet,
            accounts,
            selected,
            set_processing,
            set_tx_sigs,
            set_selected,
            set_accounts,
        );
    };

    view! {
        {move || has_selection().then(|| view! {
            <div class="glass-panel p-6 space-y-4">
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold">"Batch Clean"</h2>
                    <div class="flex gap-2">
                        <button class="text-xs text-cyber-cyan hover:underline cursor-pointer" on:click=select_all>
                            "Select All"
                        </button>
                        <span class="text-xs text-text-muted">"|"</span>
                        <button class="text-xs text-text-muted hover:text-white hover:underline cursor-pointer" on:click=select_none>
                            "Clear"
                        </button>
                    </div>
                </div>

                <div class="grid grid-cols-3 gap-4">
                    <div class="text-center">
                        <div class="text-2xl font-mono font-bold text-cyber-cyan">
                            {move || format!("{:.6}", total_rent())}
                        </div>
                        <div class="text-xs text-text-muted mt-1">"Total Rent (SOL)"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-2xl font-mono font-bold text-neon-rose">
                            {move || format!("{:.6}", fee_estimate())}
                        </div>
                        <div class="text-xs text-text-muted mt-1">"Fee (5%)"</div>
                    </div>
                    <div class="text-center">
                        <div class="text-2xl font-mono font-bold text-green-400">
                            {move || format!("{:.6}", user_gets())}
                        </div>
                        <div class="text-xs text-text-muted mt-1">"You Receive"</div>
                    </div>
                </div>

                {move || {
                    let n = count();
                    let batches = (n + 24) / 25;
                    (batches > 1).then(|| view! {
                        <p class="text-xs text-text-muted text-center">
                            {format!("{} accounts across {} transactions (max 25/tx)", n, batches)}
                        </p>
                    })
                }}

                <button
                    class=move || {
                        let base = "btn-danger w-full text-lg py-4 font-bold tracking-widest";
                        if processing.get() {
                            format!("{} opacity-70 pointer-events-none", base)
                        } else {
                            base.to_string()
                        }
                    }
                    on:click=on_incinerate
                    disabled=move || processing.get()
                >
                    {move || {
                        if processing.get() {
                            "PROCESSING...".to_string()
                        } else {
                            format!("INCINERATE {} ACCOUNTS", count())
                        }
                    }}
                </button>

                {move || processing.get().then(|| view! {
                    <div class="w-full bg-panel-dark rounded-full h-2 overflow-hidden">
                        <div
                            class="h-full bg-gradient-to-r from-neon-rose to-cyber-cyan rounded-full transition-all duration-500"
                            style="width: 50%; animation: pulse 1s infinite"
                        />
                    </div>
                })}
            </div>
        })}
    }
}
