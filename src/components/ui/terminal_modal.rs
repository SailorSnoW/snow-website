use leptos::html::Input;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::constants::{TERMINAL_WELCOME, TERMINAL_PROMPT_USER, TERMINAL_PROMPT_HOSTNAME, TERMINAL_PROMPT_PATH};

#[component]
pub fn TerminalModal(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (terminal_input, set_terminal_input) = signal("".to_string());
    let (terminal_history, set_terminal_history) = signal(Vec::<String>::new());
    let input_ref = NodeRef::<Input>::new();

    // Initialize terminal with welcome message
    Effect::new(move || {
        if is_open.get() && terminal_history.get().is_empty() {
            let welcome_msgs = TERMINAL_WELCOME.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
            set_terminal_history.set(welcome_msgs);
        }
    });

    // Handle terminal commands
    let handle_command = move |cmd: String| {
        let mut history = terminal_history.get();
        history.push(format!("SnoW@snOwOS:~$ {cmd}"));

        let response = match cmd.trim() {
            "help" => "Commands: clear, exit, quit".to_string(),
            "clear" => {
                let welcome_msgs = TERMINAL_WELCOME.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
                set_terminal_history.set(welcome_msgs);
                set_terminal_input.set("".to_string());
                return;
            }
            "exit" | "quit" => {
                on_close.run(());
                return;
            }
            _ => format!("Command '{cmd}' not found. Type 'help' for available commands! 💫"),
        };

        if !response.is_empty() {
            history.push(response);
        }
        set_terminal_history.set(history);
        set_terminal_input.set("".to_string());
    };

    // Handle Escape key to close modal and focus input when opened
    Effect::new(move || {
        if is_open.get() {
            // Focus input when modal opens
            if let Some(input_element) = input_ref.get() {
                let _ = input_element.focus();
            }

            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    on_close.run(());
                }
            }) as Box<dyn FnMut(_)>);

            if let Some(window) = web_sys::window() {
                let _ = window
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }

            closure.forget();
        }
    });

    view! {
        <div class={move || format!(
            "fixed inset-0 z-50 transition-all duration-300 {}",
            if is_open.get() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" }
        )}>
            // Terminal modal container with click outside handler
            <div
                class="relative z-10 flex items-center justify-center min-h-screen p-4 bg-catppuccin-crust/80 backdrop-blur-md"
                on:click=move |_| on_close.run(())
            >
                <div class={move || format!(
                    "bg-catppuccin-crust/95 backdrop-blur border border-catppuccin-surface0 rounded-lg shadow-2xl max-w-4xl w-full transition-all duration-300 {}",
                    if is_open.get() { "scale-100 translate-y-0" } else { "scale-95 translate-y-4" }
                )}
                on:click=move |e| e.stop_propagation()
                >
                    // Terminal header
                    <div class="flex items-center justify-between px-4 py-3 bg-catppuccin-surface0 rounded-t-lg border-b border-catppuccin-surface1">
                        <div class="flex items-center space-x-2">
                            <div class="w-3 h-3 rounded-full bg-catppuccin-red hover:bg-catppuccin-red/80 cursor-pointer transition-colors"
                                on:click=move |_| on_close.run(())
                            ></div>
                            <div class="w-3 h-3 rounded-full bg-catppuccin-yellow"></div>
                            <div class="w-3 h-3 rounded-full bg-catppuccin-green"></div>
                        </div>
                        <span class="text-catppuccin-subtext1 text-sm font-mono">"SnOwOS Terminal"</span>
                        <button
                            class="text-catppuccin-subtext1 hover:text-catppuccin-text transition-colors p-1"
                            on:click=move |_| on_close.run(())
                        >
                            "✕"
                        </button>
                    </div>

                    // Terminal content
                    <div class="p-6 font-mono text-sm h-96 overflow-y-auto bg-catppuccin-base/50">
                        // Terminal history
                        <div class="space-y-1 mb-4">
                            {move || terminal_history.get().iter().map(|line| {
                                view! {
                                    <div class="whitespace-pre-wrap text-catppuccin-text">
                                        {line.clone()}
                                    </div>
                                }
                            }).collect_view()}
                        </div>

                        // Current input line
                        <div class="flex items-center">
                            <span class="text-catppuccin-pink mr-2">{TERMINAL_PROMPT_USER}</span>
                            <span class="text-catppuccin-subtext1 mr-1">"@"</span>
                            <span class="text-catppuccin-sapphire mr-1">{TERMINAL_PROMPT_HOSTNAME}</span>
                            <span class="text-catppuccin-green mr-2">":"</span>
                            <span class="text-catppuccin-mauve mr-2">{TERMINAL_PROMPT_PATH}</span>
                            <span class="text-catppuccin-green mr-2">"$"</span>
                            <input
                                type="text"
                                class="bg-transparent outline-none flex-1 text-catppuccin-text font-mono"
                                prop:value=move || terminal_input.get()
                                on:input=move |ev| set_terminal_input.set(event_target_value(&ev))
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        handle_command(terminal_input.get());
                                    }
                                }
                                placeholder="Try 'help' to get started..."
                                node_ref=input_ref
                            />
                            <span class="text-catppuccin-text animate-pulse ml-1">"█"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

