use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub color: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct SkillCategory {
    pub title: String,
    pub color: String,
    pub icon: String,
    pub skills: Vec<Skill>,
}

fn get_skill_categories() -> Vec<SkillCategory> {
    vec![
        SkillCategory {
            title: "Languages".to_string(),
            color: "catppuccin-pink".to_string(),
            icon: "💻".to_string(),
            skills: vec![
                Skill {
                    name: "Rust".to_string(),
                    icon: "🦀".to_string(),
                    color: "bg-catppuccin-pink/20 text-catppuccin-pink border-catppuccin-pink/30".to_string(),
                    description: "Systems programming, WebAssembly, performance-critical applications".to_string(),
                },
                Skill {
                    name: "TypeScript".to_string(),
                    icon: "📘".to_string(),
                    color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire border-catppuccin-sapphire/30".to_string(),
                    description: "Type-safe JavaScript for scalable applications".to_string(),
                },
                Skill {
                    name: "JavaScript".to_string(),
                    icon: "⚡".to_string(),
                    color: "bg-catppuccin-yellow/20 text-catppuccin-yellow border-catppuccin-yellow/30".to_string(),
                    description: "Modern ES6+, Node.js, frontend & backend development".to_string(),
                },
                Skill {
                    name: "C#".to_string(),
                    icon: "🔷".to_string(),
                    color: "bg-catppuccin-mauve/20 text-catppuccin-mauve border-catppuccin-mauve/30".to_string(),
                    description: ".NET ecosystem, enterprise applications".to_string(),
                },
                Skill {
                    name: "C++".to_string(),
                    icon: "⚙️".to_string(),
                    color: "bg-catppuccin-red/20 text-catppuccin-red border-catppuccin-red/30".to_string(),
                    description: "High-performance systems programming and game development".to_string(),
                },
                Skill {
                    name: "C".to_string(),
                    icon: "🔩".to_string(),
                    color: "bg-catppuccin-surface2/20 text-catppuccin-text border-catppuccin-surface2/30".to_string(),
                    description: "Low-level programming, embedded systems, kernel development".to_string(),
                },
                Skill {
                    name: "Elixir".to_string(),
                    icon: "💜".to_string(),
                    color: "bg-catppuccin-mauve/20 text-catppuccin-mauve border-catppuccin-mauve/30".to_string(),
                    description: "Concurrent, fault-tolerant functional programming".to_string(),
                },
                Skill {
                    name: "Solidity".to_string(),
                    icon: "⬟".to_string(),
                    color: "bg-catppuccin-peach/20 text-catppuccin-peach border-catppuccin-peach/30".to_string(),
                    description: "Smart contract development for Ethereum blockchain".to_string(),
                },
                Skill {
                    name: "Lua".to_string(),
                    icon: "🌙".to_string(),
                    color: "bg-catppuccin-lavender/20 text-catppuccin-lavender border-catppuccin-lavender/30".to_string(),
                    description: "Lightweight scripting, Neovim configuration, game modding".to_string(),
                },
            ],
        },
        SkillCategory {
            title: "Frontend & Web".to_string(),
            color: "catppuccin-sapphire".to_string(),
            icon: "🎨".to_string(),
            skills: vec![
                Skill {
                    name: "Leptos".to_string(),
                    icon: "🌸".to_string(),
                    color: "bg-catppuccin-pink/20 text-catppuccin-pink border-catppuccin-pink/30".to_string(),
                    description: "Rust-based reactive web framework compiled to WebAssembly".to_string(),
                },
                Skill {
                    name: "WebAssembly".to_string(),
                    icon: "🕸️".to_string(),
                    color: "bg-catppuccin-lavender/20 text-catppuccin-lavender border-catppuccin-lavender/30".to_string(),
                    description: "High-performance web applications with near-native speed".to_string(),
                },
                Skill {
                    name: "Svelte".to_string(),
                    icon: "🔥".to_string(),
                    color: "bg-catppuccin-peach/20 text-catppuccin-peach border-catppuccin-peach/30".to_string(),
                    description: "Compile-time optimized reactive framework".to_string(),
                },
                Skill {
                    name: "Nuxt".to_string(),
                    icon: "💚".to_string(),
                    color: "bg-catppuccin-green/20 text-catppuccin-green border-catppuccin-green/30".to_string(),
                    description: "Full-stack Vue.js framework with SSR and static generation".to_string(),
                },
                Skill {
                    name: "Astro".to_string(),
                    icon: "🚀".to_string(),
                    color: "bg-catppuccin-mauve/20 text-catppuccin-mauve border-catppuccin-mauve/30".to_string(),
                    description: "Content-focused web framework with island architecture".to_string(),
                },
                Skill {
                    name: "TailwindCSS".to_string(),
                    icon: "🎭".to_string(),
                    color: "bg-catppuccin-teal/20 text-catppuccin-teal border-catppuccin-teal/30".to_string(),
                    description: "Utility-first CSS framework for rapid UI development".to_string(),
                },
            ],
        },
        SkillCategory {
            title: "Backend & Runtime".to_string(),
            color: "catppuccin-green".to_string(),
            icon: "🗄️".to_string(),
            skills: vec![
                Skill {
                    name: "Node.js".to_string(),
                    icon: "🟢".to_string(),
                    color: "bg-catppuccin-green/20 text-catppuccin-green border-catppuccin-green/30".to_string(),
                    description: "Server-side JavaScript runtime with Express.js ecosystem".to_string(),
                },
                Skill {
                    name: "Bun".to_string(),
                    icon: "🥟".to_string(),
                    color: "bg-catppuccin-peach/20 text-catppuccin-peach border-catppuccin-peach/30".to_string(),
                    description: "Fast all-in-one JavaScript runtime, bundler, and package manager".to_string(),
                },
                Skill {
                    name: "Redis".to_string(),
                    icon: "🔴".to_string(),
                    color: "bg-catppuccin-red/20 text-catppuccin-red border-catppuccin-red/30".to_string(),
                    description: "In-memory data store for caching and real-time applications".to_string(),
                },
                Skill {
                    name: "SQLite".to_string(),
                    icon: "💾".to_string(),
                    color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire border-catppuccin-sapphire/30".to_string(),
                    description: "Lightweight embedded database for local storage and prototyping".to_string(),
                },
            ],
        },
        SkillCategory {
            title: "DevOps & Systems".to_string(),
            color: "catppuccin-mauve".to_string(),
            icon: "🛠️".to_string(),
            skills: vec![
                Skill {
                    name: "Docker/Podman".to_string(),
                    icon: "🐳".to_string(),
                    color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire border-catppuccin-sapphire/30".to_string(),
                    description: "Containerization for consistent deployment environments".to_string(),
                },
                Skill {
                    name: "Nginx".to_string(),
                    icon: "🌐".to_string(),
                    color: "bg-catppuccin-green/20 text-catppuccin-green border-catppuccin-green/30".to_string(),
                    description: "High-performance web server and reverse proxy".to_string(),
                },
                Skill {
                    name: "Linux".to_string(),
                    icon: "🐧".to_string(),
                    color: "bg-catppuccin-yellow/20 text-catppuccin-yellow border-catppuccin-yellow/30".to_string(),
                    description: "System administration, bash scripting, server management".to_string(),
                },
                Skill {
                    name: "NixOS".to_string(),
                    icon: "❄️".to_string(),
                    color: "bg-catppuccin-sapphire/20 text-catppuccin-sapphire border-catppuccin-sapphire/30".to_string(),
                    description: "Declarative Linux distribution with reproducible system configurations".to_string(),
                },
                Skill {
                    name: "Debian".to_string(),
                    icon: "🌀".to_string(),
                    color: "bg-catppuccin-red/20 text-catppuccin-red border-catppuccin-red/30".to_string(),
                    description: "Stable Linux distribution for servers and production environments".to_string(),
                },
                Skill {
                    name: "Ubuntu".to_string(),
                    icon: "🔶".to_string(),
                    color: "bg-catppuccin-peach/20 text-catppuccin-peach border-catppuccin-peach/30".to_string(),
                    description: "Popular Linux distribution for development and deployment".to_string(),
                },
                Skill {
                    name: "Caddy".to_string(),
                    icon: "🏎️".to_string(),
                    color: "bg-catppuccin-green/20 text-catppuccin-green border-catppuccin-green/30".to_string(),
                    description: "Modern web server with automatic HTTPS and easy configuration".to_string(),
                },
                Skill {
                    name: "Git".to_string(),
                    icon: "🌲".to_string(),
                    color: "bg-catppuccin-peach/20 text-catppuccin-peach border-catppuccin-peach/30".to_string(),
                    description: "Version control with GitHub workflows and collaboration".to_string(),
                },
                Skill {
                    name: "SSH".to_string(),
                    icon: "🔐".to_string(),
                    color: "bg-catppuccin-surface2/20 text-catppuccin-text border-catppuccin-surface2/30".to_string(),
                    description: "Secure remote access and server management".to_string(),
                },
            ],
        },
        SkillCategory {
            title: "Development Tools".to_string(),
            color: "catppuccin-lavender".to_string(),
            icon: "⚡".to_string(),
            skills: vec![
                Skill {
                    name: "Neovim".to_string(),
                    icon: "🌿".to_string(),
                    color: "bg-catppuccin-green/20 text-catppuccin-green border-catppuccin-green/30".to_string(),
                    description: "Highly customizable terminal-based editor with Vim motions".to_string(),
                },
                Skill {
                    name: "Ghostty".to_string(),
                    icon: "👻".to_string(),
                    color: "bg-catppuccin-lavender/20 text-catppuccin-lavender border-catppuccin-lavender/30".to_string(),
                    description: "Fast, modern terminal emulator with GPU acceleration".to_string(),
                },
                Skill {
                    name: "Kitty".to_string(),
                    icon: "🐱".to_string(),
                    color: "bg-catppuccin-pink/20 text-catppuccin-pink border-catppuccin-pink/30".to_string(),
                    description: "GPU-accelerated terminal emulator with advanced features".to_string(),
                },
            ],
        },
    ]
}

