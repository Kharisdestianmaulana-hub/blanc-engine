use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use blanc_core::asset::AssetCategory;
use blanc_core::flow::FlowNodeType;
use blanc_core::installer::{UninstallMetadata, UNINSTALL_META_FILENAME};
use blanc_core::localization::{
    scene_choice_option_key, scene_choice_prompt_key, scene_text_key, speaker_key, LanguageEntry,
};
use blanc_core::scene::{
    ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption, SceneEndAction,
    ScreenEffectType, TransitionType,
};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use serde::{Deserialize, Serialize};

use crate::loader::LoadedProject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SkipMode {
    #[default]
    ReadOnly,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RuntimeUserSettings {
    #[serde(default)]
    skip_mode: SkipMode,
    #[serde(default = "default_auto_delay")]
    auto_forward_delay: f32,
    #[serde(default)]
    language: Option<String>,
}

fn default_auto_delay() -> f32 {
    2.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadMode {
    Save,
    Load,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeScreen {
    TitleScreen,
    Playing,
    PauseMenu,
    Settings { return_to_pause: bool },
    SaveLoad { mode: SaveLoadMode, return_to_pause: bool },
    Backlog,
    Ending { title: String, ending_node_id: Option<String> },
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveCharacterThumb {
    pub image_path: String,
    pub position: CharacterPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotData {
    pub scene_name: String,
    pub block_index: usize,
    #[serde(default)]
    pub timestamp_label: String,
    #[serde(default)]
    pub preview_text: String,
    #[serde(default)]
    pub saved_at_unix: u64,
    #[serde(default)]
    pub thumbnail_bg: Option<String>,
    #[serde(default)]
    pub thumbnail_characters: Vec<SaveCharacterThumb>,
}

#[derive(Debug, Clone)]
pub struct BacklogEntry {
    pub speaker: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ActiveCharacter {
    pub character_id: String,
    pub expression: String,
    pub position: CharacterPosition,
    #[allow(dead_code)]
    pub offset_x: f32,
    #[allow(dead_code)]
    pub offset_y: f32,
    pub anim_offset_x: f32,
    pub image_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ActiveDialogue {
    pub speaker: Option<String>,
    pub nameplate_position: Option<CharacterPosition>,
    pub full_text: String,
    pub visible_chars: usize,
    pub total_chars: usize,
}

impl ActiveDialogue {
    pub fn new(speaker: Option<String>, text: String) -> Self {
        let total_chars = text.chars().count();
        Self {
            speaker,
            nameplate_position: None,
            full_text: text,
            visible_chars: 0,
            total_chars,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.visible_chars >= self.total_chars
    }

    pub fn complete_now(&mut self) {
        self.visible_chars = self.total_chars;
    }

    #[allow(dead_code)]
    pub fn visible_slice(&self) -> String {
        if self.visible_chars >= self.total_chars {
            self.full_text.clone()
        } else {
            self.full_text.chars().take(self.visible_chars).collect()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveChoice {
    pub prompt: String,
    pub options: Vec<ChoiceOption>,
    #[allow(dead_code)]
    pub position: ChoiceLayoutPosition,
}

#[derive(Debug, Clone, Default)]
pub struct StageState {
    pub current_scene_name: String,
    pub block_index: usize,
    pub background_path: Option<PathBuf>,
    pub characters: Vec<ActiveCharacter>,
    pub dialogue: Option<ActiveDialogue>,
    pub active_choice: Option<ActiveChoice>,
    pub wait_remaining: Option<f32>,
    pub ui_hidden: bool,
    pub transition_hide_until_fade_in: bool,
    pub current_bgm_id: Option<String>,
    pub backlog: Vec<BacklogEntry>,
    pub fade_alpha: f32,
    pub fade_target: f32,
    pub fade_speed: f32,
    pub flash_alpha: f32,
    pub shake_remaining: f32,
    pub shake_phase: f32,
}

pub struct AudioController {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    bgm_sink: Option<Sink>,
}

impl Default for AudioController {
    fn default() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Self {
                _stream: Some(stream),
                stream_handle: Some(handle),
                bgm_sink: None,
            },
            Err(_) => Self {
                _stream: None,
                stream_handle: None,
                bgm_sink: None,
            },
        }
    }
}

impl AudioController {
    pub fn play_bgm(&mut self, path: &PathBuf, volume: f32, looping: bool) {
        self.stop_bgm();
        let Some(handle) = &self.stream_handle else {
            return;
        };
        let Ok(file) = File::open(path) else {
            return;
        };
        let Ok(source) = Decoder::new(BufReader::new(file)) else {
            return;
        };
        let Ok(sink) = Sink::try_new(handle) else {
            return;
        };
        sink.set_volume(volume.clamp(0.0, 1.0));
        if looping {
            sink.append(source.repeat_infinite());
        } else {
            sink.append(source);
        }
        self.bgm_sink = Some(sink);
    }

    pub fn stop_bgm(&mut self) {
        if let Some(sink) = self.bgm_sink.take() {
            sink.stop();
        }
    }

    pub fn play_bgm_bytes(&mut self, bytes: Vec<u8>, volume: f32, looping: bool) {
        self.stop_bgm();
        let Some(handle) = &self.stream_handle else {
            return;
        };
        let Ok(source) = Decoder::new(std::io::Cursor::new(bytes)) else {
            return;
        };
        let Ok(sink) = Sink::try_new(handle) else {
            return;
        };
        sink.set_volume(volume.clamp(0.0, 1.0));
        if looping {
            sink.append(source.repeat_infinite());
        } else {
            sink.append(source);
        }
        self.bgm_sink = Some(sink);
    }

    pub fn set_bgm_volume(&self, volume: f32) {
        if let Some(sink) = &self.bgm_sink {
            sink.set_volume(volume.clamp(0.0, 1.0));
        }
    }

    pub fn play_sfx(&self, path: &PathBuf, volume: f32) {
        let Some(handle) = &self.stream_handle else {
            return;
        };
        let Ok(file) = File::open(path) else {
            return;
        };
        let Ok(source) = Decoder::new(BufReader::new(file)) else {
            return;
        };
        if let Ok(sink) = Sink::try_new(handle) {
            sink.set_volume(volume.clamp(0.0, 1.0));
            sink.append(source);
            sink.detach();
        }
    }

    pub fn play_sfx_bytes(&self, bytes: Vec<u8>, volume: f32) {
        let Some(handle) = &self.stream_handle else {
            return;
        };
        let Ok(source) = Decoder::new(std::io::Cursor::new(bytes)) else {
            return;
        };
        if let Ok(sink) = Sink::try_new(handle) {
            sink.set_volume(volume.clamp(0.0, 1.0));
            sink.append(source);
            sink.detach();
        }
    }
}

pub struct RuntimeEngine {
    pub loaded: Option<LoadedProject>,
    pub screen: RuntimeScreen,
    pub stage: StageState,
    pub audio: AudioController,
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub text_speed: f32,
    pub auto_play: bool,
    pub skip_mode_active: bool,
    pub skip_rule: SkipMode,
    pub auto_forward_delay: f32,
    pub active_language: String,
    pub focus_index: usize,
    pub ui_anim_time: f32,
    pub toast: Option<(String, std::time::Instant)>,
    char_accumulator: f32,
    auto_timer: f32,
    skip_timer: f32,
    read_lines: std::collections::HashSet<String>,
    read_lines_dirty: bool,
    current_line_was_read: bool,
}

impl RuntimeEngine {
    pub fn new(project_dir: Option<PathBuf>, start_scene: Option<String>) -> Self {
        let mut engine = Self {
            loaded: None,
            screen: RuntimeScreen::TitleScreen,
            stage: StageState::default(),
            audio: AudioController::default(),
            master_volume: 1.0,
            bgm_volume: 0.8,
            sfx_volume: 1.0,
            text_speed: 1.0,
            auto_play: false,
            skip_mode_active: false,
            skip_rule: SkipMode::ReadOnly,
            auto_forward_delay: 2.0,
            active_language: "en".to_string(),
            focus_index: 0,
            ui_anim_time: 0.0,
            toast: None,
            char_accumulator: 0.0,
            auto_timer: 0.0,
            skip_timer: 0.0,
            read_lines: std::collections::HashSet::new(),
            read_lines_dirty: false,
            current_line_was_read: false,
        };

        match project_dir {
            Some(dir) => {
                let bpack = if dir.is_file() && dir.extension().and_then(|e| e.to_str()) == Some("bpack") {
                    Some(dir.clone())
                } else if dir.join("game.bpack").is_file() {
                    Some(dir.join("game.bpack"))
                } else {
                    None
                };
                let load_res = if let Some(pack_path) = bpack {
                    let exe_dir = pack_path.parent().unwrap_or_else(|| std::path::Path::new("."));
                    LoadedProject::load_from_bpack(exe_dir, &pack_path)
                } else {
                    LoadedProject::load_from_dir(&dir)
                };
                match load_res {
                    Ok(loaded) => {
                        engine.master_volume = loaded.config.master_volume;
                        engine.bgm_volume = loaded.config.bgm_volume;
                        engine.sfx_volume = loaded.config.sfx_volume;
                        engine.text_speed = loaded.config.text_speed.max(0.25);
                        engine.auto_forward_delay = loaded.config.auto_forward_delay.clamp(0.5, 5.0);
                        engine.active_language = loaded.locale_manifest.default_language.code.clone();
                        engine.loaded = Some(loaded);
                        engine.load_user_settings();
                        engine.load_read_history();
                        if let Some(scene_name) = start_scene {
                            engine.load_scene(&scene_name);
                        } else {
                            engine.enter_title_screen();
                        }
                    }
                    Err(err) => {
                        engine.screen = RuntimeScreen::Error(err);
                    }
                }
            }
            None => {
                engine.screen = RuntimeScreen::Error(
                    "No project directory provided.\nLaunch from Blanc Editor using 'Play Preview', or pass a project folder path as an argument.".to_string(),
                );
            }
        }

        engine
    }

    pub fn enter_title_screen(&mut self) {
        self.stop_auto_and_skip();
        self.save_read_history();
        self.screen = RuntimeScreen::TitleScreen;
        self.stage = StageState::default();
        self.focus_index = 0;
        self.audio.stop_bgm();

        if let Some(loaded) = &self.loaded {
            if let Some(bgm_id) = &loaded.config.title_bgm_asset_id {
                if let Some(path) = loaded.resolve_asset_path(bgm_id, Some(AssetCategory::Bgm)) {
                    let vol = self.master_volume * self.bgm_volume;
                    if loaded.pack.is_some() {
                        if let Some(bytes) = loaded.read_path_bytes(&path) {
                            self.audio.play_bgm_bytes(bytes, vol, true);
                        }
                    } else {
                        self.audio.play_bgm(&path, vol, true);
                    }
                }
            }
        }
    }

    pub fn start_new_game(&mut self) {
        let start_scene = self
            .loaded
            .as_ref()
            .and_then(|l| l.resolve_starting_scene());

        match start_scene {
            Some(scene_name) => {
                self.audio.stop_bgm();
                self.stage = StageState::default();
                self.load_scene(&scene_name);
            }
            None => {
                self.screen = RuntimeScreen::Ending {
                    title: "No Scenes Found".to_string(),
                    ending_node_id: None,
                };
            }
        }
    }

    fn settings_path(&self) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join("settings.json"))
    }

    fn read_history_path(&self) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join("read_history.json"))
    }

    fn load_user_settings(&mut self) {
        if let Some(path) = self.settings_path() {
            if let Ok(raw) = std::fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<RuntimeUserSettings>(&raw) {
                    self.skip_rule = settings.skip_mode;
                    self.auto_forward_delay = settings.auto_forward_delay.clamp(0.5, 5.0);
                    if let (Some(lang), Some(loaded)) = (settings.language, &self.loaded) {
                        if let Some(found) = loaded.locale_manifest.find_language_by_code(&lang) {
                            self.active_language = found.code;
                        }
                    }
                    return;
                }
            }
        }

        // First launch without settings.json: inherit installer language if supported by project
        if let Some(loaded) = &self.loaded {
            let meta_candidates = [
                std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|d| d.join(UNINSTALL_META_FILENAME))),
                Some(loaded.project_dir.join(UNINSTALL_META_FILENAME)),
            ];
            for cand in meta_candidates.into_iter().flatten() {
                if let Ok(bytes) = std::fs::read(&cand) {
                    if let Ok(meta) = serde_json::from_slice::<UninstallMetadata>(&bytes) {
                        if let Some(inst_lang) = meta.selected_lang {
                            if let Some(found) = loaded.locale_manifest.find_language_by_code(&inst_lang) {
                                self.active_language = found.code;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn save_user_settings(&self) {
        let Some(path) = self.settings_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let settings = RuntimeUserSettings {
            skip_mode: self.skip_rule,
            auto_forward_delay: self.auto_forward_delay,
            language: Some(self.active_language.clone()),
        };
        if let Ok(json) = serde_json::to_string_pretty(&settings) {
            let _ = std::fs::write(path, json);
        }
    }

    pub fn available_languages(&self) -> Vec<LanguageEntry> {
        self.loaded
            .as_ref()
            .map(|l| l.locale_manifest.all_languages())
            .unwrap_or_default()
    }

    pub fn current_language_entry(&self) -> LanguageEntry {
        if let Some(loaded) = &self.loaded {
            if let Some(found) = loaded.locale_manifest.find_language_by_code(&self.active_language) {
                return found;
            }
            return loaded.locale_manifest.default_language.clone();
        }
        LanguageEntry::new("en", "English")
    }

    pub fn set_language(&mut self, code: String) {
        if let Some(loaded) = &self.loaded {
            if let Some(found) = loaded.locale_manifest.find_language_by_code(&code) {
                self.active_language = found.code;
            } else {
                self.active_language = code;
            }
        } else {
            self.active_language = code;
        }
        self.save_user_settings();
        self.refresh_active_dialogue_translation();
    }

    pub fn tr_or(&self, key: &str, fallback: &str) -> String {
        if let Some(loaded) = &self.loaded {
            let lang_key = self.active_language.to_ascii_lowercase();
            if let Some(table) = loaded.locale_tables.get(&lang_key) {
                if let Some(val) = table.entries.get(key) {
                    if !val.is_empty() {
                        return val.clone();
                    }
                }
            }
        }
        fallback.to_string()
    }

    pub fn tr_speaker(&self, speaker: &str) -> String {
        self.tr_or(&speaker_key(speaker), speaker)
    }

    fn refresh_active_dialogue_translation(&mut self) {
        let Some(loaded) = &self.loaded else {
            return;
        };
        let Some(scene) = loaded.get_scene(&self.stage.current_scene_name) else {
            return;
        };
        let Some(block) = scene.blocks.get(self.stage.block_index).cloned() else {
            return;
        };
        match block {
            ActionBlock::Dialogue { speaker, text, .. } => {
                let spk = if speaker.trim().is_empty() {
                    None
                } else {
                    Some(self.tr_speaker(&speaker))
                };
                let translated = self.tr_or(
                    &scene_text_key(&self.stage.current_scene_name, self.stage.block_index),
                    &text,
                );
                if let Some(dlg) = &mut self.stage.dialogue {
                    let was_complete = dlg.is_complete();
                    dlg.speaker = spk;
                    dlg.total_chars = translated.chars().count();
                    dlg.full_text = translated;
                    if was_complete {
                        dlg.visible_chars = dlg.total_chars;
                    } else {
                        dlg.visible_chars = dlg.visible_chars.min(dlg.total_chars);
                    }
                }
            }
            ActionBlock::Narration { text } => {
                let translated = self.tr_or(
                    &scene_text_key(&self.stage.current_scene_name, self.stage.block_index),
                    &text,
                );
                if let Some(dlg) = &mut self.stage.dialogue {
                    let was_complete = dlg.is_complete();
                    dlg.total_chars = translated.chars().count();
                    dlg.full_text = translated;
                    if was_complete {
                        dlg.visible_chars = dlg.total_chars;
                    } else {
                        dlg.visible_chars = dlg.visible_chars.min(dlg.total_chars);
                    }
                }
            }
            ActionBlock::Choice { prompt, mut options, .. } => {
                let prompt_disp = if prompt.trim().is_empty() {
                    prompt
                } else {
                    self.tr_or(
                        &scene_choice_prompt_key(&self.stage.current_scene_name, self.stage.block_index),
                        &prompt,
                    )
                };
                for (opt_idx, opt) in options.iter_mut().enumerate() {
                    opt.text = self.tr_or(
                        &scene_choice_option_key(
                            &self.stage.current_scene_name,
                            self.stage.block_index,
                            opt_idx,
                        ),
                        &opt.text,
                    );
                }
                if let Some(choice) = &mut self.stage.active_choice {
                    choice.prompt = prompt_disp;
                    choice.options = options;
                }
            }
            _ => {}
        }
    }

    fn load_read_history(&mut self) {
        let Some(path) = self.read_history_path() else {
            return;
        };
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(items) = serde_json::from_str::<Vec<String>>(&raw) {
                self.read_lines = items.into_iter().collect();
            }
        }
    }

    pub fn save_read_history(&mut self) {
        if !self.read_lines_dirty {
            return;
        }
        let Some(path) = self.read_history_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let items: Vec<&String> = self.read_lines.iter().collect();
        if let Ok(json) = serde_json::to_string(&items) {
            if std::fs::write(path, json).is_ok() {
                self.read_lines_dirty = false;
            }
        }
    }

    pub fn stop_auto_and_skip(&mut self) {
        let was_skip = self.skip_mode_active;
        self.auto_play = false;
        self.skip_mode_active = false;
        self.auto_timer = 0.0;
        self.skip_timer = 0.0;
        if was_skip {
            self.save_read_history();
        }
    }

    pub fn toggle_auto_play(&mut self) {
        if self.screen != RuntimeScreen::Playing || self.stage.active_choice.is_some() {
            return;
        }
        self.skip_mode_active = false;
        self.skip_timer = 0.0;
        self.auto_play = !self.auto_play;
        self.auto_timer = 0.0;
    }

    pub fn toggle_skip(&mut self) {
        if self.screen != RuntimeScreen::Playing || self.stage.active_choice.is_some() {
            return;
        }
        self.auto_play = false;
        self.auto_timer = 0.0;
        if !self.skip_mode_active && self.skip_rule == SkipMode::ReadOnly && !self.current_line_was_read {
            self.show_toast("Unread text (Skip: Read Only)");
            return;
        }
        self.skip_mode_active = !self.skip_mode_active;
        self.skip_timer = 0.0;
        if !self.skip_mode_active {
            self.save_read_history();
        }
    }

    pub fn set_skip_rule(&mut self, rule: SkipMode) {
        self.skip_rule = rule;
        self.save_user_settings();
    }

    pub fn set_auto_forward_delay(&mut self, delay: f32) {
        self.auto_forward_delay = delay.clamp(0.5, 5.0);
        self.save_user_settings();
    }

    fn mark_current_line_read(&mut self) {
        let key = format!("{}::{}", self.stage.current_scene_name, self.stage.block_index);
        self.current_line_was_read = self.read_lines.contains(&key);
        if self.read_lines.insert(key) {
            self.read_lines_dirty = true;
        }
    }

    fn autosave_path(&self) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join("autosave.bsave"))
    }

    fn legacy_autosave_path(&self) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join("autosave.json"))
    }

    fn quicksave_path(&self) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join("quicksave.bsave"))
    }

    fn save_slot_path(&self, slot: usize) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join(format!("slot_{}.bsave", slot)))
    }

    fn legacy_save_slot_path(&self, slot: usize) -> Option<PathBuf> {
        self.loaded
            .as_ref()
            .map(|l| l.save_dir.join(format!("slot_{}.json", slot)))
    }

    fn read_save_file(primary: Option<PathBuf>, fallback: Option<PathBuf>) -> Option<SaveSlotData> {
        for path in [primary, fallback].into_iter().flatten() {
            if let Ok(raw) = std::fs::read_to_string(path) {
                if let Ok(save) = serde_json::from_str::<SaveSlotData>(&raw) {
                    return Some(save);
                }
            }
        }
        None
    }

    pub fn show_toast(&mut self, message: impl Into<String>) {
        self.toast = Some((message.into(), std::time::Instant::now()));
    }

    pub fn expire_toast(&mut self) {
        if let Some((_, created)) = &self.toast {
            if created.elapsed().as_millis() >= 1800 {
                self.toast = None;
            }
        }
    }

    fn encode_save_asset_path(&self, path: &PathBuf) -> String {
        if let Some(loaded) = &self.loaded {
            if let Ok(rel) = path.strip_prefix(&loaded.project_dir) {
                return rel.to_string_lossy().replace('\\', "/");
            }
        }
        path.to_string_lossy().into_owned()
    }

    pub fn resolve_save_asset_path(&self, raw: &str) -> Option<PathBuf> {
        let p = PathBuf::from(raw);
        if p.is_absolute() && p.exists() {
            return Some(p);
        }
        self.loaded
            .as_ref()
            .map(|l| l.project_dir.join(raw))
            .filter(|candidate| candidate.exists())
    }

    fn build_save_data(&self) -> SaveSlotData {
        let preview_text = self
            .stage
            .dialogue
            .as_ref()
            .map(|d| d.full_text.chars().take(64).collect::<String>())
            .unwrap_or_else(|| self.stage.current_scene_name.clone());
        let saved_at_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let thumbnail_bg = self
            .stage
            .background_path
            .as_ref()
            .map(|p| self.encode_save_asset_path(p));
        let thumbnail_characters = self
            .stage
            .characters
            .iter()
            .filter_map(|c| {
                c.image_path.as_ref().map(|p| SaveCharacterThumb {
                    image_path: self.encode_save_asset_path(p),
                    position: c.position,
                })
            })
            .collect();
        SaveSlotData {
            scene_name: self.stage.current_scene_name.clone(),
            block_index: self.stage.block_index,
            timestamp_label: format_utc_timestamp(),
            preview_text,
            saved_at_unix,
            thumbnail_bg,
            thumbnail_characters,
        }
    }

    pub fn has_quicksave(&self) -> bool {
        self.quicksave_path().map(|p| p.exists()).unwrap_or(false)
    }

    pub fn has_save_data(&self) -> bool {
        if self.autosave_path().map(|p| p.exists()).unwrap_or(false)
            || self.legacy_autosave_path().map(|p| p.exists()).unwrap_or(false)
            || self.has_quicksave()
        {
            return true;
        }
        (1..=6).any(|slot| {
            self.save_slot_path(slot).map(|p| p.exists()).unwrap_or(false)
                || self.legacy_save_slot_path(slot).map(|p| p.exists()).unwrap_or(false)
        })
    }

    pub fn list_save_slots(&self) -> Vec<Option<SaveSlotData>> {
        (1..=6)
            .map(|slot| Self::read_save_file(self.save_slot_path(slot), self.legacy_save_slot_path(slot)))
            .collect()
    }

    pub fn save_autosave(&self) {
        let Some(path) = self.autosave_path() else {
            return;
        };
        if self.stage.current_scene_name.is_empty() {
            return;
        }
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.build_save_data()) {
            let _ = std::fs::write(path, json);
        }
    }

    pub fn save_to_slot(&self, slot: usize) {
        let Some(path) = self.save_slot_path(slot) else {
            return;
        };
        if self.stage.current_scene_name.is_empty() {
            return;
        }
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.build_save_data()) {
            let _ = std::fs::write(path, json);
        }
    }

    pub fn delete_save_slot(&self, slot: usize) {
        if let Some(path) = self.save_slot_path(slot) {
            let _ = std::fs::remove_file(path);
        }
        if let Some(path) = self.legacy_save_slot_path(slot) {
            let _ = std::fs::remove_file(path);
        }
    }

    pub fn load_from_slot(&mut self, slot: usize) {
        let Some(save) = Self::read_save_file(self.save_slot_path(slot), self.legacy_save_slot_path(slot)) else {
            return;
        };
        self.restore_to_point(&save.scene_name, save.block_index);
    }

    pub fn quick_save(&mut self) -> bool {
        let Some(path) = self.quicksave_path() else {
            return false;
        };
        if self.stage.current_scene_name.is_empty() {
            return false;
        }
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.build_save_data()) {
            if std::fs::write(path, json).is_ok() {
                self.show_toast("Quick Saved");
                return true;
            }
        }
        false
    }

    pub fn quick_load(&mut self) -> bool {
        let Some(save) = Self::read_save_file(self.quicksave_path(), None) else {
            self.show_toast("No quick save found");
            return false;
        };
        self.restore_to_point(&save.scene_name, save.block_index);
        self.show_toast("Quick Loaded");
        true
    }

    pub fn continue_game(&mut self) {
        let autosave = Self::read_save_file(self.autosave_path(), self.legacy_autosave_path());
        let quicksave = Self::read_save_file(self.quicksave_path(), None);
        let latest_manual = self
            .list_save_slots()
            .into_iter()
            .flatten()
            .max_by_key(|s| s.saved_at_unix);

        let chosen = [autosave, quicksave, latest_manual]
            .into_iter()
            .flatten()
            .max_by_key(|s| s.saved_at_unix);

        if let Some(save) = chosen {
            self.restore_to_point(&save.scene_name, save.block_index);
        }
    }

    fn restore_to_point(&mut self, scene_name: &str, target_block: usize) {
        let exists = self
            .loaded
            .as_ref()
            .and_then(|l| l.get_scene(scene_name))
            .is_some();
        if !exists {
            return;
        }

        self.audio.stop_bgm();
        self.stage = StageState::default();
        self.stage.current_scene_name = scene_name.to_string();
        self.stage.block_index = 0;
        self.screen = RuntimeScreen::Playing;

        while self.stage.block_index < target_block {
            let block_opt = {
                let Some(loaded) = &self.loaded else {
                    break;
                };
                let Some(scene) = loaded.get_scene(&self.stage.current_scene_name) else {
                    break;
                };
                scene.blocks.get(self.stage.block_index).cloned()
            };
            let Some(block) = block_opt else {
                break;
            };
            match block {
                ActionBlock::Background { asset_id, .. } => {
                    if let Some(loaded) = &self.loaded {
                        self.stage.background_path =
                            loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Background));
                    }
                }
                ActionBlock::Character {
                    character_id,
                    expression,
                    position,
                    offset_x,
                    offset_y,
                    sprite_asset_id,
                    ..
                } => {
                    let image_path = self.loaded.as_ref().and_then(|loaded| {
                        sprite_asset_id
                            .as_deref()
                            .and_then(|id| loaded.resolve_asset_path(id, Some(AssetCategory::Character)))
                            .or_else(|| loaded.resolve_character_path(&character_id, &expression))
                    });
                    self.stage.characters.retain(|c| {
                        !same_character_identity(&c.character_id, &character_id)
                            && c.position != position
                    });
                    self.stage.characters.push(ActiveCharacter {
                        character_id,
                        expression,
                        position,
                        offset_x,
                        offset_y,
                        anim_offset_x: 0.0,
                        image_path,
                    });
                }
                ActionBlock::HideCharacter { character_id, .. } => {
                    self.stage
                        .characters
                        .retain(|c| !same_character_identity(&c.character_id, &character_id));
                }
                ActionBlock::PlayBgm {
                    asset_id,
                    volume,
                    looping,
                } => {
                    if let Some(loaded) = &self.loaded {
                        if let Some(p) = loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Bgm)) {
                            let effective = (self.master_volume * self.bgm_volume * volume).clamp(0.0, 1.0);
                            if loaded.pack.is_some() {
                                if let Some(bytes) = loaded.read_path_bytes(&p) {
                                    self.audio.play_bgm_bytes(bytes, effective, looping);
                                }
                            } else {
                                self.audio.play_bgm(&p, effective, looping);
                            }
                        }
                    }
                    self.stage.current_bgm_id = Some(asset_id);
                }
                ActionBlock::StopBgm { .. } => {
                    self.audio.stop_bgm();
                    self.stage.current_bgm_id = None;
                }
                ActionBlock::Dialogue { speaker, text, .. } => {
                    let spk = if speaker.trim().is_empty() { None } else { Some(speaker) };
                    self.push_backlog_entry(spk, text);
                }
                ActionBlock::Narration { text } => {
                    self.push_backlog_entry(None, text);
                }
                _ => {}
            }
            self.stage.block_index += 1;
        }
        self.step_blocks();
    }

    fn push_backlog_entry(&mut self, speaker: Option<String>, text: String) {
        if let Some(last) = self.stage.backlog.last() {
            if last.speaker == speaker && last.text == text {
                return;
            }
        }
        self.stage.backlog.push(BacklogEntry { speaker, text });
    }

    pub fn load_scene(&mut self, scene_name: &str) {
        let exists = self
            .loaded
            .as_ref()
            .and_then(|l| l.get_scene(scene_name))
            .is_some();

        if !exists {
            self.screen = RuntimeScreen::Ending {
                title: format!("Scene Not Found: {}", scene_name),
                ending_node_id: None,
            };
            return;
        }

        let transition_mode = self
            .loaded
            .as_ref()
            .map(|l| l.config.scene_transition_mode)
            .unwrap_or(blanc_core::config::SceneTransitionMode::CleanFade);

        self.stage.current_scene_name = scene_name.to_string();
        self.stage.block_index = 0;
        self.stage.dialogue = None;
        self.stage.active_choice = None;
        self.stage.wait_remaining = None;
        if transition_mode == blanc_core::config::SceneTransitionMode::CleanFade {
            self.stage.characters.clear();
            self.stage.fade_alpha = 1.0;
            self.stage.fade_target = 0.0;
            self.stage.fade_speed = 1.9;
            self.stage.transition_hide_until_fade_in = true;
        } else {
            self.stage.transition_hide_until_fade_in = false;
        }
        self.screen = RuntimeScreen::Playing;
        self.step_blocks();
        self.save_autosave();
    }

    /// Execute non-blocking setup blocks sequentially until an interactive block or scene end
    pub fn step_blocks(&mut self) {
        loop {
            let block_opt = {
                let Some(loaded) = &self.loaded else {
                    return;
                };
                let Some(scene) = loaded.get_scene(&self.stage.current_scene_name) else {
                    return;
                };
                scene.blocks.get(self.stage.block_index).cloned()
            };

            let Some(block) = block_opt else {
                // Reached end of blocks in this scene
                self.handle_scene_finished(None);
                return;
            };

            match block {
                ActionBlock::Background { asset_id, transition } => {
                    if let Some(loaded) = &self.loaded {
                        self.stage.background_path =
                            loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Background));
                    }
                    if matches!(transition, Some(TransitionType::Fade | TransitionType::Dissolve)) {
                        self.stage.fade_alpha = 0.85;
                        self.stage.fade_target = 0.0;
                        self.stage.fade_speed = 2.2;
                    }
                    self.stage.block_index += 1;
                }
                ActionBlock::Character {
                    character_id,
                    sprite_asset_id,
                    expression,
                    position,
                    offset_x,
                    offset_y,
                    transition,
                } => {
                    let image_path = self.loaded.as_ref().and_then(|l| {
                        sprite_asset_id
                            .as_deref()
                            .and_then(|id| l.resolve_asset_path(id, Some(AssetCategory::Character)))
                            .or_else(|| l.resolve_character_path(&character_id, &expression))
                    });

                    // Replace any existing sprite for the same character (even across expressions/slots)
                    // or any sprite occupying the exact same screen position slot.
                    self.stage.characters.retain(|c| {
                        !same_character_identity(&c.character_id, &character_id)
                            && c.position != position
                    });

                    let anim_offset_x = match transition {
                        Some(TransitionType::SlideLeft) => -220.0,
                        Some(TransitionType::SlideRight) => 220.0,
                        _ => 0.0,
                    };

                    self.stage.characters.push(ActiveCharacter {
                        character_id,
                        expression,
                        position,
                        offset_x,
                        offset_y,
                        anim_offset_x,
                        image_path,
                    });
                    self.stage.block_index += 1;
                }
                ActionBlock::HideCharacter { character_id, .. } => {
                    self.stage
                        .characters
                        .retain(|c| !c.character_id.eq_ignore_ascii_case(&character_id));
                    self.stage.block_index += 1;
                }
                ActionBlock::PlayBgm {
                    asset_id,
                    volume,
                    looping,
                } => {
                    if let Some(loaded) = &self.loaded {
                        if let Some(path) = loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Bgm)) {
                            let effective = (self.master_volume * self.bgm_volume * volume).clamp(0.0, 1.0);
                            if loaded.pack.is_some() {
                                if let Some(bytes) = loaded.read_path_bytes(&path) {
                                    self.audio.play_bgm_bytes(bytes, effective, looping);
                                }
                            } else {
                                self.audio.play_bgm(&path, effective, looping);
                            }
                        }
                    }
                    self.stage.current_bgm_id = Some(asset_id);
                    self.stage.block_index += 1;
                }
                ActionBlock::StopBgm { .. } => {
                    self.audio.stop_bgm();
                    self.stage.current_bgm_id = None;
                    self.stage.block_index += 1;
                }
                ActionBlock::PlaySfx { asset_id, volume } => {
                    if let Some(loaded) = &self.loaded {
                        if let Some(path) = loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Sfx)) {
                            let effective = (self.master_volume * self.sfx_volume * volume).clamp(0.0, 1.0);
                            if loaded.pack.is_some() {
                                if let Some(bytes) = loaded.read_path_bytes(&path) {
                                    self.audio.play_sfx_bytes(bytes, effective);
                                }
                            } else {
                                self.audio.play_sfx(&path, effective);
                            }
                        }
                    }
                    self.stage.block_index += 1;
                }
                ActionBlock::Dialogue { speaker, text, nameplate_position, .. } => {
                    self.stage.active_choice = None;
                    self.mark_current_line_read();
                    self.auto_timer = 0.0;
                    let spk = if speaker.trim().is_empty() {
                        None
                    } else {
                        Some(self.tr_speaker(&speaker))
                    };
                    let text_disp = self.tr_or(
                        &scene_text_key(&self.stage.current_scene_name, self.stage.block_index),
                        &text,
                    );
                    self.push_backlog_entry(spk.clone(), text_disp.clone());
                    let mut dlg = ActiveDialogue::new(spk, text_disp);
                    dlg.nameplate_position = nameplate_position;
                    if self.text_speed >= 2.0 {
                        dlg.complete_now();
                    }
                    self.stage.dialogue = Some(dlg);
                    self.char_accumulator = 0.0;
                    return;
                }
                ActionBlock::Narration { text } => {
                    self.stage.active_choice = None;
                    self.mark_current_line_read();
                    self.auto_timer = 0.0;
                    let text_disp = self.tr_or(
                        &scene_text_key(&self.stage.current_scene_name, self.stage.block_index),
                        &text,
                    );
                    self.push_backlog_entry(None, text_disp.clone());
                    let mut dlg = ActiveDialogue::new(None, text_disp);
                    if self.text_speed >= 2.0 {
                        dlg.complete_now();
                    }
                    self.stage.dialogue = Some(dlg);
                    self.char_accumulator = 0.0;
                    return;
                }
                ActionBlock::Choice {
                    prompt,
                    mut options,
                    position,
                } => {
                    self.stop_auto_and_skip();
                    let prompt_disp = if prompt.trim().is_empty() {
                        prompt
                    } else {
                        self.tr_or(
                            &scene_choice_prompt_key(&self.stage.current_scene_name, self.stage.block_index),
                            &prompt,
                        )
                    };
                    for (opt_idx, opt) in options.iter_mut().enumerate() {
                        opt.text = self.tr_or(
                            &scene_choice_option_key(
                                &self.stage.current_scene_name,
                                self.stage.block_index,
                                opt_idx,
                            ),
                            &opt.text,
                        );
                    }
                    self.stage.active_choice = Some(ActiveChoice {
                        prompt: prompt_disp,
                        options,
                        position,
                    });
                    return;
                }
                ActionBlock::Wait { seconds } => {
                    self.stage.wait_remaining = Some(seconds.max(0.05));
                    return;
                }
                ActionBlock::VisualEffect { effect, duration } => {
                    let dur = duration.max(0.1);
                    match effect {
                        ScreenEffectType::FadeIn => {
                            self.stage.fade_alpha = 1.0;
                            self.stage.fade_target = 0.0;
                            self.stage.fade_speed = 1.0 / dur;
                        }
                        ScreenEffectType::FadeOut => {
                            self.stage.fade_alpha = 0.0;
                            self.stage.fade_target = 1.0;
                            self.stage.fade_speed = 1.0 / dur;
                        }
                        ScreenEffectType::FlashWhite => {
                            self.stage.flash_alpha = 0.95;
                        }
                        ScreenEffectType::Shake => {
                            self.stage.shake_remaining = dur;
                            self.stage.shake_phase = 0.0;
                        }
                    }
                    self.stage.block_index += 1;
                }
                ActionBlock::SceneEnd { action } => {
                    self.stop_auto_and_skip();
                    self.save_read_history();
                    self.handle_scene_finished(Some(action));
                    return;
                }
            }
        }
    }

    pub fn advance(&mut self) {
        self.stop_auto_and_skip();
        self.advance_internal();
    }

    fn advance_internal(&mut self) {
        if self.screen != RuntimeScreen::Playing {
            return;
        }
        if self.stage.ui_hidden {
            self.stage.ui_hidden = false;
            return;
        }
        // Do not skip past an active choice prompt
        if self.stage.active_choice.is_some() {
            return;
        }
        if let Some(wait) = &mut self.stage.wait_remaining {
            *wait = 0.0;
            self.stage.wait_remaining = None;
            self.stage.block_index += 1;
            self.step_blocks();
            return;
        }
        if let Some(dlg) = &mut self.stage.dialogue {
            if !dlg.is_complete() {
                dlg.complete_now();
                return;
            }
        }
        self.stage.block_index += 1;
        self.step_blocks();
        self.save_autosave();
    }

    pub fn select_choice(&mut self, option_index: usize) {
        let Some(choice) = self.stage.active_choice.take() else {
            return;
        };
        let Some(option) = choice.options.get(option_index).cloned() else {
            return;
        };

        // 1. Check StoryFlowGraph wire from scene_<current_scene_name> + choice_<option_index>
        let from_node_id = format!("scene_{}", self.stage.current_scene_name);
        let from_pin_id = format!("choice_{}", option_index);
        if self.follow_flow_connection(&from_node_id, &from_pin_id) {
            return;
        }

        // 2. Fallback to explicit target_scene_id on the option
        if !option.target_scene_id.trim().is_empty() {
            self.load_scene(&option.target_scene_id);
            return;
        }

        // 3. Continue within current scene
        self.stage.block_index += 1;
        self.step_blocks();
        self.save_autosave();
    }

    pub fn tick(&mut self, dt_seconds: f32) {
        self.ui_anim_time += dt_seconds;
        if self.screen != RuntimeScreen::Playing {
            return;
        }

        // Visual fade / flash / shake / character slide animations
        if (self.stage.fade_alpha - self.stage.fade_target).abs() > 0.001 {
            let step = self.stage.fade_speed.max(0.5) * dt_seconds;
            if self.stage.fade_alpha < self.stage.fade_target {
                self.stage.fade_alpha = (self.stage.fade_alpha + step).min(self.stage.fade_target);
            } else {
                self.stage.fade_alpha = (self.stage.fade_alpha - step).max(self.stage.fade_target);
            }
        }
        if self.stage.transition_hide_until_fade_in && self.stage.fade_alpha <= 0.25 {
            self.stage.transition_hide_until_fade_in = false;
        }
        if self.stage.flash_alpha > 0.0 {
            self.stage.flash_alpha = (self.stage.flash_alpha - dt_seconds * 2.8).max(0.0);
        }
        if self.stage.shake_remaining > 0.0 {
            self.stage.shake_remaining = (self.stage.shake_remaining - dt_seconds).max(0.0);
            self.stage.shake_phase += dt_seconds * 48.0;
        }
        for ch in &mut self.stage.characters {
            if ch.anim_offset_x.abs() > 0.5 {
                ch.anim_offset_x *= 1.0 - (dt_seconds * 12.0).min(0.85);
            } else {
                ch.anim_offset_x = 0.0;
            }
        }

        // Wait block timer (skip immediately when Skip is active)
        if let Some(remaining) = &mut self.stage.wait_remaining {
            if self.skip_mode_active {
                *remaining = 0.0;
            } else {
                *remaining -= dt_seconds;
            }
            if *remaining <= 0.0 {
                self.stage.wait_remaining = None;
                self.stage.block_index += 1;
                self.step_blocks();
            }
            return;
        }

        // Typewriter reveal
        if let Some(dlg) = &mut self.stage.dialogue {
            if !dlg.is_complete() {
                let chars_per_sec = 42.0 * self.text_speed.max(0.25);
                self.char_accumulator += chars_per_sec * dt_seconds;
                let step = self.char_accumulator.floor() as usize;
                if step > 0 {
                    self.char_accumulator -= step as f32;
                    dlg.visible_chars = (dlg.visible_chars + step).min(dlg.total_chars);
                }
            }
        }

        // Skip & Auto-Play progression (paused if UI hidden; auto-stops at choices)
        if self.stage.active_choice.is_some() {
            if self.auto_play || self.skip_mode_active {
                self.stop_auto_and_skip();
            }
            return;
        }
        if self.stage.ui_hidden {
            return;
        }
        if self.skip_mode_active {
            if self.skip_rule == SkipMode::ReadOnly && !self.current_line_was_read {
                self.stop_auto_and_skip();
                self.show_toast("Reached unread text");
                return;
            }
            self.skip_timer += dt_seconds;
            if self.skip_timer >= 0.08 {
                self.skip_timer = 0.0;
                if let Some(dlg) = &mut self.stage.dialogue {
                    if !dlg.is_complete() {
                        dlg.complete_now();
                    }
                }
                self.advance_internal();
            }
        } else if self.auto_play {
            let dialogue_ready = self
                .stage
                .dialogue
                .as_ref()
                .map(|d| d.is_complete())
                .unwrap_or(true);
            if dialogue_ready && self.stage.wait_remaining.is_none() {
                self.auto_timer += dt_seconds;
                if self.auto_timer >= self.auto_forward_delay {
                    self.auto_timer = 0.0;
                    self.advance_internal();
                }
            } else {
                self.auto_timer = 0.0;
            }
        }
    }

    pub fn follow_flow_connection(&mut self, from_node_id: &str, from_pin_id: &str) -> bool {
        let target_node = {
            let Some(loaded) = &self.loaded else {
                return false;
            };
            let Some(graph) = &loaded.flow_graph else {
                return false;
            };
            let Some(conn) = graph
                .connections
                .iter()
                .find(|c| c.from_node_id == from_node_id && c.from_pin_id == from_pin_id)
            else {
                return false;
            };
            graph.find_node(&conn.to_node_id).cloned()
        };

        let Some(node) = target_node else {
            return false;
        };

        match node.node_type {
            FlowNodeType::MainMenu => {
                self.enter_title_screen();
                true
            }
            FlowNodeType::Scene { scene_name } => {
                self.load_scene(&scene_name);
                true
            }
            FlowNodeType::PauseMenu => {
                self.screen = RuntimeScreen::PauseMenu;
                true
            }
            FlowNodeType::Settings => {
                self.screen = RuntimeScreen::Settings {
                    return_to_pause: false,
                };
                true
            }
            FlowNodeType::SaveLoad => {
                self.screen = RuntimeScreen::PauseMenu;
                true
            }
            FlowNodeType::Ending { ending_name } => {
                self.screen = RuntimeScreen::Ending {
                    title: ending_name,
                    ending_node_id: Some(node.id),
                };
                true
            }
        }
    }

    fn handle_scene_finished(&mut self, explicit_action: Option<SceneEndAction>) {
        // 1. Check StoryFlowGraph "next" wire from current scene
        let from_node_id = format!("scene_{}", self.stage.current_scene_name);
        if self.follow_flow_connection(&from_node_id, "next") {
            return;
        }

        // 2. Fallback to explicit SceneEndAction
        match explicit_action.unwrap_or(SceneEndAction::ReturnToTitle) {
            SceneEndAction::JumpToScene(target) if !target.trim().is_empty() => {
                self.load_scene(&target);
            }
            SceneEndAction::NextScene => {
                let next_name = self.loaded.as_ref().and_then(|l| {
                    let idx = l
                        .scene_order
                        .iter()
                        .position(|s| s == &self.stage.current_scene_name)?;
                    l.scene_order.get(idx + 1).cloned()
                });
                if let Some(next) = next_name {
                    self.load_scene(&next);
                } else {
                    self.enter_title_screen();
                }
            }
            _ => {
                self.enter_title_screen();
            }
        }
    }

    pub fn update_volumes(&mut self) {
        self.audio.set_bgm_volume(self.master_volume * self.bgm_volume);
    }

    pub fn ui_config(&self) -> blanc_core::RuntimeUiConfig {
        self.loaded
            .as_ref()
            .map(|l| l.config.ui_config.clone())
            .unwrap_or_default()
    }

    pub fn focus_item_count(&self) -> usize {
        let ui = self.ui_config();
        match &self.screen {
            RuntimeScreen::TitleScreen => {
                if ui.main_menu.show_continue_button && self.has_save_data() {
                    5
                } else {
                    4
                }
            }
            RuntimeScreen::Playing => self
                .stage
                .active_choice
                .as_ref()
                .map(|c| c.options.len())
                .unwrap_or(0),
            RuntimeScreen::PauseMenu => {
                let p = &ui.pause_settings;
                [
                    p.show_resume_button,
                    p.show_save_button,
                    p.show_load_button,
                    p.show_settings_button,
                    p.show_return_title_button,
                    p.show_quit_button,
                ]
                .into_iter()
                .filter(|&v| v)
                .count()
                .max(1)
            }
            RuntimeScreen::Settings { .. } => 1,
            RuntimeScreen::SaveLoad { .. } => 1,
            RuntimeScreen::Backlog => 1,
            RuntimeScreen::Ending { .. } => 1,
            RuntimeScreen::Error(_) => 0,
        }
    }

    pub fn move_focus(&mut self, delta: isize) {
        let count = self.focus_item_count();
        if count == 0 {
            self.focus_index = 0;
            return;
        }
        let current = (self.focus_index % count) as isize;
        let next = (current + delta).rem_euclid(count as isize);
        self.focus_index = next as usize;
    }
}

