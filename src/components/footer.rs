use leptos::prelude::*;
use leptos_router::components::A;
use time::OffsetDateTime;

use crate::site_links::NAV_ITEMS;

use super::social_nav_icons::SocialNavIcons;

fn footer_year() -> i32 {
    OffsetDateTime::now_utc().year()
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full border-t border-light/10 bg-dark text-light" role="contentinfo">
            <div class="mx-auto flex max-w-6xl flex-col items-center gap-6 px-4 pt-10 pb-6 text-center sm:pt-12 md:gap-8">
                <A
                    href="/"
                    attr:class="font-serif text-base tracking-[0.2em] uppercase text-light transition-opacity hover:opacity-60 sm:text-xl md:text-2xl"
                >
                    "Trinidad Margni"
                </A>

                <nav
                    class="flex flex-wrap items-center justify-center gap-x-6 gap-y-3 lg:gap-8"
                    aria-label="Footer navigation"
                >
                    {NAV_ITEMS
                        .iter()
                        .copied()
                        .map(|(label, href)| {
                            view! {
                                <A
                                    href=href
                                    attr:class="cursor-pointer font-sans text-xs tracking-[0.2em] uppercase text-light transition-opacity hover:opacity-60"
                                >
                                    {label}
                                </A>
                            }
                        })
                        .collect_view()}
                </nav>

                <SocialNavIcons centered=true />

                <p class="max-w-md font-sans text-[0.65rem] leading-relaxed tracking-wide text-light/55 sm:text-xs">
                    "Designed & Built by "
                    <a
                        href="https://joaquingodoy.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="cursor-pointer text-light/70 transition-colors hover:text-light"
                    >
                        "Joaquín Godoy"
                    </a> " © " {footer_year()}
                </p>
            </div>
        </footer>
    }
}