#[component]
pub fn SkillsSection() -> impl IntoView {
    let (skills_visible, set_skills_visible) = signal(false);

    // Handle scroll events for reveal animations
    Effect::new(move || {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let window_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);

                    if let Ok(Some(skills_section)) = document.query_selector("#skills h2") {
                        let rect = skills_section.get_bounding_client_rect();
                        if rect.top() < window_height * 0.75 && rect.bottom() > 0.0 {
                            set_skills_visible.set(true);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    });

    let categories = get_skill_categories();

    view! {
        <section id="skills" class="min-h-screen flex items-center px-4 py-20">
            <div class="max-w-6xl mx-auto">
                <h2 class={move || format!(
                    "text-4xl lg:text-6xl font-bold mb-4 text-center transition-all duration-800 font-mono {}",
                    if skills_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    <span class="text-catppuccin-sapphire terminal-glow-text">"skills"</span>
                    <span class="text-catppuccin-subtext1 text-xl md:text-2xl lg:text-3xl ml-2 md:ml-4">".rs"</span>
                </h2>
                
                <p class={move || format!(
                    "text-sm md:text-lg text-catppuccin-subtext1 text-center mb-8 md:mb-16 font-mono transition-all duration-800 delay-200 {}",
                    if skills_visible.get() { "reveal-on-scroll revealed" } else { "reveal-on-scroll" }
                )}>
                    "// Technologies and stacks I've worked with and learned"
                </p>

                <div class="space-y-8">
                    {categories.into_iter().enumerate().map(|(idx, category)| {
                        view! {
                            <div class={move || format!(
                                "max-w-4xl mx-auto bg-catppuccin-surface0/80 backdrop-blur border border-{}/30 rounded-lg p-4 md:p-6 hover:border-{}/60 transition-all duration-600 {}",
                                category.color,
                                category.color,
                                if skills_visible.get() { 
                                    format!("reveal-stagger revealed delay-{}", idx * 100) 
                                } else { 
                                    "reveal-stagger".to_string() 
                                }
                            )}>
                                <div class="flex items-center gap-3 mb-6">
                                    <div class="text-2xl">{category.icon}</div>
                                    <h3 class={format!("text-xl font-bold text-{} font-mono", category.color)}>
                                        {category.title}
                                    </h3>
                                </div>

                                <div class="flex flex-wrap gap-4 justify-center">
                                    {category.skills.into_iter().map(|skill| {
                                        let skill_name = skill.name.clone();
                                        let skill_description = skill.description.clone();
                                        
                                        view! {
                                            <div class={format!(
                                                "relative group cursor-help px-3 py-2 md:px-4 md:py-3 rounded-lg border {} hover:scale-105 transition-all duration-300 flex items-center gap-2 md:gap-3 min-w-[120px] md:min-w-[140px]", 
                                                skill.color
                                            )}
                                            title={skill_description.clone()}>
                                                <div class="text-xl md:text-2xl flex-shrink-0">{skill.icon}</div>
                                                <span class="text-xs md:text-sm font-mono font-medium whitespace-nowrap">
                                                    {skill_name.clone()}
                                                </span>
                                                
                                                // Tooltip
                                                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 hidden group-hover:block z-10">
                                                    <div class="bg-catppuccin-surface1 text-catppuccin-text text-xs rounded-lg p-3 shadow-lg border border-catppuccin-surface2 max-w-64 text-center">
                                                        <div class="font-semibold mb-1">{skill_name}</div>
                                                        <div class="text-catppuccin-subtext1">{skill_description.clone()}</div>
                                                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-catppuccin-surface1"></div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}