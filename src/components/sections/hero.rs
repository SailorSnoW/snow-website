use crate::constants::PERSONAL_INFO;
use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
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
    view! {
        <section id="hero" class="min-h-screen flex items-center justify-center px-4 relative pt-20 md:pt-0">
            <div class="max-w-6xl mx-auto text-center relative z-10">
                // Main hero content with clean layout
                <div class="space-y-8">
                    // Welcome message
                    <div class="space-y-4">
                        <div class="text-catppuccin-subtext1 text-lg font-mono tracking-wide">
                            "Hello, I'm"
                        </div>

                        // Name with gradient and angel wings
                        <div class="relative flex items-center justify-center">
                            // Left wing (hidden on mobile)
                            <div class="hidden md:block absolute -left-8 lg:-left-16 text-4xl lg:text-6xl text-catppuccin-pink/60 animate-pulse scale-x-[-1]">
                                "🪽"
                            </div>

                            <h1 class="text-6xl lg:text-8xl font-bold bg-gradient-to-r from-catppuccin-pink via-catppuccin-mauve to-catppuccin-sapphire bg-clip-text text-transparent">
                                {PERSONAL_INFO.name}
                            </h1>

                            // Right wing (hidden on mobile)
                            <div class="hidden md:block absolute -right-8 lg:-right-16 text-4xl lg:text-6xl text-catppuccin-sapphire/60 animate-pulse">
                                "🪽"
                            </div>
                        </div>

                        // Role description
                        <div class="text-xl lg:text-2xl text-catppuccin-text space-y-2">
                            <div class="font-medium">
                                {PERSONAL_INFO.title}
                            </div>
                            <div class="text-catppuccin-subtext1 text-lg">
                                {PERSONAL_INFO.subtitle}
                            </div>
                        </div>
                    </div>

                    // Skills showcase with clean cards
                    <div class="flex flex-wrap justify-center gap-4 max-w-3xl mx-auto">
                        <div class="px-4 py-2 bg-catppuccin-surface0/60 backdrop-blur border border-catppuccin-pink/30 rounded-full text-catppuccin-pink text-sm font-medium hover:border-catppuccin-pink/60 transition-all duration-300 hover:scale-105 flex items-center gap-2">
                            <i class="fas fa-cog"></i>
                            "Rust"
                        </div>
                        <div class="px-4 py-2 bg-catppuccin-surface0/60 backdrop-blur border border-catppuccin-sapphire/30 rounded-full text-catppuccin-sapphire text-sm font-medium hover:border-catppuccin-sapphire/60 transition-all duration-300 hover:scale-105 flex items-center gap-2">
                            <i class="fas fa-code"></i>
                            "C#"
                        </div>
                        <div class="px-4 py-2 bg-catppuccin-surface0/60 backdrop-blur border border-catppuccin-mauve/30 rounded-full text-catppuccin-mauve text-sm font-medium hover:border-catppuccin-mauve/60 transition-all duration-300 hover:scale-105 flex items-center gap-2">
                            <i class="fab fa-js-square"></i>
                            "TypeScript"
                        </div>
                        <div class="px-4 py-2 bg-catppuccin-surface0/60 backdrop-blur border border-catppuccin-green/30 rounded-full text-catppuccin-green text-sm font-medium hover:border-catppuccin-green/60 transition-all duration-300 hover:scale-105 flex items-center gap-2">
                            <i class="fas fa-snowflake"></i>
                            "NixOS"
                        </div>
                        <div class="px-4 py-2 bg-catppuccin-surface0/60 backdrop-blur border border-catppuccin-yellow/30 rounded-full text-catppuccin-yellow text-sm font-medium hover:border-catppuccin-yellow/60 transition-all duration-300 hover:scale-105 flex items-center gap-2">
                            <i class="fas fa-server"></i>
                            "Self-Hosting"
                        </div>
                    </div>

                    // Call to action
                    <div class="space-y-4 md:space-y-6 pt-6 md:pt-8">
                        <p class="text-catppuccin-subtext1 text-base md:text-lg max-w-2xl mx-auto leading-relaxed">
                            {PERSONAL_INFO.description}
                        </p>

                        // Action buttons
                        <div class="flex flex-col sm:flex-row gap-3 md:gap-4 justify-center items-center">
                            <button class="px-6 md:px-8 py-2.5 md:py-3 bg-catppuccin-pink/20 border border-catppuccin-pink/50 rounded-lg text-catppuccin-pink font-medium hover:bg-catppuccin-pink/30 hover:border-catppuccin-pink/80 transition-all duration-300 hover:scale-105 backdrop-blur text-sm md:text-base"
                                    on:click=move |_| smooth_scroll_to("#about")>
                                "Learn More About Me ✨"
                            </button>
                            <button class="px-6 md:px-8 py-2.5 md:py-3 bg-catppuccin-sapphire/20 border border-catppuccin-sapphire/50 rounded-lg text-catppuccin-sapphire font-medium hover:bg-catppuccin-sapphire/30 hover:border-catppuccin-sapphire/80 transition-all duration-300 hover:scale-105 backdrop-blur text-sm md:text-base"
                                    on:click=move |_| smooth_scroll_to("#projects")>
                                "View My Work 💻"
                            </button>
                        </div>
                    </div>

                    // Scroll indicator
                    <div class="pt-6 md:pt-12">
                        <div class="text-catppuccin-overlay1 animate-bounce font-mono text-xs md:text-sm">
                            "// Scroll to explore ↓"
                        </div>
                    </div>
                </div>

                // Floating decorative elements
                <div class="absolute top-20 left-10 text-catppuccin-pink/30 text-2xl animate-pulse hidden lg:block">
                    "🌸"
                </div>
                <div class="absolute top-32 right-16 text-catppuccin-sapphire/30 text-xl animate-bounce hidden lg:block" style="animation-delay: 1s;">
                    "✨"
                </div>
                <div class="absolute bottom-40 left-20 text-catppuccin-mauve/30 text-lg animate-pulse hidden lg:block" style="animation-delay: 2s;">
                    "💫"
                </div>
                <div class="absolute bottom-60 right-12 text-catppuccin-yellow/30 text-xl animate-bounce hidden lg:block" style="animation-delay: 3s;">
                    "🎵"
                </div>
            </div>
        </section>
    }
}
