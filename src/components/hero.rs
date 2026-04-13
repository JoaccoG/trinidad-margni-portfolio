use leptos::prelude::*;

use crate::components::header::Header;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="home" class="relative min-h-screen flex">
            <div class="absolute inset-0 flex z-0">
                <div class="absolute inset-0 md:relative md:inset-auto md:w-1/2 overflow-hidden">
                    <img
                        src="/public/assets/images/hero-pic.png"
                        alt="Trinidad Margni"
                        class="absolute inset-0 w-full h-full object-cover object-top"
                    />
                    <div class="absolute inset-0 bg-dark/50 md:bg-dark/20" />
                </div>
                <div class="hidden md:block md:w-1/2 bg-dark" />
            </div>

            <div class="relative z-10 flex flex-col w-full">
                <Header />

                <div class="flex-1 flex flex-col items-center justify-center px-4 sm:px-8 text-center gap-3 sm:gap-6">
                    <h1 class="font-serif text-2xl sm:text-3xl md:text-5xl lg:text-6xl xl:text-7xl uppercase leading-tight text-light max-w-4xl">
                        "Leading Projects That Align Teams, Drive Results, and Deliver Impact"
                    </h1>
                    <p class="font-display text-lg sm:text-xl md:text-2xl lg:text-3xl text-light/80">
                        "Senior Project Manager"
                    </p>
                    <a
                        href="#about"
                        class="mt-2 sm:mt-4 px-5 sm:px-8 py-2.5 sm:py-3 border border-light text-light font-sans text-xs tracking-[0.2em] uppercase hover:bg-light hover:text-dark transition-colors"
                    >
                        "About Me"
                    </a>
                </div>
            </div>
        </section>
    }
}
