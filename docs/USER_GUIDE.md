# Blanc Engine User Guide

This guide covers the core workflow for creating, localizing, and distributing a visual novel with **Blanc Engine**.

---

## 1. Creating a Project & Importing Assets
1. Launch the editor (`cargo run -p blanc_editor`).
2. Create a new project or open an existing `.blanc` manifest.
3. Open **Asset Manager** to import:
   - **Backgrounds** (`.png`, `.jpg`, `.webp`)
   - **Characters** (sprites / expressions)
   - **Audio** (BGM, SFX, Voice)
   - **UI & Fonts** (custom buttons, panels, `.ttf`/`.otf` fonts)

---

## 2. Writing in Story Setup
In **Story Setup**, you can draft your script using Blanc Engine's screenplay syntax and convert it directly into scenes:

```text
[Scene: Classroom_Afternoon]
[BGM: peaceful_day]

Alice (Happy): "Hello! Welcome to Blanc Engine."
Narrator: The afternoon sun streamed through the classroom windows.

[Choice: What do you want to do?]
- Go to the rooftop -> Rooftop_Scene
- Stay in the classroom -> Classroom_Evening
```

Click **Convert to Scenes** to generate `.bscene` files automatically.

---

## 3. Editing Scenes & Story Flow
- **Scene Editor**: Fine-tune action blocks (Background, Character position/expression, Dialogue, Narration, Choice, Wait, Visual Effects like `FadeIn`, `FadeOut`, `FlashWhite`, `Shake`).
- **Story Flow**: Visually wire `Main Menu` (`New Game`) -> `Scene` nodes -> `Choice` pins -> `Ending` nodes on the node canvas.

---

## 4. Customizing UI (`UI Designer`)
Use **UI Designer** to customize:
- **Title Screen** (logo, background, button layout & textures)
- **Gameplay HUD** (dialogue box, nameplate, `Q.Save`, `Q.Load`, `Auto`, `Skip`, `History`, `Hide UI`, `Menu`)
- **Pause, Settings, Save/Load, and Backlog** screens.

---

## 5. Multi-Language Support (`Localization`)
Blanc Engine lets you support any number of languages defined by you:

1. Open the **Localization** tab.
2. Confirm your **Source / Default Language** (e.g., `en` / `English`, or `id` / `Bahasa Indonesia`).
3. Under **Add Language**, enter a language code (e.g., `ja`, `ko`, `fr`, `jv`) and display name (e.g., `日本語`, `한국어`, `Français`), then click **`+ Add Language`**.
4. Click **Sync Strings** whenever you add new dialogues or choices in your scenes.
5. Translate directly in the editor or use **Export CSV** / **Import CSV** (`key,context,source,translation`) to translate in Excel or Google Sheets.
6. In the exported game, players can switch languages anytime from **Settings → Language** (shown automatically when more than 1 language is configured).

---

## 6. Exporting & Distributing Your Game
Open the **Export** tab:
- **Portable Build**: Outputs `YourGame.exe` + encrypted `game.bpack` (or loose `GameData/`).
- **Windows Installer (`Setup.exe`)**:
  - Configure **Version**, **Publisher / Studio**, **Welcome Message**, **Terms & Privacy Policy** (per installer language), **Wizard Sidebar Photo**, and **Desktop / Start Menu Shortcuts**.
  - Click **Export Game** and watch the bottom progress bar until `YourGame_Setup.exe` is ready.
  - The generated installer also includes a per-game **`Uninstall <Game>.exe`** registered with Windows *Apps & Features*.
