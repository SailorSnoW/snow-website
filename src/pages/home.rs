use crate::components::{
    layout::navigation::Navigation,
    sections::{
        about::AboutSection, contact::ContactSection, hero::HeroSection, projects::ProjectsSection,
        skills::SkillsSection,
    },
    ui::footer::Footer,
};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen text-catppuccin-text relative overflow-x-hidden">
            // Animated background stars
            <div class="absolute inset-0 bg-stars opacity-60"></div>

            // Additional background elements for vibrancy
            <div class="absolute inset-0 opacity-20">
                <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-catppuccin-pink/10 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute bottom-1/3 right-1/4 w-80 h-80 bg-catppuccin-mauve/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
                <div class="absolute top-1/2 right-1/3 w-64 h-64 bg-catppuccin-sapphire/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 4s;"></div>
            </div>

            // Navigation component
            <Navigation />

            // Main content
            <main class="relative z-10 pt-4">
                <HeroSection />
                <AboutSection />
                <SkillsSection />
                <ProjectsSection />
                <ContactSection />
                <Footer />
            </main>
        </div>
    }
}
