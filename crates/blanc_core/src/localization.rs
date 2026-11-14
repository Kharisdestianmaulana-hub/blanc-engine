use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::scene::{ActionBlock, Scene};

pub const LOCALES_DIR: &str = "locales";
pub const LOCALE_MANIFEST_FILE: &str = "manifest.json";

/// A language configured by the game developer in a project's Localization settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageEntry {
    pub code: String,
    pub display_name: String,
}

impl LanguageEntry {
    pub fn new(code: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            code: code.into().trim().to_string(),
            display_name: display_name.into().trim().to_string(),
        }
    }
}

impl fmt::Display for LanguageEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.display_name.is_empty() {
            write!(f, "{}", self.code)
        } else {
            write!(f, "{} ({})", self.display_name, self.code)
        }
    }
}

/// Project-level localization configuration (`locales/manifest.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleManifest {
    pub default_language: LanguageEntry,
    #[serde(default)]
    pub languages: Vec<LanguageEntry>,
}

impl Default for LocaleManifest {
    fn default() -> Self {
        Self {
            default_language: LanguageEntry::new("en", "English"),
            languages: Vec::new(),
        }
    }
}

impl LocaleManifest {
    pub fn manifest_path(project_dir: &Path) -> PathBuf {
        project_dir.join(LOCALES_DIR).join(LOCALE_MANIFEST_FILE)
    }

    pub fn load_from_dir(project_dir: &Path) -> Self {
        let path = Self::manifest_path(project_dir);
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(parsed) = serde_json::from_str::<Self>(&raw) {
                return parsed;
            }
        }
        Self::default()
    }

    pub fn save_to_dir(&self, project_dir: &Path) -> Result<(), String> {
        let dir = project_dir.join(LOCALES_DIR);
        std::fs::create_dir_all(&dir).map_err(|e| format!("Failed to create locales dir: {}", e))?;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize locale manifest: {}", e))?;
        std::fs::write(Self::manifest_path(project_dir), json)
            .map_err(|e| format!("Failed to write locale manifest: {}", e))
    }

    /// Returns default language first, followed by all additional developer-configured languages.
    pub fn all_languages(&self) -> Vec<LanguageEntry> {
        let mut out = Vec::with_capacity(1 + self.languages.len());
        out.push(self.default_language.clone());
        for lang in &self.languages {
            if !lang.code.eq_ignore_ascii_case(&self.default_language.code) {
                out.push(lang.clone());
            }
        }
        out
    }

    pub fn find_language_by_code(&self, code: &str) -> Option<LanguageEntry> {
        self.all_languages()
            .into_iter()
            .find(|l| l.code.eq_ignore_ascii_case(code))
    }
}

/// Sanitize a language code so it is safe as a filename `<code_safe>.json`.
pub fn sanitize_lang_code(code: &str) -> String {
    let cleaned: String = code
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "lang".to_string()
    } else {
        cleaned
    }
}

/// Translation string table for a single language (`locales/<code>.json`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocaleStrings {
    #[serde(default)]
    pub entries: BTreeMap<String, String>,
}

impl LocaleStrings {
    pub fn file_rel_path(code: &str) -> String {
        format!("{}/{}.json", LOCALES_DIR, sanitize_lang_code(code))
    }

    pub fn file_path(project_dir: &Path, code: &str) -> PathBuf {
        project_dir
            .join(LOCALES_DIR)
            .join(format!("{}.json", sanitize_lang_code(code)))
    }

    pub fn load_from_dir(project_dir: &Path, code: &str) -> Self {
        let path = Self::file_path(project_dir, code);
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(parsed) = serde_json::from_str::<Self>(&raw) {
                return parsed;
            }
        }
        Self::default()
    }

    pub fn save_to_dir(&self, project_dir: &Path, code: &str) -> Result<(), String> {
        let dir = project_dir.join(LOCALES_DIR);
        std::fs::create_dir_all(&dir).map_err(|e| format!("Failed to create locales dir: {}", e))?;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize locale strings: {}", e))?;
        std::fs::write(Self::file_path(project_dir, code), json)
            .map_err(|e| format!("Failed to write locale strings: {}", e))
    }
}

/// Category of a localizable item in the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocaleCategory {
    Ui,
    Speaker,
    Scene(String),
}

/// A single localizable text item extracted from UI labels or scene scripts.
#[derive(Debug, Clone)]
pub struct LocalizationItem {
    pub key: String,
    pub category: LocaleCategory,
    pub context_label: String,
    pub source_text: String,
}

