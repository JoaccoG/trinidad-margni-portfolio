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

    let anchors = ["#home", "#about", "#certifications", "#contact"];
    for href in anchors {
        let selector = format!("nav a[href='{href}']");
        let link = doc.query_selector(&selector).unwrap();
        assert!(link.is_some(), "nav should have link to {href}");
    }

    let about = doc.query_selector("#about").unwrap();
    assert!(about.is_some(), "about section should exist");

    let about_text = about.unwrap().text_content().unwrap_or_default();
    assert!(
        about_text.contains("Hey there"),
        "about should contain greeting"
    );
    assert!(
        about_text.contains("United Airlines"),
        "about should contain bio"
    );

    let resume_link = doc
        .query_selector("a[href='/public/files/resume.pdf']")
        .unwrap();
    assert!(
        resume_link.is_some(),
        "about should link to English resume PDF"
    );
}

#[wasm_bindgen_test]
async fn header_nav_excludes_resume_hash() {
    mount_to_body(|| {
        view! { <App /> }
    });

    yield_now().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    assert!(
        doc.query_selector("header nav a[href='#resume']")
            .unwrap()
            .is_none(),
        "header nav should not include #resume"
    );
}

#[wasm_bindgen_test]
async fn about_resume_uses_icon_assets() {
    mount_to_body(|| {
        view! { <App /> }
    });

    yield_now().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    let about = doc
        .query_selector("#about")
        .unwrap()
        .expect("about section should exist");

    assert!(
        about
            .query_selector("details.about-resume")
            .unwrap()
            .is_some(),
        "about should expose resume details control"
    );

    let chevron = about
        .query_selector("img[src*='chevron-down.svg']")
        .unwrap();
    assert!(
        chevron.is_some(),
        "resume control should load chevron from public assets"
    );

    let external_icons = doc
        .query_selector_all("#about img[src*='external-link.svg']")
        .unwrap();
    let n = external_icons.length();
    assert!(
        n >= 2 && n.is_multiple_of(2),
        "each about block should have two external-link icons (found {n})"
    );
}
