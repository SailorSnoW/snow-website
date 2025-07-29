use leptos::html;
use leptos::prelude::*;
use web_sys::js_sys;
use crate::constants::{
    FLOATING_STARS_COUNT, SHOOTING_STARS_COUNT, STAR_MIN_SIZE, STAR_MAX_SIZE,
    STAR_MIN_OPACITY, STAR_MAX_OPACITY, STAR_MIN_DURATION, STAR_MAX_DURATION,
    STAR_MAX_DELAY, SHOOTING_STAR_MIN_DURATION, SHOOTING_STAR_MAX_DURATION,
    SHOOTING_STAR_MAX_DELAY, SHOOTING_STAR_MIN_TRAIL, SHOOTING_STAR_MAX_TRAIL,
    SHOOTING_STAR_POSITION_RANGE, SHOOTING_STAR_POSITION_OFFSET, SHOOTING_STAR_COLORS,
};

/// Generates a random floating star element with physics-based properties
fn create_floating_star() -> Result<web_sys::Element, wasm_bindgen::JsValue> {
    let window = web_sys::window().ok_or("No window available")?;
    let document = window.document().ok_or("No document available")?;
    let star = document.create_element("div")?;

    let x = js_sys::Math::random() * 100.0;
    let y = js_sys::Math::random() * 100.0;
    let size = STAR_MIN_SIZE + js_sys::Math::random() * (STAR_MAX_SIZE - STAR_MIN_SIZE);
    let opacity = STAR_MIN_OPACITY + js_sys::Math::random() * (STAR_MAX_OPACITY - STAR_MIN_OPACITY);
    let duration = STAR_MIN_DURATION + js_sys::Math::random() * (STAR_MAX_DURATION - STAR_MIN_DURATION);
    let delay = js_sys::Math::random() * STAR_MAX_DELAY;

    star.set_class_name("absolute rounded-full bg-white");
    star.set_attribute("style", &format!(
        "left: {x}%; top: {y}%; width: {size}px; height: {size}px; opacity: {opacity}; animation: float {duration}s ease-in-out infinite {delay}s alternate;"
    ))?;

    Ok(star)
}

/// Calculates shooting star trajectory and properties
struct ShootingStarConfig {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    angle: f64,
    trail_length: f64,
    duration: f64,
    delay: f64,
    color_variant: &'static str,
}

impl ShootingStarConfig {
    fn new(_index: usize) -> Self {
        let start_x = js_sys::Math::random() * SHOOTING_STAR_POSITION_RANGE + SHOOTING_STAR_POSITION_OFFSET;
        let start_y = js_sys::Math::random() * SHOOTING_STAR_POSITION_RANGE + SHOOTING_STAR_POSITION_OFFSET;
        let end_x = js_sys::Math::random() * SHOOTING_STAR_POSITION_RANGE + SHOOTING_STAR_POSITION_OFFSET;
        let end_y = js_sys::Math::random() * SHOOTING_STAR_POSITION_RANGE + SHOOTING_STAR_POSITION_OFFSET;
        
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let angle = dy.atan2(dx) * 180.0 / std::f64::consts::PI;
        
        let trail_length = SHOOTING_STAR_MIN_TRAIL + js_sys::Math::random() * (SHOOTING_STAR_MAX_TRAIL - SHOOTING_STAR_MIN_TRAIL);
        let duration = SHOOTING_STAR_MIN_DURATION + js_sys::Math::random() * (SHOOTING_STAR_MAX_DURATION - SHOOTING_STAR_MIN_DURATION);
        let delay = js_sys::Math::random() * SHOOTING_STAR_MAX_DELAY;
        
        let color_index = (js_sys::Math::random() * SHOOTING_STAR_COLORS.len() as f64) as usize;
        let color_variant = SHOOTING_STAR_COLORS[color_index.min(SHOOTING_STAR_COLORS.len() - 1)];

        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            angle,
            trail_length,
            duration,
            delay,
            color_variant,
        }
    }
}

