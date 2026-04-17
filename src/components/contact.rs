use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

const RATE_LIMIT_KEY: &str = "contact_timestamps";
const RATE_LIMIT_MAX: usize = 2;
const RATE_LIMIT_WINDOW_MS: f64 = 60_000.0;
const SUCCESS_RESET_MS: i32 = 3_000;

#[derive(Clone, PartialEq, Eq)]
enum FormState {
    Idle,
    Submitting,
    Success,
}

fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn read_timestamps() -> Vec<f64> {
    let result: Option<Vec<f64>> = (|| {
        let storage = get_storage()?;
        let raw = storage.get_item(RATE_LIMIT_KEY).ok()??;
        serde_json::from_str::<Vec<f64>>(&raw).ok()
    })();
    result.unwrap_or_default()
}

fn is_rate_limited() -> bool {
    let now = js_sys::Date::now();
    let cutoff = now - RATE_LIMIT_WINDOW_MS;
    let recent = read_timestamps()
        .into_iter()
        .filter(|&t| t > cutoff)
        .count();
    recent >= RATE_LIMIT_MAX
}

fn record_submission() {
    let Some(storage) = get_storage() else {
        return;
    };
    let now = js_sys::Date::now();
    let cutoff = now - RATE_LIMIT_WINDOW_MS;

    let mut timestamps = read_timestamps();
    timestamps.retain(|&t| t > cutoff);
    timestamps.push(now);

    if let Ok(json) = serde_json::to_string(&timestamps) {
        let _ = storage.set_item(RATE_LIMIT_KEY, &json);
    }
}

