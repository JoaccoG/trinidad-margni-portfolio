use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="flex flex-col items-center justify-center min-h-screen gap-3 sm:gap-4 px-4 text-center">
            <h1 class="text-4xl font-bold sm:text-5xl md:text-6xl">"404"</h1>
            <p class="text-sm text-gray-400 sm:text-base">"Page not found"</p>
            <a href="/" class="text-sm text-blue-400 hover:underline sm:text-base">
                "← Go home"
            </a>
        </main>
    }
}
