use leptos::prelude::*;

use crate::components::header::Header;
use crate::components::scanner::Scanner;
use crate::components::batch_panel::BatchPanel;
use crate::components::tx_status::TxStatus;
use crate::types::token_account::TokenAccountInfo;

#[component]
pub fn App() -> impl IntoView {
    // Wallet pubkey (base58), empty = disconnected
    let (wallet, set_wallet) = signal(String::new());
    // Scanned token accounts
    let (accounts, set_accounts) = signal(Vec::<TokenAccountInfo>::new());
    // Selected account indices
    let (selected, set_selected) = signal(Vec::<usize>::new());
    // Transaction signatures
    let (tx_sigs, set_tx_sigs) = signal(Vec::<(String, String)>::new()); // (sig, status)
    // Scanning state
    let (scanning, set_scanning) = signal(false);
    // Batch processing state
    let (processing, set_processing) = signal(false);

    provide_context(wallet);
    provide_context(set_wallet);
    provide_context(accounts);
    provide_context(set_accounts);
    provide_context(selected);
    provide_context(set_selected);
    provide_context(tx_sigs);
    provide_context(set_tx_sigs);
    provide_context(scanning);
    provide_context(set_scanning);
    provide_context(processing);
    provide_context(set_processing);

    view! {
        <div class="min-h-screen flex flex-col">
            <Header />
            <main class="flex-1 max-w-6xl mx-auto w-full px-4 py-8 space-y-6">
                <Scanner />
                <BatchPanel />
                <TxStatus />
            </main>
            <footer class="text-center text-text-muted text-sm py-4 font-mono">
                "Zera Janitor — reclaim your rent"
            </footer>
        </div>
    }
}