fn honeypot_value(ev: &web_sys::SubmitEvent) -> String {
    let Some(target) = ev.target() else {
        return String::new();
    };
    let Ok(form) = target.dyn_into::<web_sys::HtmlFormElement>() else {
        return String::new();
    };
    match form.query_selector("input[name=\"_gotcha\"]") {
        Ok(Some(node)) => node
            .dyn_into::<web_sys::HtmlInputElement>()
            .map(|input| input.value())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn is_valid_email(email: &str) -> bool {
    let trimmed = email.trim();
    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
        && !parts[1].starts_with('.')
        && !parts[1].ends_with('.')
}

#[allow(clippy::future_not_send)]
async fn submit_contact(email: &str, message: &str, gotcha: &str) -> Result<(), String> {
    let win = web_sys::window().ok_or("No window")?;

    let body = serde_json::json!({
        "email": email.trim(),
        "message": message.trim(),
        "_gotcha": gotcha,
    });

    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&body.to_string()));

    let request = web_sys::Request::new_with_str_and_init("/.netlify/functions/contact", &opts)
        .map_err(|_| "Failed to create request".to_string())?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|_| "Failed to set headers".to_string())?;

    let promise = win.fetch_with_request(&request);
    let resp_value = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| "Network error".to_string())?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Invalid response".to_string())?;

    if resp.ok() {
        return Ok(());
    }

    let json_promise = resp.json().map_err(|_| "Failed to read response")?;
    let json = wasm_bindgen_futures::JsFuture::from(json_promise)
        .await
        .map_err(|_| "Failed to parse response".to_string())?;

    let error = js_sys::Reflect::get(&json, &JsValue::from_str("error"))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "Something went wrong".to_string());
    Err(error)
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn Contact() -> impl IntoView {
    let form_state = RwSignal::new(FormState::Idle);
    let email = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let email_error = RwSignal::new(String::new());
    let message_error = RwSignal::new(String::new());
    let server_error = RwSignal::new(String::new());

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        if form_state.get() == FormState::Submitting {
            return;
        }

        email_error.set(String::new());
        message_error.set(String::new());
        server_error.set(String::new());

        if is_rate_limited() {
            server_error.set("Too many messages. Please wait a minute.".to_string());
            return;
        }

        let email_val = email.get();
        let message_val = message.get();
        let mut has_error = false;

        if email_val.trim().is_empty() {
            email_error.set("Email is required".to_string());
            has_error = true;
        } else if !is_valid_email(&email_val) {
            email_error.set("Invalid email format".to_string());
            has_error = true;
        }

        if message_val.trim().is_empty() {
            message_error.set("Message is required".to_string());
            has_error = true;
        }

        if has_error {
            return;
        }

        let gotcha_val = honeypot_value(&ev);

        form_state.set(FormState::Submitting);

        spawn_local(async move {
            match submit_contact(&email_val, &message_val, &gotcha_val).await {
                Ok(()) => {
                    record_submission();
                    email.set(String::new());
                    message.set(String::new());
                    form_state.set(FormState::Success);

                    let cb = Closure::once(move || {
                        if form_state.get() == FormState::Success {
                            form_state.set(FormState::Idle);
                        }
                    });
                    if let Some(win) = web_sys::window() {
                        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(),
                            SUCCESS_RESET_MS,
                        );
                    }
                    cb.forget();
                }
                Err(e) => {
                    server_error.set(e);
                    form_state.set(FormState::Idle);
                }
            }
        });
    };

    let is_submitting = move || form_state.get() == FormState::Submitting;

    let button_text = move || match form_state.get() {
        FormState::Submitting => "SENDING...",
        FormState::Success => "MESSAGE SENT!",
        FormState::Idle => "GET IN TOUCH",
    };

    view! {
        <section
            id="contact"
            class="bg-dark text-secondary py-16 md:py-24 px-4 sm:px-8 lg:px-16"
            aria-labelledby="contact-heading"
        >
            <div class="mx-auto grid max-w-6xl grid-cols-1 gap-10 md:grid-cols-[minmax(0,1fr)_minmax(0,1fr)] md:gap-14 md:items-center">
                <figure class="order-1 min-w-0 w-full overflow-hidden rounded-sm md:order-none">
                    <img
                        src="/public/assets/images/contact.png"
                        alt="Trinidad Margni"
                        class="aspect-[4/5] h-auto w-full max-h-[min(70vh,28rem)] object-cover object-top md:aspect-[3/4] md:max-h-none md:min-h-[28rem]"
                    />
                </figure>

                <div class="order-2 flex min-w-0 flex-col gap-5 text-left md:gap-6">
                    <header class="space-y-2 sm:space-y-3">
                        <p class="font-display text-xl sm:text-2xl md:text-4xl">
                            // "I\u{2019}d love to hear from you"
                            "Want to talk?"
                        </p>
                        <h2
                            id="contact-heading"
                            class="font-serif text-2xl uppercase leading-tight tracking-wide text-balance sm:text-xl md:text-2xl lg:text-3xl xl:text-4xl"
                        >
                            "Let\u{2019}s Connect"
                        </h2>
                    </header>

                    <p class="max-w-prose font-sans text-xs leading-relaxed text-secondary/80 sm:text-sm md:text-base">
                        "Interested in working together or learning more about my experience in project management and tech?"
                        <br /> "Send me a message and I\u{2019}ll be in touch!"
                    </p>

                    <form class="flex flex-col gap-3 sm:gap-4" on:submit=on_submit>
                        // Honeypot
                        <div class="absolute -left-[9999px]" aria-hidden="true">
                            <input type="text" name="_gotcha" tabindex="-1" autocomplete="off" />
                        </div>

                        <div>
                            <div class="contact-field group relative">
                                <input
                                    type="email"
                                    id="contact-email"
                                    class="peer w-full border-b border-secondary/30 bg-transparent px-0 pb-2 pt-5 font-sans text-sm text-secondary outline-none transition-colors placeholder:text-transparent focus:border-secondary"
                                    placeholder=" "
                                    maxlength="320"
                                    autocomplete="email"
                                    disabled=is_submitting
                                    prop:value=move || email.get()
                                    on:input=move |ev| {
                                        email.set(event_target_value(&ev));
                                        if !email_error.get().is_empty() {
                                            email_error.set(String::new());
                                        }
                                    }
                                />
                                <label
                                    for="contact-email"
                                    class="contact-label pointer-events-none absolute left-0 top-5 origin-top-left font-sans text-sm text-secondary/50 transition-all duration-200 peer-focus:top-0 peer-focus:scale-75 peer-focus:text-secondary peer-[:not(:placeholder-shown)]:top-0 peer-[:not(:placeholder-shown)]:scale-75 peer-[:not(:placeholder-shown)]:text-secondary"
                                >
                                    "Your email"
                                </label>
                            </div>
                            <p class="h-[14px] mt-1 font-sans text-[0.65rem] leading-[14px] text-red-400">
                                {move || email_error.get()}
                            </p>
                        </div>

                        <div>
                            <div class="contact-field group relative">
                                <textarea
                                    id="contact-message"
                                    class="peer w-full resize-none border-b border-secondary/30 bg-transparent px-0 pb-2 pt-5 font-sans text-sm text-secondary outline-none transition-colors placeholder:text-transparent focus:border-secondary"
                                    placeholder=" "
                                    maxlength="5000"
                                    rows="4"
                                    disabled=is_submitting
                                    prop:value=move || message.get()
                                    on:input=move |ev| {
                                        message.set(event_target_value(&ev));
                                        if !message_error.get().is_empty() {
                                            message_error.set(String::new());
                                        }
                                    }
                                />
                                <label
                                    for="contact-message"
                                    class="contact-label pointer-events-none absolute left-0 top-5 origin-top-left font-sans text-sm text-secondary/50 transition-all duration-200 peer-focus:top-0 peer-focus:scale-75 peer-focus:text-secondary peer-[:not(:placeholder-shown)]:top-0 peer-[:not(:placeholder-shown)]:scale-75 peer-[:not(:placeholder-shown)]:text-secondary"
                                >
                                    "Your message"
                                </label>
                            </div>
                            <p class="h-[14px] mt-1 font-sans text-[0.65rem] leading-[14px] text-red-400">
                                {move || message_error.get()}
                            </p>
                        </div>

                        <p
                            class="h-[14px] font-sans text-[0.65rem] leading-[14px] text-red-400"
                            role="alert"
                        >
                            {move || server_error.get()}
                        </p>

                        <button
                            type="submit"
                            class="w-fit border border-secondary bg-secondary px-6 py-3 font-sans text-xs tracking-[0.2em] uppercase text-dark transition-opacity hover:opacity-90 enabled:cursor-pointer disabled:cursor-not-allowed"
                            disabled=move || {
                                is_submitting() || form_state.get() == FormState::Success
                            }
                        >
                            {button_text}
                        </button>
                    </form>
                </div>
            </div>
        </section>
    }
}
