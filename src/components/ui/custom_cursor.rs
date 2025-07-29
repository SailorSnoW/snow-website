use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn CustomCursor() -> impl IntoView {
    let (cursor_pos, set_cursor_pos) = signal((0.0, 0.0));
    let (is_visible, set_is_visible) = signal(true);

    // Set up global mouse tracking
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.client_x() as f64;
            let y = event.client_y() as f64;

            set_cursor_pos.set((x, y));
            set_is_visible.set(true);
        }) as Box<dyn FnMut(_)>);

        let mouseleave_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            set_is_visible.set(false);
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
            let _ = window.add_event_listener_with_callback(
                "mouseleave",
                mouseleave_closure.as_ref().unchecked_ref(),
            );
        }

        closure.forget();
        mouseleave_closure.forget();
    });

    view! {
        // Hide default cursor (desktop only)
        <style>
            {r#"
            @media (min-width: 768px) {
                * {
                    cursor: none !important;
                }
            }
            
            .snowflake-cursor {
                pointer-events: none;
                position: fixed;
                z-index: 9999;
                transform: translate(-50%, -50%);
                color: #f5c2e7;
                font-size: 24px;
                text-shadow: 0 0 10px #f5c2e7, 0 0 20px #f5c2e7;
                animation: snowflake-rotate 12s linear infinite;
            }
            
            @media (max-width: 767px) {
                .snowflake-cursor {
                    display: none !important;
                }
            }
            
            @keyframes snowflake-rotate {
                from { transform: translate(-50%, -50%) rotate(0deg); }
                to { transform: translate(-50%, -50%) rotate(360deg); }
            }
            "#}
        </style>

        // Custom cursor (snowflake)
        {move || {
            let (cursor_x, cursor_y) = cursor_pos.get();
            if is_visible.get() {
                view! {
                    <div
                        class="snowflake-cursor"
                        style={format!("left: {cursor_x}px; top: {cursor_y}px;")}
                    >
                        <i class="fas fa-snowflake"></i>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }
        }}
    }
}

