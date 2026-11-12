use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Categories of assets supported in Blanc Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetCategory {
    Background,
    Character,
    CharacterExpression,
    Bgm,
    Sfx,
    Font,
    Ui,
    Other,
}

impl AssetCategory {
    pub const ALL: &'static [AssetCategory] = &[
        AssetCategory::Background,
        AssetCategory::Character,
        AssetCategory::CharacterExpression,
        AssetCategory::Bgm,
        AssetCategory::Sfx,
        AssetCategory::Font,
        AssetCategory::Ui,
        AssetCategory::Other,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Background => "Backgrounds",
            Self::Character => "Characters",
            Self::CharacterExpression => "Character Expressions",
            Self::Bgm => "Background Music (BGM)",
            Self::Sfx => "Sound Effects (SFX)",
            Self::Font => "Fonts",
            Self::Ui => "UI Assets",
            Self::Other => "Other Assets",
        }
    }

    pub fn subfolder(&self) -> &'static str {
        match self {
            Self::Background => "Assets/Backgrounds",
            Self::Character => "Assets/Characters",
            Self::CharacterExpression => "Assets/Characters",
            Self::Bgm => "Assets/BGM",
            Self::Sfx => "Assets/SFX",
            Self::Font => "Assets/Fonts",
            Self::Ui => "Assets/UI",
            Self::Other => "Assets/Other",
        }
    }
}

/// Represents an asset referenced within the Blanc Engine project
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub category: AssetCategory,
    pub relative_path: String,
    pub file_size: u64,
    pub created_at: DateTime<Utc>,
}

impl Asset {
    pub fn new(id: impl Into<String>, name: impl Into<String>, category: AssetCategory, relative_path: impl Into<String>, file_size: u64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            relative_path: relative_path.into(),
            file_size,
            created_at: Utc::now(),
        }
    }
}

/// Registry mapping unique Asset IDs to Asset metadata
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetRegistry {
    pub assets: HashMap<String, Asset>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn insert(&mut self, asset: Asset) {
        self.assets.insert(asset.id.clone(), asset);
    }

    pub fn get(&self, id: &str) -> Option<&Asset> {
        self.assets.get(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<Asset> {
        self.assets.remove(id)
    }

    pub fn is_supported_file_extension(path: &Path) -> bool {
        match path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_ascii_lowercase()).as_deref() {
            Some("png" | "jpg" | "jpeg" | "webp") => true,
            Some("ogg" | "wav" | "mp3") => true,
            Some("ttf" | "otf") => true,
            _ => false,
        }
    }
}
