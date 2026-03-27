# Copilot Instructions

## Project

Trinidad Margni's professional portfolio.

## Tech stack

- **Language**: Rust
- **Framework**: Leptos 0.8 (CSR)
- **Bundler**: Trunk
- **Styling**: SCSS
- **Testing**: cargo test + wasm-bindgen-test
- **Linting**: clippy (pedantic + nursery)
- **Formatting**: rustfmt + leptosfmt

## Code style

- Explicit imports — no wildcard imports (`use module::*`)
- Group imports: external crates → internal modules → local items
- Use `snake_case` for variables, functions, and modules
- Use `PascalCase` for types, traits, and components
- Prefix booleans with `is_`, `has_`, `can_`, `should_`, etc.
- Named constants over magic numbers/strings
- Add trailing semicolons on statements that return unit `()`
- Use pattern matching over chains of `if/else`
- Use `Option` and `Result` — never panic in production code

## Rust

- Deny `unsafe` code
- Avoid `.unwrap()` and `.expect()` — use `?` operator or proper error handling
- Use `clone()` only when necessary
- Prefer borrowing (`&`) over ownership when possible
- Use `impl Into<T>` for flexible function parameters

## Leptos

- One component per file
- Use `#[component]` macro for all components
- Use `signal()` for reactive state
- Use `view!` macro for templates
- Props are function parameters with explicit types

## Testing

- Integration tests live in `tests/` directory
- Unit tests live inline in `#[cfg(test)]` modules
- Use `wasm-bindgen-test` for browser-dependent tests

## Commits

Follow Conventional Commits: `<type>(optional scope): <description>` Valid types: `feat`, `fix`, `refactor`, `style`,
`test`, `docs`, `chore`, `perf`, `ci`, `deps`, `infra`
