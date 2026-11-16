use serde::{Deserialize, Serialize};

/// 9-point anchor preset (Unity RectTransform / Godot Control style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for UiAnchor {
    fn default() -> Self {
        Self::Center
    }
}

impl UiAnchor {
    pub fn all_grid() -> [[Self; 3]; 3] {
        [
            [Self::TopLeft, Self::TopCenter, Self::TopRight],
            [Self::CenterLeft, Self::Center, Self::CenterRight],
            [Self::BottomLeft, Self::BottomCenter, Self::BottomRight],
        ]
    }

    pub fn short_label(&self) -> &'static str {
        match self {
            Self::TopLeft => "TL",
            Self::TopCenter => "TC",
            Self::TopRight => "TR",
            Self::CenterLeft => "CL",
            Self::Center => "C",
            Self::CenterRight => "CR",
            Self::BottomLeft => "BL",
            Self::BottomCenter => "BC",
            Self::BottomRight => "BR",
        }
    }
}

/// Direction for button groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonDirection {
    Vertical,
    Horizontal,
}

impl Default for ButtonDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Visual button rendering style when not using custom button textures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonVisualStyle {
    SolidAccent,
    GlassDark,
    Outlined,
}

impl Default for ButtonVisualStyle {
    fn default() -> Self {
        Self::SolidAccent
    }
}

/// Position of speaker nameplate relative to dialogue box
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NameplatePosition {
    TopLeft,
    TopCenter,
    TopRight,
    InsideBox,
}

impl Default for NameplatePosition {
    fn default() -> Self {
        Self::TopLeft
    }
}

/// Global theme palette, corner radius, and button state textures
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeUiConfig {
    pub accent_hex: String,
    pub panel_hex: String,
    pub panel_opacity: f32,
    pub text_hex: String,
    pub button_text_hex: String,
    pub button_focus_text_hex: String,
    pub font_family: String,
    #[serde(default)]
    pub font_asset_id: Option<String>,
    pub border_radius: f32,
    pub button_style: ButtonVisualStyle,

    // Optional custom button textures (Asset IDs from Assets/UI)
    pub button_normal_asset_id: Option<String>,
    pub button_focus_asset_id: Option<String>,
    pub button_pressed_asset_id: Option<String>,
    pub button_disabled_asset_id: Option<String>,

    // Optional custom small button textures (HUD, Delete, Text Speed, etc.)
    pub small_button_normal_asset_id: Option<String>,
    pub small_button_focus_asset_id: Option<String>,
    pub small_button_pressed_asset_id: Option<String>,
    pub small_button_disabled_asset_id: Option<String>,

    // Optional custom slider textures & styling (Settings sliders)
    #[serde(default)]
    pub slider_track_asset_id: Option<String>,
    #[serde(default)]
    pub slider_fill_asset_id: Option<String>,
    #[serde(default)]
    pub slider_handle_asset_id: Option<String>,
    #[serde(default)]
    pub slider_track_bg_hex: String,
    #[serde(default)]
    pub slider_fill_hex: String,
    #[serde(default)]
    pub slider_handle_hex: String,
    #[serde(default = "default_slider_track_height")]
    pub slider_track_height: f32,
    #[serde(default = "default_slider_handle_size")]
    pub slider_handle_size: f32,
}

fn default_slider_track_height() -> f32 {
    6.0
}

fn default_slider_handle_size() -> f32 {
    16.0
}

impl Default for ThemeUiConfig {
    fn default() -> Self {
        Self {
            accent_hex: "#6366f1".to_string(),
            panel_hex: "#0d0f17".to_string(),
            panel_opacity: 0.88,
            text_hex: "#f8fafc".to_string(),
            button_text_hex: String::new(),
            button_focus_text_hex: String::new(),
            font_family: String::new(),
            font_asset_id: None,
            border_radius: 12.0,
            button_style: ButtonVisualStyle::SolidAccent,
            button_normal_asset_id: None,
            button_focus_asset_id: None,
            button_pressed_asset_id: None,
            button_disabled_asset_id: None,
            small_button_normal_asset_id: None,
            small_button_focus_asset_id: None,
            small_button_pressed_asset_id: None,
            small_button_disabled_asset_id: None,
            slider_track_asset_id: None,
            slider_fill_asset_id: None,
            slider_handle_asset_id: None,
            slider_track_bg_hex: String::new(),
            slider_fill_hex: String::new(),
            slider_handle_hex: String::new(),
            slider_track_height: 6.0,
            slider_handle_size: 16.0,
        }
    }
}

