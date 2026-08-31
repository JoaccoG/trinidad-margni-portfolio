<div align="center">

# Trinidad Margni — Portfolio

**The personal site of a Senior Project Manager, written in Rust and compiled to WebAssembly. No JavaScript framework anywhere in it.**

[![License: MIT](https://img.shields.io/badge/License-MIT-E1DACA?style=flat-square&labelColor=000000)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-edition_2024-E1DACA?style=flat-square&labelColor=000000)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.8_CSR-E1DACA?style=flat-square&labelColor=000000)](https://leptos.dev/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-v4-E1DACA?style=flat-square&labelColor=000000)](https://tailwindcss.com/)
[![Trunk](https://img.shields.io/badge/Trunk-bundler-E1DACA?style=flat-square&labelColor=000000)](https://trunkrs.dev/)
[![Netlify](https://img.shields.io/badge/Netlify-deploy-E1DACA?style=flat-square&labelColor=000000)](https://www.netlify.com/)

[![Build](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/build.yml/badge.svg)](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/build.yml)
[![Testing](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/test.yml/badge.svg)](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/test.yml)
[![Code audit](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/audit.yml/badge.svg)](https://github.com/JoaccoG/trinidad-margni-portfolio/actions/workflows/audit.yml)

**[▶ trinidadmargni.com](https://trinidadmargni.com)**

<img src="public/assets/images/preview.png" width="760" alt="The site's landing section: a portrait on the left half, black on the right, and the headline 'Leading projects that align teams, drive results, and deliver impact' centered across both." />

</div>

---

> A portfolio site is a solved problem. This one is solved in Rust.

**Trinidad Margni** is a Senior Project Manager who has led delivery for YouTube and United Airlines. This is her site: a single-page editorial layout, a filterable wall of 78 certifications, three downloadable résumés, and a contact form that reaches her inbox.

None of it runs on React. The front end is [Leptos](https://leptos.dev/) in client-side rendering mode, compiled to `wasm32-unknown-unknown`, bundled by [Trunk](https://trunkrs.dev/) and served from Netlify as static files plus one serverless function. Leptos gives fine-grained reactivity with no virtual DOM, and its `view!` macro is expanded by the compiler, so a typo in the markup fails the build rather than the page.

## What ships

Measured on the release build (`trunk build --release`, `opt-level = "z"` + LTO), before and after compression:

| asset | raw | gzip | brotli |
|:--|--:|--:|--:|
| WebAssembly | 545 KB | 212 KB | 174 KB |
| JS glue | 49 KB | 8.4 KB | 7.1 KB |
| CSS | 57 KB | 8.6 KB | 7.2 KB |
| **total** | **651 KB** | **229 KB** | **189 KB** |

189 KB over the wire, which lands in the same range as an equivalent React build. Rust doesn't win a size benchmark here and was never picked to. The WebAssembly is also carrying the entire certification dataset, compiled into it.

## What's on it

**Home.** A full-viewport hero, the about section with the résumé dropdown, an infinite marquee of the companies she has worked with, a preview of the certifications, and the contact form.

**`/certifications`.** The full wall: **78 certifications across 6 categories** (Project Management & CSM, Artificial Intelligence, Web Development, Digital Marketing, Product Management & e-Commerce, Communications). The data lives as JSON under `public/data/certs/`, one bundle per category, pulled into the binary at compile time with `include_str!`. There is no fetch and no loading state, and a malformed bundle fails the build instead of rendering a blank section. Filtering runs through the URL (`/certifications?category=artificial-intelligence`), so any filtered view is a shareable link.

**Résumés.** English, Spanish, and a Harvard-format, AI-friendly version, served straight from `public/files/`.

## The contact form

No third-party form widget, and no API key in the browser. The form posts to a [Netlify Function](netlify/functions/contact.mjs) that checks the method, the JSON shape, the email format and both field lengths, drops honeypot submissions, HTML-escapes what survives, and hands the message to [Resend](https://resend.com/). The key lives only in the Netlify environment.

## Checked, not trusted

`unsafe_code` is denied outright, and Clippy runs with `pedantic` and `nursery` promoted to warnings, all of them denied in CI.

**Git hooks** ([`.githooks/`](.githooks), wired through `core.hooksPath`). `pre-commit` blocks on `cargo fmt --check` and `cargo clippy -- -D warnings`. `pre-push` validates the branch name against `feature/…` or `bug/…`, then runs a release build and `cargo test`.

**GitHub Actions** runs three workflows on every pull request: the release build, `wasm-pack test --headless --chrome`, and an audit that repeats fmt and clippy.

The line between those two is worth knowing. Every assertion in [`tests/app_test.rs`](tests/app_test.rs) is a `wasm_bindgen_test`, so a plain `cargo test` on the host compiles all of them and runs none. Only `wasm-pack` puts them in front of a real DOM. When the pre-push hook prints "Running tests", it means the native ones; CI is what actually exercises the browser.

## Running locally

One-time setup, which checks the Rust toolchain, adds the `wasm32-unknown-unknown` target, installs Trunk and leptosfmt, warms the dependency cache and points git at the repo's hooks:

```bash
./scripts/setup.sh
```

Then:

```bash
trunk serve --open      # http://localhost:3000
```

```bash
trunk build --release   # production bundle → dist/
```

Everything the hooks and CI check, in one place:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
wasm-pack test --headless --chrome
```

The last line is the one no hook runs, which makes it the one worth running by hand before opening a pull request.

> `wasm-pack` downloads its own ChromeDriver for the newest Chrome and overrides `CHROMEDRIVER`, so an older local Chrome fails with a driver that dies on startup. Until Chrome catches up, skip the wrapper and drive the underlying runner directly: point `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` at wasm-pack's cached `wasm-bindgen-test-runner`, set `CHROMEDRIVER` to a driver matching your Chrome and `WASM_BINDGEN_TEST_ONLY_WEB=1`, then run `cargo test --target wasm32-unknown-unknown`.

## Environment

The contact function needs three variables: in `.env` locally, in the Netlify dashboard in production. See [`.env.example`](.env.example):

```bash
RESEND__API_KEY=re_your_api_key_here
EMAILS__FROM=Portfolio Contact <contact@mail.yourdomain.com>
EMAILS__TO=recipient@example.com
```

## Built with

Leptos 0.8 with `leptos_router` and `leptos_meta`, Tailwind CSS v4 and Trunk, on Rust edition 2024. Netlify serves the static build and runs the contact function, Resend delivers the mail. The type is GFS Didot, Montserrat and Allura.

## License

[MIT](LICENSE) © 2026 Joaquín Godoy, for the design and the engineering. Site content, imagery and résumés belong to Trinidad Margni.
