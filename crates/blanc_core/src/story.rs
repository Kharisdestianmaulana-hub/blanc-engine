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

/// A line in the story script
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StoryElement {
    Dialogue(DialogueLine),
    Narration(NarrationLine),
}

/// Chapter grouping for story elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub elements: Vec<StoryElement>,
}

impl Chapter {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            elements: Vec::new(),
        }
    }
}

/// Story container stored internally in `Story/Chapters/<chapter>.bstory`
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

    /// Parse script text (plain text or markdown or fountain style)
    pub fn parse_script(text: &str) -> Vec<StoryElement> {
        let mut elements = Vec::new();

        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // Check dialogue format: "Name: Text" or "Name (Expression): Text"
            if let Some((speaker_part, dialog_part)) = line.split_once(':') {
                let speaker_clean = speaker_part.trim();
                let dialog_clean = dialog_part.trim().trim_matches('"');

                if !speaker_clean.is_empty() && !dialog_clean.is_empty() {
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

            // Default to Narration
            elements.push(StoryElement::Narration(NarrationLine {
                id: Uuid::new_v4().to_string(),
                text: line.to_string(),
            }));
        }

        elements
    }
}
