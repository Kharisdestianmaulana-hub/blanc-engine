# Changelog

All notable changes to **Blanc Engine** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-14

### Added
- **Core (`blanc_core`)**:
  - Project manifest (`.blanc`), scene (`.bscene`), screenplay story parser (`.bst`), and node graph (`StoryFlowGraph`) serialization.
  - Encrypted `.bpack` asset archive writer and reader.
  - Developer-defined multi-language localization manifest (`locales/manifest.json`), string tables (`locales/<code>.json`), project string extraction, and RFC-4180 CSV export/import.
  - Windows executable `.ico` resource embedding and standalone installer payload packaging/extraction.
  - Customizable `RuntimeUiConfig` and design tokens.
- **Editor (`blanc_editor`)**:
  - Project Welcome screen, Overview, Asset Manager, Story Setup screenplay-to-scene converter, Scene Editor, Story Flow node canvas, UI Designer with live preview, Localization tab, Settings, and Export pipeline.
  - Background asynchronous export with sticky progress bar, custom wizard sidebar artwork, per-language Terms & Privacy Policy editor, and automatic Publisher/Studio population from project author.
- **Runtime (`blanc_runtime`)**:
  - Visual novel stage rendering, typewriter dialogue reveal, character positioning, screen effects (`FadeIn`, `FadeOut`, `FlashWhite`, `Shake`), and `rodio` BGM/SFX playback.
  - Save/Load slots with scene thumbnails, timestamps, and preview text, plus Quick Save (`F5`), Quick Load (`F9`), and Dialogue History (`B`).
  - **Auto-Play (`A`)** with configurable delay and **Skip (`S`)** with `Read Only` (persistent `read_history.json`) and `All Text` modes.
  - Dynamic **Language** selector in Settings, installer language auto-sync on first launch, and Windows CJK font loading.
  - Keyboard and gamepad (`gilrs`) navigation.
- **Installer (`blanc_installer`)**:
  - Standalone Windows Setup Wizard with 6-language UI (`en`, `id`, `ja`, `es`, `zh`, `ko`), installation progress bar, Start Menu & Desktop shortcuts, window titlebar icon, and clean per-game `Uninstall <Game>.exe`.
