use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::constants::{CONTACT_INFO, REVEAL_THRESHOLD};

#[component]
pub fn ContactSection() -> impl IntoView {
    let (contact_visible, set_contact_visible) = signal(false);

    // Handle scroll events for reveal animations
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let window_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);

                    if let Ok(Some(contact_section)) = document.query_selector("#contact h2") {
                        let rect = contact_section.get_bounding_client_rect();
                        if rect.top() < window_height * REVEAL_THRESHOLD && rect.bottom() > 0.0 {
                            set_contact_visible.set(true);
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
        <section id="contact" class="min-h-screen flex items-center px-4 py-20">
            <div class="max-w-4xl mx-auto text-center">
                <h2 class={move || format!(
                    "text-4xl lg:text-6xl font-bold mb-8 md:mb-12 transition-all duration-800 font-mono {}",
                    if contact_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    <span class="text-catppuccin-yellow terminal-glow-text">"contact"</span>
                    <span class="text-catppuccin-subtext1 text-xl md:text-2xl lg:text-3xl ml-2 md:ml-4">".json"</span>
                </h2>

                <div class={move || format!(
                    "bg-catppuccin-surface0/80 backdrop-blur border border-catppuccin-yellow/30 rounded-lg p-4 md:p-8 mb-6 md:mb-8 transition-all duration-600 delay-100 {}",
                    if contact_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                )}>
                    <div class="text-left font-mono text-xs md:text-sm overflow-x-auto">
                        <div class="text-catppuccin-text">"{"</div>
                        <div class="ml-2 md:ml-4 space-y-1 md:space-y-2">
                            <div class="flex flex-wrap">
                                <span class="text-catppuccin-yellow">"\"message\""</span>
                                <span class="text-catppuccin-text">": "</span>
                                <span class="text-catppuccin-green break-words">{format!("\"{}\"" , CONTACT_INFO.message)}</span>
                            </div>
                            <div class="flex">
                                <span class="text-catppuccin-yellow">"\"contact\""</span>
                                <span class="text-catppuccin-text">": {"</span>
                            </div>
                            <div class="ml-4 md:ml-8 space-y-1">
                                <div class="flex flex-wrap">
                                    <span class="text-catppuccin-yellow">"\"email\""</span>
                                    <span class="text-catppuccin-text">": "</span>
                                    <span class="text-catppuccin-green break-all">{format!("\"{}\"" , CONTACT_INFO.email)}</span>
                                </div>
                                <div class="flex flex-wrap">
                                    <span class="text-catppuccin-yellow">"\"github\""</span>
                                    <span class="text-catppuccin-text">": "</span>
                                    <span class="text-catppuccin-green break-all">{format!("\"{}\"" , CONTACT_INFO.github)}</span>
                                </div>
                                <div class="flex flex-wrap">
                                    <span class="text-catppuccin-yellow">"\"linkedin\""</span>
                                    <span class="text-catppuccin-text">": "</span>
                                    <span class="text-catppuccin-green break-all">{format!("\"{}\"" , CONTACT_INFO.linkedin)}</span>
                                </div>
                                <div class="flex flex-wrap">
                                    <span class="text-catppuccin-yellow">"\"discord\""</span>
                                    <span class="text-catppuccin-text">": "</span>
                                    <span class="text-catppuccin-green break-all">{format!("\"{}\"" , CONTACT_INFO.discord)}</span>
                                </div>
                            </div>
                            <div class="text-catppuccin-text ml-2 md:ml-4">"}"</div>
                        </div>
                        <div class="text-catppuccin-text">"}"</div>
                    </div>

                    <div class="flex flex-wrap justify-center gap-3 md:gap-6 mt-6 md:mt-8">
                        <a target="_blank" href="mailto:loislag@pm.me" class={move || format!(
                            "flex items-center gap-2 px-4 py-2 md:px-6 md:py-3 bg-catppuccin-pink/20 border border-catppuccin-pink/50 rounded-lg text-catppuccin-pink hover:border-catppuccin-pink/80 transition-all duration-600 delay-200 font-mono text-sm md:text-base {}",
                            if contact_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                        )}>
                            <span>"📧"</span>
                            "Email"
                        </a>
                        <a target="_blank" href="https://fr.linkedin.com/in/llagoutte" class={move || format!(
                            "flex items-center gap-2 px-4 py-2 md:px-6 md:py-3 bg-catppuccin-sapphire/20 border border-catppuccin-sapphire/50 rounded-lg text-catppuccin-sapphire hover:border-catppuccin-sapphire/80 transition-all duration-600 delay-300 font-mono text-sm md:text-base {}",
                            if contact_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                        )}>
                            <span>"💼"</span>
                            "LinkedIn"
                        </a>
                       <a target="_blank" href="github.com/sailorsnow" class={move || format!(
                            "flex items-center gap-2 px-4 py-2 md:px-6 md:py-3 bg-catppuccin-mauve/20 border border-catppuccin-mauve/50 rounded-lg text-catppuccin-mauve hover:border-catppuccin-mauve/80 transition-all duration-600 delay-400 font-mono text-sm md:text-base {}",
                            if contact_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                        )}>
                            <span>"🐱"</span>
                            "GitHub"
                        </a>
                    </div>
                </div>

                <div class={move || format!(
                    "text-catppuccin-subtext1 font-mono text-sm transition-all duration-600 delay-500 {}",
                    if contact_visible.get() { "reveal-stagger revealed" } else { "reveal-stagger" }
                )}>
                    "// Feel free to reach out"
                </div>
            </div>
        </section>
    }
}

