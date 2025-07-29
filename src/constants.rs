//! Application-wide constants for the SnoW website
//! This module centralizes all magic numbers, configuration values, and constants
//! to improve maintainability and reduce code duplication.

/// Animation durations (in milliseconds)
pub const TRANSITION_DURATION_FAST: u64 = 300;
pub const TRANSITION_DURATION_MEDIUM: u64 = 600;
pub const TRANSITION_DURATION_SLOW: u64 = 800;

// Loading screen configuration
pub const LOADING_ENTER_SKIP_DELAY: u64 = 800;
pub const LOADING_TRANSITION_DELAY: u64 = 800;

// Star animation configuration
pub const FLOATING_STARS_COUNT: u32 = 80;
pub const SHOOTING_STARS_COUNT: u32 = 4;
pub const STAR_MIN_SIZE: f64 = 0.5;
pub const STAR_MAX_SIZE: f64 = 2.0;
pub const STAR_MIN_OPACITY: f64 = 0.1;
pub const STAR_MAX_OPACITY: f64 = 0.5;
pub const STAR_MIN_DURATION: f64 = 4.0;
pub const STAR_MAX_DURATION: f64 = 10.0;
pub const STAR_MAX_DELAY: f64 = 8.0;

// Shooting star configuration
pub const SHOOTING_STAR_MIN_DURATION: f64 = 1.5;
pub const SHOOTING_STAR_MAX_DURATION: f64 = 5.5;
pub const SHOOTING_STAR_MAX_DELAY: f64 = 20.0;
pub const SHOOTING_STAR_MIN_TRAIL: f64 = 10.0;
pub const SHOOTING_STAR_MAX_TRAIL: f64 = 50.0;
pub const SHOOTING_STAR_POSITION_RANGE: f64 = 120.0;
pub const SHOOTING_STAR_POSITION_OFFSET: f64 = -10.0;

// Terminal configuration
pub const TERMINAL_HEIGHT: &str = "h-96";
pub const TERMINAL_PROMPT_USER: &str = "SnoW";
pub const TERMINAL_PROMPT_HOSTNAME: &str = "snOwOS";
pub const TERMINAL_PROMPT_PATH: &str = "~";

// Animation reveal thresholds
pub const REVEAL_THRESHOLD: f64 = 0.75;

// Color variants for shooting stars
pub const SHOOTING_STAR_COLORS: &[&str] = &[
    "from-catppuccin-pink/40 via-catppuccin-mauve/30 to-transparent",
    "from-catppuccin-sapphire/40 via-catppuccin-sky/30 to-transparent", 
    "from-catppuccin-mauve/40 via-catppuccin-pink/30 to-transparent",
];

// Personal information constants
pub const PERSONAL_INFO: PersonalInfo = PersonalInfo {
    name: "Loïs / SnoW",
    title: "Backend-Oriented Developer ✨",
    subtitle: "Backend Engineer • Music Tech Innovator 🎵 • Anime Enthusiast 🌸",
    description: "25 years old, passionate backend engineer / EDM enjoyer. Welcome to my digital realm! 🚀 (I use vim btw)",
    age: 25,
};

pub struct PersonalInfo {
    pub name: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub description: &'static str,
    pub age: u8,
}

// Contact information
pub const CONTACT_INFO: ContactInfo = ContactInfo {
    email: "sailorsnow@pm.me",
    github: "@sailorsnow",
    linkedin: "in/llagoutte",
    discord: "snxwin",
    message: "Always open to discussing new opportunities and collaborations.",
};

pub struct ContactInfo {
    pub email: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub discord: &'static str,
    pub message: &'static str,
}

// Boot sequence messages for loading screen
pub const BOOT_MESSAGES: &[(&str, u64)] = &[
    ("BIOS v2.1.4 - SnoW Systems Inc.", 200),
    ("CPU: Intel Core i7-13700K @ 3.40GHz ✨", 100),
    ("Memory: 32GB DDR5 💖", 80),
    ("", 250),
    ("Initializing protocols...", 400),
    ("Loading uwu_graphics.dll... [ OK ]", 300),
    ("Loading nekoXD.dll... [ OK ]", 200),
    ("", 250),
    ("Connecting to the internet... 📡", 500),
    ("", 250),
    ("Starting global system...", 400),
    ("Loading developer profile... [ OK ]", 300),
    ("", 200),
    ("Welcome to SnoW's Digital Realm! 🌟", 400),
];

// Terminal welcome messages
pub const TERMINAL_WELCOME: &[&str] = &[
    "🌸 Welcome to the SnOwOS terminal! 🌸",
    "Type 'help' to see available commands ✨",
];

// CSS class constants for consistency
pub mod css {
    pub const TERMINAL_OVERLAY: &str = "fixed inset-0 z-50 terminal-overlay text-green-400 font-mono text-sm p-4 overflow-hidden flex flex-col cursor-none";
    pub const TERMINAL_TRANSITION: &str = "terminal-transition";
    pub const TERMINAL_EXIT: &str = "terminal-exit";
    pub const REVEAL_ON_SCROLL: &str = "reveal-on-scroll";
    pub const REVEAL_STAGGER: &str = "reveal-stagger";
    pub const TERMINAL_GLOW_TEXT: &str = "terminal-glow-text";
    pub const CATPPUCCIN_SURFACE: &str = "bg-catppuccin-surface0/80 backdrop-blur";
}