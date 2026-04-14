use leptos::html::Div;
use leptos::prelude::*;
use web_sys::HtmlElement;

#[component]
#[allow(clippy::too_many_lines)]
pub fn CertModal(state: RwSignal<Option<(String, String)>>) -> impl IntoView {
    let dialog_ref = NodeRef::<Div>::new();
    let image_failed = RwSignal::new(false);

    Effect::new(move |_| {
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
        if state.get().is_some() {
            let _ = body.style().set_property("overflow", "hidden");
        } else {
            let _ = body.style().set_property("overflow", "");
        }
    });

    Effect::new(move |_| {
        if state.get().is_none() {
            return;
        }
        image_failed.set(false);
        if let Some(el) = dialog_ref.get() {
            let _ = el.focus();
        }
    });

    on_cleanup(move || {
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
        let _ = body.style().set_property("overflow", "");
    });

    let close = move || {
        state.set(None);
    };

    view! {
        <Show when=move || state.get().is_some()>
            <div class="cert-modal-enter fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6">
                <button
                    type="button"
                    class="absolute inset-0 bg-dark/80 backdrop-blur-sm transition-opacity"
                    aria-label="Close certificate viewer"
                    on:click=move |_| close()
                />
                <div
                    node_ref=dialog_ref
                    role="dialog"
                    aria-modal="true"
                    aria-label="Certificate image"
                    tabindex="-1"
                    class="cert-modal-active relative z-10 flex max-h-[90vh] max-w-[min(100%,56rem)] flex-col gap-4 outline-none"
                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                        if ev.key() == "Escape" {
                            ev.prevent_default();
                            close();
                        }
                    }
                >
                    <button
                        type="button"
                        class="absolute -right-1 -top-10 z-20 flex h-9 w-9 items-center justify-center border border-light/60 bg-dark/40 font-sans text-lg text-light transition-opacity hover:opacity-80 sm:right-0 sm:top-0 sm:bg-transparent sm:text-2xl"
                        aria-label="Close"
                        on:click=move |_| close()
                    >
                        "×"
                    </button>
                    {move || {
                        state
                            .get()
                            .map(|(src, alt)| {
                                view! {
                                    <Show
                                        when=move || !image_failed.get()
                                        fallback=move || {
                                            view! {
                                                <p class="rounded-sm border border-light/30 bg-dark/60 px-8 py-12 text-center font-sans text-sm text-light">
                                                    "Unable to load this certificate image."
                                                </p>
                                            }
                                        }
                                    >
                                        <img
                                            src=src.clone()
                                            alt=alt.clone()
                                            class="max-h-[85vh] max-w-[90vw] rounded-sm object-contain shadow-lg"
                                            loading="eager"
                                            decoding="async"
                                            on:error=move |_ev: leptos::ev::ErrorEvent| {
                                                image_failed.set(true);
                                            }
                                        />
                                    </Show>
                                }
                            })
                    }}

                </div>
            </div>
        </Show>
    }
}
