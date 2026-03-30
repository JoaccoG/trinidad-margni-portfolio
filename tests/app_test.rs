use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn app_mounts_without_panic() {
    use leptos::prelude::*;
    use trinidad_margni_portfolio::App;

    mount_to_body(|| {
        view! { <App /> }
    });
}
