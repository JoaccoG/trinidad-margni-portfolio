use leptos::prelude::*;

use crate::components::about::About;
use crate::components::companies::Companies;
use crate::components::hero::Hero;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
            <About />
            <Companies />
        </main>
    }
}
