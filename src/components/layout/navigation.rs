use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let (nav_open, set_nav_open) = signal(false);
    let (nav_visible, set_nav_visible) = signal(true);
    let (last_scroll_y, set_last_scroll_y) = signal(0.0);

    // Smooth scroll function
    let smooth_scroll_to = move |target_id: &str| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(Some(element)) = document.query_selector(target_id) {
                    element.scroll_into_view();
                }
            }
        }
    };

    // Handle scroll events for navbar auto-hide
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                let current_scroll = window.scroll_y().unwrap_or(0.0);
                let last_scroll = last_scroll_y.get_untracked();

                // Show navbar if scrolling up or at top, hide if scrolling down
                if current_scroll < last_scroll || current_scroll < 100.0 {
                    set_nav_visible.set(true);
                } else if current_scroll > last_scroll && current_scroll > 100.0 {
                    set_nav_visible.set(false);
                }

                set_last_scroll_y.set(current_scroll);
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    });

    view! {
        <>
            // Modern floating navigation bar
            <nav class={move || format!(
                "fixed top-3 left-6 right-6 md:left-12 md:right-12 lg:left-1/2 lg:right-auto lg:-translate-x-1/2 lg:w-auto z-40 transition-all duration-500 ease-out {}",
                if nav_visible.get() {
                    "opacity-100 transform translate-y-0"
                } else {
                    "opacity-0 transform -translate-y-3 pointer-events-none"
                }
            )}>
                <div class="bg-catppuccin-surface0/95 backdrop-blur-xl border border-catppuccin-overlay0/30 rounded-lg shadow-xl hover:shadow-2xl transition-all duration-300">
                    // Subtle glow effect
                    <div class="absolute -inset-px bg-gradient-to-r from-catppuccin-pink/25 via-catppuccin-mauve/25 to-catppuccin-sapphire/25 rounded-lg blur-sm opacity-70"></div>

                    <div class="relative px-8 py-2">
                        <div class="flex items-center justify-between lg:justify-center lg:space-x-12">
                            // Terminal prompt style logo - compact
                            <div class="flex items-center space-x-1 font-mono text-xs lg:mr-6">
                                <span class="text-catppuccin-pink font-semibold">"SnoW"</span>
                                <span class="text-catppuccin-overlay1">"@"</span>
                                <span class="text-catppuccin-sapphire">"snOwOS"</span>
                                <span class="text-catppuccin-overlay1">":"</span>
                                <span class="text-catppuccin-mauve">"~"</span>
                                <span class="text-catppuccin-green">"$"</span>
                                <span class="text-catppuccin-text animate-pulse">"█"</span>
                            </div>

                            // Desktop navigation with modern pill styling
                            <div class="hidden lg:flex items-center bg-catppuccin-surface1/40 rounded-full px-2 py-1">
                                <button class="px-5 py-1.5 text-catppuccin-subtext1 hover:text-catppuccin-pink hover:bg-catppuccin-surface0/70 rounded-full text-sm font-medium transition-all duration-200 hover:scale-105 hover:shadow-sm"
                                        on:click=move |_| smooth_scroll_to("#hero")>
                                    "📁 home"
                                </button>
                                <button class="px-5 py-1.5 text-catppuccin-subtext1 hover:text-catppuccin-sapphire hover:bg-catppuccin-surface0/70 rounded-full text-sm font-medium transition-all duration-200 hover:scale-105 hover:shadow-sm"
                                        on:click=move |_| smooth_scroll_to("#about")>
                                    "💫 about"
                                </button>
                                <button class="px-5 py-1.5 text-catppuccin-subtext1 hover:text-catppuccin-mauve hover:bg-catppuccin-surface0/70 rounded-full text-sm font-medium transition-all duration-200 hover:scale-105 hover:shadow-sm"
                                        on:click=move |_| smooth_scroll_to("#skills")>
                                    "🦀 skills"
                                </button>
                                <button class="px-5 py-1.5 text-catppuccin-subtext1 hover:text-catppuccin-green hover:bg-catppuccin-surface0/70 rounded-full text-sm font-medium transition-all duration-200 hover:scale-105 hover:shadow-sm"
                                        on:click=move |_| smooth_scroll_to("#projects")>
                                    "💻 projects"
                                </button>
                                <button class="px-5 py-1.5 text-catppuccin-subtext1 hover:text-catppuccin-yellow hover:bg-catppuccin-surface0/70 rounded-full text-sm font-medium transition-all duration-200 hover:scale-105 hover:shadow-sm"
                                        on:click=move |_| smooth_scroll_to("#contact")>
                                    "📧 contact"
                                </button>
                            </div>

                            // Mobile menu button
                            <div class="lg:hidden">
                                <button
                                    class="p-1.5 text-catppuccin-subtext1 hover:text-catppuccin-pink hover:bg-catppuccin-surface0/70 rounded-full transition-all duration-200 hover:scale-110"
                                    on:click=move |_| set_nav_open.set(true)
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            // Mobile navigation overlay
            <nav class={move || format!(
                "fixed top-0 right-0 h-full w-80 bg-catppuccin-base/95 backdrop-blur-xl border-l border-catppuccin-pink/30 transform transition-transform duration-300 z-50 {}",
                if nav_open.get() { "translate-x-0" } else { "translate-x-full" }
            )}>
                <div class="p-6">
                    <button
                        class="absolute top-4 right-4 text-catppuccin-pink hover:text-catppuccin-mauve transition-colors text-2xl"
                        on:click=move |_| set_nav_open.set(false)
                    >
                        "✕"
                    </button>

                    <div class="mt-12 space-y-4 font-mono">
                        <h3 class="text-catppuccin-pink text-xl font-bold mb-6 terminal-glow-text">
                            "// File Explorer"
                        </h3>

                        <button class="block w-full text-left text-catppuccin-sapphire hover:text-catppuccin-pink transition-colors py-2 px-4 rounded border border-transparent hover:border-catppuccin-sapphire/30"
                                on:click=move |_| {
                                    smooth_scroll_to("#hero");
                                    set_nav_open.set(false);
                                }>
                            "📁 home.md"
                        </button>
                        <button class="block w-full text-left text-catppuccin-sapphire hover:text-catppuccin-pink transition-colors py-2 px-4 rounded border border-transparent hover:border-catppuccin-sapphire/30"
                                on:click=move |_| {
                                    smooth_scroll_to("#about");
                                    set_nav_open.set(false);
                                }>
                            "📄 about.md"
                        </button>
                        <button class="block w-full text-left text-catppuccin-sapphire hover:text-catppuccin-pink transition-colors py-2 px-4 rounded border border-transparent hover:border-catppuccin-sapphire/30"
                                on:click=move |_| {
                                    smooth_scroll_to("#skills");
                                    set_nav_open.set(false);
                                }>
                            "🦀 skills.rs"
                        </button>
                        <button class="block w-full text-left text-catppuccin-sapphire hover:text-catppuccin-pink transition-colors py-2 px-4 rounded border border-transparent hover:border-catppuccin-sapphire/30"
                                on:click=move |_| {
                                    smooth_scroll_to("#projects");
                                    set_nav_open.set(false);
                                }>
                            "💻 projects.tsx"
                        </button>
                        <button class="block w-full text-left text-catppuccin-sapphire hover:text-catppuccin-pink transition-colors py-2 px-4 rounded border border-transparent hover:border-catppuccin-sapphire/30"
                                on:click=move |_| {
                                    smooth_scroll_to("#contact");
                                    set_nav_open.set(false);
                                }>
                            "📧 contact.json"
                        </button>
                    </div>
                </div>
            </nav>
        </>
    }
}