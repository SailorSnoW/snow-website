# Justfile for snow-website (Rust + Leptos + Trunk + Cloudflare Pages)

# Default recipe
default:
    @just --list

# Install dependencies and tools
install:
    cargo install trunk wasm-bindgen-cli
    rustup target add wasm32-unknown-unknown

# Development server
dev:
    trunk serve --open

# Build for development
build:
    trunk build

# Build for production
build-release:
    trunk build --release

# Clean build artifacts
clean:
    trunk clean
    cargo clean
    rm -rf dist/

# Format code
fmt:
    cargo fmt

# Check code without building
check:
    cargo check

# Run clippy for linting
lint:
    cargo clippy -- -D warnings

# Fix clippy issues automatically
fix:
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt

# Watch and rebuild on file changes
watch:
    trunk watch

# Deploy to Cloudflare Pages (requires wrangler)
deploy: build-release
    wrangler deploy

# Preview deployment locally
preview: build-release
    wrangler dev

# Show project info
info:
    @echo "Project: snow-website"
    @echo "Framework: Leptos (Rust)"
    @echo "Build tool: Trunk"
    @echo "Target: wasm32-unknown-unknown"
    @echo "Deploy: Cloudflare Pages"

# Update dependencies
update:
    cargo update

# Audit dependencies for security issues
audit:
    cargo audit

# Full CI pipeline (check, lint, build)
ci: check lint build-release

# Quick development setup
setup: install
    @echo "Setup complete! Run 'just dev' to start development server"
