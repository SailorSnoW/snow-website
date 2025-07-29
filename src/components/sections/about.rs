use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn AboutSection() -> impl IntoView {
    let (about_visible, set_about_visible) = signal(false);

    // Handle scroll events for reveal animations
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let window_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);

                    if let Ok(Some(about_section)) = document.query_selector("#about h2") {
                        let rect = about_section.get_bounding_client_rect();
                        if rect.top() < window_height * 0.75 && rect.bottom() > 0.0 {
                            set_about_visible.set(true);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ =
                window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    });

    view! {
        <section id="about" class="min-h-screen flex items-center px-4 py-20">
            <div class="max-w-7xl mx-auto">
                <h2 class={move || format!(
                    "text-4xl lg:text-6xl font-bold mb-16 text-center transition-all duration-800 font-mono {}",
                    if about_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    <span class="text-catppuccin-pink terminal-glow-text">"about"</span>
                    <span class="text-catppuccin-subtext1 text-xl md:text-2xl lg:text-3xl ml-2 md:ml-4">".md"</span>
                </h2>

                // Main card container
                <div class={move || format!(
                    "relative bg-gradient-to-br from-catppuccin-surface0/90 to-catppuccin-surface1/90 backdrop-blur-xl border border-catppuccin-pink/20 rounded-3xl shadow-2xl overflow-hidden transition-all duration-1000 {}",
                    if about_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    // Decorative elements
                    <div class="absolute top-0 right-0 w-32 h-32 bg-gradient-to-bl from-catppuccin-pink/20 to-transparent rounded-full blur-2xl"></div>
                    <div class="absolute bottom-0 left-0 w-24 h-24 bg-gradient-to-tr from-catppuccin-sapphire/20 to-transparent rounded-full blur-2xl"></div>

                    // Floating sakura petals
                    <div class="absolute top-8 right-8 text-catppuccin-pink/40 text-lg animate-bounce" style="animation-delay: 0s;">"🌸"</div>
                    <div class="absolute top-16 right-24 text-catppuccin-mauve/40 text-sm animate-bounce" style="animation-delay: 1s;">"✨"</div>
                    <div class="absolute bottom-12 left-12 text-catppuccin-sapphire/40 text-lg animate-bounce" style="animation-delay: 2s;">"💫"</div>

                    <div class="grid lg:grid-cols-3 gap-6 md:gap-8 p-4 md:p-8 lg:p-12 relative z-10">
                        // Profile image section
                        <div class={move || format!(
                            "lg:col-span-1 flex flex-col items-center transition-all duration-1200 delay-200 {}",
                            if about_visible.get() { "reveal-slide-left revealed" } else { "reveal-slide-left" }
                        )}>
                            // Profile image container with multiple borders
                            <div class="relative mb-6">
                                // Outer glow ring
                                <div class="absolute -inset-4 bg-gradient-to-r from-catppuccin-pink via-catppuccin-mauve to-catppuccin-sapphire rounded-full blur-lg opacity-60 animate-pulse"></div>

                                // Middle ring
                                <div class="relative w-40 h-40 sm:w-48 sm:h-48 lg:w-56 lg:h-56 rounded-full bg-gradient-to-br from-catppuccin-pink/30 to-catppuccin-sapphire/30 p-1">
                                    // Inner ring
                                    <div class="w-full h-full rounded-full bg-gradient-to-br from-catppuccin-surface0 to-catppuccin-surface1 p-2">
                                        // Profile image
                                        <div class="w-full h-full rounded-full bg-gradient-to-br from-catppuccin-pink/20 via-catppuccin-mauve/20 to-catppuccin-sapphire/20 flex items-center justify-center relative overflow-hidden border-2 border-catppuccin-pink/20">
                                            <img
                                                src="/logo-about.jpg"
                                                alt="SnoW Profile"
                                                class="w-full h-full object-cover rounded-full"
                                            />

                                            // Cute sparkle overlay
                                            <div class="absolute top-4 right-4 text-catppuccin-yellow text-xl animate-ping">"✨"</div>
                                            <div class="absolute bottom-6 left-4 text-catppuccin-pink text-sm animate-pulse">"💖"</div>
                                        </div>
                                    </div>
                                </div>

                                // Status indicator
                                <div class="absolute bottom-2 right-2 w-6 h-6 bg-catppuccin-green rounded-full border-3 border-catppuccin-surface0 flex items-center justify-center">
                                    <div class="w-3 h-3 bg-catppuccin-green rounded-full animate-pulse"></div>
                                </div>
                            </div>

                            // Name and title
                            <div class="text-center mb-6">
                                <h3 class="text-xl sm:text-2xl lg:text-3xl font-bold bg-gradient-to-r from-catppuccin-pink to-catppuccin-mauve bg-clip-text text-transparent mb-2">
                                    "SnoW"
                                </h3>
                                <p class="text-catppuccin-sapphire font-mono text-sm lg:text-base">
                                    "Backend-Oriented Developer"
                                </p>
                                <p class="text-catppuccin-subtext1 text-xs lg:text-sm mt-1">
                                    "🎵 Music Tech • 🌸 Anime Enthusiast • 🦀 Backend Expert"
                                </p>
                            </div>

                            // Quick stats
                            <div class="grid grid-cols-3 gap-2 sm:gap-4 w-full text-center">
                                <div class="bg-catppuccin-surface0/50 rounded-lg p-2 sm:p-3">
                                    <div class="text-catppuccin-pink font-bold text-base sm:text-lg">"25"</div>
                                    <div class="text-catppuccin-subtext1 text-xs">Years Old</div>
                                </div>
                                <div class="bg-catppuccin-surface0/50 rounded-lg p-2 sm:p-3">
                                    <div class="text-catppuccin-sapphire font-bold text-base sm:text-lg">"3+"</div>
                                    <div class="text-catppuccin-subtext1 text-xs">Years CTO</div>
                                </div>
                                <div class="bg-catppuccin-surface0/50 rounded-lg p-2 sm:p-3">
                                    <div class="text-catppuccin-mauve font-bold text-base sm:text-lg">"🎵"</div>
                                    <div class="text-catppuccin-subtext1 text-xs">Music Tech</div>
                                </div>
                            </div>
                        </div>

                        // Content section
                        <div class="lg:col-span-2 space-y-6">
                            // Main description
                            <div class={move || format!(
                                "transition-all duration-1200 delay-400 {}",
                                if about_visible.get() { "reveal-slide-right revealed" } else { "reveal-slide-right" }
                            )}>
                                <div class="bg-catppuccin-surface0/30 rounded-2xl p-6 mb-6 border border-catppuccin-pink/10">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-8 h-8 bg-gradient-to-r from-catppuccin-pink to-catppuccin-mauve rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm">"💫"</span>
                                        </div>
                                        <h4 class="text-xl font-bold text-catppuccin-text">"About Me"</h4>
                                    </div>
                                    <p class="text-catppuccin-text leading-relaxed mb-4">
                                        "Hello! I'm SnoW, a 25-year-old backend engineer, Co-Founder & CTO of Allfeat. For the past 3 years, I've been building a decentralized protocol/database for securing and certifying metadata in the music industry."
                                    </p>
                                    <p class="text-catppuccin-subtext1 leading-relaxed">
                                        "I'm passionate about backend development and love experimenting with new technologies, exotic frameworks, and cutting-edge tools. When I'm not coding, you'll find me gaming (mainly LoL and e-sports), self-hosting servers on NixOS, listening to EDM/Rap/Pop, watching anime, or diving deep into Unix systems and DevOps."
                                    </p>
                                </div>
                            </div>

                            // Skills & Interests grid
                            <div class={move || format!(
                                "grid md:grid-cols-2 gap-6 transition-all duration-1200 delay-600 {}",
                                if about_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                            )}>
                                // Technical Skills
                                <div class="bg-catppuccin-surface0/30 rounded-2xl p-6 border border-catppuccin-sapphire/10">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-8 h-8 bg-gradient-to-r from-catppuccin-sapphire to-catppuccin-blue rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm">"💻"</span>
                                        </div>
                                        <h4 class="text-lg font-bold text-catppuccin-sapphire">"Tech Stack"</h4>
                                    </div>
                                    <div class="space-y-2">
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-cog text-catppuccin-peach"></i>
                                            <span class="text-catppuccin-text text-sm">"Rust (Primary)"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-code text-catppuccin-sapphire"></i>
                                            <span class="text-catppuccin-text text-sm">"C# & .NET"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fab fa-js-square text-catppuccin-green"></i>
                                            <span class="text-catppuccin-text text-sm">"TypeScript"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-snowflake text-catppuccin-mauve"></i>
                                            <span class="text-catppuccin-text text-sm">"NixOS & Nix"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-database text-catppuccin-yellow"></i>
                                            <span class="text-catppuccin-text text-sm">"Database Design"</span>
                                        </div>
                                    </div>
                                </div>

                                // Interests
                                <div class="bg-catppuccin-surface0/30 rounded-2xl p-6 border border-catppuccin-pink/10">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-8 h-8 bg-gradient-to-r from-catppuccin-pink to-catppuccin-red rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm">"🌸"</span>
                                        </div>
                                        <h4 class="text-lg font-bold text-catppuccin-pink">"Interests"</h4>
                                    </div>
                                    <div class="space-y-2">
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-music text-catppuccin-pink"></i>
                                            <span class="text-catppuccin-text text-sm">"EDM, Rap/Hip-Hop, Pop"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-gamepad text-catppuccin-sapphire"></i>
                                            <span class="text-catppuccin-text text-sm">"Gaming (LoL, E-sports)"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-tv text-catppuccin-mauve"></i>
                                            <span class="text-catppuccin-text text-sm">"Anime & Series"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fab fa-linux text-catppuccin-yellow"></i>
                                            <span class="text-catppuccin-text text-sm">"Unix Systems"</span>
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <i class="fas fa-server text-catppuccin-teal"></i>
                                            <span class="text-catppuccin-text text-sm">"Server Self-Hosting"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
