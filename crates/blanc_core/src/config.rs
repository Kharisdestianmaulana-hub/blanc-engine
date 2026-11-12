use serde::{Deserialize, Serialize};

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
            master_volume: 1.0,
            bgm_volume: 0.8,
            sfx_volume: 1.0,
            text_speed: 1.0,
            auto_forward_delay: 2.0,
        }
    }
}
