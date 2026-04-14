use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::cert_card::CertCard;
use crate::components::cert_modal::CertModal;
use crate::data::certifications::certification_store;

#[component]
#[allow(clippy::too_many_lines)]
pub fn Certifications() -> impl IntoView {
    let store = certification_store();
    let total_certifications = store.certifications.len();
    let first_id = store
        .categories
        .first()
        .map(|c| c.id.clone())
        .unwrap_or_default();
    let active_category = RwSignal::new(first_id);
    let modal_state = RwSignal::new(None::<(String, String)>);

    let featured_for_active = Memo::new(move |_| {
        let cat = active_category.get();
        store
            .featured_for_category(&cat)
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    let on_open_modal = Callback::new(move |(src, alt): (String, String)| {
        modal_state.set(Some((src, alt)));
    });

    view! {
        <section
            id="certifications"
            class="bg-secondary text-dark py-16 md:py-24 px-4 sm:px-8 lg:px-16"
            aria-labelledby="certifications-heading"
        >
            <div class="mx-auto max-w-6xl">
                <header class="mb-10 space-y-2 text-center sm:mb-12 sm:space-y-3">
                    <p class="font-display text-xl sm:text-2xl md:text-4xl">"always learning"</p>
                    <h2
                        id="certifications-heading"
                        class="font-serif text-2xl uppercase leading-tight tracking-wide text-balance sm:text-xl md:text-2xl lg:text-3xl xl:text-4xl"
                    >
                        "Certifications"
                    </h2>
                </header>

                <div class="grid grid-cols-1 gap-10 md:grid-cols-[minmax(0,13rem)_minmax(0,1fr)] md:gap-12 lg:gap-14">
                    <nav
                        class="cert-tabs-scroll mb-2 flex min-w-0 flex-row gap-2 overflow-x-auto pb-2 md:mb-0 md:flex-col md:gap-1 md:overflow-visible md:pb-0"
                        role="tablist"
                        aria-label="Certification categories"
                    >
                        {store
                            .categories
                            .iter()
                            .map(|cat| {
                                let id_for_tab = cat.id.clone();
                                let id_for_selected = id_for_tab.clone();
                                let id_for_class = id_for_tab.clone();
                                let id_for_click = id_for_tab.clone();
                                let name = cat.name.clone();
                                view! {
                                    <button
                                        type="button"
                                        role="tab"
                                        id=format!("cert-tab-{id_for_tab}")
                                        aria-controls="certifications-panel"
                                        aria-selected=move || {
                                            (active_category.get() == id_for_selected).to_string()
                                        }
                                        class=move || {
                                            let mobile = "cursor-pointer shrink-0 rounded-full border px-4 py-2 text-left font-sans text-[0.65rem] tracking-wide uppercase transition-colors md:rounded-none md:border-0 md:px-0 md:py-3 md:pl-4 md:pr-2 md:text-xs";
                                            if active_category.get() == id_for_class {
                                                format!(
                                                    "{mobile} cert-tab-active border-dark bg-dark text-light md:border-l-2 md:border-dark md:bg-transparent md:font-semibold md:text-dark",
                                                )
                                            } else {
                                                format!(
                                                    "{mobile} border-dark/30 bg-light/50 text-dark hover:border-dark/60 md:border-l-2 md:border-transparent md:bg-transparent md:text-dark/70 md:hover:border-dark/40 md:hover:text-dark",
                                                )
                                            }
                                        }
                                        on:click=move |_| active_category.set(id_for_click.clone())
                                    >
                                        {name}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </nav>

                    <div
                        id="certifications-panel"
                        role="tabpanel"
                        aria-labelledby=move || format!("cert-tab-{}", active_category.get())
                        class="min-w-0"
                    >
                        <Show
                            when=move || featured_for_active.with(Vec::is_empty)
                            fallback=move || {
                                view! {
                                    <div class="cert-home-featured-grid grid grid-cols-1 gap-3 max-[480px]:gap-2 sm:grid-cols-2 sm:gap-3 lg:grid-cols-3 lg:gap-4">
                                        {move || {
                                            featured_for_active
                                                .get()
                                                .into_iter()
                                                .map(|cert| {
                                                    view! { <CertCard cert=cert on_open=on_open_modal /> }
                                                })
                                                .collect_view()
                                        }}
                                    </div>
                                    <div class="mt-8 flex flex-col items-start gap-6 sm:flex-row sm:items-center sm:justify-between">
                                        <A
                                            href=move || {
                                                format!(
                                                    "/certifications?category={}",
                                                    active_category.get(),
                                                )
                                            }
                                            attr:class="font-sans text-xs tracking-wide text-dark underline decoration-dark/30 underline-offset-4 transition-colors hover:decoration-dark"
                                        >
                                            {move || {
                                                let cat = active_category.get();
                                                let total = store.count_for_category(&cat);
                                                format!("See all {total} certifications →")
                                            }}
                                        </A>
                                    </div>
                                }
                            }
                        >
                            {move || {
                                view! {
                                    <div class="space-y-4">
                                        <p class="font-sans text-sm text-dark/80">
                                            "No featured certifications in this category yet."
                                        </p>
                                        <A
                                            href=move || {
                                                format!(
                                                    "/certifications?category={}",
                                                    active_category.get(),
                                                )
                                            }
                                            attr:class="inline-flex font-sans text-xs tracking-wide text-dark underline decoration-dark/30 underline-offset-4 transition-colors hover:decoration-dark"
                                        >
                                            {move || {
                                                let cat = active_category.get();
                                                let total = store.count_for_category(&cat);
                                                format!("See all {total} certifications →")
                                            }}
                                        </A>
                                    </div>
                                }
                            }}

                        </Show>
                    </div>
                </div>

                <div class="mt-10 flex justify-center border-t border-dark/10 pt-10">
                    <A
                        href="/certifications"
                        attr:class="inline-flex border border-dark bg-dark px-6 py-3 font-sans text-xs tracking-[0.2em] text-light uppercase transition-colors hover:opacity-90"
                    >
                        {format!("Explore all certifications ({total_certifications})")}
                    </A>
                </div>
            </div>
            <CertModal state=modal_state />
        </section>
    }
}
