use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="flex flex-col items-center justify-center min-h-screen gap-4">
            <h1 class="text-6xl font-bold">"404"</h1>
            <p class="text-gray-400">"Page not found"</p>
            <a href="/" class="text-blue-400 hover:underline">
                "← Go home"
            </a>
        </main>
    }
}
