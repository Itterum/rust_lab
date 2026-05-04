# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace for small, focused experiments.  
Top-level files:
- `Cargo.toml` / `Cargo.lock`: workspace manifest and lockfile
- `README.md`: setup and run instructions
- `architecture_recommendations.md`: architecture notes

Crates live under `apps/`:
- `apps/inquire_app`: CLI validation flow
- `apps/raylib_app`: raylib-based 2D simulation
- `apps/server_app`: Axum + SQLx (SQLite) backend
- `apps/test_samples`: compact regex/sandbox examples

Keep new experiments isolated as separate crates in `apps/`.

## Build, Test, and Development Commands
Run from repository root unless noted:
- `cargo build --workspace` — build all crates
- `cargo test --workspace` — run all tests
- `cargo fmt --all` — format code
- `cargo clippy --workspace --all-targets -- -D warnings` — lint and fail on warnings
- `cargo run -p server_app` — run a specific crate (replace package name as needed)

For `server_app`, configure env vars (see `apps/server_app/.env.example`) before running.

## Coding Style & Naming Conventions
- Use standard Rust formatting (`rustfmt`), 4-space indentation.
- Prefer idiomatic Rust naming:
  - `snake_case` for functions/modules/files
  - `PascalCase` for structs/enums/traits
  - `SCREAMING_SNAKE_CASE` for constants
- Keep crates focused; avoid cross-crate coupling for one-off experiments.
- Treat clippy warnings as errors in CI/local checks.

## Testing Guidelines
- Use Rust built-in test framework (`#[test]`, `#[tokio::test]` for async).
- Place unit tests near implementation (`src/main.rs` or `src/lib.rs` in `mod tests`).
- Name tests by behavior, e.g. `get_user_returns_not_found_for_missing_id`.
- Run targeted tests while iterating, e.g. `cargo test -p server_app`, then full workspace tests before PR.

## Commit & Pull Request Guidelines
Current history favors short, imperative commit messages (e.g. `init`, `test axum app`, `Add project README...`).  
Preferred format for new commits:
- concise imperative summary
- optional scope prefix, e.g. `server_app: add user endpoint tests`

PRs should include:
- what changed and why
- affected crate(s) and commands run (`fmt`, `clippy`, `test`)
- config/runtime notes (especially for `server_app` env settings)
