use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Dialogue element in a story
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub id: String,
    pub speaker: String,
    pub expression: Option<String>,
    pub text: String,
}

/// Narration element in a story
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrationLine {
    pub id: String,
    pub text: String,
}

/// A decision branch with multiple player choices
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub text: String,
    pub target_scene: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceBranch {
    pub id: String,
    pub prompt: Option<String>,
    pub options: Vec<ChoiceOption>,
}

/// Scene transition marker in story
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneMarker {
    pub id: String,
    pub scene_name: String,
}

/// Audio cue in story (BGM or SFX)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioMarker {
    pub id: String,
    pub is_bgm: bool,
    pub track_name: String,
}

/// A line or command in the story script
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StoryElement {
    Scene(SceneMarker),
    Audio(AudioMarker),
    Dialogue(DialogueLine),
    Narration(NarrationLine),
    Choice(ChoiceBranch),
}

/// Chapter grouping for story elements stored in `Story/Chapters/<chapter>.bstory`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub raw_script: String,
    pub elements: Vec<StoryElement>,
}

impl Chapter {
    pub const EXTENSION: &'static str = "bstory";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            raw_script: String::new(),
            elements: Vec::new(),
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let chapter: Self = serde_json::from_str(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(chapter)
    }
}

/// Story container managing chapters
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    pub chapters: Vec<Chapter>,
}

impl Story {
    pub fn new() -> Self {
        Self {
            chapters: Vec::new(),
        }
    }

    /// Parse script text (Plain Text, Markdown, or Fountain screenplay format)
    pub fn parse_script(text: &str) -> Vec<StoryElement> {
        let mut elements = Vec::new();
        let mut in_choice_block = false;
        let mut current_choice_options = Vec::new();
        let mut current_choice_prompt = None;

        for raw_line in text.lines() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with("//") || (line.starts_with('#') && !line.starts_with("# ") && !line.starts_with("## ")) {
                // If line is empty and we were collecting choices, finish the choice
                if in_choice_block && !current_choice_options.is_empty() && line.is_empty() {
                    elements.push(StoryElement::Choice(ChoiceBranch {
                        id: Uuid::new_v4().to_string(),
                        prompt: current_choice_prompt.take(),
                        options: std::mem::take(&mut current_choice_options),
                    }));
                    in_choice_block = false;
                }
                continue;
            }

            // Check Choice start: [Choice: ...] or [Choice]
            if line.starts_with("[Choice") {
                in_choice_block = true;
                current_choice_options.clear();
                if let Some((_, prompt_part)) = line.split_once(':') {
                    let prompt = prompt_part.trim_end_matches(']').trim();
                    if !prompt.is_empty() {
                        current_choice_prompt = Some(prompt.to_string());
                    }
                }
                continue;
            }

            // If inside choice block, parse choice items: - "Text" -> TargetScene or * Option -> Target
            if in_choice_block {
                if line.ends_with(']') || line.starts_with(']') {
                    elements.push(StoryElement::Choice(ChoiceBranch {
                        id: Uuid::new_v4().to_string(),
                        prompt: current_choice_prompt.take(),
                        options: std::mem::take(&mut current_choice_options),
                    }));
                    in_choice_block = false;
                    continue;
                }

                let cleaned_choice = line.trim_start_matches('-').trim_start_matches('*').trim();
                if let Some((opt_text, target)) = cleaned_choice.split_once("->") {
                    let text = opt_text.trim().trim_matches('"').to_string();
                    let target_scene = target.trim().trim_end_matches(']').trim().to_string();
                    if !text.is_empty() && !target_scene.is_empty() {
                        current_choice_options.push(ChoiceOption { text, target_scene });
                        continue;
                    }
                }
            }

            // Check Scene markers: [Scene: Name] or Fountain format (INT. / EXT.)
            if line.starts_with("[Scene:") && line.ends_with(']') {
                let name = line[7..line.len() - 1].trim();
                elements.push(StoryElement::Scene(SceneMarker {
                    id: Uuid::new_v4().to_string(),
                    scene_name: name.to_string(),
                }));
                continue;
            } else if line.starts_with("INT. ") || line.starts_with("EXT. ") {
                elements.push(StoryElement::Scene(SceneMarker {
                    id: Uuid::new_v4().to_string(),
                    scene_name: line.to_string(),
                }));
                continue;
            } else if let Some(md_title) = line.strip_prefix("## ").or_else(|| line.strip_prefix("# ")) {
                let name = md_title.trim();
                if !name.is_empty() {
                    elements.push(StoryElement::Scene(SceneMarker {
                        id: Uuid::new_v4().to_string(),
                        scene_name: name.to_string(),
                    }));
                    continue;
                }
            }

            // Check Audio tags: [BGM: name] or [SFX: name]
            if line.starts_with("[BGM:") && line.ends_with(']') {
                let track = line[5..line.len() - 1].trim();
                elements.push(StoryElement::Audio(AudioMarker {
                    id: Uuid::new_v4().to_string(),
                    is_bgm: true,
                    track_name: track.to_string(),
                }));
                continue;
            }
            if line.starts_with("[SFX:") && line.ends_with(']') {
                let track = line[5..line.len() - 1].trim();
                elements.push(StoryElement::Audio(AudioMarker {
                    id: Uuid::new_v4().to_string(),
                    is_bgm: false,
                    track_name: track.to_string(),
                }));
                continue;
            }

            // Check Dialogue: "Speaker (Emotion): Text" or "Speaker: Text"
            if let Some((speaker_part, dialog_part)) = line.split_once(':') {
                let speaker_clean = speaker_part.trim();
                let dialog_clean = dialog_part.trim().trim_matches('"');

                // Ensure it's not a tag like [Tag: ...]
                if !speaker_clean.starts_with('[') && !speaker_clean.is_empty() && !dialog_clean.is_empty() {
                    let (speaker, expression) = if let Some((name, expr)) = speaker_clean.split_once('(') {
                        (name.trim().to_string(), Some(expr.trim_end_matches(')').trim().to_string()))
                    } else {
                        (speaker_clean.to_string(), None)
                    };

                    elements.push(StoryElement::Dialogue(DialogueLine {
                        id: Uuid::new_v4().to_string(),
                        speaker,
                        expression,
                        text: dialog_clean.to_string(),
                    }));
                    continue;
                }
            }

            // Default fallback: Narration
            elements.push(StoryElement::Narration(NarrationLine {
                id: Uuid::new_v4().to_string(),
                text: line.to_string(),
            }));
        }

        // Flush remaining choices if script ends inside choice block
        if in_choice_block && !current_choice_options.is_empty() {
            elements.push(StoryElement::Choice(ChoiceBranch {
                id: Uuid::new_v4().to_string(),
                prompt: current_choice_prompt.take(),
                options: current_choice_options,
            }));
        }

        elements
    }
}
