use std::cmp::Ordering;
use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub order: u32,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Certification {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub year: u16,
    pub category: String,
    pub is_featured: bool,
    /// Lower values sort first among certs with explicit order. Omitted or `null` sorts after
    /// all ordered items in the same category; ties use original JSON order.
    #[serde(default)]
    pub order: Option<u32>,
    pub image: String,
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CertificationsData {
    pub categories: Vec<Category>,
    pub certifications: Vec<Certification>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CategoriesManifest {
    categories: Vec<Category>,
    /// Same order as `include_str!` bundle constants below (JSON has no spread; this documents intent).
    certification_bundles: Vec<String>,
    #[serde(default)]
    certifications: Vec<Certification>,
}

const EXPECTED_CERTIFICATION_BUNDLES: &[&str] = &[
    "1-project-management-and-csm.json",
    "2-artificial-intelligence.json",
    "3-web-development.json",
    "4-digital-marketing.json",
    "5-product-management-and-e-commerce.json",
    "6-communications.json",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CertificationBundle {
    certifications: Vec<Certification>,
}

const CATEGORIES_MANIFEST: &str = include_str!("../../public/data/certs/certifications.json");
const BUNDLE_PROJECT_MANAGEMENT: &str =
    include_str!("../../public/data/certs/categories/1-project-management-and-csm.json");
const BUNDLE_ARTIFICIAL_INTELLIGENCE: &str =
    include_str!("../../public/data/certs/categories/2-artificial-intelligence.json");
const BUNDLE_WEB_DEVELOPMENT: &str =
    include_str!("../../public/data/certs/categories/3-web-development.json");
const BUNDLE_DIGITAL_MARKETING: &str =
    include_str!("../../public/data/certs/categories/4-digital-marketing.json");
const BUNDLE_PRODUCT_MANAGEMENT: &str =
    include_str!("../../public/data/certs/categories/5-product-management-and-e-commerce.json");
const BUNDLE_COMMUNICATIONS: &str =
    include_str!("../../public/data/certs/categories/6-communications.json");

#[must_use]
pub fn load_certifications_data() -> CertificationsData {
    let manifest: CategoriesManifest = match serde_json::from_str(CATEGORIES_MANIFEST) {
        Ok(m) => m,
        Err(err) => {
            log::error!("Failed to parse public/data/certs/certifications.json: {err}");
            return CertificationsData::default();
        }
    };

    let expected: Vec<String> = EXPECTED_CERTIFICATION_BUNDLES
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    if manifest.certification_bundles != expected {
        log::error!(
            "certificationBundles must be exactly {expected:?} (same order as bundle includes in certifications.rs)"
        );
        return CertificationsData::default();
    }

    let mut certifications: Vec<Certification> = Vec::new();
    for (raw, label) in [
        (
            BUNDLE_PROJECT_MANAGEMENT,
            "1-project-management-and-csm.json",
        ),
        (
            BUNDLE_ARTIFICIAL_INTELLIGENCE,
            "2-artificial-intelligence.json",
        ),
        (BUNDLE_WEB_DEVELOPMENT, "3-web-development.json"),
        (BUNDLE_DIGITAL_MARKETING, "4-digital-marketing.json"),
        (
            BUNDLE_PRODUCT_MANAGEMENT,
            "5-product-management-and-e-commerce.json",
        ),
        (BUNDLE_COMMUNICATIONS, "6-communications.json"),
    ] {
        match serde_json::from_str::<CertificationBundle>(raw) {
            Ok(bundle) => certifications.extend(bundle.certifications),
            Err(err) => log::error!("Failed to parse certification bundle {label}: {err}"),
        }
    }

    if !manifest.certifications.is_empty() {
        log::warn!(
            "certifications.json had non-empty \"certifications\"; inline entries are ignored (use per-category bundles)."
        );
    }

    let mut data = CertificationsData {
        categories: manifest.categories,
        certifications,
    };

    data.categories.sort_by_key(|c| c.order);
    let mut indexed: Vec<(usize, Certification)> =
        data.certifications.into_iter().enumerate().collect();
    indexed.sort_by(|(ia, a), (ib, b)| {
        a.category
            .cmp(&b.category)
            .then_with(|| cert_order_within_category((*ia, a), (*ib, b)))
    });
    data.certifications = indexed.into_iter().map(|(_, c)| c).collect();
    data
}

#[must_use]
pub fn certification_store() -> &'static CertificationsData {
    static STORE: LazyLock<CertificationsData> = LazyLock::new(load_certifications_data);
    &STORE
}

impl CertificationsData {
    #[must_use]
    pub fn is_valid_category(&self, id: &str) -> bool {
        self.categories.iter().any(|c| c.id == id)
    }

    /// Featured items for a category, sorted by `order` ascending then `id` (lowest `order` first).
    #[must_use]
    pub fn featured_for_category(&self, category_id: &str) -> Vec<&Certification> {
        let mut out: Vec<_> = self
            .certifications
            .iter()
            .filter(|c| c.is_featured && c.category == category_id)
            .collect();
        out.sort_by(|a, b| match (a.order, b.order) {
            (Some(oa), Some(ob)) => oa.cmp(&ob).then_with(|| a.id.cmp(&b.id)),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.id.cmp(&b.id),
        });
        out
    }

    #[must_use]
    pub fn count_for_category(&self, category_id: &str) -> usize {
        self.certifications
            .iter()
            .filter(|c| c.category == category_id)
            .count()
    }

    #[must_use]
    pub fn certifications_for_category(&self, category_id: &str) -> Vec<&Certification> {
        self.certifications
            .iter()
            .filter(|c| c.category == category_id)
            .collect()
    }
}

/// `Some` before `None`; two `Some` compared by value; two `None` keep document order (`ia`/`ib`).
fn cert_order_within_category(
    (ia, a): (usize, &Certification),
    (ib, b): (usize, &Certification),
) -> Ordering {
    match (a.order, b.order) {
        (Some(oa), Some(ob)) => oa.cmp(&ob).then_with(|| a.id.cmp(&b.id)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => ia.cmp(&ib),
    }
}
