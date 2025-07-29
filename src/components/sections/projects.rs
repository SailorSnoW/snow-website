use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub technologies: Vec<Technology>,
    pub link: Option<String>,
    pub hover_color: String,     // e.g., "text-catppuccin-pink"
    pub border_color: String, // e.g., "border-catppuccin-pink/30 hover:border-catppuccin-pink/60"
    pub animation_class: String, // e.g., "reveal-slide-left"
}

#[derive(Debug, Clone)]
pub enum TechnologyType {
    // Languages
    Rust,

    // Frontend
    Leptos,

    // Backend/Database
    Database,

    // Infrastructure/DevOps
    NixOS,
    DevOps,

    // Blockchain/Crypto
    Blockchain,
    Cryptography,

    // Tools/Other
    HomeManager,
    Nix,
    Discord,
    Sqlite,
    Api,
    WebAssembly,
    TailwindCss,
}

impl TechnologyType {
    pub fn to_technology(&self) -> Technology {
        match self {
            // Languages
            TechnologyType::Rust => Technology {
                name: "Rust".to_string(),
                color: "bg-catppuccin-pink/20 text-catppuccin-pink".to_string(),
            },

            // Frontend
            TechnologyType::Leptos => Technology {
                name: "Leptos".to_string(),
                color: "bg-catppuccin-pink/20 text-catppuccin-pink".to_string(),
            },

            // Backend/Database
            TechnologyType::Database => Technology {
                name: "Database".to_string(),
                color: "bg-catppuccin-mauve/20 text-catppuccin-mauve".to_string(),
            },

            // Infrastructure/DevOps
            TechnologyType::NixOS => Technology {
                name: "NixOS".to_string(),
                color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire".to_string(),
            },
            TechnologyType::DevOps => Technology {
                name: "DevOps".to_string(),
                color: "bg-catppuccin-yellow/20 text-catppuccin-yellow".to_string(),
            },

            // Blockchain/Crypto
            TechnologyType::Blockchain => Technology {
                name: "Blockchain".to_string(),
                color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire".to_string(),
            },
            TechnologyType::Cryptography => Technology {
                name: "Cryptography".to_string(),
                color: "bg-catppuccin-green/20 text-catppuccin-green".to_string(),
            },

            // Tools/Other
            TechnologyType::HomeManager => Technology {
                name: "Home Manager".to_string(),
                color: "bg-catppuccin-green/20 text-catppuccin-green".to_string(),
            },
            TechnologyType::Nix => Technology {
                name: "Nix".to_string(),
                color: "bg-catppuccin-mauve/20 text-catppuccin-mauve".to_string(),
            },
            TechnologyType::Discord => Technology {
                name: "Discord".to_string(),
                color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire".to_string(),
            },
            TechnologyType::Sqlite => Technology {
                name: "SQLite".to_string(),
                color: "bg-catppuccin-green/20 text-catppuccin-green".to_string(),
            },
            TechnologyType::Api => Technology {
                name: "API".to_string(),
                color: "bg-catppuccin-yellow/20 text-catppuccin-yellow".to_string(),
            },
            TechnologyType::WebAssembly => Technology {
                name: "WebAssembly".to_string(),
                color: "bg-catppuccin-mauve/20 text-catppuccin-mauve".to_string(),
            },
            TechnologyType::TailwindCss => Technology {
                name: "TailwindCSS".to_string(),
                color: "bg-catppuccin-teal/20 text-catppuccin-teal".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Technology {
    pub name: String,
    pub color: String, // Background and text color classes
}

fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Allfeat".to_string(),
            icon: "🎵".to_string(),
            description: "Decentralized protocol/database for securing and certifying metadata in the music industry. Built with Rust for maximum performance and security.".to_string(),
            technologies: vec![
                TechnologyType::Rust.to_technology(),
                TechnologyType::Blockchain.to_technology(),
                TechnologyType::Database.to_technology(),
                TechnologyType::Cryptography.to_technology(),
            ],
            link: Some("https://allfeat.com".to_string()),
            hover_color: "group-hover:text-catppuccin-pink".to_string(),
            border_color: "border-catppuccin-pink/30 hover:border-catppuccin-pink/60".to_string(),
            animation_class: "reveal-slide-left".to_string(),
        },
        Project {
            name: "NixOS Config".to_string(),
            icon: "❄️".to_string(),
            description: "My personal NixOS configuration with custom modules, dotfiles management, and development environments. Reproducible and declarative system setup.".to_string(),
            technologies: vec![
                TechnologyType::NixOS.to_technology(),
                TechnologyType::Nix.to_technology(),
                TechnologyType::HomeManager.to_technology(),
                TechnologyType::DevOps.to_technology(),
            ],
            link: Some("https://github.com/sailorsnow/nixos-config".to_string()),
            hover_color: "group-hover:text-catppuccin-sapphire".to_string(),
            border_color: "border-catppuccin-sapphire/30 hover:border-catppuccin-sapphire/60".to_string(),
            animation_class: "reveal-slide-right".to_string(),
        },
        Project {
            name: "Tentrackule".to_string(),
            icon: "🐙".to_string(),
            description: "Discord bot written in Rust that tracks League of Legends/TFT players and sends alerts when matches are completed. Uses Riot Games API with rate limiting and local SQLite storage.".to_string(),
            technologies: vec![
                TechnologyType::Rust.to_technology(),
                TechnologyType::Discord.to_technology(),
                TechnologyType::Sqlite.to_technology(),
                TechnologyType::Api.to_technology(),
            ],
            link: Some("https://github.com/SailorSnoW/Tentrackule".to_string()),
            hover_color: "group-hover:text-catppuccin-green".to_string(),
            border_color: "border-catppuccin-green/30 hover:border-catppuccin-green/60".to_string(),
            animation_class: "reveal-slide-left".to_string(),
        },
        Project {
            name: "Portfolio Website".to_string(),
            icon: "🌸".to_string(),
            description: "Personal portfolio website built with Rust and WebAssembly. Features anime-inspired design, terminal aesthetics, and modern animations. Built with Leptos framework and compiled to WASM.".to_string(),
            technologies: vec![
                TechnologyType::Rust.to_technology(),
                TechnologyType::Leptos.to_technology(),
                TechnologyType::WebAssembly.to_technology(),
                TechnologyType::TailwindCss.to_technology(),
            ],
            link: Some("https://github.com/SailorSnoW/snow-website".to_string()),
            hover_color: "group-hover:text-catppuccin-pink".to_string(),
            border_color: "border-catppuccin-pink/30 hover:border-catppuccin-pink/60".to_string(),
            animation_class: "reveal-slide-right".to_string(),
        },
    ]
}

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let (projects_visible, set_projects_visible) = signal(false);

