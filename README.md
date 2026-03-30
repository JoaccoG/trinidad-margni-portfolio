# Trinidad Margni's Portfolio

Professional portfolio for a Senior Project Manager built with [Rust](https://www.rust-lang.org/) and [Leptos](https://leptos.dev/).

## Tech Stack

- **Language:** Rust
- **Framework:** Leptos (CSR)
- **Styling:** Tailwind
- **Bundler:** Trunk
- **Hosting:** Netlify

## Setup Project (one time only)

```bash
./scripts/setup.sh
```

## Development

```bash
# Run dev server
trunk serve --open
```

```bash
# Build for production
trunk build --release
```

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run Rust unit tests
cargo test

# Run WASM/browser integration tests
wasm-pack test --headless --chrome
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
