use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

use crate::data::certifications::Certification;

const CERT_IMAGE_FALLBACK_SRC: &str = "/public/assets/certs/placeholder.svg";

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn CertCard(
    cert: Certification,
    #[prop(into)] on_open: Callback<(String, String)>,
) -> impl IntoView {
    let alt_for_modal = format!("{} — {}", cert.name, cert.issuer);
    let cert_image = cert.image.clone();
    let aria_label = format!(
        "View certificate: {} — {}, {}",
        cert.name, cert.issuer, cert.year
    );

    view! {
        <button
            type="button"
            class="group relative aspect-[14/9] w-full min-h-36 min-w-[min(100%,14rem)] cursor-pointer overflow-hidden rounded-sm bg-dark text-left shadow-sm transition-shadow hover:shadow-md focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-dark max-[480px]:aspect-[5/3] max-[480px]:min-h-28 max-[480px]:min-w-0 max-[480px]:rounded-[2px]"
            aria-label=aria_label
            on:click=move |_| {
                on_open.run((cert_image.clone(), alt_for_modal.clone()));
            }
        >
            <div class="absolute inset-0 isolate overflow-hidden rounded-sm">
                <img
                    src=cert.image.clone()
                    alt=""
                    width="560"
                    height="360"
                    class="absolute inset-[-1px] box-border h-[calc(100%+2px)] w-[calc(100%+2px)] max-w-none origin-center backface-hidden transform-gpu object-cover object-top transition-transform duration-200 ease-out motion-reduce:transition-none motion-reduce:group-hover:scale-100 group-hover:scale-[1.025]"
                    loading="lazy"
                    decoding="async"
                    aria-hidden="true"
                    on:error=move |ev: leptos::ev::ErrorEvent| {
                        if let Some(t) = ev.target()
                            && let Ok(img) = t.dyn_into::<HtmlImageElement>()
                        {
                            img.set_src(CERT_IMAGE_FALLBACK_SRC);
                        }
                    }
                />
            </div>
            <div
                class="pointer-events-none absolute inset-x-0 bottom-0 flex flex-col justify-end bg-gradient-to-t from-dark/95 via-dark/65 to-transparent pb-2 pl-2 pr-2 pt-12 max-[480px]:pb-1.5 max-[480px]:pl-1.5 max-[480px]:pr-1.5 max-[480px]:pt-9 sm:pb-2.5 sm:pl-2.5 sm:pr-2.5 sm:pt-14"
                aria-hidden="true"
            >
                <p class="line-clamp-2 font-serif text-xs font-semibold leading-snug pb-1 text-balance text-light max-[480px]:pb-0.5 max-[480px]:text-[0.65rem] max-[480px]:leading-tight sm:text-sm">
                    {cert.name.clone()}
                </p>
                <div class="flex min-h-0 items-baseline justify-between gap-2 border-t border-light/15 pt-1 font-sans text-[0.55rem] tracking-wide text-light/90 uppercase max-[480px]:gap-1 max-[480px]:pt-0.5 max-[480px]:text-[0.5rem] sm:pt-1.5 sm:text-[0.6rem]">
                    <span class="min-w-0 truncate">{cert.issuer.clone()}</span>
                    <span class="shrink-0 tabular-nums">{cert.year}</span>
                </div>
            </div>
        </button>
    }
}
