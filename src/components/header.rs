use leptos::prelude::*;
use web_sys::HtmlElement;

#[component]
#[allow(clippy::too_many_lines)]
pub fn Header() -> impl IntoView {
    const NAV_ITEMS: [(&str, &str); 4] = [
        ("Home", "#home"),
        ("About", "#about"),
        ("Certifications", "#certifications"),
        ("Contact", "#contact"),
    ];

    let (menu_open, set_menu_open) = signal(false);

    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };

    Effect::new(move |_| {
        let is_open = menu_open.get();
        let Some(win) = web_sys::window() else {
            return;
        };
        let Some(doc) = win.document() else {
            return;
        };
        let Some(body) = doc.body() else {
            return;
        };
        let body: &HtmlElement = body.as_ref();
        if is_open {
            let _ = body.style().set_property("overflow", "hidden");
        } else {
            let _ = body.style().set_property("overflow", "");
        }
    });

    let social_icons = || {
        view! {
            <div class="flex items-center gap-4">
                <a
                    href="https://www.linkedin.com/in/trinidadmargni"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:opacity-60 transition-opacity"
                    aria-label="LinkedIn"
                >
                    <img
                        src="/public/assets/icons/linkedin.svg"
                        alt="LinkedIn"
                        class="w-[18px] h-[18px] invert"
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
                        alt="Instagram"
                        class="w-[18px] h-[18px] invert"
                    />
                </a>
            </div>
        }
    };

    view! {
        <header class="w-full pt-8 pb-4 z-50 relative">
            <div class="flex flex-col items-center gap-4 px-4 sm:px-8">
                <div class="w-full flex items-center justify-center relative z-50">
                    <a
                        href="#home"
                        class="font-serif text-base sm:text-xl md:text-2xl tracking-[0.15em] sm:tracking-[0.3em] uppercase text-light px-12 md:px-0"
                    >
                        "Trinidad Margni"
                    </a>

                    <div class="hidden md:flex absolute right-0 items-center gap-4">
                        {social_icons()}
                    </div>

                    <button
                        type="button"
                        class="md:hidden absolute right-0 flex h-9 w-9 -translate-y-0.4 items-center justify-center text-light hover:opacity-80 cursor-pointer"
                        aria-expanded=move || menu_open.get().to_string()
                        aria-controls="mobile-menu"
                        aria-label=move || {
                            if menu_open.get() { "Close menu" } else { "Open menu" }
                        }
                        on:click=toggle_menu
                    >
                        {move || {
                            if menu_open.get() {
                                view! {
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        class="h-7 w-7 shrink-0"
                                        aria-hidden="true"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            stroke-linecap="round"
                                            d="M7 7l10 10M17 7L7 17"
                                        />
                                    </svg>
                                }
                            } else {
                                view! {
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        class="h-7 w-7 shrink-0"
                                        aria-hidden="true"
                                    >
                                        <path
                                            stroke="currentColor"
                                            stroke-width="1.5"
                                            stroke-linecap="round"
                                            d="M5 8h14M5 12h14M5 16h14"
                                        />
                                    </svg>
                                }
                            }
                        }}

                    </button>
                </div>

                <nav
                    class="hidden md:flex items-center gap-6 lg:gap-8"
                    aria-label="Main navigation"
                >
                    {NAV_ITEMS
                        .iter()
                        .copied()
                        .map(|(label, href)| {
                            view! {
                                <a
                                    href=href
                                    class="font-sans text-xs tracking-[0.2em] uppercase text-light hover:opacity-60 transition-opacity"
                                >
                                    {label}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
            </div>

            <div
                id="mobile-menu"
                class=move || {
                    let base = "fixed inset-0 z-40 flex md:hidden flex-col items-center justify-center gap-10 bg-dark/95 px-8 transition-opacity duration-300 ease-out";
                    if menu_open.get() {
                        format!("{base} opacity-100 pointer-events-auto")
                    } else {
                        format!("{base} opacity-0 pointer-events-none")
                    }
                }
                aria-hidden=move || (!menu_open.get()).to_string()
            >
                <nav
                    class=move || {
                        let base = "flex flex-col items-center gap-8 transition-[opacity,transform] duration-300 ease-out";
                        if menu_open.get() {
                            format!("{base} opacity-100 translate-y-0 delay-75")
                        } else {
                            format!("{base} opacity-0 translate-y-2 delay-0")
                        }
                    }
                    aria-label="Mobile navigation"
                >
                    {NAV_ITEMS
                        .iter()
                        .copied()
                        .map(|(label, href)| {
                            view! {
                                <a
                                    href=href
                                    class="font-sans text-xs tracking-[0.2em] uppercase text-light hover:opacity-60 transition-opacity sm:text-sm"
                                    on:click=move |_| {
                                        set_menu_open.set(false);
                                    }
                                >
                                    {label}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>

                <div class="flex items-center gap-6 pt-4">{social_icons()}</div>
            </div>
        </header>
    }
}