/// Main Menu / Title Screen UI layout & brand configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MainMenuUiConfig {
    pub brand_anchor: UiAnchor,
    pub brand_offset_x: f32,
    pub brand_offset_y: f32,
    pub logo_width: f32,
    pub show_title_text: bool,
    pub show_author_text: bool,
    pub text_hex: String,
    pub button_text_hex: String,
    pub button_focus_text_hex: String,

    pub buttons_anchor: UiAnchor,
    pub buttons_offset_x: f32,
    pub buttons_offset_y: f32,
    pub buttons_direction: ButtonDirection,
    pub buttons_width: f32,

    pub backdrop_dimming: f32,
    pub panel_asset_id: Option<String>,

    pub show_continue_button: bool,
    pub continue_label: String,
    pub new_game_label: String,
    pub settings_label: String,
    pub quit_label: String,
}

impl Default for MainMenuUiConfig {
    fn default() -> Self {
        Self {
            brand_anchor: UiAnchor::Center,
            brand_offset_x: 0.0,
            brand_offset_y: -175.0,
            logo_width: 96.0,
            show_title_text: true,
            show_author_text: true,
            text_hex: String::new(),
            button_text_hex: String::new(),
            button_focus_text_hex: String::new(),

            buttons_anchor: UiAnchor::Center,
            buttons_offset_x: 0.0,
            buttons_offset_y: 115.0,
            buttons_direction: ButtonDirection::Vertical,
            buttons_width: 240.0,

            backdrop_dimming: 0.35,
            panel_asset_id: None,

            show_continue_button: true,
            continue_label: "Continue".to_string(),
            new_game_label: "New Game".to_string(),
            settings_label: "Settings".to_string(),
            quit_label: "Quit".to_string(),
        }
    }
}

/// Gameplay HUD, Dialogue Box, Nameplate & Choice Buttons configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameplayUiConfig {
    pub dialogue_anchor: UiAnchor,
    pub dialogue_offset_x: f32,
    pub dialogue_offset_y: f32,
    pub dialogue_full_width: bool,
    pub dialogue_width: f32,
    pub dialogue_opacity: f32,
    pub dialogue_font_size: f32,
    pub dialogue_text_hex: String,
    pub dialogue_box_asset_id: Option<String>,
    pub advance_prompt_label: String,
    pub advance_prompt_asset_id: Option<String>,
    pub advance_prompt_frame2_asset_id: Option<String>,
    pub advance_prompt_frame3_asset_id: Option<String>,

    pub nameplate_position: NameplatePosition,
    pub nameplate_use_accent: bool,
    pub nameplate_text_hex: String,
    pub nameplate_asset_id: Option<String>,

    pub choices_anchor: UiAnchor,
    pub choices_offset_x: f32,
    pub choices_offset_y: f32,
    pub choices_width: f32,
    pub choices_style: ButtonVisualStyle,
    pub choice_text_hex: String,
    pub choice_button_asset_id: Option<String>,
    pub choice_button_hover_asset_id: Option<String>,
    pub choice_button_disabled_asset_id: Option<String>,

    pub hud_anchor: UiAnchor,
    pub hud_offset_x: f32,
    pub hud_offset_y: f32,
    pub hud_text_hex: String,
    pub show_scene_label: bool,
    pub show_quick_save_button: bool,
    pub show_quick_load_button: bool,
    #[serde(default = "default_true")]
    pub show_auto_button: bool,
    #[serde(default = "default_true")]
    pub show_skip_button: bool,
    pub show_history_button: bool,
    pub show_hide_ui_button: bool,
    pub show_menu_button: bool,
}

fn default_true() -> bool {
    true
}

