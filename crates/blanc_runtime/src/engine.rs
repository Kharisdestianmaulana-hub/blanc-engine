use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use blanc_core::asset::AssetCategory;
use blanc_core::flow::FlowNodeType;
use blanc_core::scene::{
    ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption, SceneEndAction,
};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use crate::loader::LoadedProject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeScreen {
    TitleScreen,
    Playing,
    PauseMenu,
    Settings { return_to_pause: bool },
    Ending { title: String, ending_node_id: Option<String> },
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ActiveCharacter {
    pub character_id: String,
    pub expression: String,
    pub position: CharacterPosition,
    pub offset_x: f32,
    pub offset_y: f32,
    pub image_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ActiveDialogue {
    pub speaker: Option<String>,
    pub full_text: String,
    pub visible_chars: usize,
    pub total_chars: usize,
}

impl ActiveDialogue {
    pub fn new(speaker: Option<String>, text: String) -> Self {
        let total_chars = text.chars().count();
        Self {
            speaker,
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
    pub current_bgm_id: Option<String>,
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
    char_accumulator: f32,
}

impl RuntimeEngine {
    pub fn new(project_dir: Option<PathBuf>) -> Self {
        let mut engine = Self {
            loaded: None,
            screen: RuntimeScreen::TitleScreen,
            stage: StageState::default(),
            audio: AudioController::default(),
            master_volume: 1.0,
            bgm_volume: 0.8,
            sfx_volume: 1.0,
            text_speed: 1.0,
            char_accumulator: 0.0,
        };

        match project_dir {
            Some(dir) => match LoadedProject::load_from_dir(&dir) {
                Ok(loaded) => {
                    engine.master_volume = loaded.config.master_volume;
                    engine.bgm_volume = loaded.config.bgm_volume;
                    engine.sfx_volume = loaded.config.sfx_volume;
                    engine.text_speed = loaded.config.text_speed.max(0.25);
                    engine.loaded = Some(loaded);
                    engine.enter_title_screen();
                }
                Err(err) => {
                    engine.screen = RuntimeScreen::Error(err);
                }
            },
            None => {
                engine.screen = RuntimeScreen::Error(
                    "No project directory provided.\nLaunch from Blanc Editor using 'Play Preview', or pass a project folder path as an argument.".to_string(),
                );
            }
        }

        engine
    }

    pub fn enter_title_screen(&mut self) {
        self.screen = RuntimeScreen::TitleScreen;
        self.stage = StageState::default();
        self.audio.stop_bgm();

        if let Some(loaded) = &self.loaded {
            if let Some(bgm_id) = &loaded.config.title_bgm_asset_id {
                if let Some(path) = loaded.resolve_asset_path(bgm_id, Some(AssetCategory::Bgm)) {
                    let vol = self.master_volume * self.bgm_volume;
                    self.audio.play_bgm(&path, vol, true);
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

        self.stage.current_scene_name = scene_name.to_string();
        self.stage.block_index = 0;
        self.stage.dialogue = None;
        self.stage.active_choice = None;
        self.stage.wait_remaining = None;
        self.screen = RuntimeScreen::Playing;
        self.step_blocks();
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
                ActionBlock::Background { asset_id, .. } => {
                    if let Some(loaded) = &self.loaded {
                        self.stage.background_path =
                            loaded.resolve_asset_path(&asset_id, Some(AssetCategory::Background));
                    }
                    self.stage.block_index += 1;
                }
                ActionBlock::Character {
                    character_id,
                    expression,
                    position,
                    offset_x,
                    offset_y,
                    ..
                } => {
                    let image_path = self
                        .loaded
                        .as_ref()
                        .and_then(|l| l.resolve_character_path(&character_id, &expression));

                    if let Some(existing) = self
                        .stage
                        .characters
                        .iter_mut()
                        .find(|c| c.character_id.eq_ignore_ascii_case(&character_id))
                    {
                        existing.expression = expression;
                        existing.position = position;
                        existing.offset_x = offset_x;
                        existing.offset_y = offset_y;
                        if image_path.is_some() {
                            existing.image_path = image_path;
                        }
                    } else {
                        self.stage.characters.push(ActiveCharacter {
                            character_id,
                            expression,
                            position,
                            offset_x,
                            offset_y,
                            image_path,
                        });
                    }
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
                            self.audio.play_bgm(&path, effective, looping);
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
                            self.audio.play_sfx(&path, effective);
                        }
                    }
                    self.stage.block_index += 1;
                }
                ActionBlock::Dialogue { speaker, text, .. } => {
                    self.stage.active_choice = None;
                    let mut dlg = ActiveDialogue::new(
                        if speaker.trim().is_empty() {
                            None
                        } else {
                            Some(speaker)
                        },
                        text,
                    );
                    if self.text_speed >= 2.0 {
                        dlg.complete_now();
                    }
                    self.stage.dialogue = Some(dlg);
                    self.char_accumulator = 0.0;
                    return;
                }
                ActionBlock::Narration { text } => {
                    self.stage.active_choice = None;
                    let mut dlg = ActiveDialogue::new(None, text);
                    if self.text_speed >= 2.0 {
                        dlg.complete_now();
                    }
                    self.stage.dialogue = Some(dlg);
                    self.char_accumulator = 0.0;
                    return;
                }
                ActionBlock::Choice {
                    prompt,
                    options,
                    position,
                } => {
                    self.stage.active_choice = Some(ActiveChoice {
                        prompt,
                        options,
                        position,
                    });
                    return;
                }
                ActionBlock::Wait { seconds } => {
                    self.stage.wait_remaining = Some(seconds.max(0.05));
                    return;
                }
                ActionBlock::SceneEnd { action } => {
                    self.handle_scene_finished(Some(action));
                    return;
                }
            }
        }
    }

    pub fn advance(&mut self) {
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
    }

    pub fn tick(&mut self, dt_seconds: f32) {
        if self.screen != RuntimeScreen::Playing {
            return;
        }

        // Wait block timer
        if let Some(remaining) = &mut self.stage.wait_remaining {
            *remaining -= dt_seconds;
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

        let mut engine = RuntimeEngine::new(Some(temp_dir.clone()));
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
