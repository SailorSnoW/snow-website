pub mod components;
pub mod constants;
pub mod pages;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Top-Level pages
use crate::components::ui::animated_background::AnimatedBackground;
use crate::components::ui::custom_cursor::CustomCursor;
use crate::components::ui::discord_badge::DiscordBadge;
use crate::components::ui::loading_screen::LoadingScreen;
use crate::components::ui::terminal_badge::TerminalBadge;
use crate::components::ui::terminal_modal::TerminalModal;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (loading_complete, set_loading_complete) = signal(false);
    let (terminal_open, set_terminal_open) = signal(false);

    // Disable body scroll when loading or when terminal is open
    Effect::new(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    if !loading_complete.get() || terminal_open.get() {
                        let _ = body.style().set_property("overflow", "hidden");
                    } else {
                        let _ = body.style().set_property("overflow", "");
                    }
                }
            }
        }
    });

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

        // sets the document title
        <Title text="Loïs L. / SnoW" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class={move || format!(
            "min-h-screen relative {}",
            if !loading_complete.get() { "overflow-hidden" } else { "" }
        )}>
            // Animated background with stars
            <AnimatedBackground />

            // Always render the homepage background and content
            <div class={move ||
                if !loading_complete.get() { "pointer-events-none" } else { "" }
            }>
                <Router>
                    <Routes fallback=|| view! { <Home /> }>
                        <Route path=path!("/") view=Home />
                    </Routes>
                </Router>
            </div>

            // Terminal badge (always visible when loading is complete)
            {move || {
                if loading_complete.get() {
                    view! {
                        <TerminalBadge on_click=Callback::new(move |_| set_terminal_open.set(true)) />
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }
            }}

            // Discord badge (always visible when loading is complete)
            {move || {
                if loading_complete.get() {
                    view! {
                        <DiscordBadge />
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }
            }}

            // Terminal modal
            <TerminalModal
                is_open=terminal_open
                on_close=Callback::new(move |_| set_terminal_open.set(false))
            />

            // Loading screen overlay
            {move || {
                if !loading_complete.get() {
                    view! {
                        <LoadingScreen on_complete=move || set_loading_complete.set(true) />
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }
            }}
            
            // Custom cursor (always visible when loading is complete)
            {move || {
                if loading_complete.get() {
                    view! {
                        <CustomCursor />
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }
            }}
        </div>
    }
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    // Developer signature in console
    web_sys::console::log_1(&"Made with 💖 by SnoW".into());
    
    leptos::mount::mount_to_body(App);
}