impl Default for GameplayUiConfig {
    fn default() -> Self {
        Self {
            dialogue_anchor: UiAnchor::BottomCenter,
            dialogue_offset_x: 0.0,
            dialogue_offset_y: -24.0,
            dialogue_full_width: false,
            dialogue_width: 880.0,
            dialogue_opacity: 0.88,
            dialogue_font_size: 18.0,
            dialogue_text_hex: String::new(),
            dialogue_box_asset_id: None,
            advance_prompt_label: "Click / Space / A ->".to_string(),
            advance_prompt_asset_id: None,
            advance_prompt_frame2_asset_id: None,
            advance_prompt_frame3_asset_id: None,

            nameplate_position: NameplatePosition::TopLeft,
            nameplate_use_accent: true,
            nameplate_text_hex: String::new(),
            nameplate_asset_id: None,

            choices_anchor: UiAnchor::Center,
            choices_offset_x: 0.0,
            choices_offset_y: -20.0,
            choices_width: 440.0,
            choices_style: ButtonVisualStyle::GlassDark,
            choice_text_hex: String::new(),
            choice_button_asset_id: None,
            choice_button_hover_asset_id: None,
            choice_button_disabled_asset_id: None,

            hud_anchor: UiAnchor::TopRight,
            hud_offset_x: 0.0,
            hud_offset_y: 0.0,
            hud_text_hex: String::new(),
            show_scene_label: false,
            show_quick_save_button: true,
            show_quick_load_button: true,
            show_auto_button: true,
            show_skip_button: true,
            show_history_button: true,
            show_hide_ui_button: true,
            show_menu_button: true,
        }
    }
}

/// Button layout mode for Pause Menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PauseButtonsLayout {
    #[default]
    Vertical,
    Horizontal,
    TwoRowGrid,
}

/// Structural layout template for Pause Menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PauseLayoutTemplate {
    #[default]
    CenteredCard,
    LeftSidebar,
    RightSidebar,
    BottomBar,
    TopBar,
}

impl PauseLayoutTemplate {
    pub const ALL: &'static [Self] = &[
        Self::CenteredCard,
        Self::LeftSidebar,
        Self::RightSidebar,
        Self::BottomBar,
        Self::TopBar,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CenteredCard => "Centered Card",
            Self::LeftSidebar => "Left Sidebar",
            Self::RightSidebar => "Right Sidebar",
            Self::BottomBar => "Bottom Bar",
            Self::TopBar => "Top Bar",
        }
    }
}

/// Structural layout template for Settings Screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SettingsLayoutTemplate {
    #[default]
    Classic,
    TopTabs,
    LeftSidebar,
    RightSidebar,
    TwoColumnSplit,
}

impl SettingsLayoutTemplate {
    pub const ALL: &'static [Self] = &[
        Self::Classic,
        Self::TopTabs,
        Self::LeftSidebar,
        Self::RightSidebar,
        Self::TwoColumnSplit,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic (All)",
            Self::TopTabs => "Top Tabs",
            Self::LeftSidebar => "Left Sidebar",
            Self::RightSidebar => "Right Sidebar",
            Self::TwoColumnSplit => "2-Column Split",
        }
    }

    pub fn uses_tabs(self) -> bool {
        matches!(self, Self::TopTabs | Self::LeftSidebar | Self::RightSidebar)
    }
}

