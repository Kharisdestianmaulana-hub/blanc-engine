# Changelog

All notable changes to **Blanc Engine** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-16

### Added
- **Core (`blanc_core`)**:
  - Project manifest (`.blanc`), scene (`.bscene`), screenplay story parser (`.bst`, supporting `[Scene:]`, `INT./EXT.`, and Markdown `#`/`##` scene headings), and node graph (`StoryFlowGraph`) serialization.
  - Pre-play and pre-export **Project Validator** (`validator.rs`) checking missing assets, broken Story Flow links, and invalid scene references.
  - Encrypted `.bpack` asset archive writer and reader.
  - Developer-defined multi-language localization manifest (`locales/manifest.json` with `story_language` support), string tables (`locales/<code>.json`), project string extraction, and RFC-4180 CSV export/import.
  - Windows executable `.ico` resource embedding and standalone installer payload packaging/extraction.
  - Customizable `RuntimeUiConfig` (including dedicated `CreditsUiConfig` and `GameOverUiConfig`) and design tokens.
- **Editor (`blanc_editor`)**:
  - Project Welcome screen with **Learn & Docs** (section-by-section Story Script Format guide), Hub Settings (including **Periodic Auto-Save** interval `1/3/5/10 min`), real-time **Project Overview** checklist, Asset Manager with **Relink Missing Asset**, Story Setup (`? Format Guide` + `Story Script Language`), Scene Editor, Story Flow node canvas (viewport-clipped text), UI Designer with live preview (real project logo, **Credits Roll**, and **Game Over** tabs), Localization tab (vertical `SECTIONS` progress column, `STORY LANGUAGE`, and smart cross-group search), Settings (Project Version & Author), and Export pipeline.
  - **Unsaved Changes Guard & `Ctrl+S`**, **Periodic Auto-Save**, **Undo / Redo (`Ctrl+Z` / `Ctrl+Y`)**, and **Play Preview <-> Close Preview** toggle.
  - Background asynchronous export with sticky progress bar, custom wizard sidebar artwork, per-language Terms & Privacy Policy editor, and automatic Publisher/Studio population from project author.
- **Runtime (`blanc_runtime`)**:
  - Visual novel stage rendering, typewriter dialogue reveal, solid non-transparent non-speaker sprite dimming, character positioning, screen effects (`FadeIn`, `FadeOut`, `FlashWhite`, `Shake`), and `rodio` BGM/SFX playback.
  - Dedicated **Credits Roll** (scrolling credits + fading corner `Continue >>` button) and **Game Over** screens.
  - Save/Load slots with scene thumbnails, timestamps, and preview text, plus Quick Save (`F5`), Quick Load (`F9`), and Dialogue History (`B`).
  - **Auto-Play (`A`)** with configurable delay and **Skip (`S`)** with `Read Only` (persistent `read_history.json`) and `All Text` modes.
  - Dynamic **Language** selector in Settings, installer language auto-sync on first launch, and Windows CJK font loading.
  - Keyboard and gamepad (`gilrs`) navigation.
- **Installer (`blanc_installer`)**:
  - Standalone Windows Setup Wizard with 6-language UI (`en`, `id`, `ja`, `es`, `zh`, `ko`), installation progress bar, Start Menu & Desktop shortcuts, window titlebar icon, and clean per-game `Uninstall <Game>.exe`.
