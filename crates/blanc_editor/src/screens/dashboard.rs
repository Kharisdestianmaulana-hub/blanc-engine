use std::path::PathBuf;
use iced::widget::{button, canvas, column, container, image, row, scrollable, stack, text, text_editor, text_input, Space};
use iced::{Alignment, Element, Length};

use blanc_core::asset::{Asset, AssetCategory};
use blanc_core::config::{AspectScalingMode, SystemScreensConfig};
use blanc_core::flow::{FlowNodeType, StoryFlowGraph};
use blanc_core::project::Project;
use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption as SceneChoiceOption, Scene};
use blanc_core::story::{Story, StoryElement};
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::icons::*;
use crate::screens::flow_canvas::{FlowCanvasAction, FlowCanvasState, FlowCanvasWidget, FlowInteractionMode};
use crate::theme::{
    asset_card_button_style, badge_container_style, callout_container_style, canvas_container_style, card_container_style,
    code_container_style, danger_icon_button_style, icon_button_style, input_field_style,
    meter_bg_style, meter_fill_style, modal_backdrop_style, primary_button_style, project_card_style, secondary_button_style,
    sidebar_button_style, sidebar_container_style, subtle_button_style, text_editor_style,
};

pub const DEFAULT_SCRIPT_TEMPLATE: &str = r#"// Blanc Engine Screenplay Script
// Syntax: Speaker (Emotion): "Dialogue" | [Scene: Name] | [BGM: Track] | [Choice: ...]

[Scene: Classroom_Afternoon]
[BGM: peaceful_afternoon]

Alice (Happy): "Good afternoon! Are you ready for the cultural festival?"
Bob (Thinking): "Almost done. Just need to finalize our club's presentation booth."

The golden afternoon light poured through the tall windows, casting long shadows across the empty desks.

Alice: "The weather is so lovely today. Would you like to take a quick walk in the courtyard?"

[Choice: How will Bob respond?
  - "Take a walk with Alice" -> Scene_Courtyard
  - "Stay and finish preparations" -> Scene_Classroom_Work
]
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetPickerTarget {
    Block(usize),
    TitleBackground,
    TitleBgm,
}

#[derive(Debug, Clone)]
pub struct AssetPickerModalState {
    pub target: AssetPickerTarget,
    pub category: AssetCategory,
    pub search_query: String,
    pub selected_asset_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    PlayPreview,
    CloseProject,
    SelectWorkflowTab(WorkflowTab),
    OpenProjectFolder,

    // Status message dismiss
    DismissSceneStatusMessage,
    DismissSettingsStatusMessage,

    // Asset Manager messages
    SelectAssetCategory(Option<AssetCategory>),
    AssetSearchChanged(String),
    ImportAssetClicked(AssetCategory),
    DeleteAssetClicked(String),
    OpenAssetFolder(AssetCategory),
    RevealAssetInExplorer(String),

    // Story Setup messages
    ScriptEditorAction(text_editor::Action),
    LoadScriptTemplate,
    ImportScriptFileClicked,
    SaveStoryChapterClicked,
    ConvertScriptToScenesClicked,

    // Scene Editor messages
    SelectScene(usize),
    CreateNewScene,
    DeleteScene(usize),
    SaveCurrentScene,
    OpenScenesFolder,
    AddBlockToScene(ActionBlock),
    RemoveBlockFromScene(usize),
    MoveBlockUp(usize),
    MoveBlockDown(usize),
    UpdateBlockCharacterPosition(usize, CharacterPosition),
    UpdateBlockChoicePosition(usize, ChoiceLayoutPosition),
    UpdateSpeakerName(usize, String),
    UpdateDialogueText(usize, String),
    UpdateChoicePrompt(usize, String),

    // Custom Modal Asset Picker messages
    OpenAssetPickerModal { target: AssetPickerTarget, category: AssetCategory },
    CloseAssetPickerModal,
    AssetPickerSearchChanged(String),
    SelectAssetInModal(String),
    ConfirmAssetPickerSelection,
    ImportAssetFromModal(AssetCategory),

    // Project & System Screen Settings messages
    UpdateTitleText(String),
    UpdateStartingScene(String),
    UpdateAspectScalingMode(AspectScalingMode),
    UpdateFullscreenDefault(bool),
    UpdateDefaultResolution(u32, u32),
    UpdateMasterVolume(f32),
    UpdateBgmVolume(f32),
    UpdateSfxVolume(f32),
    UpdateTextSpeed(f32),
    UpdateAutoForwardDelay(f32),
    SaveSystemSettings,

    // Story Flow messages
    FlowAction(FlowCanvasAction),
    FlowAddScene(String),
    FlowAddSystemNode(String),
    FlowRemoveNode(String),
    FlowSetMode(FlowInteractionMode),
    FlowToggleSnapGrid,
    FlowFocusAll,
    FlowAutoLayout,
    FlowResetView,
    FlowZoomIn,
    FlowZoomOut,
    FlowSearchChanged(String),
    DismissFlowStatusMessage,
    SaveStoryFlow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowTab {
    Dashboard,
    Assets,
    Story,
    SceneEditor,
    StoryFlow,
    Settings,
}

impl WorkflowTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Dashboard => "Overview",
            Self::Assets => "Asset Manager",
            Self::Story => "Story Setup",
            Self::SceneEditor => "Scene Editor",
            Self::StoryFlow => "Story Flow",
            Self::Settings => "Settings",
        }
    }
}

pub struct DashboardState {
    pub active_tab: WorkflowTab,

    // Asset Manager state
    pub selected_category: Option<AssetCategory>,
    pub asset_search_query: String,
    pub status_message: Option<String>,

    // Story Setup state
    pub script_content: text_editor::Content,
    pub parsed_story_elements: Vec<StoryElement>,
    pub story_chapter_name: String,
    pub story_status_message: Option<String>,

    // Scene Editor state
    pub scenes_list: Vec<String>,
    pub selected_scene_index: usize,
    pub current_scene: Option<Scene>,
    pub scene_status_message: Option<String>,

    // Custom Modal Asset Picker state
    pub asset_picker_modal: Option<AssetPickerModalState>,

    // Settings state
    pub system_screens_config: SystemScreensConfig,
    pub settings_status_message: Option<String>,

    // Story Flow state
    pub flow_graph: StoryFlowGraph,
    #[allow(dead_code)]
    pub flow_canvas_state: FlowCanvasState,
    pub flow_selected_node_id: Option<String>,
    pub flow_mode: FlowInteractionMode,
    pub flow_snap_grid: bool,
    pub flow_search_query: String,
    pub flow_status_message: Option<String>,
    pub all_scenes: Vec<Scene>,
}

impl DashboardState {
    pub fn create_starter_scene() -> Scene {
        let mut opening_scene = Scene::new("01_opening", "Chapter_01");
        opening_scene.add_block(ActionBlock::Background {
            asset_id: "classroom_afternoon".to_string(),
            transition: Some(blanc_core::scene::TransitionType::Fade),
        });
        opening_scene.add_block(ActionBlock::Character {
            character_id: "Alice".to_string(),
            expression: "Happy".to_string(),
            position: CharacterPosition::Center,
            offset_x: 0.0,
            offset_y: 0.0,
            transition: Some(blanc_core::scene::TransitionType::Fade),
        });
        opening_scene.add_block(ActionBlock::Dialogue {
            speaker: "Alice".to_string(),
            text: "Hello! Welcome to Blanc Engine's Visual Scene Editor.".to_string(),
            voice_asset_id: None,
        });
        opening_scene.add_block(ActionBlock::Choice {
            prompt: "What would you like to explore next?".to_string(),
            position: ChoiceLayoutPosition::Center,
            options: vec![
                SceneChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Explore Classroom".to_string(),
                    target_scene_id: "Scene_Classroom".to_string(),
                },
                SceneChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Head Outside to Courtyard".to_string(),
                    target_scene_id: "Scene_Courtyard".to_string(),
                },
            ],
        });
        opening_scene
    }

    pub fn init_for_project(project_dir: &std::path::Path) -> Self {
        let story_chapter_name = "Chapter_01".to_string();
        let scenes_dir = project_dir.join("Scenes").join(&story_chapter_name);
        let _ = std::fs::create_dir_all(&scenes_dir);

        let mut scenes = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("bscene") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        scenes.push(stem.to_string());
                    }
                }
            }
        }
        scenes.sort();

        let current_scene = if scenes.is_empty() {
            let starter = Self::create_starter_scene();
            let _ = starter.save_to_file(&scenes_dir.join("01_opening.bscene"));
            scenes.push("01_opening".to_string());
            Some(starter)
        } else {
            let first_scene_path = scenes_dir.join(format!("{}.bscene", scenes[0]));
            if let Ok(loaded) = Scene::load_from_file(&first_scene_path) {
                if loaded.blocks.is_empty() && scenes[0] == "01_opening" {
                    let starter = Self::create_starter_scene();
                    let _ = starter.save_to_file(&first_scene_path);
                    Some(starter)
                } else {
                    Some(loaded)
                }
            } else {
                let starter = Self::create_starter_scene();
                Some(starter)
            }
        };

        // Check if Story chapter exists
        let story_path = project_dir.join("Story/Chapters").join(format!("{}.bstory", story_chapter_name));
        let (script_content, parsed_elements) = if story_path.exists() {
            if let Ok(chapter) = blanc_core::story::Chapter::load_from_file(&story_path) {
                (
                    text_editor::Content::with_text(&chapter.raw_script),
                    chapter.elements,
                )
            } else {
                (
                    text_editor::Content::with_text(DEFAULT_SCRIPT_TEMPLATE),
                    Story::parse_script(DEFAULT_SCRIPT_TEMPLATE),
                )
            }
        } else {
            (
                text_editor::Content::with_text(DEFAULT_SCRIPT_TEMPLATE),
                Story::parse_script(DEFAULT_SCRIPT_TEMPLATE),
            )
        };

        let config_path = project_dir.join("Settings/system_screens.bconfig");
        let system_screens_config = SystemScreensConfig::load_from_file(&config_path)
            .unwrap_or_default();

        let flow_graph_path = project_dir.join("Story/story_flow.bgraph");
        let flow_graph = if flow_graph_path.exists() {
            StoryFlowGraph::load_from_file(&flow_graph_path)
                .unwrap_or_else(|_| StoryFlowGraph::starter_graph(scenes.first().map(|s| s.as_str())))
        } else {
            let starter = StoryFlowGraph::starter_graph(scenes.first().map(|s| s.as_str()));
            let _ = starter.save_to_file(&flow_graph_path);
            starter
        };

        let mut all_scenes = Vec::new();
        for scene_name in &scenes {
            let path = scenes_dir.join(format!("{}.bscene", scene_name));
            if let Ok(loaded) = Scene::load_from_file(&path) {
                all_scenes.push(loaded);
            }
        }

        Self {
            active_tab: WorkflowTab::Dashboard,
            selected_category: Some(AssetCategory::Background),
            asset_search_query: String::new(),
            status_message: None,

            script_content,
            parsed_story_elements: parsed_elements,
            story_chapter_name,
            story_status_message: None,

            scenes_list: scenes,
            selected_scene_index: 0,
            current_scene,
            scene_status_message: None,

            asset_picker_modal: None,
            system_screens_config,
            settings_status_message: None,

            flow_graph,
            flow_canvas_state: FlowCanvasState::default(),
            flow_selected_node_id: None,
            flow_mode: FlowInteractionMode::Select,
            flow_snap_grid: true,
            flow_search_query: String::new(),
            flow_status_message: None,
            all_scenes,
        }
    }

    pub fn refresh_scenes(&mut self, project_dir: &std::path::Path) {
        let scenes_dir = project_dir.join("Scenes").join(&self.story_chapter_name);
        let _ = std::fs::create_dir_all(&scenes_dir);
        let mut scenes = Vec::new();
        let mut all_scenes = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("bscene") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        scenes.push(stem.to_string());
                        if let Ok(loaded) = Scene::load_from_file(&path) {
                            all_scenes.push(loaded);
                        }
                    }
                }
            }
        }
        scenes.sort();
        if scenes.is_empty() {
            scenes.push("01_opening".to_string());
        }
        self.scenes_list = scenes;
        self.all_scenes = all_scenes;
        if self.selected_scene_index >= self.scenes_list.len() {
            self.selected_scene_index = 0;
        }
    }
}

