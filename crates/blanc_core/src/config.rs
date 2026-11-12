use std::path::Path;
use serde::{Deserialize, Serialize};

/// Aspect ratio scaling mode for runtime display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectScalingMode {
    Cover,       // Smart Cover / Aspect Fill (Recommended: 100% fullscreen, no black bars, zero distortion)
    Fit,         // Pixel-perfect fit with letterbox / pillarbox bars
    AmbientBlur, // Main 16:9 art centered with dynamic blurred wings on ultrawide
}

impl Default for AspectScalingMode {
    fn default() -> Self {
        Self::Cover
    }
}

impl AspectScalingMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Cover => "Smart Cover (Aspect Fill - No Black Bars)",
            Self::Fit => "Fixed Ratio (Letterbox Bars)",
            Self::AmbientBlur => "Ambient Blur (Ultrawide Wings)",
        }
    }
}

/// Configuration for Built-in System Screens (Main Menu, Settings, Save/Load, In-Game Pause)
/// Stored in `Settings/system_screens.bconfig`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemScreensConfig {
    pub title_text: String,
    pub title_background_asset_id: Option<String>,
    pub title_bgm_asset_id: Option<String>,
    pub logo_asset_id: Option<String>,
    pub starting_scene_id: Option<String>,

    // Display & Resolution
    pub default_resolution_width: u32,
    pub default_resolution_height: u32,
    pub fullscreen_default: bool,
    #[serde(default)]
    pub aspect_scaling_mode: AspectScalingMode,

    // Audio Defaults
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,

    // Text & Gameplay Defaults
    pub text_speed: f32,
    pub auto_forward_delay: f32,
}

impl Default for SystemScreensConfig {
    fn default() -> Self {
        Self {
            title_text: "My Visual Novel".to_string(),
            title_background_asset_id: None,
            title_bgm_asset_id: None,
            logo_asset_id: None,
            starting_scene_id: None,
            default_resolution_width: 1920,
            default_resolution_height: 1080,
            fullscreen_default: false,
            aspect_scaling_mode: AspectScalingMode::Cover,
            master_volume: 1.0,
            bgm_volume: 0.8,
            sfx_volume: 1.0,
            text_speed: 1.0,
            auto_forward_delay: 2.0,
        }
    }
}

impl SystemScreensConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
