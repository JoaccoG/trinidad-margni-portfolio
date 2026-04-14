use std::rc::Rc;
use std::sync::Arc;

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::{NavigateOptions, hooks::use_navigate, hooks::use_query_map};

use crate::components::cert_card::CertCard;
use crate::components::cert_modal::CertModal;
use crate::components::footer::Footer;
use crate::data::certifications::{Certification, certification_store};

type FilterFn = dyn Fn(Option<&str>);

#[component]
#[allow(clippy::too_many_lines)]
pub fn CertificationsPage() -> impl IntoView {
    let store = certification_store();
    let query = use_query_map();
    let navigate = use_navigate();

    let active_filter = Memo::new(move |_| {
        query
            .get()
            .get_str("category")
            .filter(|c| store.is_valid_category(c))
            .map(std::string::ToString::to_string)
    });

    let filtered_certs = Memo::new(move |_| {
        active_filter
            .get()
            .map(|cat_id| {
                store
                    .certifications_for_category(&cat_id)
                    .into_iter()
                    .cloned()
                    .collect::<Vec<Certification>>()
            })
            .unwrap_or_default()
    });

    let modal_state = RwSignal::new(None::<(String, String)>);
    let on_open_modal = Callback::new(move |(src, alt): (String, String)| {
        modal_state.set(Some((src, alt)));
    });

    let navigate_filter: Rc<FilterFn> = Rc::new({
        let navigate = navigate.clone();
        move |next: Option<&str>| {
            let opts = NavigateOptions {
                replace: true,
                scroll: false,
                ..Default::default()
            };
            match next {
                None => navigate("/certifications", opts),
                Some(c) if store.is_valid_category(c) => {
                    navigate(&format!("/certifications?category={c}"), opts);
                }
                Some(_) => {}
            }
        }
    });

    view! {
        <>
            <Title text="Certifications — Trinidad Margni" />
            <div class="min-h-screen bg-secondary text-dark">
                <div class="border-b border-dark/10 bg-light/40 px-4 py-4 sm:px-8">
                    <div class="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-4">
                        <A href="/">
                            <span class="font-serif text-sm tracking-[0.2em] uppercase text-dark hover:opacity-70 sm:text-base">
                                "Trinidad Margni"
                            </span>
                        </A>
                        <A href="/">
                            <span class="font-sans text-xs tracking-[0.2em] text-dark uppercase underline decoration-dark/30 underline-offset-4 transition-colors hover:decoration-dark">
                                "← Back to portfolio"
                            </span>
                        </A>
                    </div>
                </div>

                <section class="px-4 py-12 sm:px-8 lg:px-16" aria-labelledby="all-certs-heading">
                    <header class="mx-auto mb-10 max-w-6xl space-y-3 text-center sm:mb-12">
                        <p class="font-display text-xl sm:text-2xl md:text-4xl">
                            "always learning"
                        </p>
                        <h1
                            id="all-certs-heading"
                            class="font-serif text-2xl uppercase leading-tight tracking-wide text-balance sm:text-3xl md:text-4xl lg:text-5xl"
                        >
                            "All certifications"
                        </h1>
                        <p class="mx-auto max-w-2xl font-sans text-xs leading-relaxed text-dark/85 sm:text-sm md:text-base">
                            "Explore my complete list of certifications across all categories and disciplines."
                        </p>
                    </header>

                    <div class="cert-tabs-scroll mx-auto mb-10 flex max-w-6xl flex-wrap gap-2 pb-1">
                        <button
                            type="button"
                            class=move || {
                                let base = "rounded-full border px-4 py-2 font-sans text-[0.65rem] tracking-wide uppercase transition-colors sm:text-xs";
                                if active_filter.get().is_none() {
                                    format!("{base} border-dark bg-dark text-light")
                                } else {
                                    format!(
                                        "{base} border-dark/30 bg-light/60 text-dark hover:border-dark/60",
                                    )
                                }
                            }
                            aria-pressed=move || active_filter.get().is_none().to_string()
                            on:click={
                                let navigate_filter = navigate_filter.clone();
                                move |_| navigate_filter(None)
                            }
                        >
                            "All"
                        </button>
                        {store
                            .categories
                            .iter()
                            .map(|cat| {
                                let id_cmp = cat.id.clone();
                                let id_aria = cat.id.clone();
                                let id_click = cat.id.clone();
                                let name = cat.name.clone();
                                let navigate_filter = navigate_filter.clone();
                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let base = "rounded-full border px-4 py-2 font-sans text-[0.65rem] tracking-wide uppercase transition-colors sm:text-xs";
                                            let is_on = active_filter.get().as_deref()
                                                == Some(id_cmp.as_str());
                                            if is_on {
                                                format!("{base} border-dark bg-dark text-light")
                                            } else {
                                                format!(
                                                    "{base} border-dark/30 bg-light/60 text-dark hover:border-dark/60",
                                                )
                                            }
                                        }
                                        aria-pressed=move || {
                                            (active_filter.get().as_deref() == Some(id_aria.as_str()))
                                                .to_string()
                                        }
                                        on:click=move |_| {
                                            navigate_filter(Some(id_click.as_str()));
                                        }
                                    >
                                        {name}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>

                    <div class="mx-auto max-w-6xl">
                        <Show when=move || {
                            active_filter.get().is_some()
                        }>
                            {move || {
                                view! {
                                    <Show
                                        when=move || filtered_certs.with(Vec::is_empty)
                                        fallback=move || {
                                            view! {
                                                <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-3 lg:grid-cols-3 lg:gap-3 xl:grid-cols-4 xl:gap-3">
                                                    {move || {
                                                        filtered_certs
                                                            .get()
                                                            .into_iter()
                                                            .map(|cert| {
                                                                view! { <CertCard cert=cert on_open=on_open_modal /> }
                                                            })
                                                            .collect_view()
                                                    }}
                                                </div>
                                            }
                                        }
                                    >
                                        <p class="font-sans text-sm text-dark/80">
                                            "Coming soon — certifications in this category will appear here."
                                        </p>
                                    </Show>
                                }
                            }}

                        </Show>
                        <Show when=move || {
                            active_filter.get().is_none()
                        }>
                            {move || {
                                view! {
                                    <div class="flex flex-col gap-14">
                                        {store
                                            .categories
                                            .iter()
                                            .map(|cat| {
                                                let cat_id = cat.id.clone();
                                                let cat_name = cat.name.clone();
                                                let cat_id_empty = cat_id.clone();
                                                let cat_name_empty = cat_name.clone();
                                                let certs = Arc::new(
                                                    store
                                                        .certifications_for_category(&cat_id)
                                                        .into_iter()
                                                        .cloned()
                                                        .collect::<Vec<Certification>>(),
                                                );
                                                let certs_check = certs.clone();
                                                view! {
                                                    <Show
                                                        when=move || certs_check.is_empty()
                                                        fallback=move || {
                                                            let certs = certs.clone();
                                                            let cat_id_fb = cat_id.clone();
                                                            let cat_name_fb = cat_name.clone();
                                                            view! {
                                                                <section
                                                                    class="space-y-5"
                                                                    aria-labelledby=format!("cat-{cat_id_fb}")
                                                                >
                                                                    <h2
                                                                        id=format!("cat-{cat_id_fb}")
                                                                        class="flex items-center gap-3 font-serif text-lg uppercase tracking-wide sm:text-xl"
                                                                    >
                                                                        {cat_name_fb}
                                                                        <span class="h-px min-w-[2rem] flex-1 bg-dark/25" />
                                                                    </h2>
                                                                    <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-3 lg:grid-cols-3 lg:gap-3 xl:grid-cols-4 xl:gap-3">
                                                                        {certs
                                                                            .iter()
                                                                            .cloned()
                                                                            .map(|cert| {
                                                                                view! { <CertCard cert=cert on_open=on_open_modal /> }
                                                                            })
                                                                            .collect_view()}
                                                                    </div>
                                                                </section>
                                                            }
                                                        }
                                                    >
                                                        <section
                                                            class="space-y-3"
                                                            aria-labelledby=format!("cat-{cat_id_empty}")
                                                        >
                                                            <h2
                                                                id=format!("cat-{cat_id_empty}")
                                                                class="flex items-center gap-3 font-serif text-lg uppercase tracking-wide sm:text-xl"
                                                            >
                                                                {cat_name_empty.clone()}
                                                                <span class="h-px min-w-[2rem] flex-1 bg-dark/25" />
                                                            </h2>
                                                            <p class="font-sans text-sm text-dark/70">"Coming soon."</p>
                                                        </section>
                                                    </Show>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_view()
                            }}

                        </Show>
                    </div>
                </section>
                <Footer />
            </div>
            <CertModal state=modal_state />
        </>
    }
}
