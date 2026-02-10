use leptos::prelude::*;

#[component]
pub fn TxStatus() -> impl IntoView {
    let tx_sigs = expect_context::<ReadSignal<Vec<(String, String)>>>();

    let has_sigs = move || !tx_sigs.get().is_empty();

    view! {
        {move || has_sigs().then(|| {
            let sigs = tx_sigs.get();
            view! {
                <div class="glass-panel p-6 space-y-3">
                    <h2 class="text-lg font-semibold">"Transactions"</h2>
                    <div class="space-y-2">
                        {sigs.into_iter().map(|(sig, status)| {
                            let sig_short = if sig.len() > 16 {
                                format!("{}...{}", &sig[..8], &sig[sig.len()-8..])
                            } else {
                                sig.clone()
                            };
                            let status_color = match status.as_str() {
                                "confirmed" => "text-green-400",
                                "error" => "text-neon-rose",
                                _ => "text-yellow-400",
                            };
                            let explorer_url = format!(
                                "https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2F127.0.0.1%3A8899",
                                sig
                            );
                            view! {
                                <div class="flex items-center justify-between py-2 px-3 rounded-lg bg-vault-dark/50">
                                    <a
                                        href=explorer_url
                                        target="_blank"
                                        class="font-mono text-sm text-cyber-cyan hover:underline"
                                    >
                                        {sig_short}
                                    </a>
                                    <span class=format!("text-sm font-semibold {}", status_color)>
                                        {status}
                                    </span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            }
        })}
    }
}
