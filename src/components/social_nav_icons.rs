use leptos::prelude::*;

use crate::site_links::{INSTAGRAM_URL, LINKEDIN_URL};

#[component]
pub fn SocialNavIcons(
    #[prop(default = false)] is_centered: bool,
    #[prop(default = true)] is_inverted: bool,
) -> impl IntoView {
    let wrapper = if is_centered {
        "flex items-center justify-center gap-4"
    } else {
        "flex items-center gap-4"
    };

    let icon_class = if is_inverted {
        "h-[18px] w-[18px] invert"
    } else {
        "h-[18px] w-[18px]"
    };

    view! {
        <div class=wrapper>
            <a
                href=LINKEDIN_URL
                target="_blank"
                rel="noopener noreferrer"
                class="hover:opacity-60 transition-opacity"
                aria-label="LinkedIn"
            >
                <img
                    src="/public/assets/icons/linkedin.svg"
                    alt=""
                    class=icon_class
                    aria-hidden="true"
                />
            </a>
            <a
                href=INSTAGRAM_URL
                target="_blank"
                rel="noopener noreferrer"
                class="hover:opacity-60 transition-opacity"
                aria-label="Instagram"
            >
                <img
                    src="/public/assets/icons/instagram.svg"
                    alt=""
                    class=icon_class
                    aria-hidden="true"
                />
            </a>
        </div>
    }
}
