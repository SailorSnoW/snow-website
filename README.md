# "SnOwOS" Website 🌸

My personal website/portfolio built with **Rust** and **WebAssembly**, showcasing a terminal/pastel/anime-inspired aesthetic with animated backgrounds and interactive components.

## 🚀 Tech Stack

- **[Leptos](https://leptos.dev/)** - Reactive web framework in Rust
- **WebAssembly** - High-performance client-side execution
- **TailwindCSS** - Utility-first styling with Catppuccin theme
- **Trunk** - Build tool for Rust/WASM applications

## ✨ Features

- **BIOS-style loading screen** with animated boot sequence
- **Interactive terminal modal** with custom commands
- **Dynamic star field background** with shooting stars
- **Responsive design** optimized for all devices
- **Terminal aesthetic** with neon glow effects
- **Smooth animations** and scroll-based reveals

## 🛠️ Development

```bash
# Install dependencies
cargo install trunk
rustup target add wasm32-unknown-unknown

# Development server
trunk serve --port 3000 --open

# Production build
trunk build --release
```

## 🎨 Design Philosophy

Combines the aesthetics of retro computing with modern web technologies, featuring:

- Terminal/BIOS inspired UI elements
- Catppuccin color palette
- Anime-inspired kawaii touches
- Clean, professional layout

## 📂 Project Structure

```
src/
├── components/          # Reusable UI components
│   ├── sections/        # Page sections (hero, about, skills, etc.)
│   └── ui/             # UI primitives (modals, badges, etc.)
├── constants.rs        # Application constants and configuration
├── pages/              # Page components
└── lib.rs              # Main application entry point
```

## 🌟 Key Components

- **Loading Screen**: BIOS-style boot sequence with customizable messages
- **Terminal Modal**: Interactive terminal with command support
- **Animated Background**: Dynamic star field with physics-based animations
- **Skills Section**: Categorized technology showcase
- **Contact Form**: JSON-styled contact information display

