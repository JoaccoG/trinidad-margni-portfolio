use leptos::prelude::*;

const COMPANIES: [(&str, &str); 6] = [
    ("YouTube", "/public/assets/images/companies/youtube.png"),
    (
        "United Airlines",
        "/public/assets/images/companies/united-airlines.png",
    ),
    ("VML", "/public/assets/images/companies/vml.png"),
    ("Monks", "/public/assets/images/companies/monks.png"),
    ("Nortal", "/public/assets/images/companies/nortal.png"),
    (
        "Snappy Commerce",
        "/public/assets/images/companies/snappy-commerce.png",
    ),
];

const REPEAT_COUNT: usize = 3;

#[component]
pub fn Companies() -> impl IntoView {
    let logo_set = || {
        view! {
            <div
                class="flex items-center gap-12 sm:gap-16 md:gap-20 pr-12 sm:pr-16 md:pr-20 shrink-0"
                aria-hidden="true"
            >
                {(0..REPEAT_COUNT)
                    .flat_map(|_| COMPANIES.iter().copied())
                    .map(|(name, src)| {
                        view! {
                            <img
                                src=src
                                alt=name
                                width="150"
                                height="40"
                                class="h-6 sm:h-8 md:h-10 w-auto object-contain brightness-0 invert shrink-0"
                            />
                        }
                    })
                    .collect_view()}
            </div>
        }
    };

    view! {
        <section
            class="bg-dark py-6 sm:py-8 md:py-10 overflow-hidden"
            aria-label="Companies I've worked with"
        >
            <div class="companies-track flex items-center w-max">{logo_set()} {logo_set()}</div>
        </section>
    }
}
