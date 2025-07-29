use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative z-10 mt-20 py-8 pb-20 md:pb-8 border-t border-catppuccin-surface0/30">
            <div class="max-w-4xl mx-auto px-4 text-center">
                <div class="text-catppuccin-subtext1 text-sm font-mono">
                    "Made with "
                    <span class="text-pink-400 terminal-glow-text" style="text-shadow: 0 0 10px #f5c2e7, 0 0 20px #f5c2e7, 0 0 30px #f5c2e7;">
                        "♥"
                    </span>
                    " by "
                    <span class="text-catppuccin-sapphire font-medium">
                        "SnoW"
                    </span>
                    " • Built with "
                    <span class="text-catppuccin-peach">
                        "Rust"
                    </span>
                    " + "
                    <span class="text-catppuccin-mauve">
                        "WebAssembly"
                    </span>
                </div>
            </div>
        </footer>
    }
}

