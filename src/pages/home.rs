use leptos::prelude::*;

use crate::components::about::About;
use crate::components::hero::Hero;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
            <About />
        </main>
    }
}
