use leptos::prelude::*;
use leptos_router::components::A;

#[cfg(target_arch = "wasm32")]
fn footer_year() -> i32 {
    i32::try_from(js_sys::Date::new_0().get_full_year()).unwrap_or(2026)
}

#[cfg(not(target_arch = "wasm32"))]
fn footer_year() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    const SECS_PER_DAY: u64 = 86_400;
    let Ok(d) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 2026;
    };
    let days = d.as_secs() / SECS_PER_DAY;
    let years_since_1970 = i32::try_from(days / 365).unwrap_or(0);
    1970 + years_since_1970
}

#[component]
pub fn Footer() -> impl IntoView {
    const NAV_ITEMS: [(&str, &str); 4] = [
        ("Home", "/#home"),
        ("About", "/#about"),
        ("Certifications", "/#certifications"),
        ("Contact", "/#contact"),
    ];

    let social_icons = || {
        view! {
            <div class="flex items-center justify-center gap-4">
                <a
                    href="https://www.linkedin.com/in/trinidadmargni"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:opacity-60 transition-opacity"
                    aria-label="LinkedIn"
                >
                    <img
                        src="/public/assets/icons/linkedin.svg"
                        alt=""
                        class="h-[18px] w-[18px] invert"
                    />
                </a>
                <a
                    href="https://www.instagram.com/trinidadmargni"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:opacity-60 transition-opacity"
                    aria-label="Instagram"
                >
                    <img
                        src="/public/assets/icons/instagram.svg"
                        alt=""
                        class="h-[18px] w-[18px] invert"
                    />
                </a>
            </div>
        }
    };

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

                {social_icons()}

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
