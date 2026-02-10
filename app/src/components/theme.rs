use leptos::prelude::*;

#[component]
pub fn GlassPanel(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("glass-panel {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn Toast(
    message: String,
    #[prop(default = "info")] variant: &'static str,
    on_dismiss: impl Fn() + 'static,
) -> impl IntoView {
    let (bg, border, text) = match variant {
        "error" => ("bg-neon-rose/10", "border-neon-rose/30", "text-neon-rose"),
        "success" => ("bg-green-500/10", "border-green-500/30", "text-green-400"),
        _ => ("bg-cyber-cyan/10", "border-cyber-cyan/30", "text-cyber-cyan"),
    };

    view! {
        <div class=format!(
            "fixed bottom-4 right-4 {} border {} rounded-xl px-5 py-3 {} backdrop-blur-xl shadow-lg flex items-center gap-3 animate-cascade z-50",
            bg, border, text
        )>
            <span class="text-sm font-medium">{message}</span>
            <button
                class="text-white/50 hover:text-white cursor-pointer"
                on:click=move |_| on_dismiss()
            >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>
        </div>
    }
}
