use leptos::prelude::*;

use crate::components::hero::Hero;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
        </main>
    }
}
