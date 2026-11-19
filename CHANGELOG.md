# Changelog

All notable changes to **Blanc Engine** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - Unreleased

### Added
- **Core (`blanc_core`)**:
  - Global variable system (`VariableDefinition`, `VariableType`, `VariableValue`) supporting `Boolean` flags, `Number` counters/stats, and `String` text.
  - New action blocks: `ModifyVariable` (Set, Add, Subtract, Multiply, Toggle), `CameraEffect` (Shake, Zoom, Pan), and `ScreenFlash` (White, Red, Custom Color).
  - Story Flow Graph `ConditionNode` for no-code `If / Else If / Else` branch evaluation with multi-variable logical expressions (`AND` / `OR`).
  - Choice requirement & conditional visibility predicates (hide option or disable with lock state).
  - Free-form UI canvas schema (`WidgetTree`, `UiWidget`, `AnchorPreset`, `NineSliceConfig`, and data-binding definitions).
  - Persistent game data storage (`persistent_data.bdata`) for unlockable assets across game sessions and save slots.
  - Rich text dialogue parser (`InlineTagParser`) supporting `{w=sec}` typewriter pauses, `{speed=val}` overrides, `{shake}`, `{wave}`, and `{color=#hex}` styles.
- **Editor (`blanc_editor`)**:
  - Next-gen 3-panel UI Canvas Studio:
    - **Hierarchy Tree**: Screen widget tree with parent-child nesting, reordering, visibility, and lock controls.
    - **Interactive Gizmo Canvas**: 1920x1080 viewport with drag-and-drop widget placement, 8-point resize handles, snap-to-grid, smart alignment guides, and live preview toggle.
    - **Inspector & Binding**: Transform properties, anchor presets, nine-slice texture margins, widget visual states (*Idle*, *Hover*, *Pressed*, *Disabled*), click/hover SFX, and zero-code action/data binding.
  - Variable Manager modal in Project Settings for defining game variables and default initial values.
  - Visual Condition Node editor on the Story Flow canvas with intuitive branch routing.
  - Inline text styling toolbar in Scene Editor for quick insertion of pauses, font formatting, and animated text effects.
  - Asset Manager tagging: `[x] Unlockable in CG Gallery` with custom title, character tagging, and thumbnail selection.
  - In-app asynchronous update checker in the Project Hub header with GitHub Release notifications.
  - Native macOS export target (`.app` application bundle and `.dmg` disk image).
- **Runtime (`blanc_runtime`)**:
  - Free-form hierarchical widget canvas rendering engine replacing rigid template-based screens.
  - Dynamic typewriter text renderer with animated procedural effects (`Shake`, `Wave`), inline color styling, and pause timers.
  - Dynamic 2D camera system: screen shake with adjustable intensity and duration, smooth camera zoom and pan with easing curves.
  - Built-in CG Gallery viewer screen (thumbnail grid with locked silhouettes, multi-page navigation, and fullscreen image viewer).
  - Built-in Music Room screen with BGM track list, unlock-on-hear tracking, and interactive playback controls.
  - Variable and branching condition evaluator integrated with dialogue choices and `.bsave` serialization.
  - Persistent asset unlock management synchronized to `%APPDATA%` (Windows) and `~/Library/Application Support` (macOS).
- **Installer & Distribution (`blanc_installer` / CI-CD)**:
  - macOS Universal 2 Binary packaging (`aarch64-apple-darwin` for Apple Silicon M1-M4 and `x86_64-apple-darwin` for Intel) via `lipo`.
  - Automated multi-platform GitHub Actions CI matrix building Windows `.exe` and macOS `.dmg` on release tags.

## [0.1.1] - 2026-11-20

### Fixed & Improved
- **Core (`blanc_core`)**:
  - Added path-traversal protection (`safe_join_relative`) for installer payload extraction and asset deletion/resolution.
  - Added file-size and offset bounds validation when opening `.bpack` archives and self-extracting installer payloads to prevent out-of-memory crashes on corrupted files.
  - Added `atomic_write` helper (temporary file + rename) for `.blanc`, `.bscene`, `.bgraph`, `.bconfig`, and localization files to prevent data corruption on unexpected shutdowns.
  - Added `validate_project_folder_for_deletion` safety guard to block accidental deletion of user profile or system directories (`Documents`, `Desktop`, drive roots).
- **Runtime (`blanc_runtime`)**:
  - Added full keyboard & gamepad navigation to switch Settings category tabs (`Audio`, `Gameplay`, `System`) using `Left`/`Right` when the tab bar is focused, plus `Tab`, `Q`, and `E` shortcuts.
  - Added cycle/recursion depth guard (`MAX_SCENE_TRANSITION_DEPTH`) during scene transitions so empty or cyclic Story Flow links never trigger a stack overflow or freeze.
  - Fixed `restore_to_point` (Load Save Game) so intermediate `PlayBgm` blocks are not decoded/played multiple times in a single frame, and unified `HideCharacter` identity matching between live gameplay and save restoration.
  - Hardened save-file thumbnail asset resolution and switched save/settings/read-history writes to atomic file writes.
- **Editor (`blanc_editor`)**:
  - Added **Localization & Multi-Language Support** documentation topic to **Learn & Docs**.
  - Updated destructive delete buttons across **Welcome Hub** and **Editor** (projects, scenes, languages) to use the trash icon (`icon_trash`).
  - Added a confirmation dialog when deleting a project from **Project Hub** with options to **Remove from Hub Only** or **Delete Project & Folder** (protected by directory safety validation).
  - Excluded `Builds/` and `Backups/` folders (and symlink cycles) from being packed into `game.bpack` / installer payloads during Export.
  - Verified bundled binary contents in `%LOCALAPPDATA%` by byte comparison and added explicit connect/read timeouts to the **Submit Your Games** HTTP client.
- **Installer (`blanc_installer`)**:
  - Recorded the exact list of installed files in `.blanc_uninstall.json` so uninstallation removes only game-owned files and only removes the install folder if it is empty.
  - Hardened `stop_running_game_process` executable name validation and bounded directory size recursion depth.

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