    // Handle scroll events for reveal animations
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let window_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);

                    if let Ok(Some(projects_section)) = document.query_selector("#projects h2") {
                        let rect = projects_section.get_bounding_client_rect();
                        if rect.top() < window_height * 0.75 && rect.bottom() > 0.0 {
                            set_projects_visible.set(true);
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

    let projects = get_projects();

    view! {
        <section id="projects" class="min-h-screen flex items-center px-4 py-20">
            <div class="max-w-6xl mx-auto">
                <h2 class={move || format!(
                    "text-4xl lg:text-6xl font-bold mb-12 text-center transition-all duration-800 font-mono {}",
                    if projects_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    <span class="text-catppuccin-mauve terminal-glow-text">"projects"</span>
                    <span class="text-catppuccin-subtext1 text-xl md:text-2xl lg:text-3xl ml-2 md:ml-4">".rs"</span>
                </h2>

                <div class="grid md:grid-cols-2 xl:grid-cols-3 gap-4 md:gap-8">
                    {projects.into_iter().map(|project| {
                        view! {
                            <div class={move || format!(
                                "bg-catppuccin-surface0/80 backdrop-blur border {} rounded-lg p-4 md:p-6 transition-all duration-1000 group {}",
                                project.border_color,
                                if projects_visible.get() {
                                    format!("{} revealed", project.animation_class)
                                } else {
                                    project.animation_class.clone()
                                }
                            )}>
                                <div class="flex items-center gap-3 mb-4">
                                    <div class="text-2xl">{project.icon}</div>
                                    <h3 class="text-lg md:text-xl font-bold text-catppuccin-text font-mono">{project.name}</h3>
                                </div>

                                <p class="text-catppuccin-text mb-4">{project.description}</p>

                                <div class="flex flex-wrap gap-2 mb-4">
                                    {project.technologies.into_iter().map(|tech| {
                                        view! {
                                            <span class={format!("px-2 py-1 {} text-xs rounded font-mono", tech.color)}>
                                                {tech.name}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>

                                {if let Some(link) = project.link {
                                    view! {
                                        <a href={link} target="_blank" class={format!("text-sm text-catppuccin-subtext1 {} transition-colors font-mono inline-flex items-center gap-2", project.hover_color)}>
                                            <i class="fas fa-external-link-alt"></i>
                                            "View Project →"
                                        </a>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class={format!("text-sm text-catppuccin-subtext1 {} transition-colors font-mono", project.hover_color)}>
                                            "// Private Repository"
                                        </div>
                                    }.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
