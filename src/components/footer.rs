use leptos::prelude::*;
use time::OffsetDateTime;

use crate::site_links::{HOME_LOGO_HREF, NAV_ITEMS};

use super::social_nav_icons::SocialNavIcons;

fn footer_year() -> i32 {
    OffsetDateTime::now_utc().year()
}

#[component]
pub fn Footer(#[prop(default = false)] is_dark: bool) -> impl IntoView {
    let (footer_cls, name_cls, nav_cls, credit_cls, credit_link_cls) = if is_dark {
        (
            "w-full border-t border-light/10 bg-dark text-light",
            "cursor-pointer font-serif text-base tracking-[0.2em] uppercase text-light transition-opacity hover:opacity-60 sm:text-xl md:text-2xl",
            "cursor-pointer font-sans text-xs tracking-[0.2em] uppercase text-light transition-opacity hover:opacity-60",
            "max-w-md font-sans text-[0.65rem] leading-relaxed tracking-wide text-light/55 sm:text-xs",
            "cursor-pointer text-light/70 transition-colors hover:text-light",
        )
    } else {
        (
            "w-full border-t border-dark/10 bg-primary text-dark",
            "cursor-pointer font-serif text-base tracking-[0.2em] uppercase text-dark transition-opacity hover:opacity-60 sm:text-xl md:text-2xl",
            "cursor-pointer font-sans text-xs tracking-[0.2em] uppercase text-dark transition-opacity hover:opacity-60",
            "max-w-md font-sans text-[0.65rem] leading-relaxed tracking-wide text-dark/55 sm:text-xs",
            "cursor-pointer text-dark/70 transition-colors hover:text-dark",
        )
    };

    view! {
        <footer class=footer_cls role="contentinfo">
            <div class="mx-auto flex max-w-6xl flex-col items-center gap-6 px-4 pt-10 pb-6 text-center sm:pt-12 md:gap-4">
                <a href=HOME_LOGO_HREF class=name_cls>
                    "Trinidad Margni"
                </a>

                <nav
                    class="flex flex-wrap items-center justify-center gap-x-6 gap-y-3 lg:gap-8"
                    aria-label="Footer navigation"
                >
                    {NAV_ITEMS
                        .iter()
                        .copied()
                        .map(|(label, href)| {
                            view! {
                                <a href=href class=nav_cls>
                                    {label}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>

                <SocialNavIcons is_centered=true is_inverted=is_dark />

                <p class=credit_cls>
                    "Designed & Built by "
                    <a
                        href="https://joaquingodoy.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class=credit_link_cls
                    >
                        "Joaquín Godoy"
                    </a> " © " {footer_year()}
                </p>
            </div>
        </footer>
    }
}