fn same_character_identity(a: &str, b: &str) -> bool {
    if a.eq_ignore_ascii_case(b) {
        return true;
    }
    let key_a = character_base_key(a);
    let key_b = character_base_key(b);
    !key_a.is_empty() && key_a == key_b
}

fn character_base_key(raw: &str) -> String {
    let mut s = raw.trim().to_lowercase();
    for prefix in [
        "assets_characters_",
        "assets_expressions_",
        "assets_character_expressions_",
    ] {
        if let Some(rest) = s.strip_prefix(prefix) {
            s = rest.to_string();
            break;
        }
    }
    if let Some((_, after)) = s.split_once("el_char_") {
        s = after.to_string();
    }
    for suffix in ["_png", "_webp", "_jpg", "_jpeg"] {
        if let Some(base) = s.strip_suffix(suffix) {
            s = base.to_string();
            break;
        }
    }
    for token in [
        "_neutral_",
        "_smile_",
        "_worried_",
        "_laughing_",
        "_sad_",
        "_annoyed_",
        "_surprised_",
        "_embarrassed_",
        "_happy_",
        "_angry_",
        "_day_",
        "_sunset_",
        "_night_",
    ] {
        if let Some((head, _)) = s.split_once(token) {
            return head.to_string();
        }
    }
    s
}

