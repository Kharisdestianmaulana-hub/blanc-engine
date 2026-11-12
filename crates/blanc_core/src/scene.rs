use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Character position presets on screen
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CharacterPosition {
    Left,
    Center,
    Right,
}

impl Default for CharacterPosition {
    fn default() -> Self {
        Self::Center
    }
}

impl CharacterPosition {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Center => "Center",
            Self::Right => "Right",
        }
    }
}

/// Visual layout position for Choice decision buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceLayoutPosition {
    Center,
    AboveDialogueBox,
    Left,
    Right,
}

impl Default for ChoiceLayoutPosition {
    fn default() -> Self {
        Self::Center
    }
}

impl ChoiceLayoutPosition {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Center => "Center Screen",
            Self::AboveDialogueBox => "Above Dialogue",
            Self::Left => "Left Aligned",
            Self::Right => "Right Aligned",
        }
    }
}

/// Visual transition effects between scene states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    Dissolve,
    Cut,
    SlideLeft,
    SlideRight,
}

impl Default for TransitionType {
    fn default() -> Self {
        Self::Fade
    }
}

/// An option in a Choice Action Block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub id: String,
    pub text: String,
    pub target_scene_id: String,
}

/// Action to execute at the end of a scene
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneEndAction {
    NextScene,
    JumpToScene(String),
    ReturnToTitle,
}

impl Default for SceneEndAction {
    fn default() -> Self {
        Self::ReturnToTitle
    }
}

/// Individual Action Block in a vertical sequence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ActionBlock {
    Background {
        asset_id: String,
        transition: Option<TransitionType>,
    },
    Character {
        character_id: String,
        expression: String,
        position: CharacterPosition,
        offset_x: f32,
        offset_y: f32,
        transition: Option<TransitionType>,
    },
    HideCharacter {
        character_id: String,
        transition: Option<TransitionType>,
    },
    Dialogue {
        speaker: String,
        text: String,
        voice_asset_id: Option<String>,
    },
    Narration {
        text: String,
    },
    PlayBgm {
        asset_id: String,
        volume: f32,
        looping: bool,
    },
    StopBgm {
        fade_out_seconds: f32,
    },
    PlaySfx {
        asset_id: String,
        volume: f32,
    },
    Wait {
        seconds: f32,
    },
    Choice {
        prompt: String,
        options: Vec<ChoiceOption>,
        #[serde(default)]
        position: ChoiceLayoutPosition,
    },
    SceneEnd {
        action: SceneEndAction,
    },
}

impl ActionBlock {
    pub fn block_type_name(&self) -> &'static str {
        match self {
            Self::Background { .. } => "Background",
            Self::Character { .. } => "Character",
            Self::HideCharacter { .. } => "Hide Character",
            Self::Dialogue { .. } => "Dialogue",
            Self::Narration { .. } => "Narration",
            Self::PlayBgm { .. } => "Play BGM",
            Self::StopBgm { .. } => "Stop BGM",
            Self::PlaySfx { .. } => "Play SFX",
            Self::Wait { .. } => "Wait",
            Self::Choice { .. } => "Choice",
            Self::SceneEnd { .. } => "Scene End",
        }
    }
}

/// Navigation link between scenes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneNavigation {
    pub current_scene_id: String,
    pub next_scene_id: Option<String>,
}

/// Story Scene data structure stored as `<name>.bscene`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub chapter_id: String,
    pub blocks: Vec<ActionBlock>,
}

impl Scene {
    pub const EXTENSION: &'static str = "bscene";

    pub fn new(name: impl Into<String>, chapter_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            chapter_id: chapter_id.into(),
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: ActionBlock) {
        self.blocks.push(block);
    }

    pub fn insert_block(&mut self, index: usize, block: ActionBlock) {
        if index <= self.blocks.len() {
            self.blocks.insert(index, block);
        } else {
            self.blocks.push(block);
        }
    }

    pub fn remove_block(&mut self, index: usize) -> Option<ActionBlock> {
        if index < self.blocks.len() {
            Some(self.blocks.remove(index))
        } else {
            None
        }
    }

    pub fn move_block_up(&mut self, index: usize) {
        if index > 0 && index < self.blocks.len() {
            self.blocks.swap(index, index - 1);
        }
    }

    pub fn move_block_down(&mut self, index: usize) {
        if index + 1 < self.blocks.len() {
            self.blocks.swap(index, index + 1);
        }
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let scene: Self = serde_json::from_str(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(scene)
    }
}
