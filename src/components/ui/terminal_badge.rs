use leptos::prelude::*;

#[component]
pub fn TerminalBadge(
    #[prop(into)] on_click: Callback<()>
) -> impl IntoView {
    view! {
        <div class="fixed bottom-4 md:bottom-6 right-4 md:right-6 z-40">
            <button
                class="group w-10 h-10 md:w-12 md:h-12 bg-catppuccin-surface0/50 backdrop-blur-sm border-2 border-catppuccin-surface1/40 rounded-full shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 hover:bg-catppuccin-surface0/80 hover:border-catppuccin-green/60 flex items-center justify-center"
                on:click=move |_| on_click.run(())
            >
                <i class="fas fa-terminal text-catppuccin-text/90 group-hover:text-catppuccin-green transition-colors duration-300 text-sm md:text-base"></i>
                
                // Tooltip
                <div class="absolute bottom-full right-0 mb-3 hidden group-hover:block">
                    <div class="bg-catppuccin-surface1/90 backdrop-blur-sm text-catppuccin-text text-xs rounded-lg px-3 py-2 shadow-lg border border-catppuccin-surface2/50 whitespace-nowrap font-mono">
                        "Terminal"
                        <div class="absolute top-full right-4 border-4 border-transparent border-t-catppuccin-surface1/90"></div>
                    </div>
                </div>
            </button>
        </div>
    }
}