/// Standard runtime UI keys: `(key, context_label, default_english_text)`.
pub const STANDARD_UI_KEYS: &[(&str, &str, &str)] = &[
    ("ui.title.continue", "UI • Title Screen", "Continue"),
    ("ui.title.new_game", "UI • Title Screen", "New Game"),
    ("ui.title.load_game", "UI • Title Screen", "Load Game"),
    ("ui.title.settings", "UI • Title Screen", "Settings"),
    ("ui.title.quit", "UI • Title Screen", "Exit"),
    ("ui.pause.title", "UI • Pause Menu", "Paused"),
    ("ui.pause.resume", "UI • Pause Menu", "Resume"),
    ("ui.pause.save", "UI • Pause Menu", "Save Game"),
    ("ui.pause.load", "UI • Pause Menu", "Load Game"),
    ("ui.pause.settings", "UI • Pause Menu", "Settings"),
    ("ui.pause.return_title", "UI • Pause Menu", "Return to Title"),
    ("ui.pause.quit", "UI • Pause Menu", "Quit Game"),
    ("ui.hud.qsave", "UI • In-Game HUD", "Q.Save (F5)"),
    ("ui.hud.qload", "UI • In-Game HUD", "Q.Load (F9)"),
    ("ui.hud.auto", "UI • In-Game HUD", "Auto (A)"),
    ("ui.hud.auto_active", "UI • In-Game HUD", "Auto ●"),
    ("ui.hud.skip", "UI • In-Game HUD", "Skip (S)"),
    ("ui.hud.skip_active", "UI • In-Game HUD", "Skip ▶▶"),
    ("ui.hud.history", "UI • In-Game HUD", "History (B)"),
    ("ui.hud.hide_ui", "UI • In-Game HUD", "Hide UI (H)"),
    ("ui.hud.menu", "UI • In-Game HUD", "Menu (ESC)"),
    ("ui.settings.title", "UI • Settings", "Settings"),
    ("ui.settings.master_vol", "UI • Settings", "Master Volume"),
    ("ui.settings.bgm_vol", "UI • Settings", "BGM Volume"),
    ("ui.settings.sfx_vol", "UI • Settings", "SFX Volume"),
    ("ui.settings.text_speed", "UI • Settings", "Text Speed"),
    ("ui.settings.auto_delay", "UI • Settings", "Auto Delay"),
    ("ui.settings.skip_mode", "UI • Settings", "Skip Mode"),
    ("ui.settings.skip_read", "UI • Settings", "Read Only"),
    ("ui.settings.skip_all", "UI • Settings", "All Text"),
    ("ui.settings.language", "UI • Settings", "Language"),
    ("ui.settings.back", "UI • Settings", "Back"),
    ("ui.saveload.save_title", "UI • Save / Load", "Save Game"),
    ("ui.saveload.load_title", "UI • Save / Load", "Load Game"),
    ("ui.saveload.empty", "UI • Save / Load", "Empty Slot"),
    ("ui.saveload.save_btn", "UI • Save / Load", "Save"),
    ("ui.saveload.load_btn", "UI • Save / Load", "Load"),
    ("ui.saveload.del_btn", "UI • Save / Load", "Del"),
    ("ui.saveload.back", "UI • Save / Load", "Back"),
    ("ui.backlog.title", "UI • History", "Dialogue History"),
    ("ui.backlog.empty", "UI • History", "No dialogue history yet."),
    ("ui.backlog.close", "UI • History", "Close"),
];

pub fn speaker_key(name: &str) -> String {
    format!("speaker.{}", name.trim())
}

pub fn scene_text_key(scene_name: &str, block_index: usize) -> String {
    format!("scene.{}.block.{}.text", scene_name, block_index)
}

pub fn scene_choice_prompt_key(scene_name: &str, block_index: usize) -> String {
    format!("scene.{}.block.{}.prompt", scene_name, block_index)
}

pub fn scene_choice_option_key(scene_name: &str, block_index: usize, opt_index: usize) -> String {
    format!(
        "scene.{}.block.{}.choice.{}",
        scene_name, block_index, opt_index
    )
}

fn collect_bscene_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_bscene_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("bscene") {
            out.push(path);
        }
    }
}

