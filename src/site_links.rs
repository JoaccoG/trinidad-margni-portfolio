//! Shared navigation labels and external URLs for header and footer.

pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/trinidadmargni";
pub const INSTAGRAM_URL: &str = "https://www.instagram.com/trinidadmargni";

/// Hash routes from site root (`/`). Safe from any in-app URL (home or certifications).
pub const NAV_ITEMS: [(&str, &str); 4] = [
    ("Home", "/#home"),
    ("About", "/#about"),
    ("Certifications", "/#certifications"),
    ("Contact", "/#contact"),
];

pub const HOME_LOGO_HREF: &str = "/#home";