/// Pause Menu & In-Game Settings screen customization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PauseSettingsUiConfig {
    pub pause_show_panel: bool,
    pub pause_show_title: bool,
    pub pause_title_anchor: UiAnchor,
    pub pause_title_offset_x: f32,
    pub pause_title_offset_y: f32,

    pub pause_anchor: UiAnchor,
    pub pause_offset_x: f32,
    pub pause_offset_y: f32,
    pub pause_width: f32,
    pub pause_button_width: f32,
    pub pause_direction: ButtonDirection,
    pub pause_layout: PauseButtonsLayout,
    pub pause_layout_template: PauseLayoutTemplate,
    pub pause_overlay_dimming: f32,
    pub pause_panel_asset_id: Option<String>,

    pub pause_title: String,
    pub pause_text_hex: String,
    pub pause_button_text_hex: String,
    pub resume_label: String,
    pub save_label: String,
    pub load_label: String,
    pub settings_label: String,
    pub return_title_label: String,
    pub quit_label: String,

    pub show_resume_button: bool,
    pub show_save_button: bool,
    pub show_load_button: bool,
    pub show_settings_button: bool,
    pub show_return_title_button: bool,
    pub show_quit_button: bool,

    pub settings_show_panel: bool,
    pub settings_layout_template: SettingsLayoutTemplate,
    pub settings_overlay_dimming: f32,
    pub settings_show_title: bool,
    pub settings_title_anchor: UiAnchor,
    pub settings_title_offset_x: f32,
    pub settings_title_offset_y: f32,

    pub settings_anchor: UiAnchor,
    pub settings_offset_x: f32,
    pub settings_offset_y: f32,
    pub settings_width: f32,
    pub settings_panel_asset_id: Option<String>,

    pub settings_back_anchor: UiAnchor,
    pub settings_back_offset_x: f32,
    pub settings_back_offset_y: f32,
    pub settings_back_width: f32,

    pub settings_title: String,
    pub settings_text_hex: String,
    pub settings_button_text_hex: String,
    pub master_volume_label: String,
    pub bgm_volume_label: String,
    pub sfx_volume_label: String,
    pub text_speed_label: String,
    pub back_label: String,
}

impl Default for PauseSettingsUiConfig {
    fn default() -> Self {
        Self {
            pause_show_panel: false,
            pause_show_title: true,
            pause_title_anchor: UiAnchor::Center,
            pause_title_offset_x: 0.0,
            pause_title_offset_y: -190.0,

            pause_anchor: UiAnchor::Center,
            pause_offset_x: 0.0,
            pause_offset_y: 25.0,
            pause_width: 340.0,
            pause_button_width: 210.0,
            pause_direction: ButtonDirection::Vertical,
            pause_layout: PauseButtonsLayout::Vertical,
            pause_layout_template: PauseLayoutTemplate::CenteredCard,
            pause_overlay_dimming: 0.55,
            pause_panel_asset_id: None,

            pause_title: "Paused".to_string(),
            pause_text_hex: String::new(),
            pause_button_text_hex: String::new(),
            resume_label: "Resume".to_string(),
            save_label: "Save Game".to_string(),
            load_label: "Load Game".to_string(),
            settings_label: "Settings".to_string(),
            return_title_label: "Return to Title".to_string(),
            quit_label: "Quit Game".to_string(),

            show_resume_button: true,
            show_save_button: true,
            show_load_button: true,
            show_settings_button: true,
            show_return_title_button: true,
            show_quit_button: true,

            settings_show_panel: false,
            settings_layout_template: SettingsLayoutTemplate::Classic,
            settings_overlay_dimming: 0.55,
            settings_show_title: true,
            settings_title_anchor: UiAnchor::Center,
            settings_title_offset_x: 0.0,
            settings_title_offset_y: -180.0,

            settings_anchor: UiAnchor::Center,
            settings_offset_x: 0.0,
            settings_offset_y: 0.0,
            settings_width: 520.0,
            settings_panel_asset_id: None,

            settings_back_anchor: UiAnchor::Center,
            settings_back_offset_x: 0.0,
            settings_back_offset_y: 185.0,
            settings_back_width: 180.0,

            settings_title: "Settings".to_string(),
            settings_text_hex: String::new(),
            settings_button_text_hex: String::new(),
            master_volume_label: "Master Volume".to_string(),
            bgm_volume_label: "BGM Volume".to_string(),
            sfx_volume_label: "SFX Volume".to_string(),
            text_speed_label: "Text Speed".to_string(),
            back_label: "Back".to_string(),
        }
    }
}

/// Slot grid/list layout for Save & Load screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SaveLoadSlotLayout {
    #[default]
    TwoColumns,
    ThreeColumns,
    List,
}

impl SaveLoadSlotLayout {
    pub const ALL: &'static [SaveLoadSlotLayout] = &[
        SaveLoadSlotLayout::TwoColumns,
        SaveLoadSlotLayout::ThreeColumns,
        SaveLoadSlotLayout::List,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SaveLoadSlotLayout::TwoColumns => "2 Columns (3x2)",
            SaveLoadSlotLayout::ThreeColumns => "3 Columns (2x3)",
            SaveLoadSlotLayout::List => "Vertical List",
        }
    }
}

