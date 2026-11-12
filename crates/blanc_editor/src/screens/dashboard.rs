use std::path::PathBuf;
use iced::widget::{button, column, container, row, scrollable, text, text_editor, text_input, Space};
use iced::{Alignment, Element, Length};

use blanc_core::asset::{Asset, AssetCategory};
use blanc_core::project::Project;
use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption as SceneChoiceOption, Scene};
use blanc_core::story::{Story, StoryElement};
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::icons::*;
use crate::theme::{
    badge_container_style, callout_container_style, card_container_style, code_container_style,
    danger_icon_button_style, icon_button_style, input_field_style, primary_button_style,
    project_card_style, secondary_button_style, sidebar_button_style, sidebar_container_style,
    subtle_button_style, text_editor_style,
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

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    PlayPreview,
    CloseProject,
    SelectWorkflowTab(WorkflowTab),
    OpenProjectFolder,

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowTab {
    Dashboard,
    Assets,
    Story,
    SceneEditor,
    Settings,
}

impl WorkflowTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Dashboard => "Overview",
            Self::Assets => "Asset Manager",
            Self::Story => "Story Setup",
            Self::SceneEditor => "Scene Editor",
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
        }
    }

    pub fn refresh_scenes(&mut self, project_dir: &std::path::Path) {
        let scenes_dir = project_dir.join("Scenes").join(&self.story_chapter_name);
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
        if scenes.is_empty() {
            scenes.push("01_opening".to_string());
        }
        self.scenes_list = scenes;
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
            WorkflowTab::Settings => self.view_placeholder_content("Project & System Screen Settings", "Runtime UI configurations (Main Menu, Save/Load, In-Game Pause) coming in Phase 5."),
        };

        let layout = column![top_bar, workflow_bar, main_content]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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

        let actions = row![btn_folder, btn_play, btn_close]
            .spacing(SPACE_3)
            .align_y(Alignment::Center);

        let bar_content = row![info_col, Space::with_width(Length::Fill), actions]
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

                let info_row = row![
                    text(format!("Character: {}", character_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Expression: {}", expression)).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                ]
                .spacing(SPACE_2);

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
                let content = row![
                    text(format!("Asset: {}", asset_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Transition: {}", trans_name)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_2);

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
                let info = row![
                    text(format!("Track: {}", asset_id)).size(BODY_SM).color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Volume: {:.0}%", volume * 100.0)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text("•").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(if *looping { "Looping: Yes" } else { "Looping: No" }).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_2);

                (
                    "Play BGM",
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
    // PLACEHOLDER FOR UPCOMING PHASES
    // ==========================================
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
