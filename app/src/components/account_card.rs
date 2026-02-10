use leptos::prelude::*;

use crate::types::token_account::TokenAccountInfo;

#[component]
pub fn AccountCard(
    account: TokenAccountInfo,
    selected: bool,
    on_toggle: impl Fn() + 'static,
) -> impl IntoView {
    let border_class = if selected {
        "glass-panel border-cyber-cyan/50 shadow-glow"
    } else {
        "glass-panel hover:border-white/10"
    };

    let rent = account.rent_sol();
    let mint = account.mint_short();
    let addr_short = if account.address.len() > 8 {
        format!(
            "{}...{}",
            &account.address[..4],
            &account.address[account.address.len() - 4..]
        )
    } else {
        account.address.clone()
    };

    view! {
        <div
            class=format!("{} p-4 cursor-pointer transition-all duration-200", border_class)
            on:click=move |_| on_toggle()
        >
            <div class="flex items-start justify-between mb-3">
                <div class="flex items-center gap-2">
                    <div class=move || {
                        if selected {
                            "w-4 h-4 rounded border border-cyber-cyan bg-cyber-cyan/30 flex items-center justify-center"
                        } else {
                            "w-4 h-4 rounded border border-white/20"
                        }
                    }>
                        {selected.then(|| view! {
                            <svg class="w-3 h-3 text-cyber-cyan" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                <path d="M5 13l4 4L19 7" />
                            </svg>
                        })}
                    </div>
                    <span class="font-mono text-xs text-text-muted">{addr_short}</span>
                </div>
            </div>
            <div class="space-y-1.5">
                <div class="flex justify-between text-sm">
                    <span class="text-text-muted">"Mint"</span>
                    <span class="font-mono text-xs">{mint}</span>
                </div>
                <div class="flex justify-between text-sm">
                    <span class="text-text-muted">"Balance"</span>
                    <span class="font-mono">{account.amount.to_string()}</span>
                </div>
                <div class="flex justify-between text-sm">
                    <span class="text-text-muted">"Rent"</span>
                    <span class="font-mono text-cyber-cyan">{format!("{:.6} SOL", rent)}</span>
                </div>
            </div>
        </div>
    }
}