impl Default for DashboardState {
    fn default() -> Self {
        let initial_script = DEFAULT_SCRIPT_TEMPLATE.to_string();
        let parsed = Story::parse_script(&initial_script);

        Self {
            active_tab: WorkflowTab::Dashboard,
            selected_category: Some(AssetCategory::Background),
            asset_search_query: String::new(),
            status_message: None,

            script_content: text_editor::Content::with_text(&initial_script),
            parsed_story_elements: parsed,
            story_chapter_name: "Chapter_01".to_string(),
            story_status_message: None,

            scenes_list: vec!["01_opening".to_string()],
            selected_scene_index: 0,
            current_scene: Some(Self::create_starter_scene()),
            scene_status_message: None,

            asset_picker_modal: None,
            system_screens_config: SystemScreensConfig::default(),
            settings_status_message: None,

            flow_graph: StoryFlowGraph::default(),
            flow_canvas_state: FlowCanvasState::default(),
            flow_selected_node_id: None,
            flow_mode: FlowInteractionMode::Select,
            flow_snap_grid: true,
            flow_search_query: String::new(),
            flow_status_message: None,
            all_scenes: vec![Self::create_starter_scene()],
        }
    }
}

impl DashboardState {
    pub fn view<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let top_bar = self.view_top_bar(project, project_dir);
        let workflow_bar = self.view_workflow_bar();

        let main_content = match self.active_tab {
            WorkflowTab::Dashboard => self.view_overview_content(project, project_dir),
            WorkflowTab::Assets => self.view_assets_content(project, project_dir),
            WorkflowTab::Story => self.view_story_content(project, project_dir),
            WorkflowTab::SceneEditor => self.view_scene_editor_content(project, project_dir),
            WorkflowTab::StoryFlow => self.view_story_flow_content(project, project_dir),
            WorkflowTab::Settings => self.view_settings_content(project, project_dir),
        };

