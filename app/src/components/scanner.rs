use leptos::prelude::*;

use crate::components::account_card::AccountCard;
use crate::services::scanner::scan_token_accounts;
use crate::types::token_account::TokenAccountInfo;

#[component]
pub fn Scanner() -> impl IntoView {
    let wallet = expect_context::<ReadSignal<String>>();
    let accounts = expect_context::<ReadSignal<Vec<TokenAccountInfo>>>();
    let scanning = expect_context::<ReadSignal<bool>>();
    let set_scanning = expect_context::<WriteSignal<bool>>();
    let selected = expect_context::<ReadSignal<Vec<usize>>>();
    let set_selected = expect_context::<WriteSignal<Vec<usize>>>();

    let connected = move || !wallet.get().is_empty();
    let has_accounts = move || !accounts.get().is_empty();

    let on_scan = move |_| {
        let pubkey = wallet.get();
        if pubkey.is_empty() {
            return;
        }
        set_scanning.set(true);
        set_selected.set(vec![]);
        scan_token_accounts(pubkey);
    };

    let toggle = move |idx: usize| {
        set_selected.update(|sel| {
            if let Some(pos) = sel.iter().position(|&i| i == idx) {
                sel.remove(pos);
            } else {
                sel.push(idx);
            }
        });
    };

    view! {
        <div class="space-y-4">
            {move || connected().then(|| view! {
                <div class="flex items-center gap-4">
                    <button
                        class=move || {
                            let base = "btn-primary flex items-center gap-2";
                            if scanning.get() {
                                format!("{} animate-radar opacity-70 pointer-events-none", base)
                            } else {
                                base.to_string()
                            }
                        }
                        on:click=on_scan
                        disabled=move || scanning.get()
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="10" />
                            <circle cx="12" cy="12" r="3" />
                        </svg>
                        {move || if scanning.get() { "Scanning..." } else { "Scan Wallet" }}
                    </button>
                    {move || has_accounts().then(|| view! {
                        <span class="text-sm text-text-muted">
                            {move || format!("{} closeable accounts found", accounts.get().len())}
                        </span>
                    })}
                </div>
            })}

            {move || has_accounts().then(|| {
                let accs = accounts.get();
                let sel = selected.get();
                view! {
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                        {accs.into_iter().enumerate().map(|(idx, acc)| {
                            let is_selected = sel.contains(&idx);
                            let toggle = toggle.clone();
                            view! {
                                <div
                                    class="animate-cascade"
                                    style=format!("animation-delay: {}ms", idx * 60)
                                >
                                    <AccountCard
                                        account=acc
                                        selected=is_selected
                                        on_toggle=move || toggle(idx)
                                    />
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            })}
        </div>
    }
}
