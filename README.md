<p align="center">
  <img src="LogoBlancEngine.png" alt="Blanc Engine Logo" width="120" />
</p>

<h1 align="center">Blanc Engine</h1>

<p align="center">
  An open-source visual novel creation suite &amp; runtime engine written in <strong>Rust</strong>.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2021%20Edition-orange?logo=rust" alt="Rust 2021" />
  <img src="https://img.shields.io/badge/GUI-iced%200.13-blue" alt="iced 0.13" />
  <img src="https://img.shields.io/badge/License-GPL--3.0--or--later-green" alt="GPL-3.0-or-later" />
</p>

---

## Overview

**Blanc Engine** is a desktop-first visual novel engine designed to take a story from screenplay to a distributable standalone game. It provides an integrated **Visual Editor**, a **Standalone Runtime Player**, an encrypted **`.bpack` Asset Packer**, a developer-defined **Multi-Language Localization System**, and a built-in **Windows Setup Wizard & Uninstaller** — all in one Rust workspace.

---

## Key Features

### Visual Editor (`blanc_editor`)
- **Project Overview & Asset Manager**: Organize backgrounds, character sprites, BGM, SFX, voices, and UI assets with grid/list views and audio preview.
- **Story Setup**: Screenplay-style script editor (`Speaker (Emotion): "Dialogue"`, `[Scene: ...]`, `[BGM: ...]`, `[Choice: ...]`) that converts directly into visual scenes.
- **Scene Editor**: Block-based visual timeline for backgrounds, character positioning/expressions, dialogue, narration, choices, wait timers, and screen effects (`FadeIn`, `FadeOut`, `FlashWhite`, `Shake`).
- **Story Flow**: Interactive node-based graph editor connecting `Main Menu`, `Scenes`, `Choice` branches, `Pause Menu`, `Settings`, and `Ending` nodes.
- **UI Designer**: Live-preview customization for Title Screen, Gameplay HUD, Dialogue Box, Pause/Settings menus, Save/Load slots, and Backlog.
- **Localization Tab**: Developer-controlled multi-language tooling — add any number of languages (`code` + `display name`), translate UI labels, unique character names, and scene dialogues/choices, track per-language completion, and **Export / Import CSV**.
- **Export Pipeline**: Package games into **Portable** builds (`Game.exe` + encrypted `game.bpack` or `GameData/`) or a standalone **Windows Installer (`Setup.exe`)** with custom version, publisher, terms/privacy policy, sidebar artwork, and embedded application icon.

### Standalone Runtime Player (`blanc_runtime`)
- **Playback & Presentation**: Typewriter text reveal, character sprite transitions, screen effects, and BGM/SFX playback via `rodio`.
- **Auto-Play & Skip Modes**:
  - **Auto-Play (`A`)** with configurable delay (`0.5s – 5.0s`).
  - **Skip (`S`)** with **`Read Only`** (tracks read lines persistently in `read_history.json` and stops at unread text) and **`All Text`** modes, automatically pausing at choices.
- **Save / Load & Backlog**: Multiple save slots with scene thumbnails, timestamps, and dialogue previews; Quick Save (`F5`) / Quick Load (`F9`); and scrollable Dialogue History (`B`).
- **Multi-Language & CJK Support**: Dynamic **Language** selector in Settings (when >1 language is configured), automatic first-launch language sync from the Windows Installer, and system CJK font loading.
- **Input Support**: Mouse, keyboard shortcuts, and gamepad navigation via `gilrs`.

### Windows Setup Wizard & Uninstaller (`blanc_installer`)
- **Single-File `Setup.exe`**: Extracts `game.bpack` or `GameData/` with a live progress bar.
- **6-Language Installer UI**: Built-in wizard translations for English, Bahasa Indonesia, Japanese, Spanish, Simplified Chinese, and Korean.
- **Clean Uninstaller**: Generates a per-game `Uninstall <Game>.exe`, Start Menu/Desktop shortcuts, Windows *Apps & Features* registry entries, and self-cleaning uninstall flow.

---

## Workspace Architecture

| Crate | Path | Description |
| :--- | :--- | :--- |
| **`blanc_core`** | [`crates/blanc_core`](crates/blanc_core) | Core data models (`Project`, `Scene`, `Story`, `StoryFlowGraph`, `SystemScreensConfig`, `RuntimeUiConfig`), `.bpack` encryption/packing, localization & CSV tools, and Windows icon/installer packaging. |
| **`blanc_editor`** | [`crates/blanc_editor`](crates/blanc_editor) | Desktop visual novel editor built with `iced` 0.13. |
| **`blanc_runtime`** | [`crates/blanc_runtime`](crates/blanc_runtime) | Standalone visual novel player (`Game.exe`). |
| **`blanc_installer`** | [`crates/blanc_installer`](crates/blanc_installer) | Standalone Windows Setup Wizard and Uninstaller stub. |

---

## Getting Started

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Rust 2021 edition, stable toolchain)
- Windows 10/11 (primary target for installer & shortcuts; core/editor/runtime also build on other desktop targets supported by `iced` & `rodio`)

### Clone & Build

```bash
git clone https://github.com/Kharisdestianmaulana-hub/blanc-engine.git
cd blanc-engine
cargo build --workspace
```

### Run the Editor

```bash
cargo run -p blanc_editor
```

> **Tip:** To export games from the editor, ensure `blanc_runtime` and `blanc_installer` are built in the same target output directory (`cargo build --workspace`) so the editor can locate `blanc_runtime.exe` and `blanc_installer.exe` next to `blanc_editor.exe`.

### Run Tests

```bash
cargo test --workspace
```

---

## Runtime Keyboard Shortcuts

| Key | Action |
| :--- | :--- |
| `Space` / `Enter` / Click | Advance dialogue / confirm selection |
| `A` | Toggle **Auto-Play** |
| `S` | Toggle **Skip** |
| `B` | Open **Dialogue History (Backlog)** |
| `H` | Hide / show UI |
| `F5` | Quick Save |
| `F9` | Quick Load |
| `F11` | Toggle Fullscreen |
| `Esc` | Pause / Back |

---

## Documentation

Additional design and specification documents are included in this repository:
- [`PRD.md`](PRD.md) — Product Requirements Document
- [`SRS.md`](SRS.md) — Software Requirements Specification
- [`Design_System.md`](Design_System.md) — UI & Visual Design System

---

## Contributing

Contributions, bug reports, and feature suggestions are welcome!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m "Add my feature"`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the **GNU General Public License v3.0 or later (`GPL-3.0-or-later`)** — see [`Cargo.toml`](Cargo.toml) for details.
