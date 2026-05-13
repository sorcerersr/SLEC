# AGENTS.md

AI coding agent instructions for **SLEC**

## Project Overview

SLEC - **S**orcerers **L**ong **E**xposure **C**alculator - A photography tool to calculate shutter time for long exposures when using ND filters.

**Project Type:** Dioxus Web Application (Rust frontend compiling to WASM)
**Primary Language:** Rust (100% of codebase)
**Framework:** Dioxus 0.7.1
**Target Platform:** Web (WASM)

## Architecture

**Project Structure:**
- `assets/` - Static assets (pico.min.css)
- `locales/` - i18n translation files (en.yml, de-DE.yml)
- `src/` - Source code
  - `main.rs` - App entrypoint, global state (AppState), view routing, Header component
  - `model.rs` - Domain models: ShutterSpeed (53 predefined speeds), Filter (ND8/ND64/ND1000)
  - `calculator.rs` - Calculator view with shutter speed slider, filter checkboxes, final exposure display
  - `timer.rs` - Countdown timer view using gloo_timers
  - `settings.rs` - Settings view with language selection and about section
  - `languages.rs` - Language enum (English/German), locale persistence via LocalStorage
  - `components/` - Reusable UI components (mod.rs, back_button, custom_filter, darkmode_toggle, exposure, slider)
- `tests/` - Integration test directory (currently empty)

**State Management:**
- Global app state via Dioxus `GlobalSignal<AppState>`
- State fields: `language: Language`, `view: View`, `shutter_speed: f64`, `total_fstop_reduction: f64`
- View enum: `Calculator`, `Settings`, `Timer(u64)`
- LocalStorage persistence for filters and locale

**Key Patterns:**
- Single-page app with view switching (no router)
- rsx! macro for declarative UI
- `use_signal` for local component state
- `spawn(async move {})` for async timers
- `rust_i18n::t!()` for internationalization

## Key Files

**Configuration:**
- `Cargo.toml` - Rust dependencies (dioxus 0.7.1, gloo-*, rust-i18n, serde, wasm-bindgen)
- `Dioxus.toml` - Dioxus app config (name, default_platform=web, out_dir=dist, asset_dir=assets)
- `locales/en.yml`, `locales/de-DE.yml` - i18n translations

**Documentation:**
- `README.md` - Project description, prerequisites, live link
- `LICENSE`
- `IMPROVEMENTS.md` - Planned improvements
- `PLAN_darkmode_toggle.md` - Feature plan document

**Source Modules:**
- `src/main.rs` - App entrypoint, AppState, routing
- `src/model.rs` - ShutterSpeed, Filter domain models
- `src/calculator.rs` - Calculator view components
- `src/timer.rs` - Timer view component
- `src/settings.rs` - Settings view components
- `src/languages.rs` - Language enum and locale management
- `src/components/mod.rs` - Component module exports

## Development Commands

**Build:**
```bash
cargo build
```

**Run dev server:**
```bash
dx serve
```

**Test:**
```bash
cargo test
```

**Check:**
```bash
cargo check
```

**Format:**
```bash
cargo fmt
```

**Clippy:**
```bash
cargo clippy
```

## Code Style Guidelines

- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `#![allow(non_snake_case)]` at crate root for component names (Dioxus convention)
- Handle errors explicitly with `Result<T, E>` or `unwrap()` for internal state
- Use `use_signal` for local component state, `Signal::global` for app-wide state
- Prefer `rsx!` macro for declarative UI rendering
- Use `rust_i18n::t!()` for all user-facing strings
- Tests live inline in source files under `#[cfg(test)] mod tests`
- Use `gloo_console::log!` for debugging in WASM context
- LocalStorage keys: `"filters"`, `"locale"`

## Build Profiles

Custom profiles in `Cargo.toml`:
- `wasm-dev` - Inherited from dev, opt-level 1
- `server-dev` - Inherited from dev
- `android-dev` - Inherited from dev

## AI Coding Assistance Notes

**Important Considerations:**
- This is a WASM-targeted web app, not a native binary - no `std::fs`, `std::net`, etc.
- Use `gloo-*` crates for browser APIs (gloo-console, gloo-storage, gloo-timers)
- `web-sys` is used for Window and MediaQueryList (dark mode detection)
- Tests are inline in source files, not in `tests/` directory
- The app uses global state, not props-based routing - be careful when modifying AppState
- `shutter_speed` and `total_fstop_reduction` in AppState have TODOs to move to local state
- The `exposure.rs` component uses `humantime::format_duration` for display formatting
- Filter persistence uses serde_json + LocalStorage
- Language switching via `rust_i18n::set_locale()` and `Language` enum

---

*Updated based on actual project analysis.*
