#![allow(clippy::future_not_send)]

use leptos::prelude::*;
use trinidad_margni_portfolio::App;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

async fn yield_now() {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}

#[wasm_bindgen_test]
async fn app_renders_header_and_hero() {
    mount_to_body(|| {
        view! { <App /> }
    });

    yield_now().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    let header = doc.query_selector("header").unwrap();
    assert!(header.is_some(), "header element should exist");

    let header_text = header.unwrap().text_content().unwrap_or_default();
    assert!(
        header_text.contains("Trinidad Margni"),
        "header should contain name"
    );
    assert!(header_text.contains("Home"), "header should have Home link");
    assert!(
        header_text.contains("About"),
        "header should have About link"
    );
    assert!(
        header_text.contains("Contact"),
        "header should have Contact link"
    );

    let hero = doc.query_selector("#home").unwrap();
    assert!(hero.is_some(), "hero section should exist");

    let hero_text = hero.unwrap().text_content().unwrap_or_default();
    assert!(
        hero_text.contains("Leading Projects"),
        "hero should have title"
    );
    assert!(
        hero_text.contains("Senior Project Manager"),
        "hero should have subtitle"
    );
    assert!(hero_text.contains("About Me"), "hero should have CTA");

    let img = doc
        .query_selector("#home img[alt='Trinidad Margni']")
        .unwrap();
    assert!(img.is_some(), "hero should have portrait image");

    let src = img.unwrap().get_attribute("src").unwrap_or_default();
    assert!(
        src.contains("hero-pic"),
        "image src should reference hero-pic"
    );

    let anchors = ["#home", "#about", "#resume", "#certifications", "#contact"];
    for href in anchors {
        let selector = format!("nav a[href='{href}']");
        let link = doc.query_selector(&selector).unwrap();
        assert!(link.is_some(), "nav should have link to {href}");
    }
}
