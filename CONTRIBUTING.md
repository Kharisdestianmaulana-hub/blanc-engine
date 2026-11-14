# Contributing to Blanc Engine

Thank you for your interest in contributing to **Blanc Engine**! Every contribution—from bug reports and documentation improvements to new engine features—is welcome.

---

## Development Setup

1. **Install Rust**: Ensure you have the latest stable Rust toolchain via [rustup](https://rustup.rs/).
2. **Clone the repository**:
   ```bash
   git clone https://github.com/Kharisdestianmaulana-hub/blanc-engine.git
   cd blanc-engine
   ```
3. **Build the workspace**:
   ```bash
   cargo build --workspace
   ```
4. **Run the test suite**:
   ```bash
   cargo test --workspace
   ```

---

## Workspace Structure

- `crates/blanc_core` — Core data models, scene/story parsing, `.bpack` archive encryption, localization/CSV tools, UI config, and installer packaging.
- `crates/blanc_editor` — Visual novel editor (`iced` 0.13).
- `crates/blanc_runtime` — Standalone game player (`Game.exe`).
- `crates/blanc_installer` — Standalone Windows Setup Wizard & Uninstaller stub.

> **Note on Exporting from the Editor:** When testing game exports from `blanc_editor`, make sure `blanc_runtime` and `blanc_installer` are compiled in the same target directory (`cargo build --workspace`) so the editor can bundle them.

---

## Coding Guidelines

- Keep changes focused and modular; avoid duplicating logic across crates.
- Add unit or integration tests in `crates/blanc_core/tests/` or inline `#[cfg(test)]` modules when adding core serialization, parsing, or runtime engine behavior.
- Ensure `cargo test --workspace` and `cargo build --workspace` pass cleanly before opening a Pull Request.
- Format code with `cargo fmt` and check for warnings with `cargo clippy --workspace` when possible.

---

## Pull Request Process

1. Fork the repository and create a branch from `main` (e.g., `feature/my-feature` or `fix/issue-description`).
2. Make your changes and verify all workspace tests pass (`cargo test --workspace`).
3. Write a clear commit message and PR description explaining *what* changed and *why*.
4. Link any related GitHub Issues in your PR description.

---

## Reporting Bugs & Requesting Features

Please use the GitHub Issue templates in `.github/ISSUE_TEMPLATE/` to report bugs or propose new features. Include your OS, Rust version (`rustc --version`), and reproduction steps when reporting an issue.