        let layout = column![top_bar, workflow_bar, main_content]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        let base_view: Element<'a, DashboardMessage> = container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(modal) = &self.asset_picker_modal {
            let modal_overlay = self.view_asset_picker_modal(modal, project, project_dir);
            stack![base_view, modal_overlay].into()
        } else {
            base_view
        }
    }

    // ==========================================
    // TOP BAR
    // ==========================================
    fn view_top_bar<'a>(&'a self, project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let project_name = text(&project.name)
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let manifest_label = text(format!("{}.blanc", project.name.replace(' ', "_")))
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let saved_badge = text("● Saved")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b));

        let info_col = column![
            project_name,
            row![manifest_label, Space::with_width(SPACE_3), saved_badge].align_y(Alignment::Center),
        ]
        .spacing(SPACE_1);

        let btn_folder = button(
            row![icon_folder(), text("Open Folder").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenProjectFolder);

        let btn_play = button(
            row![icon_play(), text("Play Preview").size(BODY_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::PlayPreview);

        let btn_close = button(text("Close Project").size(BODY_SM))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::CloseProject);

        let brand_logo = engine_logo(28.0);

        let brand_and_info = row![
            brand_logo,
            info_col,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let actions = row![btn_folder, btn_play, btn_close]
            .spacing(SPACE_3)
            .align_y(Alignment::Center);

        let bar_content = row![brand_and_info, Space::with_width(Length::Fill), actions]
            .padding([SPACE_3, SPACE_5])
            .align_y(Alignment::Center)
            .width(Length::Fill);

        container(bar_content)
            .width(Length::Fill)
            .style(sidebar_container_style)
            .into()
    }

    // ==========================================
    // WORKFLOW NAVIGATION BAR
    // ==========================================
    fn view_workflow_bar(&self) -> Element<'_, DashboardMessage> {
        let tabs = [
            WorkflowTab::Dashboard,
            WorkflowTab::Assets,
            WorkflowTab::Story,
            WorkflowTab::SceneEditor,
            WorkflowTab::StoryFlow,
            WorkflowTab::Settings,
        ];

        let mut tab_row = row![].spacing(SPACE_2);

        for tab in tabs {
            let is_active = self.active_tab == tab;
            let tab_btn = button(text(tab.label()).size(LABEL_MD))
                .padding([SPACE_2, SPACE_4])
                .style(sidebar_button_style(is_active))
                .on_press(DashboardMessage::SelectWorkflowTab(tab));

            tab_row = tab_row.push(tab_btn);
        }

        container(tab_row.padding([SPACE_2, SPACE_5]).width(Length::Fill))
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    BACKGROUND_PRIMARY.r,
                    BACKGROUND_PRIMARY.g,
                    BACKGROUND_PRIMARY.b,
                ))),
                border: iced::Border {
                    width: 1.0,
                    color: iced::Color::from_rgb8(BORDER_DEFAULT.r, BORDER_DEFAULT.g, BORDER_DEFAULT.b),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    // ==========================================
    // TAB 1: OVERVIEW CONTENT
    // ==========================================
    fn view_overview_content<'a>(&'a self, project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let overview_title = text("Project Overview")
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let card_author = column![
            text("Author / Studio").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.author).size(BODY_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let card_version = column![
            text("Project Version").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.version).size(BODY_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let card_assets = column![
            text("Registered Assets").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} files", project.assets.assets.len()))
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let summary_cards = row![
            container(card_author).padding(SPACE_4).width(Length::Fixed(220.0)).style(card_container_style),
            container(card_version).padding(SPACE_4).width(Length::Fixed(220.0)).style(card_container_style),
            container(card_assets).padding(SPACE_4).width(Length::Fixed(220.0)).style(card_container_style),
        ]
        .spacing(SPACE_4);

        let next_step_box = container(
            column![
                text("Step 4: Block-Based Scene Editor").size(HEADING_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                text("Switch to 'Scene Editor' to sequence modular action blocks: Dialogue, Character Sprites (with Left/Center/Right presets), Backgrounds, Audio, and Decision Choices.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_2),
                button(
                    row![icon_film(), text("Open Scene Editor").size(BODY_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_2, SPACE_4])
                .style(primary_button_style)
                .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::SceneEditor)),
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let content = column![overview_title, summary_cards, next_step_box]
            .spacing(SPACE_5)
            .padding(SPACE_6)
            .width(Length::Fill);

        scrollable(content).width(Length::Fill).height(Length::Fill).into()
    }

    // ==========================================
    // TAB 2: ASSET MANAGER
    // ==========================================
    fn view_assets_content<'a>(&'a self, project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let sidebar = self.view_asset_categories_sidebar(project);
        let main_pane = self.view_asset_list_pane(project);

        let split = row![sidebar, main_pane]
            .width(Length::Fill)
            .height(Length::Fill);

        container(split)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_asset_categories_sidebar<'a>(&'a self, project: &'a Project) -> Element<'a, DashboardMessage> {
        let title = text("CATEGORIES")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let total_count = project.assets.assets.len();
        let btn_all = button(
            row![
                icon_folder(),
                text("All Assets").size(LABEL_MD),
                Space::with_width(Length::Fill),
                text(format!("{}", total_count))
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.selected_category.is_none()))
        .on_press(DashboardMessage::SelectAssetCategory(None));

        let mut cat_col = column![title, btn_all].spacing(SPACE_1);

        for cat in AssetCategory::ALL {
            let is_selected = self.selected_category == Some(*cat);
            let count = project
                .assets
                .assets
                .values()
                .filter(|a| a.category == *cat)
                .count();

            let cat_icon = match cat {
                AssetCategory::Background => icon_image(),
                AssetCategory::Character | AssetCategory::CharacterExpression => icon_user(),
                AssetCategory::Bgm => icon_music(),
                AssetCategory::Sfx => icon_volume(),
                AssetCategory::Font => icon_type(),
                AssetCategory::Ui | AssetCategory::Other => icon_blocks(),
            };

            let btn = button(
                row![
                    cat_icon,
                    text(cat.display_name()).size(LABEL_MD),
                    Space::with_width(Length::Fill),
                    text(format!("{}", count))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(is_selected))
            .on_press(DashboardMessage::SelectAssetCategory(Some(*cat)));

            cat_col = cat_col.push(btn);
        }

        let sidebar_container = container(cat_col)
            .padding(SPACE_4)
            .width(Length::Fixed(260.0))
            .height(Length::Fill)
            .style(sidebar_container_style);

        sidebar_container.into()
    }

    fn view_asset_list_pane<'a>(&'a self, project: &'a Project) -> Element<'a, DashboardMessage> {
        let (category_title, category_subtitle, current_cat) = match self.selected_category {
            Some(cat) => (
                cat.display_name(),
                match cat {
                    AssetCategory::Background => "Background illustrations (1920×1080 recommended, PNG/WebP/JPG)",
                    AssetCategory::Character => "Full character sprites (Transparent PNG/WebP)",
                    AssetCategory::CharacterExpression => "Facial expression variants for character sprites",
                    AssetCategory::Bgm => "Background music tracks (Seamless looping OGG/WAV/MP3)",
                    AssetCategory::Sfx => "Short sound effects (WAV/OGG/MP3)",
                    AssetCategory::Font => "TrueType and OpenType fonts (TTF/OTF)",
                    AssetCategory::Ui => "Custom UI frames, button textures, and dialogue boxes",
                    AssetCategory::Other => "General files and miscellaneous game assets",
                },
                cat,
            ),
            None => ("All Assets", "All files currently registered in project manifest", AssetCategory::Background),
        };

        let title_col = column![
            text(category_title)
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text(category_subtitle)
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let search_input = text_input("Search assets...", &self.asset_search_query)
            .on_input(DashboardMessage::AssetSearchChanged)
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fixed(240.0))
            .style(input_field_style);

        let btn_open_folder = button(
            row![icon_folder(), text("Folder").size(LABEL_MD)]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenAssetFolder(current_cat));

        let btn_import = button(
            row![icon_plus(), text("Import Asset...").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::ImportAssetClicked(current_cat));

        let action_bar = row![
            title_col,
            Space::with_width(Length::Fill),
            search_input,
            btn_open_folder,
            btn_import,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let q_lower = self.asset_search_query.to_lowercase();
        let filtered_assets: Vec<&Asset> = project
            .assets
            .assets
            .values()
            .filter(|a| {
                if let Some(cat) = self.selected_category {
                    if a.category != cat {
                        return false;
                    }
                }
                if q_lower.is_empty() {
                    true
                } else {
                    a.name.to_lowercase().contains(&q_lower)
                        || a.id.to_lowercase().contains(&q_lower)
                        || a.relative_path.to_lowercase().contains(&q_lower)
                }
            })
            .collect();

        let list_content: Element<'_, DashboardMessage> = if filtered_assets.is_empty() {
            let empty_col = column![
                icon_gamepad(),
                text(if self.asset_search_query.is_empty() {
                    "No Assets In This Category"
                } else {
                    "No Matching Assets"
                })
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text(if self.asset_search_query.is_empty() {
                    "Click '+ Import Asset...' to add files, or drop files directly into the project folder."
                } else {
                    "Try searching with another asset name or file format."
                })
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_2),
                button(
                    row![icon_plus(), text("Import Asset Now").size(BODY_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_2, SPACE_4])
                .style(primary_button_style)
                .on_press(DashboardMessage::ImportAssetClicked(current_cat)),
            ]
            .spacing(SPACE_3)
            .align_x(Alignment::Center);

            container(empty_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let mut asset_list = column![].spacing(SPACE_3).width(Length::Fill);

            for asset in filtered_assets {
                let card = self.view_asset_card(asset);
                asset_list = asset_list.push(card);
            }

            scrollable(asset_list).width(Length::Fill).height(Length::Fill).into()
        };

        let mut pane = column![action_bar, Space::with_height(SPACE_4), list_content]
            .spacing(SPACE_2)
            .padding(SPACE_6)
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(msg) = &self.status_message {
            let status_banner = container(
                text(msg)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
            )
            .padding(SPACE_3)
            .style(callout_container_style);
            pane = pane.push(status_banner);
        }

        pane.into()
    }

    fn view_asset_card<'a>(&'a self, asset: &'a Asset) -> Element<'a, DashboardMessage> {
        let cat_icon = match asset.category {
            AssetCategory::Background => icon_image(),
            AssetCategory::Character | AssetCategory::CharacterExpression => icon_user(),
            AssetCategory::Bgm => icon_music(),
            AssetCategory::Sfx => icon_volume(),
            AssetCategory::Font => icon_type(),
            AssetCategory::Ui | AssetCategory::Other => icon_blocks(),
        };

        let icon_box = container(cat_icon)
            .padding(SPACE_3)
            .style(card_container_style);

        let size_str = format_bytes(asset.file_size);

        let details = column![
            row![
                text(&asset.name)
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                container(
                    text(asset.category.display_name())
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                )
                .padding([2.0, 6.0])
                .style(badge_container_style),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
            text(&asset.relative_path)
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            row![
                text(format!("ID: {}", asset.id))
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text(size_str)
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
        ]
        .spacing(SPACE_1);

        let btn_reveal = button(
            row![icon_external_link(), text("Explorer").size(LABEL_SM)]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(subtle_button_style)
        .on_press(DashboardMessage::RevealAssetInExplorer(asset.relative_path.clone()));

        let btn_delete = button(icon_trash())
            .padding([SPACE_2, SPACE_3])
            .style(subtle_button_style)
            .on_press(DashboardMessage::DeleteAssetClicked(asset.id.clone()));

        let actions = row![btn_reveal, btn_delete]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

        let card_content = row![
            icon_box,
            Space::with_width(SPACE_3),
            details,
            Space::with_width(Length::Fill),
            actions,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(card_content)
            .padding(SPACE_4)
            .width(Length::Fill)
            .style(project_card_style)
            .into()
    }

    // ==========================================
    // TAB 3: STORY SETUP & SCREENPLAY IMPORTER
    // ==========================================
    fn view_story_content<'a>(&'a self, _project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let title_col = column![
            text("Story Setup & Screenplay Importer")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Draft or import screenplays (.txt, .md, .fountain) and parse them into visual story scenes")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let btn_template = button(
            row![icon_file_text(), text("Load Template").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::LoadScriptTemplate);

        let btn_import = button(
            row![icon_download(), text("Import File...").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::ImportScriptFileClicked);

        let btn_save = button(
            row![icon_save(), text("Save (.bstory)").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::SaveStoryChapterClicked);

        let btn_convert = button(
            row![icon_zap(), text("Convert to Scenes").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::ConvertScriptToScenesClicked);

        let action_bar = row![
            title_col,
            Space::with_width(Length::Fill),
            btn_template,
            btn_import,
            btn_save,
            btn_convert,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let left_editor_pane = self.view_script_editor_pane();
        let right_preview_pane = self.view_script_parsed_preview_pane();

        let split_view = row![
            left_editor_pane,
            Space::with_width(SPACE_4),
            right_preview_pane,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let mut pane = column![action_bar, Space::with_height(SPACE_4), split_view]
            .spacing(SPACE_2)
            .padding(SPACE_6)
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(msg) = &self.story_status_message {
            let status_banner = container(
                text(msg)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
            )
            .padding(SPACE_3)
            .style(callout_container_style);
            pane = pane.push(status_banner);
        }

        pane.into()
    }

    fn view_script_editor_pane(&self) -> Element<'_, DashboardMessage> {
        let pane_title = row![
            icon_edit(),
            text("SCRIPT EDITOR")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            Space::with_width(Length::Fill),
            text("Format: Speaker (Emotion): \"Text\"")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let editor = text_editor(&self.script_content)
            .on_action(DashboardMessage::ScriptEditorAction)
            .padding(SPACE_3)
            .style(text_editor_style);

        let line_count = self.script_content.line_count();
        let stats_row = row![
            text(format!("{} lines", line_count))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} parsed elements", self.parsed_story_elements.len()))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let layout = column![
            pane_title,
            Space::with_height(SPACE_2),
            container(editor).width(Length::Fill).height(Length::Fill),
            Space::with_height(SPACE_2),
            stats_row,
        ]
        .width(Length::FillPortion(1))
        .height(Length::Fill);

        container(layout)
            .padding(SPACE_4)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .style(card_container_style)
            .into()
    }

    fn view_script_parsed_preview_pane(&self) -> Element<'_, DashboardMessage> {
        let pane_title = row![
            icon_film(),
            text("LIVE SCENE & DIALOGUE STREAM")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            Space::with_width(Length::Fill),
            text(format!("{} Elements", self.parsed_story_elements.len()))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let content: Element<'_, DashboardMessage> = if self.parsed_story_elements.is_empty() {
            let empty = column![
                icon_file_text(),
                text("No Parsed Script Elements")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Type dialogue on the left editor or load the sample template.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_2)
            .align_x(Alignment::Center);

            container(empty)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let mut stream_col = column![].spacing(SPACE_3).width(Length::Fill);

            for elem in &self.parsed_story_elements {
                let card = self.view_parsed_element_card(elem);
                stream_col = stream_col.push(card);
            }

            scrollable(stream_col).width(Length::Fill).height(Length::Fill).into()
        };

        let layout = column![
            pane_title,
            Space::with_height(SPACE_2),
            container(content).width(Length::Fill).height(Length::Fill),
        ]
        .width(Length::FillPortion(1))
        .height(Length::Fill);

        container(layout)
            .padding(SPACE_4)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .style(card_container_style)
            .into()
    }

    fn view_parsed_element_card<'a>(&'a self, elem: &'a StoryElement) -> Element<'a, DashboardMessage> {
        match elem {
            StoryElement::Scene(s) => {
                let banner = row![
                    icon_film(),
                    text(format!("SCENE: {}", s.scene_name))
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                container(banner)
                    .padding([SPACE_2, SPACE_4])
                    .width(Length::Fill)
                    .style(callout_container_style)
                    .into()
            }
            StoryElement::Audio(a) => {
                let badge_label = if a.is_bgm { "BGM" } else { "SFX" };
                let audio_row = row![
                    if a.is_bgm { icon_music() } else { icon_volume() },
                    text(format!("[{}] {}", badge_label, a.track_name))
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                container(audio_row)
                    .padding([SPACE_2, SPACE_3])
                    .width(Length::Fill)
                    .style(code_container_style)
                    .into()
            }
            StoryElement::Dialogue(d) => {
                let mut speaker_row = row![
                    icon_message(),
                    text(&d.speaker)
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                if let Some(expr) = &d.expression {
                    let expr_badge = container(
                        text(expr)
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    )
                    .padding([1.0, 6.0])
                    .style(badge_container_style);
                    speaker_row = speaker_row.push(expr_badge);
                }

                let speech_text = text(format!("\"{}\"", d.text))
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

                let card_col = column![speaker_row, speech_text].spacing(SPACE_2);

                container(card_col)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(project_card_style)
                    .into()
            }
            StoryElement::Narration(n) => {
                let narrative_text = text(&n.text)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

                container(narrative_text)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(card_container_style)
                    .into()
            }
            StoryElement::Choice(c) => {
                let prompt_text = c.prompt.as_deref().unwrap_or("Branching Decision");
                let mut choice_col = column![
                    row![
                        icon_branch(),
                        text(prompt_text)
                            .size(LABEL_MD)
                            .color(iced::Color::from_rgb8(STATUS_WARNING.r, STATUS_WARNING.g, STATUS_WARNING.b)),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
                ]
                .spacing(SPACE_2);

                for (idx, opt) in c.options.iter().enumerate() {
                    let opt_row = row![
                        text(format!("{}. \"{}\"", idx + 1, opt.text))
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        Space::with_width(Length::Fill),
                        text(format!("➔ {}", opt.target_scene))
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center);

                    choice_col = choice_col.push(container(opt_row).padding(SPACE_2).style(code_container_style));
                }

                container(choice_col)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(project_card_style)
                    .into()
            }
        }
    }

    // ==========================================
    // TAB 4: BLOCK-BASED SCENE EDITOR
    // ==========================================
    fn view_scene_editor_content<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let left_sidebar = self.view_scene_selector_sidebar(project_dir);
        let main_canvas = self.view_scene_canvas_pane(project);

        let split = row![left_sidebar, main_canvas]
            .width(Length::Fill)
            .height(Length::Fill);

        container(split)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_scene_selector_sidebar<'a>(&'a self, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let title_row = row![
            icon_film(),
            text("SCENES")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            Space::with_width(Length::Fill),
            text(format!("{}", self.scenes_list.len()))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        ]
        .align_y(Alignment::Center);

        let mut scenes_col = column![title_row].spacing(SPACE_2);

        for (idx, scene_name) in self.scenes_list.iter().enumerate() {
            let is_selected = self.selected_scene_index == idx;
            let block_count = if is_selected {
                self.current_scene.as_ref().map(|s| s.blocks.len()).unwrap_or(0)
            } else {
                0
            };

            let btn_scene = button(
                row![
                    icon_film(),
                    text(scene_name).size(LABEL_MD),
                    Space::with_width(Length::Fill),
                    if is_selected {
                        text(format!("{} blocks", block_count))
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b))
                    } else {
                        text("").size(LABEL_SM)
                    },
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(is_selected))
            .on_press(DashboardMessage::SelectScene(idx));

            let can_delete = self.scenes_list.len() > 1;
            let item_row: Element<'a, DashboardMessage> = if can_delete {
                let btn_delete = button(icon_close())
                    .padding([SPACE_1, SPACE_2])
                    .style(danger_icon_button_style)
                    .on_press(DashboardMessage::DeleteScene(idx));

                row![btn_scene, btn_delete]
                    .spacing(SPACE_1)
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .into()
            } else {
                btn_scene.into()
            };

            scenes_col = scenes_col.push(item_row);
        }

        let btn_new_scene = button(
            row![icon_plus(), text("New Scene").size(BODY_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(primary_button_style)
        .on_press(DashboardMessage::CreateNewScene);

        let btn_open_folder = button(
            row![icon_folder(), text("Scenes Folder").size(LABEL_SM)]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(subtle_button_style)
        .on_press(DashboardMessage::OpenScenesFolder);

        let sidebar_layout = column![
            scenes_col,
            Space::with_height(Length::Fill),
            btn_new_scene,
            btn_open_folder,
        ]
        .spacing(SPACE_2)
        .padding(SPACE_4)
        .width(Length::Fixed(260.0))
        .height(Length::Fill);

        container(sidebar_layout)
            .style(sidebar_container_style)
            .height(Length::Fill)
            .into()
    }

    fn view_scene_canvas_pane<'a>(&'a self, _project: &'a Project) -> Element<'a, DashboardMessage> {
        let Some(scene) = &self.current_scene else {
            return container(text("No scene selected.").size(BODY_MD))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
        };

        // Header bar with Scene Title and Save/Action buttons
        let scene_header = column![
            row![
                text(format!("Scene: {}", scene.name))
                    .size(HEADING_LG)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                container(
                    text(&scene.chapter_id)
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                )
                .padding([2.0, 8.0])
                .style(badge_container_style),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
            text(format!("{} action blocks sequenced in this scene", scene.blocks.len()))
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let btn_save = button(
            row![icon_save(), text("Save Scene").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(secondary_button_style)
        .on_press(DashboardMessage::SaveCurrentScene);

        let top_action_bar = row![
            scene_header,
            Space::with_width(Length::Fill),
            btn_save,
        ]
        .align_y(Alignment::Center);

        // Add Block Quick Bar
        let bar_label = text("ADD ACTION BLOCK:")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let btn_add_dialogue = button(
            row![icon_message(), text("Dialogue").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::Dialogue {
            speaker: "Alice".to_string(),
            text: "Type your spoken dialogue here...".to_string(),
            voice_asset_id: None,
        }));

        let btn_add_char = button(
            row![icon_user(), text("Character").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::Character {
            character_id: "Alice".to_string(),
            expression: "Normal".to_string(),
            position: CharacterPosition::Center,
            offset_x: 0.0,
            offset_y: 0.0,
            transition: Some(blanc_core::scene::TransitionType::Fade),
        }));

        let btn_add_bg = button(
            row![icon_image(), text("Background").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::Background {
            asset_id: "classroom_day".to_string(),
            transition: Some(blanc_core::scene::TransitionType::Fade),
        }));

        let btn_add_choice = button(
            row![icon_branch(), text("Choice").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::Choice {
            prompt: "What will the player choose?".to_string(),
            position: ChoiceLayoutPosition::Center,
            options: vec![
                SceneChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Option A".to_string(),
                    target_scene_id: "Scene_Next".to_string(),
                },
                SceneChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Option B".to_string(),
                    target_scene_id: "Scene_Alternative".to_string(),
                },
            ],
        }));

        let btn_add_bgm = button(
            row![icon_music(), text("BGM Audio").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::PlayBgm {
            asset_id: "theme_peaceful".to_string(),
            volume: 1.0,
            looping: true,
        }));

        let add_bar = container(
            row![
                bar_label,
                btn_add_dialogue,
                btn_add_char,
                btn_add_bg,
                btn_add_choice,
                btn_add_bgm,
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(card_container_style);

        // Blocks Stack list
        let mut blocks_col = column![].spacing(SPACE_3).width(Length::Fill);

        for (idx, block) in scene.blocks.iter().enumerate() {
            let card = self.view_action_block_card(idx, block, scene.blocks.len());
            blocks_col = blocks_col.push(card);
        }

        let scroll_pane = scrollable(blocks_col).width(Length::Fill).height(Length::Fill);

        let mut pane = column![
            top_action_bar,
            Space::with_height(SPACE_3),
            add_bar,
            Space::with_height(SPACE_3),
            scroll_pane,
        ]
        .spacing(SPACE_2)
        .padding(SPACE_6)
        .width(Length::Fill)
        .height(Length::Fill);

        if let Some(msg) = &self.scene_status_message {
            let dismiss_btn = button(icon_close())
                .padding([2.0, 6.0])
                .style(danger_icon_button_style)
                .on_press(DashboardMessage::DismissSceneStatusMessage);

            let status_content = row![
                icon_check(),
                text(msg)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                Space::with_width(Length::Fill),
                dismiss_btn,
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

            let status_banner = container(status_content)
                .padding([SPACE_2, SPACE_4])
                .style(callout_container_style);
            pane = pane.push(status_banner);
        }

        pane.into()
    }

    fn view_action_block_card<'a>(&'a self, idx: usize, block: &'a ActionBlock, total: usize) -> Element<'a, DashboardMessage> {
        let order_badge = container(
            text(format!("#{}", idx + 1))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        )
        .padding([2.0, 6.0])
        .style(code_container_style);

        // Block Type Header & Custom Inline Controls
        let (block_type_label, block_icon, content_element): (&'static str, iced::widget::svg::Svg<'a, iced::Theme>, Element<'a, DashboardMessage>) = match block {
            ActionBlock::Character { character_id, expression, position, .. } => {
                let pos_buttons = row![
                    text("Screen Position:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("Left").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == CharacterPosition::Left { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockCharacterPosition(idx, CharacterPosition::Left)),
                    button(text("Center").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == CharacterPosition::Center { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockCharacterPosition(idx, CharacterPosition::Center)),
                    button(text("Right").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == CharacterPosition::Right { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockCharacterPosition(idx, CharacterPosition::Right)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                let btn_pick_char = button(
                    row![icon_user(), text("Select Sprite...").size(LABEL_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_1, SPACE_3])
                .style(primary_button_style)
                .on_press(DashboardMessage::OpenAssetPickerModal {
                    target: AssetPickerTarget::Block(idx),
                    category: AssetCategory::Character,
                });

                let info_row = row![
                    text(format!("Character: {}", character_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Expression: {}", expression)).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    Space::with_width(Length::Fill),
                    btn_pick_char,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                (
                    "Character Sprite",
                    icon_user(),
                    column![info_row, pos_buttons].spacing(SPACE_2).into(),
                )
            }
            ActionBlock::Dialogue { speaker, text: spoken_text, .. } => {
                let speaker_input = text_input("Speaker", speaker)
                    .on_input(move |val| DashboardMessage::UpdateSpeakerName(idx, val))
                    .padding(SPACE_1)
                    .width(Length::Fixed(160.0))
                    .style(input_field_style);

                let text_field = text_input("Spoken dialogue...", spoken_text)
                    .on_input(move |val| DashboardMessage::UpdateDialogueText(idx, val))
                    .padding(SPACE_1)
                    .width(Length::Fill)
                    .style(input_field_style);

                let inputs_row = row![
                    speaker_input,
                    text_field,
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center);

                (
                    "Dialogue",
                    icon_message(),
                    inputs_row.into(),
                )
            }
            ActionBlock::Background { asset_id, transition } => {
                let trans_name = transition.map(|t| format!("{:?}", t)).unwrap_or_else(|| "Cut".to_string());
                let display_id = if asset_id.is_empty() { "None (Click to choose)".to_string() } else { asset_id.clone() };

                let btn_pick_bg = button(
                    row![icon_image(), text(if asset_id.is_empty() { "Select Background..." } else { "Change Background..." }).size(LABEL_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_1, SPACE_3])
                .style(primary_button_style)
                .on_press(DashboardMessage::OpenAssetPickerModal {
                    target: AssetPickerTarget::Block(idx),
                    category: AssetCategory::Background,
                });

                let content = row![
                    text(format!("Asset: {}", display_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Transition: {}", trans_name)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    Space::with_width(Length::Fill),
                    btn_pick_bg,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                (
                    "Background",
                    icon_image(),
                    content.into(),
                )
            }
            ActionBlock::Choice { prompt, options, position } => {
                let prompt_input = text_input("Question / Prompt", prompt)
                    .on_input(move |val| DashboardMessage::UpdateChoicePrompt(idx, val))
                    .padding(SPACE_1)
                    .width(Length::Fill)
                    .style(input_field_style);

                let pos_buttons = row![
                    text("Button Layout:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("Center Screen").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == ChoiceLayoutPosition::Center { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockChoicePosition(idx, ChoiceLayoutPosition::Center)),
                    button(text("Above Dialogue").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == ChoiceLayoutPosition::AboveDialogueBox { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockChoicePosition(idx, ChoiceLayoutPosition::AboveDialogueBox)),
                    button(text("Left").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == ChoiceLayoutPosition::Left { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockChoicePosition(idx, ChoiceLayoutPosition::Left)),
                    button(text("Right").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *position == ChoiceLayoutPosition::Right { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockChoicePosition(idx, ChoiceLayoutPosition::Right)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                let mut opts_col = column![prompt_input, pos_buttons].spacing(SPACE_2);

                for (opt_idx, opt) in options.iter().enumerate() {
                    let opt_item = row![
                        text(format!("{}. \"{}\"", opt_idx + 1, opt.text)).size(BODY_SM),
                        Space::with_width(Length::Fill),
                        text(format!("➔ Jump to: {}", opt.target_scene_id))
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center);

                    opts_col = opts_col.push(container(opt_item).padding(SPACE_2).style(code_container_style));
                }

                (
                    "Branching Player Choice",
                    icon_branch(),
                    opts_col.into(),
                )
            }
            ActionBlock::PlayBgm { asset_id, volume, looping } => {
                let btn_pick_bgm = button(
                    row![icon_music(), text("Choose BGM Track...").size(LABEL_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_1, SPACE_3])
                .style(primary_button_style)
                .on_press(DashboardMessage::OpenAssetPickerModal {
                    target: AssetPickerTarget::Block(idx),
                    category: AssetCategory::Bgm,
                });

                let info = row![
                    text(format!("Track: {}", asset_id)).size(BODY_SM).color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Volume: {:.0}%", volume * 100.0)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(if *looping { "Looping: Yes" } else { "Looping: No" }).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    Space::with_width(Length::Fill),
                    btn_pick_bgm,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                (
                    "Play BGM",
                    icon_music(),
                    info.into(),
                )
            }
            ActionBlock::PlaySfx { asset_id, volume } => {
                let btn_pick_sfx = button(
                    row![icon_music(), text("Choose SFX Track...").size(LABEL_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_1, SPACE_3])
                .style(primary_button_style)
                .on_press(DashboardMessage::OpenAssetPickerModal {
                    target: AssetPickerTarget::Block(idx),
                    category: AssetCategory::Sfx,
                });

                let info = row![
                    text(format!("SFX: {}", asset_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Volume: {:.0}%", volume * 100.0)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    Space::with_width(Length::Fill),
                    btn_pick_sfx,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                (
                    "Play SFX",
                    icon_music(),
                    info.into(),
                )
            }
            ActionBlock::Narration { text: narrative } => {
                let text_field = text_input("Narration text...", narrative)
                    .on_input(move |val| DashboardMessage::UpdateDialogueText(idx, val))
                    .padding(SPACE_1)
                    .width(Length::Fill)
                    .style(input_field_style);

                (
                    "Narration",
                    icon_file_text(),
                    text_field.into(),
                )
            }
            other => (
                other.block_type_name(),
                icon_blocks(),
                text("Custom block action").size(BODY_SM).into(),
            ),
        };

        // Header line for each block
        let block_header = row![
            order_badge,
            block_icon,
            text(block_type_label)
                .size(LABEL_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        // Control buttons (Move Up, Move Down, Delete)
        let btn_up = button(icon_arrow_up())
            .padding([SPACE_1, SPACE_2])
            .style(icon_button_style)
            .on_press_maybe(if idx > 0 { Some(DashboardMessage::MoveBlockUp(idx)) } else { None });

        let btn_down = button(icon_arrow_down())
            .padding([SPACE_1, SPACE_2])
            .style(icon_button_style)
            .on_press_maybe(if idx + 1 < total { Some(DashboardMessage::MoveBlockDown(idx)) } else { None });

        let btn_delete = button(icon_trash())
            .padding([SPACE_1, SPACE_2])
            .style(icon_button_style)
            .on_press(DashboardMessage::RemoveBlockFromScene(idx));

        let control_buttons = row![btn_up, btn_down, btn_delete]
            .spacing(SPACE_1)
            .align_y(Alignment::Center);

        let top_row = row![block_header, Space::with_width(Length::Fill), control_buttons]
            .align_y(Alignment::Center);

        let card_layout = column![
            top_row,
            Space::with_height(SPACE_1),
            content_element,
        ]
        .spacing(SPACE_2);

        container(card_layout)
            .padding(SPACE_4)
            .width(Length::Fill)
            .style(project_card_style)
            .into()
    }

    // ==========================================
    // CUSTOM MODAL ASSET PICKER
    // ==========================================
    fn view_asset_picker_modal<'a>(
        &'a self,
        modal: &'a AssetPickerModalState,
        project: &'a Project,
        project_dir: &'a PathBuf,
    ) -> Element<'a, DashboardMessage> {
        let (title, cat_name) = match modal.category {
            AssetCategory::Background => ("Select Background Illustration", "Backgrounds"),
            AssetCategory::Character => ("Select Character Sprite", "Characters"),
            AssetCategory::CharacterExpression => ("Select Character Expression", "Expressions"),
            AssetCategory::Bgm => ("Select Background Music (BGM)", "BGM"),
            AssetCategory::Sfx => ("Select Sound Effect (SFX)", "SFX"),
            AssetCategory::Font => ("Select Font Asset", "Fonts"),
            AssetCategory::Ui => ("Select UI Asset", "UI"),
            AssetCategory::Other => ("Select Asset", "Other"),
        };

        // Header
        let title_text = text(title)
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let badge = container(
            text(cat_name)
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        )
        .padding([2.0, 8.0])
        .style(badge_container_style);

        let btn_close = button(icon_close())
            .padding([SPACE_1, SPACE_2])
            .style(danger_icon_button_style)
            .on_press(DashboardMessage::CloseAssetPickerModal);

        let header_row = row![
            title_text,
            Space::with_width(SPACE_2),
            badge,
            Space::with_width(Length::Fill),
            btn_close,
        ]
        .align_y(Alignment::Center);

        // Search & Import Bar
        let search_box = text_input("Search assets by name...", &modal.search_query)
            .on_input(DashboardMessage::AssetPickerSearchChanged)
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(input_field_style);

        let btn_import = button(
            row![icon_plus(), text("Import New Asset...").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::ImportAssetFromModal(modal.category));

        let toolbar = row![search_box, btn_import].spacing(SPACE_3).align_y(Alignment::Center);

        // Filter assets
        let q_lower = modal.search_query.to_lowercase();
        let assets: Vec<&Asset> = project
            .assets
            .assets
            .values()
            .filter(|a| {
                let matches_cat = if modal.category == AssetCategory::Character {
                    a.category == AssetCategory::Character || a.category == AssetCategory::CharacterExpression
                } else {
                    a.category == modal.category
                };
                if !matches_cat {
                    return false;
                }
                if q_lower.is_empty() {
                    true
                } else {
                    a.name.to_lowercase().contains(&q_lower)
                        || a.id.to_lowercase().contains(&q_lower)
                        || a.relative_path.to_lowercase().contains(&q_lower)
                }
            })
            .collect();

        let grid_content: Element<'a, DashboardMessage> = if assets.is_empty() {
            let empty_view = column![
                icon_image(),
                text(if modal.search_query.is_empty() {
                    "No Assets Available in This Category"
                } else {
                    "No Assets Match Your Search"
                })
                .size(HEADING_SM)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Click 'Import New Asset...' to add files directly from your computer.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                Space::with_height(SPACE_2),
                button(
                    row![icon_plus(), text("Import Asset Now").size(LABEL_MD)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_2, SPACE_4])
                .style(primary_button_style)
                .on_press(DashboardMessage::ImportAssetFromModal(modal.category)),
            ]
            .spacing(SPACE_2)
            .align_x(Alignment::Center);

            container(empty_view)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let mut cards_col = column![].spacing(SPACE_3).width(Length::Fill);
            let mut current_row = row![].spacing(SPACE_3).width(Length::Fill);
            let mut in_row = 0;

            for asset in &assets {
                let is_selected = modal.selected_asset_id.as_deref() == Some(&asset.id);
                let full_path = project_dir.join(&asset.relative_path);

                let preview_element: Element<'a, DashboardMessage> = if modal.category == AssetCategory::Background
                    || modal.category == AssetCategory::Character
                    || modal.category == AssetCategory::CharacterExpression
                {
                    if full_path.exists() {
                        container(
                            image(image::Handle::from_path(&full_path))
                                .content_fit(iced::ContentFit::Cover)
                                .width(Length::Fixed(190.0))
                                .height(Length::Fixed(110.0)),
                        )
                        .style(code_container_style)
                        .into()
                    } else {
                        container(icon_image())
                            .width(Length::Fixed(190.0))
                            .height(Length::Fixed(110.0))
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                            .style(code_container_style)
                            .into()
                    }
                } else {
                    container(icon_music())
                        .width(Length::Fixed(190.0))
                        .height(Length::Fixed(80.0))
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .style(code_container_style)
                        .into()
                };

                let card_body = column![
                    preview_element,
                    row![
                        text(&asset.name)
                            .size(LABEL_MD)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        Space::with_width(Length::Fill),
                        if is_selected {
                            text("✓ Selected").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b))
                        } else {
                            text("").size(LABEL_SM)
                        },
                    ]
                    .align_y(Alignment::Center),
                    text(format!("{} • {}", &asset.relative_path, format_bytes(asset.file_size)))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_2);

                let card_btn = button(card_body)
                    .padding(SPACE_3)
                    .width(Length::Fixed(220.0))
                    .style(asset_card_button_style(is_selected))
                    .on_press(DashboardMessage::SelectAssetInModal(asset.id.clone()));

                current_row = current_row.push(card_btn);
                in_row += 1;

                if in_row == 3 {
                    cards_col = cards_col.push(current_row);
                    current_row = row![].spacing(SPACE_3).width(Length::Fill);
                    in_row = 0;
                }
            }

            if in_row > 0 {
                cards_col = cards_col.push(current_row);
            }

            scrollable(cards_col).width(Length::Fill).height(Length::Fill).into()
        };

        // Footer
        let selected_label = if let Some(sel_id) = &modal.selected_asset_id {
            let asset_name = project.assets.assets.get(sel_id).map(|a| a.name.as_str()).unwrap_or(sel_id);
            format!("Selected: {}", asset_name)
        } else {
            "No asset selected".to_string()
        };

        let footer_info = text(selected_label)
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let btn_cancel = button(text("Cancel").size(LABEL_MD))
            .padding([SPACE_2, SPACE_4])
            .style(secondary_button_style)
            .on_press(DashboardMessage::CloseAssetPickerModal);

        let btn_confirm = if modal.selected_asset_id.is_some() {
            button(
                row![icon_sparkle(), text("Use Selected Asset").size(LABEL_MD)]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_4])
            .style(primary_button_style)
            .on_press(DashboardMessage::ConfirmAssetPickerSelection)
        } else {
            button(text("Use Selected Asset").size(LABEL_MD))
                .padding([SPACE_2, SPACE_4])
                .style(subtle_button_style)
        };

        let footer_row = row![
            footer_info,
            Space::with_width(Length::Fill),
            btn_cancel,
            btn_confirm,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let dialog_box = column![
            header_row,
            toolbar,
            container(grid_content)
                .height(Length::Fixed(400.0))
                .width(Length::Fill),
            footer_row,
        ]
        .spacing(SPACE_4);

        let modal_card = container(dialog_box)
            .width(Length::Fixed(780.0))
            .padding(SPACE_6)
            .style(card_container_style);

        container(modal_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_backdrop_style)
            .into()
    }

    // ==========================================
    // TAB 4: STORY FLOW (VISUAL NODE GRAPH)
    // ==========================================
    fn view_story_flow_content<'a>(&'a self, _project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        // --- 1. Left Sidebar (Controls & Available Scenes) ---
        let title_row = row![
            icon_branch(),
            column![
                text("Story Flow Map").size(HEADING_SM),
                text("Visual Narrative Graph & Branching")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(2.0),
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        // Toolbar
        let btn_save = button(
            row![icon_save(), text("Save Story Flow").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .width(Length::Fill)
        .style(primary_button_style)
        .on_press(DashboardMessage::SaveStoryFlow);

        // Summary Card
        let total_nodes = self.flow_graph.nodes.len();
        let total_conns = self.flow_graph.connections.len();
        let scene_nodes_count = self
            .flow_graph
            .nodes
            .iter()
            .filter(|n| matches!(n.node_type, FlowNodeType::Scene { .. }))
            .count();
        let all_connected = self.flow_graph.nodes.iter().all(|n| match &n.node_type {
            FlowNodeType::Scene { .. } => self
                .flow_graph
                .connections
                .iter()
                .any(|c| c.from_node_id == n.id),
            _ => true,
        });

        let status_badge = if all_connected {
            container(
                row![
                    icon_check(),
                    text("All Paths Connected")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(
                            STATUS_SUCCESS.r,
                            STATUS_SUCCESS.g,
                            STATUS_SUCCESS.b,
                        )),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
            )
            .padding([2.0, 6.0])
            .style(code_container_style)
        } else {
            container(
                text("⚠ Unconnected Path(s)")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(
                        STATUS_WARNING.r,
                        STATUS_WARNING.g,
                        STATUS_WARNING.b,
                    )),
            )
            .padding([2.0, 6.0])
            .style(code_container_style)
        };

        let summary_card = container(
            column![row![
                text(format!(
                    "{} Nodes ({} Scenes) · {} Wires",
                    total_nodes, scene_nodes_count, total_conns
                ))
                .size(LABEL_SM),
                Space::with_width(Length::Fill),
                status_badge,
            ]
            .align_y(Alignment::Center)]
            .spacing(SPACE_2),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        // Section 1: Available Scenes List
        let search_input = text_input("Search scenes...", &self.flow_search_query)
            .on_input(DashboardMessage::FlowSearchChanged)
            .padding(SPACE_2)
            .size(LABEL_SM)
            .style(input_field_style);

        let mut scenes_list_col = column![].spacing(SPACE_2).width(Length::Fill);

        for scene_name in &self.scenes_list {
            if !self.flow_search_query.is_empty()
                && !scene_name
                    .to_lowercase()
                    .contains(&self.flow_search_query.to_lowercase())
            {
                continue;
            }
            let node_id = format!("scene_{}", scene_name);
            let is_on_canvas = self.flow_graph.nodes.iter().any(|n| n.id == node_id);

            let row_item = if is_on_canvas {
                row![
                    icon_film(),
                    text(scene_name).size(BODY_SM),
                    Space::with_width(Length::Fill),
                    button(icon_close())
                        .padding([2.0, 6.0])
                        .style(danger_icon_button_style)
                        .on_press(DashboardMessage::FlowRemoveNode(node_id)),
                    container(
                        text("In Flow")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(
                                STATUS_SUCCESS.r,
                                STATUS_SUCCESS.g,
                                STATUS_SUCCESS.b,
                            )),
                    )
                    .padding([2.0, 6.0])
                    .style(code_container_style),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
            } else {
                row![
                    icon_film(),
                    text(scene_name).size(BODY_SM),
                    Space::with_width(Length::Fill),
                    button(text("+ Add").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(primary_button_style)
                        .on_press(DashboardMessage::FlowAddScene(scene_name.clone())),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
            };

            scenes_list_col = scenes_list_col.push(
                container(row_item)
                    .width(Length::Fill)
                    .style(callout_container_style),
            );
        }

        // Section 2: System Nodes Library (Pendekatan A)
        let system_library_defs = [
            ("pause_menu", "Pause / In-Game Menu", "system_pause_menu"),
            ("settings", "In-Game Settings", "system_settings"),
            ("save_load", "Save & Load Screen", "system_save_load"),
            ("return_to_title", "Return to Title", "ending_return_to_title"),
            ("credits", "Credits Roll", "credits"),
            ("game_over", "Game Over Screen", "game_over"),
        ];

        let mut sys_nodes_col = column![].spacing(SPACE_2).width(Length::Fill);
        for (sys_type, label, check_id) in system_library_defs {
            let is_on_canvas = if check_id == "credits" || check_id == "game_over" {
                self.flow_graph.nodes.iter().any(|n| n.title == label)
            } else {
                self.flow_graph.nodes.iter().any(|n| n.id == check_id)
            };

            let row_item = if is_on_canvas {
                row![
                    icon_sparkle(),
                    text(label).size(BODY_SM),
                    Space::with_width(Length::Fill),
                    container(
                        text("In Flow")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(
                                STATUS_SUCCESS.r,
                                STATUS_SUCCESS.g,
                                STATUS_SUCCESS.b,
                            )),
                    )
                    .padding([2.0, 6.0])
                    .style(code_container_style),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
            } else {
                row![
                    icon_sparkle(),
                    text(label).size(BODY_SM),
                    Space::with_width(Length::Fill),
                    button(text("+ Add").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(primary_button_style)
                        .on_press(DashboardMessage::FlowAddSystemNode(sys_type.to_string())),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
            };

            sys_nodes_col = sys_nodes_col.push(
                container(row_item)
                    .width(Length::Fill)
                    .style(callout_container_style),
            );
        }

        // Canvas Tips & Shortcuts card at bottom
        let tips_card = container(
            column![
                text("Interaction & Shortcuts:")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(
                        TEXT_PRIMARY.r,
                        TEXT_PRIMARY.g,
                        TEXT_PRIMARY.b,
                    )),
                text("• V : Select & Move mode")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• C : Connect Wire mode")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• X : Cut Wire mode (Click wire/pin)")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• F : Focus / Frame All nodes")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• A : Auto Align layout")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• S : Toggle Grid Snap")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text("• Double-click scene to open editor")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        let library_scroll = scrollable(
            column![
                text("Available Scenes:")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(
                        TEXT_MUTED.r,
                        TEXT_MUTED.g,
                        TEXT_MUTED.b,
                    )),
                search_input,
                scenes_list_col,
                Space::with_height(SPACE_2),
                text("System Nodes (Library):")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(
                        TEXT_MUTED.r,
                        TEXT_MUTED.g,
                        TEXT_MUTED.b,
                    )),
                sys_nodes_col,
            ]
            .spacing(SPACE_2),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let mut sidebar = column![
            title_row,
            btn_save,
            summary_card,
            library_scroll,
            tips_card,
        ]
        .spacing(SPACE_3)
        .padding(SPACE_4)
        .width(Length::Fixed(320.0))
        .height(Length::Fill);

        if let Some(msg) = &self.flow_status_message {
            let dismiss_btn = button(icon_close())
                .padding([2.0, 6.0])
                .style(danger_icon_button_style)
                .on_press(DashboardMessage::DismissFlowStatusMessage);

            let banner = container(
                row![
                    icon_check(),
                    text(msg).size(BODY_SM).color(iced::Color::from_rgb8(
                        STATUS_SUCCESS.r,
                        STATUS_SUCCESS.g,
                        STATUS_SUCCESS.b,
                    )),
                    Space::with_width(Length::Fill),
                    dismiss_btn,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_4])
            .style(callout_container_style);

            sidebar = sidebar.push(banner);
        }

        let sidebar_container = container(sidebar)
            .width(Length::Fixed(320.0))
            .height(Length::Fill)
            .style(sidebar_container_style);

        // --- 2. Right Canvas Area & Top Toolbar ---
        // Mode Buttons
        let is_sel = self.flow_mode == FlowInteractionMode::Select;
        let btn_mode_select = button(
            row![
                icon_branch(),
                text("Select (V)").size(LABEL_SM),
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(if is_sel { primary_button_style } else { subtle_button_style })
        .on_press(DashboardMessage::FlowSetMode(FlowInteractionMode::Select));

        let is_conn = self.flow_mode == FlowInteractionMode::Connect;
        let btn_mode_connect = button(
            row![
                icon_branch(),
                text("Connect (C)").size(LABEL_SM),
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(if is_conn { primary_button_style } else { subtle_button_style })
        .on_press(DashboardMessage::FlowSetMode(FlowInteractionMode::Connect));

        let is_cut = self.flow_mode == FlowInteractionMode::CutWire;
        let btn_mode_cut = button(
            row![
                icon_close(),
                text("Cut Wire (X)").size(LABEL_SM),
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(if is_cut { primary_button_style } else { subtle_button_style })
        .on_press(DashboardMessage::FlowSetMode(FlowInteractionMode::CutWire));

        // View Buttons
        let btn_zoom_in = button(text("Zoom +").size(LABEL_SM))
            .padding([SPACE_1, SPACE_2])
            .style(subtle_button_style)
            .on_press(DashboardMessage::FlowZoomIn);

        let btn_zoom_out = button(text("Zoom -").size(LABEL_SM))
            .padding([SPACE_1, SPACE_2])
            .style(subtle_button_style)
            .on_press(DashboardMessage::FlowZoomOut);

        let btn_reset = button(text("Reset (0)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_2])
            .style(subtle_button_style)
            .on_press(DashboardMessage::FlowResetView);

        let btn_focus_all = button(text("Focus All (F)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_2])
            .style(subtle_button_style)
            .on_press(DashboardMessage::FlowFocusAll);

        let btn_auto_align = button(text("Auto Align (A)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_2])
            .style(subtle_button_style)
            .on_press(DashboardMessage::FlowAutoLayout);

        let btn_snap_grid = button(
            row![
                text(if self.flow_snap_grid { "Grid Snap: ON (S)" } else { "Grid Snap: OFF (S)" }).size(LABEL_SM),
            ]
            .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(if self.flow_snap_grid { primary_button_style } else { subtle_button_style })
        .on_press(DashboardMessage::FlowToggleSnapGrid);

        let mode_badge = match self.flow_mode {
            FlowInteractionMode::Select => container(text("Tool: Select & Move [V]").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)))
                .padding([2.0, 8.0])
                .style(code_container_style),
            FlowInteractionMode::Connect => container(text("Tool: Connect Wires [C]").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                .padding([2.0, 8.0])
                .style(badge_container_style),
            FlowInteractionMode::CutWire => container(text("Tool: Wire Cutter [X]").size(LABEL_SM).color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)))
                .padding([2.0, 8.0])
                .style(code_container_style),
        };

        let top_toolbar = container(
            row![
                text("Mode:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                btn_mode_select,
                btn_mode_connect,
                btn_mode_cut,
                Space::with_width(SPACE_2),
                text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                Space::with_width(SPACE_2),
                text("View:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                btn_zoom_in,
                btn_zoom_out,
                btn_reset,
                btn_focus_all,
                btn_auto_align,
                Space::with_width(SPACE_2),
                text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                Space::with_width(SPACE_2),
                btn_snap_grid,
                Space::with_width(Length::Fill),
                mode_badge,
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_container_style);

        let canvas_widget = canvas(FlowCanvasWidget {
            graph: &self.flow_graph,
            scenes: &self.all_scenes,
            selected_node_id: self.flow_selected_node_id.as_deref(),
            mode: self.flow_mode,
            snap_grid: self.flow_snap_grid,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let canvas_element: Element<'a, FlowCanvasAction> = canvas_widget.into();
        let canvas_container = container(canvas_element.map(DashboardMessage::FlowAction))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(canvas_container_style);

        let canvas_area = column![
            top_toolbar,
            canvas_container,
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

        row![sidebar_container, canvas_area]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // ==========================================
    // TAB 5: PROJECT & SYSTEM SCREEN SETTINGS
    // ==========================================
    fn view_settings_content<'a>(&'a self, project: &'a Project, _project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let title_col = column![
            text("Project & System Screen Settings")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Configure runtime title screen visuals, aspect ratio scaling, audio defaults, and gameplay parameters.")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let btn_save = button(
            row![icon_save(), text("Save System Settings").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::SaveSystemSettings);

        let header_row = row![title_col, Space::with_width(Length::Fill), btn_save]
            .spacing(SPACE_3)
            .align_y(Alignment::Center);

        // --- Card 1: Title Screen Visuals & Navigation ---
        let card1_title = row![
            icon_sparkle(),
            text("Title Screen & Navigation").size(HEADING_SM),
            Space::with_width(SPACE_2),
            container(text("RUNTIME STARTUP").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                .padding([2.0, 6.0])
                .style(badge_container_style),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        // Left Column: Identity & Starting Scene
        let title_text_label = text("Game Title Text (Displayed on Main Menu):")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));
        let title_text_input = text_input("e.g. Echoes of Memories", &self.system_screens_config.title_text)
            .on_input(DashboardMessage::UpdateTitleText)
            .padding([SPACE_2, SPACE_3])
            .style(input_field_style);

        let starting_scene_label = text("Starting Scene (New Game Target):")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));
        let current_start_scene = self.system_screens_config.starting_scene_id.as_deref().unwrap_or(
            self.scenes_list.first().map(|s| s.as_str()).unwrap_or("01_opening")
        );

        let mut scenes_row = row![].spacing(SPACE_2).align_y(Alignment::Center);
        for scene_name in &self.scenes_list {
            let is_selected = scene_name == current_start_scene;
            let btn_scene = button(
                row![
                    icon_film(),
                    text(scene_name).size(LABEL_SM),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
            )
            .padding([SPACE_2, SPACE_3])
            .style(if is_selected { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateStartingScene(scene_name.clone()));
            scenes_row = scenes_row.push(btn_scene);
        }

        let left_col = column![
            title_text_label,
            title_text_input,
            Space::with_height(SPACE_1),
            starting_scene_label,
            scenes_row,
        ]
        .spacing(SPACE_2)
        .width(Length::FillPortion(1));

        // Right Column: Title Screen Assets (Background & BGM)
        let bg_display = self.system_screens_config.title_background_asset_id.as_deref().unwrap_or("Default / None");
        let is_bg_custom = self.system_screens_config.title_background_asset_id.is_some();
        let btn_choose_bg = button(
            row![icon_image(), text("Choose Background...").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenAssetPickerModal {
            target: AssetPickerTarget::TitleBackground,
            category: AssetCategory::Background,
        });

        let bg_box = container(
            row![
                column![
                    text("Title Screen Artwork").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    row![
                        text("Asset: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        text(bg_display).size(BODY_SM).color(if is_bg_custom { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) } else { iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b) }),
                    ].align_y(Alignment::Center),
                ].spacing(SPACE_1),
                Space::with_width(Length::Fill),
                btn_choose_bg,
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        let bgm_display = self.system_screens_config.title_bgm_asset_id.as_deref().unwrap_or("Default / None");
        let is_bgm_custom = self.system_screens_config.title_bgm_asset_id.is_some();
        let btn_choose_bgm = button(
            row![icon_music(), text("Choose BGM...").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenAssetPickerModal {
            target: AssetPickerTarget::TitleBgm,
            category: AssetCategory::Bgm,
        });

        let bgm_box = container(
            row![
                column![
                    text("Main Menu Music (BGM)").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    row![
                        text("Track: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        text(bgm_display).size(BODY_SM).color(if is_bgm_custom { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) } else { iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b) }),
                    ].align_y(Alignment::Center),
                ].spacing(SPACE_1),
                Space::with_width(Length::Fill),
                btn_choose_bgm,
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        let right_col = column![
            bg_box,
            bgm_box,
        ]
        .spacing(SPACE_2)
        .width(Length::FillPortion(1));

        let card1_body = row![left_col, right_col].spacing(SPACE_4);

        let card1 = container(
            column![
                card1_title,
                card1_body,
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // --- Card 2: Display & Aspect Ratio Scaling ---
        let card2_title = row![icon_image(), text("Display & Aspect Ratio Adaptation").size(HEADING_SM)]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

        let scaling_desc = text("Configure how visuals adapt across 16:9, 16:10, and ultrawide screens without stretching or distortion.")
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let current_mode = self.system_screens_config.aspect_scaling_mode;
        let is_cover = current_mode == AspectScalingMode::Cover;
        let btn_mode_cover = button(
            column![
                row![
                    text("Smart Cover").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    Space::with_width(Length::Fill),
                    container(text("RECOMMENDED").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                        .padding([2.0, 6.0])
                        .style(badge_container_style),
                ]
                .align_y(Alignment::Center),
                text("Aspect Fill (100% Full Screen)").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                text("No black bars. Scales symmetrically to fill the screen while preserving 1:1 pixel aspect ratios.")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(asset_card_button_style(is_cover))
        .on_press(DashboardMessage::UpdateAspectScalingMode(AspectScalingMode::Cover));

        let is_blur = current_mode == AspectScalingMode::AmbientBlur;
        let btn_mode_blur = button(
            column![
                row![
                    text("Ambient Blur").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    Space::with_width(Length::Fill),
                    container(text("ULTRAWIDE").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                        .padding([2.0, 6.0])
                        .style(badge_container_style),
                ]
                .align_y(Alignment::Center),
                text("Dynamic Blur Wings (21:9)").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                text("Keeps crisp 16:9 art centered while softly blurring and tinting side wings on ultrawide monitors.")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(asset_card_button_style(is_blur))
        .on_press(DashboardMessage::UpdateAspectScalingMode(AspectScalingMode::AmbientBlur));

        let is_fit = current_mode == AspectScalingMode::Fit;
        let btn_mode_fit = button(
            column![
                row![
                    text("Fixed Ratio").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    Space::with_width(Length::Fill),
                    container(text("CINEMATIC").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)))
                        .padding([2.0, 6.0])
                        .style(code_container_style),
                ]
                .align_y(Alignment::Center),
                text("Letterbox & Pillarbox").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                text("Strict 16:9 canvas with solid cinema black borders for pixel-perfect composition on any monitor.")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(asset_card_button_style(is_fit))
        .on_press(DashboardMessage::UpdateAspectScalingMode(AspectScalingMode::Fit));

        let modes_row = row![btn_mode_cover, btn_mode_blur, btn_mode_fit].spacing(SPACE_3);

        // Resolution & Window Mode
        let res_label = container(
            text("Default Target Resolution:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
        )
        .width(170);

        let cur_w = self.system_screens_config.default_resolution_width;
        let cur_h = self.system_screens_config.default_resolution_height;

        let res_1080p = button(text("1920×1080 (Full HD)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if cur_w == 1920 && cur_h == 1080 { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateDefaultResolution(1920, 1080));

        let res_1440p = button(text("2560×1440 (2K QHD)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if cur_w == 2560 && cur_h == 1440 { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateDefaultResolution(2560, 1440));

        let res_720p = button(text("1280×720 (HD)").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if cur_w == 1280 && cur_h == 720 { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateDefaultResolution(1280, 720));

        let res_row = row![res_label, res_1080p, res_1440p, res_720p].spacing(SPACE_2).align_y(Alignment::Center);

        let is_fullscreen = self.system_screens_config.fullscreen_default;
        let win_label = container(
            text("Window Launch Mode:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
        )
        .width(170);

        let btn_windowed = button(text("Windowed Mode").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if !is_fullscreen { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateFullscreenDefault(false));

        let btn_fullscreen = button(text("Fullscreen Mode").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if is_fullscreen { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::UpdateFullscreenDefault(true));

        let win_row = row![win_label, btn_windowed, btn_fullscreen].spacing(SPACE_2).align_y(Alignment::Center);

        let card2 = container(
            column![
                card2_title,
                scaling_desc,
                modes_row,
                Space::with_height(SPACE_1),
                res_row,
                win_row,
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // --- Card 3: Audio Defaults ---
        let card3_title = row![icon_volume(), text("Default Audio Volumes").size(HEADING_SM)]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

        let audio_desc = text("Configure initial sound levels for players when starting a new game or resetting preferences.")
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        // Master Volume Row
        let master_vol = self.system_screens_config.master_volume;
        let master_fill = (master_vol.clamp(0.0, 1.0) * 140.0) as f32;
        let master_bar = container(
            row![
                container(Space::with_width(Length::Fixed(master_fill)))
                    .height(Length::Fixed(6.0))
                    .style(meter_fill_style)
            ]
        )
        .width(Length::Fixed(140.0))
        .height(Length::Fixed(6.0))
        .style(meter_bg_style);

        let master_badge = container(
            text(format!("{:.0}%", master_vol * 100.0))
                .size(LABEL_SM)
                .color(if master_vol == 0.0 { iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b) } else { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) })
        )
        .width(48)
        .align_x(Alignment::Center);

        let is_m_100 = (master_vol - 1.0).abs() < 0.01;
        let is_m_80 = (master_vol - 0.8).abs() < 0.01;
        let is_m_50 = (master_vol - 0.5).abs() < 0.01;
        let is_m_0 = master_vol == 0.0;

        let master_row = row![
            container(row![icon_volume(), text("Master Volume").size(LABEL_SM)].spacing(SPACE_2).align_y(Alignment::Center)).width(170),
            master_bar,
            master_badge,
            Space::with_width(SPACE_2),
            button(text("100%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_m_100 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateMasterVolume(1.0)),
            button(text("80%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_m_80 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateMasterVolume(0.8)),
            button(text("50%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_m_50 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateMasterVolume(0.5)),
            button(text("Mute").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_m_0 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateMasterVolume(0.0)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        // BGM Volume Row
        let bgm_vol = self.system_screens_config.bgm_volume;
        let bgm_fill = (bgm_vol.clamp(0.0, 1.0) * 140.0) as f32;
        let bgm_bar = container(
            row![
                container(Space::with_width(Length::Fixed(bgm_fill)))
                    .height(Length::Fixed(6.0))
                    .style(meter_fill_style)
            ]
        )
        .width(Length::Fixed(140.0))
        .height(Length::Fixed(6.0))
        .style(meter_bg_style);

        let bgm_badge = container(
            text(format!("{:.0}%", bgm_vol * 100.0))
                .size(LABEL_SM)
                .color(if bgm_vol == 0.0 { iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b) } else { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) })
        )
        .width(48)
        .align_x(Alignment::Center);

        let is_b_100 = (bgm_vol - 1.0).abs() < 0.01;
        let is_b_80 = (bgm_vol - 0.8).abs() < 0.01;
        let is_b_50 = (bgm_vol - 0.5).abs() < 0.01;
        let is_b_0 = bgm_vol == 0.0;

        let bgm_vol_row = row![
            container(row![icon_music(), text("BGM Music").size(LABEL_SM)].spacing(SPACE_2).align_y(Alignment::Center)).width(170),
            bgm_bar,
            bgm_badge,
            Space::with_width(SPACE_2),
            button(text("100%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_b_100 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateBgmVolume(1.0)),
            button(text("80%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_b_80 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateBgmVolume(0.8)),
            button(text("50%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_b_50 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateBgmVolume(0.5)),
            button(text("Mute").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_b_0 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateBgmVolume(0.0)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        // SFX Volume Row
        let sfx_vol = self.system_screens_config.sfx_volume;
        let sfx_fill = (sfx_vol.clamp(0.0, 1.0) * 140.0) as f32;
        let sfx_bar = container(
            row![
                container(Space::with_width(Length::Fixed(sfx_fill)))
                    .height(Length::Fixed(6.0))
                    .style(meter_fill_style)
            ]
        )
        .width(Length::Fixed(140.0))
        .height(Length::Fixed(6.0))
        .style(meter_bg_style);

        let sfx_badge = container(
            text(format!("{:.0}%", sfx_vol * 100.0))
                .size(LABEL_SM)
                .color(if sfx_vol == 0.0 { iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b) } else { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) })
        )
        .width(48)
        .align_x(Alignment::Center);

        let is_s_100 = (sfx_vol - 1.0).abs() < 0.01;
        let is_s_80 = (sfx_vol - 0.8).abs() < 0.01;
        let is_s_50 = (sfx_vol - 0.5).abs() < 0.01;
        let is_s_0 = sfx_vol == 0.0;

        let sfx_vol_row = row![
            container(row![icon_zap(), text("Sound Effects (SFX)").size(LABEL_SM)].spacing(SPACE_2).align_y(Alignment::Center)).width(170),
            sfx_bar,
            sfx_badge,
            Space::with_width(SPACE_2),
            button(text("100%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_s_100 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateSfxVolume(1.0)),
            button(text("80%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_s_80 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateSfxVolume(0.8)),
            button(text("50%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_s_50 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateSfxVolume(0.5)),
            button(text("Mute").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_s_0 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateSfxVolume(0.0)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let card3 = container(
            column![
                card3_title,
                audio_desc,
                master_row,
                bgm_vol_row,
                sfx_vol_row,
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // --- Card 4: Gameplay & Text Speed ---
        let card4_title = row![icon_file_text(), text("Text & Dialogue Playback").size(HEADING_SM)]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

        let playback_desc = text("Control typewriter animation speed and auto-forward waiting interval during scene dialogue.")
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let cur_speed = self.system_screens_config.text_speed;
        let speed_label = container(
            text("Text Display Speed:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
        )
        .width(170);

        let speed_row = row![
            speed_label,
            button(text("Slow (0.5×)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_speed - 0.5).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateTextSpeed(0.5)),
            button(text("Normal (1.0×)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_speed - 1.0).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateTextSpeed(1.0)),
            button(text("Fast (1.5×)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_speed - 1.5).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateTextSpeed(1.5)),
            button(text("Instant (2.0×)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_speed - 2.0).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateTextSpeed(2.0)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let cur_delay = self.system_screens_config.auto_forward_delay;
        let delay_label = container(
            text("Auto-Forward Delay:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
        )
        .width(170);

        let delay_row = row![
            delay_label,
            button(text("Fast (1.5s)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_delay - 1.5).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateAutoForwardDelay(1.5)),
            button(text("Normal (3.0s)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_delay - 3.0).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateAutoForwardDelay(3.0)),
            button(text("Relaxed (5.0s)").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if (cur_delay - 5.0).abs() < 0.01 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateAutoForwardDelay(5.0)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let card4 = container(
            column![
                card4_title,
                playback_desc,
                speed_row,
                delay_row,
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // --- Card 5: Project & License Info ---
        let item_project = row![
            text("Project Name: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.name).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1)
        .align_y(Alignment::Center);

        let item_author = row![
            text("Author: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.author).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1)
        .align_y(Alignment::Center);

        let item_license = row![
            text("License: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text("GNU GPLv3 (100% Creator Ownership)")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
        ]
        .spacing(SPACE_1)
        .align_y(Alignment::Center);

        let item_engine = row![
            text("Engine: ").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text("Blanc Engine 0.1.0-alpha")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        ]
        .spacing(SPACE_1)
        .align_y(Alignment::Center);

        let meta_row = row![
            item_project,
            Space::with_width(SPACE_5),
            item_author,
            Space::with_width(SPACE_5),
            item_license,
            Space::with_width(SPACE_5),
            item_engine,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let engine_badge_logo = engine_logo_squircle(40.0);

        let card5_content = row![
            engine_badge_logo,
            column![
                row![icon_info(), text("Project & Engine Specifications").size(HEADING_SM)]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
                meta_row,
            ]
            .spacing(SPACE_2),
        ]
        .spacing(SPACE_4)
        .align_y(Alignment::Center);

        let card5 = container(card5_content)
            .padding(SPACE_4)
            .width(Length::Fill)
            .style(code_container_style);

        let mut content_col = column![
            header_row,
            card1,
            card2,
            card3,
            card4,
            card5,
        ]
        .spacing(SPACE_4)
        .width(Length::Fill);

        if let Some(msg) = &self.settings_status_message {
            let dismiss_btn = button(icon_close())
                .padding([2.0, 6.0])
                .style(danger_icon_button_style)
                .on_press(DashboardMessage::DismissSettingsStatusMessage);

            let banner = container(
                row![
                    icon_check(),
                    text(msg).size(BODY_SM).color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                    Space::with_width(Length::Fill),
                    dismiss_btn,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
            )
            .padding([SPACE_2, SPACE_4])
            .style(callout_container_style);

            content_col = content_col.push(banner);
        }

        let centered_content = container(content_col)
            .max_width(1120.0)
            .width(Length::Fill);

        scrollable(
            container(centered_content)
                .width(Length::Fill)
                .padding(SPACE_6)
                .center_x(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    // ==========================================
    // PLACEHOLDER FOR UPCOMING PHASES
    // ==========================================
    #[allow(dead_code)]
    fn view_placeholder_content<'a>(&'a self, title: &'a str, desc: &'a str) -> Element<'a, DashboardMessage> {
        let content = container(
            column![
                icon_sparkle(),
                text(title)
                    .size(HEADING_MD)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text(desc)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_3)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

        content.into()
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