fn format_utc_timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    let rem = secs % 86400;
    let hours = rem / 3600;
    let mins = (rem % 3600) / 60;
    // Civil date from days since 1970-01-01 (Howard Hinnant algorithm)
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    format!("{:04}-{:02}-{:02} {:02}:{:02} UTC", year, m, d, hours, mins)
}

impl Drop for RuntimeEngine {
    fn drop(&mut self) {
        self.save_read_history();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blanc_core::flow::StoryFlowGraph;
    use blanc_core::project::Project;
    use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption, Scene};

    #[test]
    fn test_runtime_engine_playback_and_story_flow_branching() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("blanc_runtime_test_{}", unique_id));
        let mut project = Project::create_project_structure(&temp_dir, "Test VN", "Tester").unwrap();
        project.starting_scene_id = Some("01_intro".to_string());
        let manifest_path = temp_dir.join("Test_VN.blanc");
        project.save_manifest(&manifest_path).unwrap();

        let mut scene1 = Scene::new("01_intro", "ch1");
        scene1.add_block(ActionBlock::Character {
            character_id: "Alice".to_string(),
            sprite_asset_id: None,
            expression: "Happy".to_string(),
            position: CharacterPosition::Center,
            offset_x: 0.0,
            offset_y: 0.0,
            transition: None,
        });
        scene1.add_block(ActionBlock::Dialogue {
            speaker: "Alice".to_string(),
            text: "Hello!".to_string(),
            voice_asset_id: None,
            nameplate_position: None,
        });
        scene1.add_block(ActionBlock::Choice {
            prompt: "Where to go?".to_string(),
            options: vec![ChoiceOption {
                id: "opt1".to_string(),
                text: "Go to Garden".to_string(),
                target_scene_id: String::new(),
            }],
            position: ChoiceLayoutPosition::Center,
        });
        scene1.save_to_file(&temp_dir.join("Scenes").join("01_intro.bscene")).unwrap();

        let mut scene2 = Scene::new("02_garden", "ch1");
        scene2.add_block(ActionBlock::Narration {
            text: "You arrived at the garden.".to_string(),
        });
        scene2.save_to_file(&temp_dir.join("Scenes").join("02_garden.bscene")).unwrap();

        let mut flow = StoryFlowGraph::starter_graph(Some("01_intro"));
        flow.add_scene_node("02_garden", 600.0, 200.0);
        flow.connect_pins("scene_01_intro", "choice_0", "scene_02_garden", "in");
        flow.connect_pins("scene_02_garden", "next", "ending_return_to_title", "in");
        flow.connect_pins("ending_return_to_title", "title", "system_main_menu", "in");
        flow.save_to_file(&temp_dir.join("Story").join("story_flow.bgraph")).unwrap();

        let mut engine = RuntimeEngine::new(Some(temp_dir.clone()), None);
        assert_eq!(engine.screen, RuntimeScreen::TitleScreen);

        engine.start_new_game();
        assert_eq!(engine.screen, RuntimeScreen::Playing);
        assert_eq!(engine.stage.current_scene_name, "01_intro");
        assert_eq!(engine.stage.characters.len(), 1);
        assert_eq!(engine.stage.dialogue.as_ref().unwrap().full_text, "Hello!");

        // First advance finishes typewriter, second advance moves to Choice block
        engine.advance();
        assert!(engine.stage.dialogue.as_ref().unwrap().is_complete());
        engine.advance();
        assert!(engine.stage.active_choice.is_some());

        // Select choice_0 -> jumps to 02_garden via StoryFlowGraph
        engine.select_choice(0);
        assert_eq!(engine.stage.current_scene_name, "02_garden");
        assert_eq!(
            engine.stage.dialogue.as_ref().unwrap().full_text,
            "You arrived at the garden."
        );

        // Finish narration -> reaches ending_return_to_title
        engine.advance();
        engine.advance();
        assert!(matches!(engine.screen, RuntimeScreen::Ending { .. }));

        // Follow ending's title wire back to Main Menu
        assert!(engine.follow_flow_connection("ending_return_to_title", "title"));
        assert_eq!(engine.screen, RuntimeScreen::TitleScreen);

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
