use leptos::prelude::*;

use crate::components::about::About;
use crate::components::certifications::Certifications;
use crate::components::companies::Companies;
use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::hero::Hero;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <Hero />
            <About />
            <Companies />
            <Certifications />
            <Contact />
            <Footer />
        </main>
    }
}