/// Structural layout template for Save & Load screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SaveLoadLayoutTemplate {
    #[default]
    Classic,
    TopBar,
    LeftSidebar,
    RightSidebar,
    BottomStrip,
}

impl SaveLoadLayoutTemplate {
    pub const ALL: &'static [Self] = &[
        Self::Classic,
        Self::TopBar,
        Self::LeftSidebar,
        Self::RightSidebar,
        Self::BottomStrip,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::TopBar => "Top Bar",
            Self::LeftSidebar => "Left Sidebar",
            Self::RightSidebar => "Right Sidebar",
            Self::BottomStrip => "Bottom Strip",
        }
    }
}

/// Customizable Save & Load screen layout, labels, and skinning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveLoadUiConfig {
    pub layout_template: SaveLoadLayoutTemplate,
    pub show_panel: bool,
    pub show_title: bool,
    pub title_anchor: UiAnchor,
    pub title_offset_x: f32,
    pub title_offset_y: f32,
    pub panel_anchor: UiAnchor,
    pub panel_offset_x: f32,
    pub panel_offset_y: f32,
    pub panel_width: f32,
    pub back_anchor: UiAnchor,
    pub back_offset_x: f32,
    pub back_offset_y: f32,
    pub back_width: f32,
    pub slot_layout: SaveLoadSlotLayout,
    pub overlay_dimming: f32,
    pub title_text_hex: String,
    pub slot_text_hex: String,
    pub button_text_hex: String,
    pub panel_asset_id: Option<String>,
    pub slot_card_asset_id: Option<String>,
    pub slot_card_hover_asset_id: Option<String>,
    pub slot_card_disabled_asset_id: Option<String>,
    pub save_title: String,
    pub load_title: String,
    pub empty_slot_label: String,
    pub save_btn_label: String,
    pub overwrite_btn_label: String,
    pub load_btn_label: String,
    pub delete_btn_label: String,
    pub back_label: String,
}

impl Default for SaveLoadUiConfig {
    fn default() -> Self {
        Self {
            layout_template: SaveLoadLayoutTemplate::Classic,
            show_panel: false,
            show_title: true,
            title_anchor: UiAnchor::Center,
            title_offset_x: 0.0,
            title_offset_y: -245.0,
            panel_anchor: UiAnchor::Center,
            panel_offset_x: 0.0,
            panel_offset_y: -10.0,
            panel_width: 680.0,
            back_anchor: UiAnchor::Center,
            back_offset_x: 0.0,
            back_offset_y: 250.0,
            back_width: 160.0,
            slot_layout: SaveLoadSlotLayout::TwoColumns,
            overlay_dimming: 0.55,
            title_text_hex: String::new(),
            slot_text_hex: String::new(),
            button_text_hex: String::new(),
            panel_asset_id: None,
            slot_card_asset_id: None,
            slot_card_hover_asset_id: None,
            slot_card_disabled_asset_id: None,
            save_title: "Save Game".to_string(),
            load_title: "Load Game".to_string(),
            empty_slot_label: "Empty Slot".to_string(),
            save_btn_label: "Save".to_string(),
            overwrite_btn_label: "Overwrite".to_string(),
            load_btn_label: "Load".to_string(),
            delete_btn_label: "Del".to_string(),
            back_label: "Back".to_string(),
        }
    }
}

/// Structural layout template for History / Backlog screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HistoryLayoutTemplate {
    #[default]
    CenteredModal,
    TopBarReader,
    LeftDrawer,
    RightDrawer,
}

impl HistoryLayoutTemplate {
    pub const ALL: &'static [Self] = &[
        Self::CenteredModal,
        Self::TopBarReader,
        Self::LeftDrawer,
        Self::RightDrawer,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CenteredModal => "Centered Modal",
            Self::TopBarReader => "Top Bar Reader",
            Self::LeftDrawer => "Left Drawer",
            Self::RightDrawer => "Right Drawer",
        }
    }
}

