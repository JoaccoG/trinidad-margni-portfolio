use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod data;
mod pages;
mod site_links;

use pages::certifications::CertificationsPage;
use pages::home::Home;
use pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />
        <Title text="Trinidad Margni — Senior Project Manager" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/certifications") view=CertificationsPage />
            </Routes>
        </Router>
    }
}
