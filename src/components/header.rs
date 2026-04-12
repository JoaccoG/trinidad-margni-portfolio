use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items: [(&str, &str); 5] = [
        ("Home", "#"),
        ("About", "#about"),
        ("Resume", "#resume"),
        ("Certifications", "#certifications"),
        ("Contact", "#contact"),
    ];

    view! {
        <header class="w-full pt-8 pb-4 z-20 relative">
            <div class="flex flex-col items-center gap-4 px-8">
                <div class="w-full flex items-center justify-center relative">
                    <a href="#" class="font-serif text-2xl tracking-[0.3em] uppercase text-light">
                        "Trinidad Margni"
                    </a>

                    <div class="absolute right-0 flex items-center gap-4">
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
                </div>

                <nav class="flex items-center gap-8">
                    {nav_items
                        .into_iter()
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
        </header>
    }
}