/// Creates a shooting star element with custom animation keyframes
fn create_shooting_star(config: &ShootingStarConfig, index: usize) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
    let window = web_sys::window().ok_or("No window available")?;
    let document = window.document().ok_or("No document available")?;
    
    let shooting_star = document.create_element("div")?;
    
    shooting_star.set_class_name(&format!(
        "absolute rounded-full bg-gradient-to-r {}",
        config.color_variant
    ));
    
    shooting_star.set_attribute("style", &format!(
        "left: {}%; top: {}%; width: 1px; height: 1px; animation: shootingStar{} {}s linear infinite {}s; transform-origin: center; box-shadow: 0 0 6px 1px rgba(245, 194, 231, 0.2);",
        config.start_x, config.start_y, index, config.duration, config.delay
    ))?;
    
    // Create custom keyframes for this shooting star
    let end_translate_x = (config.end_x - config.start_x) * 10.0;
    let end_translate_y = (config.end_y - config.start_y) * 10.0;
    
    let keyframes = format!(
        "@keyframes shootingStar{} {{
            0% {{
                transform: translateX(0) translateY(0) rotate({}deg);
                opacity: 0;
                width: 1px;
                height: 1px;
            }}
            10% {{
                opacity: 1;
                width: {}px;
                height: 1px;
            }}
            90% {{
                opacity: 1;
                width: {}px;
                height: 1px;
            }}
            100% {{
                transform: translateX({}px) translateY({}px) rotate({}deg);
                opacity: 0;
                width: 1px;
                height: 1px;
            }}
        }}",
        index, config.angle, config.trail_length, config.trail_length,
        end_translate_x, end_translate_y, config.angle
    );
    
    // Inject keyframes into document head
    let style_element = document.create_element("style")?;
    style_element.set_text_content(Some(&keyframes));
    
    if let Some(head) = document.head() {
        head.append_child(&style_element)?;
    }
    
    Ok(shooting_star)
}

/// Populates the container with floating and shooting stars
fn populate_stars_container(container: &web_sys::Element) -> Result<(), wasm_bindgen::JsValue> {
    // Clear existing content
    container.set_inner_html("");
    
    // Generate floating stars
    for _ in 0..FLOATING_STARS_COUNT {
        if let Ok(star) = create_floating_star() {
            let _ = container.append_child(&star);
        }
    }

    
    // Generate shooting stars
    for i in 0..SHOOTING_STARS_COUNT {
        let config = ShootingStarConfig::new(i as usize);
        if let Ok(shooting_star) = create_shooting_star(&config, i as usize) {
            let _ = container.append_child(&shooting_star);
        }
    }
    
    Ok(())
}

#[component]
pub fn AnimatedBackground() -> impl IntoView {
    let stars_container_ref = NodeRef::<html::Div>::new();

    // Generate stars on mount with proper error handling
    Effect::new(move || {
        if let Some(container) = stars_container_ref.get() {
            // Populate stars container and log errors if any
            if let Err(_e) = populate_stars_container(&container) {
                web_sys::console::warn_1(&"Failed to create stars".into());
            }
        }
    });

    view! {
        <div class="fixed inset-0 overflow-hidden pointer-events-none">
            // Gradient background
            <div class="absolute inset-0 bg-gradient-to-br from-catppuccin-crust via-catppuccin-mantle to-catppuccin-base"></div>

            // Nebula-like overlay (more subtle)
            <div class="absolute inset-0 bg-gradient-to-tr from-catppuccin-pink/3 via-transparent to-catppuccin-sapphire/3"></div>
            <div class="absolute inset-0 bg-gradient-to-bl from-catppuccin-mauve/2 via-transparent to-catppuccin-green/2"></div>

            // Stars container
            <div node_ref=stars_container_ref class="absolute inset-0"></div>

            // Twinkling animation overlay (more subtle)
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_25%_25%,rgba(245,194,231,0.04)_0%,transparent_60%)] animate-pulse"></div>
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_75%_75%,rgba(137,180,250,0.04)_0%,transparent_60%)] animate-pulse" style="animation-delay: 3s;"></div>
            <div class="absolute inset-0 bg-[radial-gradient(circle_at_50%_50%,rgba(203,166,247,0.04)_0%,transparent_60%)] animate-pulse" style="animation-delay: 6s;"></div>

            // CSS animations
            <style>
                {r#"
                @keyframes float {
                    0% { transform: translateY(0px) translateX(0px) rotate(0deg); }
                    25% { transform: translateY(-10px) translateX(5px) rotate(90deg); }
                    50% { transform: translateY(-5px) translateX(-5px) rotate(180deg); }
                    75% { transform: translateY(-15px) translateX(3px) rotate(270deg); }
                    100% { transform: translateY(0px) translateX(0px) rotate(360deg); }
                }
                
                
                @keyframes twinkle {
                    0%, 100% { opacity: 0.3; transform: scale(1); }
                    50% { opacity: 1; transform: scale(1.2); }
                }
                "#}
            </style>
        </div>
    }
}

