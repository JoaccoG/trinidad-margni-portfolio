#![allow(clippy::future_not_send)]

use leptos::mount::mount_to;
use leptos::prelude::*;
use trinidad_margni_portfolio::App;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

const LEPTOS_TEST_HOST_ID: &str = "__leptos_wasm_test_host";

fn leptos_test_host() -> web_sys::HtmlElement {
    let win = web_sys::window().expect("window");
    let doc = win.document().expect("document");
    if let Ok(Some(found)) = doc.query_selector(&format!("#{LEPTOS_TEST_HOST_ID}")) {
        return found.dyn_into().expect("test host should be HtmlElement");
    }
    let el = doc
        .create_element("div")
        .expect("create test host")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("div should be HtmlElement");
    el.set_id(LEPTOS_TEST_HOST_ID);
    doc.body()
        .expect("document.body")
        .append_child(&el)
        .expect("append test host");
    el
}

async fn prepare_test_mount() {
    let win = web_sys::window().expect("window");
    win.history()
        .expect("window.history")
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/"))
        .expect("reset URL to / for isolated router state");
    let host = leptos_test_host();
    let host_el: &web_sys::Element = host.dyn_ref().expect("host as Element");
    host_el.set_inner_html("");
    yield_now().await;
}

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
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
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

    let anchors = ["/#home", "/#about", "/#certifications", "/#contact"];
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
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
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
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
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

#[wasm_bindgen_test]
async fn home_certifications_explore_link_navigates_to_full_page() {
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
        view! { <App /> }
    });

    yield_now().await;

    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();

    let section = doc
        .query_selector("#certifications")
        .unwrap()
        .expect("certifications section");
    let section_text = section.text_content().unwrap_or_default();
    assert!(
        section_text.contains("Explore all certifications"),
        "home should link to full catalog"
    );
    assert!(
        section_text.contains('(') && section_text.contains(')'),
        "explore link should include total count in parentheses"
    );

    let link = section
        .query_selector("a[href='/certifications']")
        .unwrap()
        .expect("explore all certifications link");
    link.dyn_into::<web_sys::HtmlAnchorElement>()
        .expect("explore link should be an anchor")
        .click();

    yield_now().await;
    yield_now().await;

    let heading = doc
        .query_selector("#all-certs-heading")
        .unwrap()
        .expect("certifications page heading after client navigation");
    assert!(
        heading
            .text_content()
            .unwrap_or_default()
            .contains("All certifications")
    );
}

#[wasm_bindgen_test]
async fn certifications_category_query_from_home_see_all_link() {
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
        view! { <App /> }
    });

    yield_now().await;

    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();

    doc.query_selector("#cert-tab-web-development")
        .unwrap()
        .expect("web development tab")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("tab should be an element")
        .click();

    yield_now().await;

    let section = doc
        .query_selector("#certifications")
        .unwrap()
        .expect("certifications section");
    let link = section
        .query_selector("a[href='/certifications?category=web-development']")
        .unwrap()
        .expect("see all link for active category");
    link.dyn_into::<web_sys::HtmlAnchorElement>()
        .expect("see all link should be an anchor")
        .click();

    yield_now().await;
    yield_now().await;

    let search = win.location().search().unwrap_or_default();
    assert!(
        search.contains("category=web-development"),
        "URL should carry category filter, got search={search}"
    );
}

#[wasm_bindgen_test]
async fn footer_renders_credit_and_portfolio_links() {
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
        view! { <App /> }
    });

    yield_now().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    let footer = doc
        .query_selector("footer[role='contentinfo']")
        .unwrap()
        .expect("footer landmark");
    let footer_text = footer.text_content().unwrap_or_default();
    assert!(
        footer_text.contains("Designed & Built by"),
        "footer should show credit line"
    );
    assert!(
        footer_text.contains("Joaquín Godoy"),
        "footer should name designer"
    );

    assert!(
        doc.query_selector("footer a[href='https://joaquingodoy.com']")
            .unwrap()
            .is_some(),
        "footer should link to designer site"
    );
    assert!(
        doc.query_selector("footer a[href='/#home']")
            .unwrap()
            .is_some(),
        "footer nav should include home hash link"
    );
}

#[wasm_bindgen_test]
async fn contact_section_renders_form() {
    prepare_test_mount().await;
    let _app = mount_to(leptos_test_host(), || {
        view! { <App /> }
    });

    yield_now().await;

    let doc = web_sys::window().unwrap().document().unwrap();

    let section = doc
        .query_selector("#contact")
        .unwrap()
        .expect("contact section should exist");

    let section_text = section.text_content().unwrap_or_default();
    assert!(
        section_text.contains("Let\u{2019}s Connect"),
        "contact should have heading"
    );

    assert!(
        doc.query_selector("#contact input[type='email']")
            .unwrap()
            .is_some(),
        "contact form should have email input"
    );
    assert!(
        doc.query_selector("#contact textarea").unwrap().is_some(),
        "contact form should have message textarea"
    );
    assert!(
        doc.query_selector("#contact button[type='submit']")
            .unwrap()
            .is_some(),
        "contact form should have submit button"
    );
}