/// Dialogue History / Backlog screen configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HistoryUiConfig {
    pub layout_template: HistoryLayoutTemplate,
    pub show_panel: bool,
    pub panel_width: f32,
    pub panel_height: f32,
    pub overlay_dimming: f32,
    pub title_label: String,
    pub empty_label: String,
    pub narration_label: String,
    pub back_label: String,
    pub title_text_hex: String,
    pub speaker_text_hex: String,
    pub entry_text_hex: String,
    pub button_text_hex: String,
    pub panel_asset_id: Option<String>,
    pub entry_card_asset_id: Option<String>,
}

impl Default for HistoryUiConfig {
    fn default() -> Self {
        Self {
            layout_template: HistoryLayoutTemplate::CenteredModal,
            show_panel: true,
            panel_width: 780.0,
            panel_height: 540.0,
            overlay_dimming: 0.55,
            title_label: "Dialogue History".to_string(),
            empty_label: "No dialogue history yet.".to_string(),
            narration_label: "Narration".to_string(),
            back_label: "Back".to_string(),
            title_text_hex: String::new(),
            speaker_text_hex: String::new(),
            entry_text_hex: String::new(),
            button_text_hex: String::new(),
            panel_asset_id: None,
            entry_card_asset_id: None,
        }
    }
}

/// Quick layout presets for Ending / Return to Title screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndingQuickPreset {
    Centered,
    LeftPanel,
    RightPanel,
    BottomBar,
}

/// Ending / Return to Title screen UI customization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EndingUiConfig {
    pub show_panel: bool,
    pub show_logo: bool,
    pub use_stage_backdrop: bool,
    pub overlay_dimming: f32,
    pub panel_width: f32,
    pub button_width: f32,
    pub panel_anchor: UiAnchor,
    pub panel_offset_x: f32,
    pub panel_offset_y: f32,
    /// 0 = Left, 1 = Center, 2 = Right
    pub content_align: u8,
    pub title_label: String,
    pub subtitle_label: String,
    pub button_label: String,
    pub title_text_hex: String,
    pub subtitle_text_hex: String,
    pub button_text_hex: String,
    pub panel_asset_id: Option<String>,
    pub background_asset_id: Option<String>,
}

impl Default for EndingUiConfig {
    fn default() -> Self {
        Self {
            show_panel: true,
            show_logo: true,
            use_stage_backdrop: true,
            overlay_dimming: 0.65,
            panel_width: 480.0,
            button_width: 280.0,
            panel_anchor: UiAnchor::Center,
            panel_offset_x: 0.0,
            panel_offset_y: 0.0,
            content_align: 1,
            title_label: "Return to Title".to_string(),
            subtitle_label: "End of Story Arc".to_string(),
            button_label: "Continue to Title Screen".to_string(),
            title_text_hex: String::new(),
            subtitle_text_hex: String::new(),
            button_text_hex: String::new(),
            panel_asset_id: None,
            background_asset_id: None,
        }
    }
}

/// Credits Roll screen UI customization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreditsUiConfig {
    pub show_logo: bool,
    pub overlay_dimming: f32,
    pub panel_width: f32,
    pub scroll_speed: f32,
    pub auto_return_to_title: bool,
    pub title_label: String,
    pub subtitle_label: String,
    pub credits_text: String,
    pub skip_button_label: String,
    pub title_text_hex: String,
    pub body_text_hex: String,
    pub background_hex: String,
}

impl Default for CreditsUiConfig {
    fn default() -> Self {
        Self {
            show_logo: true,
            overlay_dimming: 0.88,
            panel_width: 680.0,
            scroll_speed: 42.0,
            auto_return_to_title: true,
            title_label: "Credits".to_string(),
            subtitle_label: "Thank you for playing".to_string(),
            credits_text: "Story & Direction\nYour Name\n\nCharacter Art & Backgrounds\nYour Name\n\nMusic & Sound Design\nYour Name\n\nBuilt with Blanc Engine".to_string(),
            skip_button_label: "Continue".to_string(),
            title_text_hex: String::new(),
            body_text_hex: String::new(),
            background_hex: "#0B101D".to_string(),
        }
    }
}

