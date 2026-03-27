use leptos::prelude::*;
use leptos_meta::{Html, Meta, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod pages;

use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />
        <Title text="Trinidad Margni — Senior Project Manager" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