/// Extract all UI labels, unique character speaker names, and scene dialogue/choice items from a project.
pub fn collect_project_items(project_dir: &Path) -> Vec<LocalizationItem> {
    let mut items = Vec::new();
    for &(key, ctx, default_text) in STANDARD_UI_KEYS {
        items.push(LocalizationItem {
            key: key.to_string(),
            category: LocaleCategory::Ui,
            context_label: ctx.to_string(),
            source_text: default_text.to_string(),
        });
    }

    let mut scene_files = Vec::new();
    collect_bscene_files(&project_dir.join("Scenes"), &mut scene_files);

    let mut scenes: Vec<Scene> = Vec::new();
    let mut speaker_names: BTreeSet<String> = BTreeSet::new();

    for path in scene_files {
        if let Ok(scene) = Scene::load_from_file(&path) {
            for block in &scene.blocks {
                if let ActionBlock::Dialogue { speaker, .. } = block {
                    let trimmed = speaker.trim();
                    if !trimmed.is_empty() {
                        speaker_names.insert(trimmed.to_string());
                    }
                }
            }
            scenes.push(scene);
        }
    }

    for name in speaker_names {
        items.push(LocalizationItem {
            key: speaker_key(&name),
            category: LocaleCategory::Speaker,
            context_label: "Character Name".to_string(),
            source_text: name,
        });
    }

    scenes.sort_by(|a, b| a.name.cmp(&b.name));
    for scene in scenes {
        for (idx, block) in scene.blocks.iter().enumerate() {
            match block {
                ActionBlock::Dialogue { speaker, text, .. } => {
                    let who = if speaker.trim().is_empty() {
                        "Dialogue"
                    } else {
                        speaker.trim()
                    };
                    items.push(LocalizationItem {
                        key: scene_text_key(&scene.name, idx),
                        category: LocaleCategory::Scene(scene.name.clone()),
                        context_label: format!("{} • #{} ({})", scene.name, idx + 1, who),
                        source_text: text.clone(),
                    });
                }
                ActionBlock::Narration { text } => {
                    items.push(LocalizationItem {
                        key: scene_text_key(&scene.name, idx),
                        category: LocaleCategory::Scene(scene.name.clone()),
                        context_label: format!("{} • #{} (Narration)", scene.name, idx + 1),
                        source_text: text.clone(),
                    });
                }
                ActionBlock::Choice {
                    prompt, options, ..
                } => {
                    if !prompt.trim().is_empty() {
                        items.push(LocalizationItem {
                            key: scene_choice_prompt_key(&scene.name, idx),
                            category: LocaleCategory::Scene(scene.name.clone()),
                            context_label: format!("{} • #{} (Choice Prompt)", scene.name, idx + 1),
                            source_text: prompt.clone(),
                        });
                    }
                    for (opt_idx, opt) in options.iter().enumerate() {
                        items.push(LocalizationItem {
                            key: scene_choice_option_key(&scene.name, idx, opt_idx),
                            category: LocaleCategory::Scene(scene.name.clone()),
                            context_label: format!(
                                "{} • #{} (Choice #{})",
                                scene.name,
                                idx + 1,
                                opt_idx + 1
                            ),
                            source_text: opt.text.clone(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    items
}

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Export localization items and existing translations to an RFC-4180 CSV string.
pub fn export_csv(items: &[LocalizationItem], strings: &LocaleStrings) -> String {
    let mut out = String::from("key,context,source,translation\r\n");
    for item in items {
        let trans = strings
            .entries
            .get(&item.key)
            .map(|s| s.as_str())
            .unwrap_or("");
        out.push_str(&format!(
            "{},{},{},{}\r\n",
            csv_escape(&item.key),
            csv_escape(&item.context_label),
            csv_escape(&item.source_text),
            csv_escape(trans)
        ));
    }
    out
}

fn parse_csv_rows(input: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
        } else {
            match c {
                '"' => in_quotes = true,
                ',' => {
                    row.push(std::mem::take(&mut field));
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    row.push(std::mem::take(&mut field));
                    rows.push(std::mem::take(&mut row));
                }
                '\n' => {
                    row.push(std::mem::take(&mut field));
                    rows.push(std::mem::take(&mut row));
                }
                _ => field.push(c),
            }
        }
    }

    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }

    rows
}

/// Import translations from CSV (`key,context,source,translation` or `key,translation`).
/// Returns the number of non-empty translations imported.
pub fn import_csv(csv_text: &str, strings: &mut LocaleStrings) -> usize {
    let cleaned = csv_text.trim_start_matches('\u{feff}');
    let rows = parse_csv_rows(cleaned);
    let mut imported = 0usize;

    for (idx, row) in rows.into_iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let key = row[0].trim();
        if idx == 0 && key.eq_ignore_ascii_case("key") {
            continue;
        }
        if key.is_empty() {
            continue;
        }
        let translation = if row.len() >= 4 {
            row[3].clone()
        } else if row.len() >= 2 {
            row[1].clone()
        } else {
            continue;
        };
        if translation.is_empty() {
            strings.entries.remove(key);
        } else {
            strings.entries.insert(key.to_string(), translation);
            imported += 1;
        }
    }

    imported
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_manifest_and_csv_roundtrip() {
        let items = vec![LocalizationItem {
            key: "scene.Intro.block.0.text".to_string(),
            category: LocaleCategory::Scene("Intro".to_string()),
            context_label: "Intro • #1 (Hana)".to_string(),
            source_text: "Hello, \"world\"!\nLine 2".to_string(),
        }];
        let mut strings = LocaleStrings::default();
        strings.entries.insert(
            "scene.Intro.block.0.text".to_string(),
            "Halo, \"dunia\"!\nBaris 2".to_string(),
        );

        let csv = export_csv(&items, &strings);
        let mut reimported = LocaleStrings::default();
        let count = import_csv(&csv, &mut reimported);
        assert_eq!(count, 1);
        assert_eq!(
            reimported.entries.get("scene.Intro.block.0.text").map(String::as_str),
            Some("Halo, \"dunia\"!\nBaris 2")
        );
    }
}