/// Game Over screen UI customization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameOverUiConfig {
    pub show_panel: bool,
    pub overlay_dimming: f32,
    pub panel_width: f32,
    pub button_width: f32,
    pub show_load_button: bool,
    pub show_quick_load_button: bool,
    pub title_label: String,
    pub subtitle_label: String,
    pub load_button_label: String,
    pub quick_load_button_label: String,
    pub title_button_label: String,
    pub title_text_hex: String,
    pub subtitle_text_hex: String,
    pub button_text_hex: String,
    pub background_hex: String,
}

impl Default for GameOverUiConfig {
    fn default() -> Self {
        Self {
            show_panel: true,
            overlay_dimming: 0.85,
            panel_width: 480.0,
            button_width: 260.0,
            show_load_button: true,
            show_quick_load_button: true,
            title_label: "GAME OVER".to_string(),
            subtitle_label: "Your journey ends here.".to_string(),
            load_button_label: "Load Save".to_string(),
            quick_load_button_label: "Quick Load".to_string(),
            title_button_label: "Return to Title".to_string(),
            title_text_hex: "#F87171".to_string(),
            subtitle_text_hex: String::new(),
            button_text_hex: String::new(),
            background_hex: "#120B10".to_string(),
        }
    }
}

/// Complete customizable runtime UI configuration
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeUiConfig {
    pub theme: ThemeUiConfig,
    pub main_menu: MainMenuUiConfig,
    pub gameplay: GameplayUiConfig,
    pub pause_settings: PauseSettingsUiConfig,
    pub save_load: SaveLoadUiConfig,
    pub history: HistoryUiConfig,
    pub ending: EndingUiConfig,
    pub credits: CreditsUiConfig,
    pub game_over: GameOverUiConfig,
}

/// Parse `#RRGGBB` or `RRGGBB` into RGB bytes
pub fn parse_hex_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Extract the Font Family name (NameID 1 or 16) from an OpenType/TrueType `.ttf`/`.otf` byte buffer.
pub fn read_ttf_family_name(bytes: &[u8]) -> Option<String> {
    let read_u16 = |offset: usize| -> Option<u16> {
        let s = bytes.get(offset..offset + 2)?;
        Some(u16::from_be_bytes([s[0], s[1]]))
    };
    let read_u32 = |offset: usize| -> Option<u32> {
        let s = bytes.get(offset..offset + 4)?;
        Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    };

    let num_tables = read_u16(4)? as usize;
    let mut name_table_offset = None;
    for i in 0..num_tables {
        let rec = 12 + i * 16;
        let tag = bytes.get(rec..rec + 4)?;
        if tag == b"name" {
            name_table_offset = Some(read_u32(rec + 8)? as usize);
            break;
        }
    }
    let table = name_table_offset?;
    let count = read_u16(table + 2)? as usize;
    let string_offset = table + read_u16(table + 4)? as usize;

    let mut fallback_mac: Option<String> = None;
    for i in 0..count {
        let rec = table + 6 + i * 12;
        let platform_id = read_u16(rec)?;
        let encoding_id = read_u16(rec + 2)?;
        let name_id = read_u16(rec + 6)?;
        let length = read_u16(rec + 8)? as usize;
        let offset = string_offset + read_u16(rec + 10)? as usize;

        if name_id != 1 && name_id != 16 {
            continue;
        }
        let raw = bytes.get(offset..offset + length)?;
        if platform_id == 3 && (encoding_id == 1 || encoding_id == 10) && raw.len() >= 2 {
            let units: Vec<u16> = raw
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            if let Ok(s) = String::from_utf16(&units) {
                let trimmed = s.trim().to_string();
                if !trimmed.is_empty() {
                    if name_id == 1 {
                        return Some(trimmed);
                    }
                    fallback_mac = Some(trimmed);
                }
            }
        } else if platform_id == 1 && fallback_mac.is_none() {
            let s = String::from_utf8_lossy(raw).trim().to_string();
            if !s.is_empty() {
                fallback_mac = Some(s);
            }
        }
    }
    fallback_mac
}
