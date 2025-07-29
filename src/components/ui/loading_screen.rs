use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use crate::constants::{
    BOOT_MESSAGES, LOADING_ENTER_SKIP_DELAY, LOADING_TRANSITION_DELAY,
    css::{TERMINAL_OVERLAY, TERMINAL_TRANSITION, TERMINAL_EXIT, TERMINAL_GLOW_TEXT}
};

#[component]
pub fn LoadingScreen<F>(on_complete: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let (current_line, set_current_line) = signal(0);
    let (is_transitioning, set_is_transitioning) = signal(false);

    let total_lines = BOOT_MESSAGES.len();

    // Auto-advance through boot sequence
    Effect::new(move || {
        let current = current_line.get();
        if current < total_lines {
            let delay = BOOT_MESSAGES[current].1;
            set_timeout(
                move || {
                    set_current_line.update(|n| *n += 1);
                },
                Duration::from_millis(delay),
            );
        } else {
            // Boot sequence complete, start transition
            set_timeout(
                move || {
                    set_is_transitioning.set(true);
                },
                Duration::from_millis(LOADING_ENTER_SKIP_DELAY),
            );
        }
    });

    // Handle keyboard events to skip loading
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Enter" {
                set_is_transitioning.set(true);
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
        }

        // Keep closure alive for the duration of the effect
        closure.forget();
    });

    // Handle transition effect
    Effect::new(move || {
        if is_transitioning.get() {
            set_timeout(
                {
                    let on_complete = on_complete.clone();
                    move || on_complete()
                },
                Duration::from_millis(LOADING_TRANSITION_DELAY),
            );
        }
    });

    view! {
        <div 
            class={move || {
                let base_classes = TERMINAL_OVERLAY;
                if is_transitioning.get() {
                    format!("{base_classes} {TERMINAL_EXIT}")
                } else {
                    format!("{base_classes} {TERMINAL_TRANSITION}")
                }
            }}
            on:click=move |_| set_is_transitioning.set(true)
        >
            <div class="flex-1 overflow-y-auto relative z-10">
                <div class="mb-4 flex flex-col md:flex-row items-center md:items-start">
                    // Boot screen logo
                    <div class="mr-0 md:mr-6 mb-4 md:mb-0 flex-shrink-0">
                        <img
                            src="/bootscreen.jpg"
                            alt="SnoW Systems Logo"
                            class="w-24 h-24 md:w-32 md:h-32 object-contain rounded border border-pink-400/30"
                        />
                    </div>

                    // BIOS info
                    <div class="flex-1 text-center md:text-left">
                        <div class={format!("text-pink-400 text-lg md:text-xl font-bold mb-2 {TERMINAL_GLOW_TEXT}")}>
                            // Mobile version (shorter)
                            <span class="block md:hidden">
                                "SnOwOS BIOS v2.1.4"
                            </span>
                            // Desktop version (full)
                            <span class="hidden md:block">
                                "⋆｡‧˚ʚ♡ɞ˚‧｡⋆ SnOwOS BIOS v2.1.4 ⋆｡‧˚ʚ♡ɞ˚‧｡⋆"
                            </span>
                        </div>
                        <div class="text-gray-400 text-sm terminal-line">
                            "Copyright (C) 2024 SnoW Systems ✨"
                        </div>
                    </div>
                </div>

                <div class="space-y-1">
                    {move || {
                        let current = current_line.get();
                        (0..current.min(BOOT_MESSAGES.len())).map(|i| {
                            let (line, _delay) = BOOT_MESSAGES[i];
                            let line_class = if line.contains("[ OK ]") {
                                "text-green-400"
                            } else if line.contains("Welcome") || line.contains("magical") {
                                "text-cyan-400 font-bold"
                            } else {
                                "text-green-300"
                            };

                            let cursor = if i == current.saturating_sub(1) && !line.is_empty() {
                                " █"
                            } else {
                                ""
                            };

                            view! {
                                <div class={format!("{line_class} terminal-line text-xs md:text-sm")}>
                                    {format!("{line}{cursor}")}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>

                {move || {
                    if is_transitioning.get() {
                        view! {
                            <div class="mt-8 text-center">
                                <div class="text-cyan-400 text-base md:text-lg font-bold animate-pulse terminal-line">
                                    "Entering SnOwOS... ✨"
                                </div>
                                <div class="text-pink-400 text-sm mt-2 animate-bounce terminal-line">
                                    "♡ ♡ ♡"
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div class=""></div> }.into_any()
                    }
                }}
            </div>

            <div class="mt-4 relative z-10 text-center">
                {move || {
                    if is_transitioning.get() {
                        view! {
                            <div class="text-gray-500 text-xs terminal-line">
                                ""
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                // Loading hearts animation
                                <div class="flex justify-center items-center gap-2 md:gap-4 text-pink-400 text-2xl md:text-3xl font-bold">
                                    <span class="animate-bounce terminal-glow-text" style="animation-delay: 0s; text-shadow: 0 0 10px #f5c2e7, 0 0 20px #f5c2e7, 0 0 30px #f5c2e7;">
                                        "♥"
                                    </span>
                                    <span class="animate-bounce terminal-glow-text" style="animation-delay: 0.2s; text-shadow: 0 0 10px #f5c2e7, 0 0 20px #f5c2e7, 0 0 30px #f5c2e7;">
                                        "♥"
                                    </span>
                                    <span class="animate-bounce terminal-glow-text" style="animation-delay: 0.4s; text-shadow: 0 0 10px #f5c2e7, 0 0 20px #f5c2e7, 0 0 30px #f5c2e7;">
                                        "♥"
                                    </span>
                                </div>

                                // Press Enter hint
                                <div class="text-gray-500 text-xs animate-pulse terminal-line">
                                    // Mobile message
                                    <span class="block md:hidden">
                                        "Tap to skip..."
                                    </span>
                                    // Desktop message
                                    <span class="hidden md:block">
                                        "Press Enter to skip..."
                                    </span>
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
