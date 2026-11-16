use std::collections::HashMap;
use std::path::PathBuf;
use iced::widget::{
    button, canvas, column, container, image, progress_bar, row, scrollable,
    stack, text, text_editor, text_input, Space,
};
use iced::{Alignment, Element, Length};

use blanc_core::asset::{Asset, AssetCategory};
use blanc_core::config::{AspectScalingMode, SceneTransitionMode, SystemScreensConfig};
use blanc_core::flow::{FlowNodeType, StoryFlowGraph};
use blanc_core::installer::{
    default_privacy_and_terms_en, default_privacy_and_terms_map, CustomPolicyLanguage,
    ExportBuildKind, InstallerLang,
};
use blanc_core::localization::{
    collect_project_items, LocaleCategory, LocaleManifest, LocaleStrings, LocalizationItem,
};
use blanc_core::pack::ExportSaveMode;
use blanc_core::project::Project;
use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceLayoutPosition, ChoiceOption as SceneChoiceOption, Scene};
use blanc_core::story::{Story, StoryElement};
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::icons::*;
use crate::screens::flow_canvas::{FlowCanvasAction, FlowCanvasState, FlowCanvasWidget, FlowInteractionMode};
use crate::screens::ui_designer::{
    view_ui_designer, UiAssetSlot, UiDesignerMessage, UiDesignerSections, UiDesignerSubTab,
    UiPreviewMode,
};
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
    UiAsset(UiAssetSlot),
}

#[derive(Debug, Clone)]
pub struct AssetPickerModalState {
    pub target: AssetPickerTarget,
    pub category: AssetCategory,
    pub search_query: String,
    pub selected_asset_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceUndoSnapshot {
    pub selected_scene_index: usize,
    pub current_scene: Option<Scene>,
    pub flow_graph: StoryFlowGraph,
    pub system_screens_config: SystemScreensConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingUnsavedAction {
    CloseProject,
    ExitApp(iced::window::Id),
}

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    PlayPreview,
    ClosePreview,
    CloseProject,
    SaveAllProjectChanges,
    Undo,
    Redo,
    ValidateProject,
    CloseValidationModal,
    ConfirmSaveAndContinue,
    ConfirmDiscardAndContinue,
    CancelUnsavedPrompt,
    SelectWorkflowTab(WorkflowTab),
    OpenProjectFolder,

    // Status message dismiss
    DismissSceneStatusMessage,
    DismissSettingsStatusMessage,

    // Asset Manager messages
    SelectAssetCategory(Option<AssetCategory>),
    AssetSearchChanged(String),
    SetAssetViewMode(AssetViewMode),
    SelectAssetForPreview(String),
    ToggleAssetPreviewBg,
    PlayAssetAudioPreview(String),
    StopAssetAudioPreview,
    ImportAssetClicked(AssetCategory),
    DeleteAssetClicked(String),
    ConfirmDeleteAsset,
    CancelDeleteAsset,
    RelinkAssetClicked(String),
    OpenAssetFolder(AssetCategory),
    RevealAssetInExplorer(String),

    // Story Setup messages
    ScriptEditorAction(text_editor::Action),
    LoadScriptTemplate,
    ImportScriptFileClicked,
    SaveStoryChapterClicked,
    ConvertScriptToScenesClicked,
    SetStoryScriptLanguage(String, String),
    StoryLangCustomCodeChanged(String),
    OpenFormatGuide,
    CloseFormatGuide,

    // Scene Editor messages
    SelectScene(usize),
    CreateNewScene,
    DeleteScene(usize),
    SaveCurrentScene,
    PlayFromCurrentScene,
    OpenScenesFolder,
    AddBlockToScene(ActionBlock),
    RemoveBlockFromScene(usize),
    MoveBlockUp(usize),
    MoveBlockDown(usize),
    UpdateBlockCharacterPosition(usize, CharacterPosition),
    UpdateBlockTransition(usize, Option<blanc_core::scene::TransitionType>),
    UpdateVisualEffect(usize, blanc_core::scene::ScreenEffectType, f32),
    UpdateBlockChoicePosition(usize, ChoiceLayoutPosition),
    UpdateSpeakerName(usize, String),
    UpdateDialogueText(usize, String),
    UpdateDialogueNameplatePosition(usize, Option<CharacterPosition>),
    ClearBlockVoice(usize),
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
    UpdateProjectVersion(String),
    UpdateProjectAuthor(String),
    UpdateAspectScalingMode(AspectScalingMode),
    UpdateFullscreenDefault(bool),
    UpdateDefaultResolution(u32, u32),
    UpdateMasterVolume(f32),
    UpdateBgmVolume(f32),
    UpdateSfxVolume(f32),
    UpdateVoiceVolume(f32),
    UpdateTextSpeed(f32),
    UpdateAutoForwardDelay(f32),
    UpdateSceneTransitionMode(SceneTransitionMode),
    SaveSystemSettings,

    // Project Logo & Branding messages
    SetUseCustomLogo(bool),
    ChooseProjectLogo,
    ProjectLogoSelected(Option<PathBuf>),

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

    // UI Designer messages
    UiDesigner(UiDesignerMessage),
    ImportUiPackFolder,
    UiPackFolderSelected(Option<PathBuf>),

    // Export messages
    SetExportExeName(String),
    SetExportOutputDir(String),
    BrowseExportOutputDir,
    ExportOutputDirSelected(Option<PathBuf>),
    SetExportPacked(bool),
    SetExportSaveMode(ExportSaveMode),
    SetExportBuildKind(ExportBuildKind),
    SetInstallerVersion(String),
    SetInstallerPublisher(String),
    SetInstallerWelcome(String),
    SelectInstallerPolicyLang(String),
    InstallerPolicyEditorAction(text_editor::Action),
    ResetInstallerPolicyDefault,
    ToggleAddInstallerPolicyLang,
    SetNewPolicyLangCode(String),
    SetNewPolicyLangName(String),
    ConfirmAddInstallerPolicyLang,
    RemoveCustomPolicyLang(String),
    SetInstallerDesktopShortcut(bool),
    SetInstallerStartMenuShortcut(bool),
    SetInstallerIncludeLogo(bool),
    SetInstallerIncludeBanner(bool),
    PickInstallerCustomBanner,
    ClearInstallerCustomBanner,
    RunExportGame,
    ExportProgress(f32, String),
    ExportDone(Result<String, String>),
    OpenExportFolder,

    // Scene Editor scroll preservation
    SceneEditorBlocksScrolled(scrollable::AbsoluteOffset),

    // Localization tab
    LocaleSyncFromProject,
    LocaleSelectLanguage(String),
    LocaleNewCodeChanged(String),
    LocaleNewNameChanged(String),
    LocaleDefaultCodeChanged(String),
    LocaleDefaultNameChanged(String),
    LocaleSaveDefaultLanguage,
    LocaleAddLanguage,
    LocaleRemoveLanguage(String),
    LocaleSearchChanged(String),
    LocaleFilterScopeChanged(LocaleFilterScope),
    LocaleToggleUntranslatedOnly(bool),
    LocaleTranslationChanged(String, String),
    LocaleSaveAll,
    LocaleExportCsv,
    LocaleImportCsv,
    LocaleToggleDefaultEditor,
    LocaleToggleStoryLangEditor,
    LocaleToggleAddForm,
    LocaleFillPresetLanguage(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocaleFilterScope {
    All,
    Ui,
    Speaker,
    Scene(String),
}

impl std::fmt::Display for LocaleFilterScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All Strings"),
            Self::Ui => write!(f, "UI Labels"),
            Self::Speaker => write!(f, "Character Names"),
            Self::Scene(name) => write!(f, "Scene: {}", name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetViewMode {
    List,
    Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowTab {
    Dashboard,
    Assets,
    Story,
    SceneEditor,
    StoryFlow,
    UiDesigner,
    Localization,
    Settings,
    Export,
}

impl WorkflowTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Dashboard => "Overview",
            Self::Assets => "Asset Manager",
            Self::Story => "Story Setup",
            Self::SceneEditor => "Scene Editor",
            Self::StoryFlow => "Story Flow",
            Self::UiDesigner => "UI Designer",
            Self::Localization => "Localization",
            Self::Settings => "Settings",
            Self::Export => "Export",
        }
    }
}

pub struct DashboardState {
    pub active_tab: WorkflowTab,

    // Asset Manager state
    pub selected_category: Option<AssetCategory>,
    pub asset_search_query: String,
    pub asset_view_mode: AssetViewMode,
    pub selected_preview_asset_id: Option<String>,
    pub asset_preview_light_bg: bool,
    pub playing_audio_asset_id: Option<String>,
    pub audio_stream: Option<(rodio::OutputStream, rodio::OutputStreamHandle)>,
    pub audio_sink: Option<rodio::Sink>,
    pub status_message: Option<String>,

    // Story Setup state
    pub script_content: text_editor::Content,
    pub parsed_story_elements: Vec<StoryElement>,
    pub story_chapter_name: String,
    pub story_status_message: Option<String>,
    pub story_lang_custom_input: String,
    pub show_format_guide_modal: bool,

    // Scene Editor state
    pub scenes_list: Vec<String>,
    pub selected_scene_index: usize,
    pub current_scene: Option<Scene>,
    pub scene_status_message: Option<String>,
    pub scene_blocks_scroll_offset: scrollable::AbsoluteOffset,

    // Custom Modal Asset Picker state
    pub asset_picker_modal: Option<AssetPickerModalState>,
    pub pending_delete_asset: Option<(String, String, Vec<String>)>,
    pub has_unsaved_changes: bool,
    pub is_preview_running: bool,
    pub pending_unsaved_action: Option<PendingUnsavedAction>,
    pub undo_stack: Vec<WorkspaceUndoSnapshot>,
    pub redo_stack: Vec<WorkspaceUndoSnapshot>,
    pub last_undo_burst: Option<(std::time::Instant, &'static str)>,
    pub validation_report: Option<blanc_core::validator::ValidationReport>,
    pub show_validation_modal: bool,

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

    // UI Designer state
    pub ui_designer_sub_tab: UiDesignerSubTab,
    pub ui_designer_sections: UiDesignerSections,
    pub ui_preview_mode: UiPreviewMode,

    // Export state
    pub export_exe_name: String,
    pub export_output_dir: String,
    pub export_packed: bool,
    pub export_save_mode: ExportSaveMode,
    pub export_build_kind: ExportBuildKind,
    pub installer_version: String,
    pub installer_publisher: String,
    pub installer_welcome_text: String,
    pub installer_policy_lang: String,
    pub installer_policy_texts: std::collections::BTreeMap<String, String>,
    pub installer_policy_editor: text_editor::Content,
    pub installer_custom_policy_langs: Vec<CustomPolicyLanguage>,
    pub installer_add_lang_open: bool,
    pub installer_new_lang_code: String,
    pub installer_new_lang_name: String,
    pub installer_desktop_shortcut: bool,
    pub installer_start_menu_shortcut: bool,
    pub installer_include_logo: bool,
    pub installer_include_banner: bool,
    pub installer_custom_banner_path: Option<PathBuf>,
    pub export_in_progress: bool,
    pub export_progress: f32,
    pub export_stage_label: String,
    pub export_status_message: Option<String>,

    // Localization tab state
    pub locale_manifest: LocaleManifest,
    pub locale_items: Vec<LocalizationItem>,
    pub locale_tables: HashMap<String, LocaleStrings>,
    pub locale_selected_code: String,
    pub locale_new_code: String,
    pub locale_new_name: String,
    pub locale_default_code: String,
    pub locale_default_name: String,
    pub locale_search_query: String,
    pub locale_filter_scope: LocaleFilterScope,
    pub locale_untranslated_only: bool,
    pub locale_status_message: Option<String>,
    pub locale_show_default_editor: bool,
    pub locale_show_add_form: bool,
    pub locale_story_lang_open: bool,
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
            sprite_asset_id: None,
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
            nameplate_position: None,
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

        let locale_manifest = LocaleManifest::load_from_dir(project_dir);
        let locale_items = collect_project_items(project_dir);
        let mut locale_tables = HashMap::new();
        for lang in locale_manifest.all_languages() {
            let strings = LocaleStrings::load_from_dir(project_dir, &lang.code);
            locale_tables.insert(lang.code.to_ascii_lowercase(), strings);
        }
        let locale_selected_code = locale_manifest
            .languages
            .first()
            .map(|l| l.code.clone())
            .unwrap_or_else(|| locale_manifest.default_language.code.clone());
        let locale_default_code = locale_manifest.default_language.code.clone();
        let locale_default_name = locale_manifest.default_language.display_name.clone();

        Self {
            active_tab: WorkflowTab::Dashboard,
            selected_category: Some(AssetCategory::Background),
            asset_search_query: String::new(),
            asset_view_mode: AssetViewMode::Grid,
            selected_preview_asset_id: None,
            asset_preview_light_bg: false,
            playing_audio_asset_id: None,
            audio_stream: None,
            audio_sink: None,
            status_message: None,

            script_content,
            parsed_story_elements: parsed_elements,
            story_chapter_name,
            story_status_message: None,
            story_lang_custom_input: String::new(),
            show_format_guide_modal: false,

            scenes_list: scenes,
            selected_scene_index: 0,
            current_scene,
            scene_status_message: None,
            scene_blocks_scroll_offset: scrollable::AbsoluteOffset::default(),

            asset_picker_modal: None,
            pending_delete_asset: None,
            has_unsaved_changes: false,
            is_preview_running: false,
            pending_unsaved_action: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_undo_burst: None,
            validation_report: None,
            show_validation_modal: false,
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
            ui_designer_sub_tab: UiDesignerSubTab::ThemeAndButtons,
            ui_designer_sections: UiDesignerSections::default(),
            ui_preview_mode: UiPreviewMode::MainMenu,
            export_exe_name: String::new(),
            export_output_dir: project_dir.join("Exports").to_string_lossy().to_string(),
            export_packed: true,
            export_save_mode: ExportSaveMode::Portable,
            export_build_kind: ExportBuildKind::Portable,
            installer_version: "1.0.0".to_string(),
            installer_publisher: String::new(),
            installer_welcome_text: "Welcome! Click Next to install the game on your computer.".to_string(),
            installer_policy_lang: InstallerLang::En.code().to_string(),
            installer_policy_texts: default_privacy_and_terms_map(),
            installer_policy_editor: text_editor::Content::with_text(&default_privacy_and_terms_en(
                "{GAME_TITLE}",
                "{PUBLISHER}",
            )),
            installer_custom_policy_langs: Vec::new(),
            installer_add_lang_open: false,
            installer_new_lang_code: String::new(),
            installer_new_lang_name: String::new(),
            installer_desktop_shortcut: true,
            installer_start_menu_shortcut: true,
            installer_include_logo: true,
            installer_include_banner: true,
            installer_custom_banner_path: None,
            export_in_progress: false,
            export_progress: 0.0,
            export_stage_label: String::new(),
            export_status_message: None,

            locale_manifest,
            locale_items,
            locale_tables,
            locale_selected_code,
            locale_new_code: String::new(),
            locale_new_name: String::new(),
            locale_default_code,
            locale_default_name,
            locale_search_query: String::new(),
            locale_filter_scope: LocaleFilterScope::Ui,
            locale_untranslated_only: false,
            locale_status_message: None,
            locale_show_default_editor: false,
            locale_show_add_form: false,
            locale_story_lang_open: false,
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

        let locale_manifest = LocaleManifest::default();
        let locale_items = Vec::new();
        let locale_tables = HashMap::new();
        let locale_selected_code = locale_manifest.default_language.code.clone();
        let locale_default_code = locale_manifest.default_language.code.clone();
        let locale_default_name = locale_manifest.default_language.display_name.clone();

        Self {
            active_tab: WorkflowTab::Dashboard,
            selected_category: Some(AssetCategory::Background),
            asset_search_query: String::new(),
            asset_view_mode: AssetViewMode::Grid,
            selected_preview_asset_id: None,
            asset_preview_light_bg: false,
            playing_audio_asset_id: None,
            audio_stream: None,
            audio_sink: None,
            status_message: None,

            script_content: text_editor::Content::with_text(&initial_script),
            parsed_story_elements: parsed,
            story_chapter_name: "Chapter_01".to_string(),
            story_status_message: None,
            story_lang_custom_input: String::new(),
            show_format_guide_modal: false,

            scenes_list: vec!["01_opening".to_string()],
            selected_scene_index: 0,
            current_scene: Some(Self::create_starter_scene()),
            scene_status_message: None,
            scene_blocks_scroll_offset: scrollable::AbsoluteOffset::default(),

            asset_picker_modal: None,
            pending_delete_asset: None,
            has_unsaved_changes: false,
            is_preview_running: false,
            pending_unsaved_action: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_undo_burst: None,
            validation_report: None,
            show_validation_modal: false,
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
            ui_designer_sub_tab: UiDesignerSubTab::ThemeAndButtons,
            ui_designer_sections: UiDesignerSections::default(),
            ui_preview_mode: UiPreviewMode::MainMenu,
            export_exe_name: String::new(),
            export_output_dir: String::new(),
            export_packed: true,
            export_save_mode: ExportSaveMode::Portable,
            export_build_kind: ExportBuildKind::Portable,
            installer_version: "1.0.0".to_string(),
            installer_publisher: String::new(),
            installer_welcome_text: "Welcome! Click Next to install the game on your computer.".to_string(),
            installer_policy_lang: InstallerLang::En.code().to_string(),
            installer_policy_texts: default_privacy_and_terms_map(),
            installer_policy_editor: text_editor::Content::with_text(&default_privacy_and_terms_en(
                "{GAME_TITLE}",
                "{PUBLISHER}",
            )),
            installer_custom_policy_langs: Vec::new(),
            installer_add_lang_open: false,
            installer_new_lang_code: String::new(),
            installer_new_lang_name: String::new(),
            installer_desktop_shortcut: true,
            installer_start_menu_shortcut: true,
            installer_include_logo: true,
            installer_include_banner: true,
            installer_custom_banner_path: None,
            export_in_progress: false,
            export_progress: 0.0,
            export_stage_label: String::new(),
            export_status_message: None,

            locale_manifest,
            locale_items,
            locale_tables,
            locale_selected_code,
            locale_new_code: String::new(),
            locale_new_name: String::new(),
            locale_default_code,
            locale_default_name,
            locale_search_query: String::new(),
            locale_filter_scope: LocaleFilterScope::Ui,
            locale_untranslated_only: false,
            locale_status_message: None,
            locale_show_default_editor: false,
            locale_show_add_form: false,
            locale_story_lang_open: false,
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
            WorkflowTab::UiDesigner => view_ui_designer(
                &self.system_screens_config.ui_config,
                self.ui_designer_sub_tab,
                &self.ui_designer_sections,
                self.ui_preview_mode,
                &self.system_screens_config.title_text,
                self.system_screens_config.title_background_asset_id.as_deref(),
                self.system_screens_config.logo_asset_id.as_deref(),
                project,
                project_dir,
                self.settings_status_message.as_deref(),
            ),
            WorkflowTab::Localization => self.view_localization_content(),
            WorkflowTab::Settings => self.view_settings_content(project, project_dir),
            WorkflowTab::Export => self.view_export_content(project, project_dir),
        };

        let layout = column![top_bar, workflow_bar, main_content]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        let base_view: Element<'a, DashboardMessage> = container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        let mut root_stack = stack![base_view];
        if let Some(modal) = &self.asset_picker_modal {
            let modal_overlay = self.view_asset_picker_modal(modal, project, project_dir);
            root_stack = root_stack.push(modal_overlay);
        }
        if let Some((_id, name, used_in)) = &self.pending_delete_asset {
            let mut modal_col = column![
                text("Delete Asset?")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text(format!("Are you sure you want to permanently delete '{}' from the project and disk?", name))
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_2);

            if !used_in.is_empty() {
                modal_col = modal_col.push(
                    container(
                        column![
                            text("Warning: This asset is currently referenced in:")
                                .size(LABEL_SM)
                                .color(iced::Color::from_rgb8(STATUS_WARNING.r, STATUS_WARNING.g, STATUS_WARNING.b)),
                            text(used_in.join(", "))
                                .size(BODY_SM)
                                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        ]
                        .spacing(SPACE_1),
                    )
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(callout_container_style),
                );
            }

            modal_col = modal_col.push(
                row![
                    Space::with_width(Length::Fill),
                    button(text("Cancel").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::CancelDeleteAsset),
                    button(text("Delete Permanently").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(danger_icon_button_style)
                        .on_press(DashboardMessage::ConfirmDeleteAsset),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            );

            let confirm_overlay = container(
                container(modal_col)
                    .padding(SPACE_5)
                    .width(Length::Fixed(460.0))
                    .style(card_container_style),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_backdrop_style);

            root_stack = root_stack.push(confirm_overlay);
        }
        if self.pending_unsaved_action.is_some() {
            let unsaved_modal = container(
                container(
                    column![
                        text("Unsaved Changes")
                            .size(HEADING_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        text("You have unsaved changes in this project. Do you want to save everything before closing?")
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                        Space::with_height(SPACE_1),
                        row![
                            Space::with_width(Length::Fill),
                            button(text("Cancel").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_3])
                                .style(secondary_button_style)
                                .on_press(DashboardMessage::CancelUnsavedPrompt),
                            button(text("Discard Changes").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_3])
                                .style(danger_icon_button_style)
                                .on_press(DashboardMessage::ConfirmDiscardAndContinue),
                            button(text("Save & Close").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_4])
                                .style(primary_button_style)
                                .on_press(DashboardMessage::ConfirmSaveAndContinue),
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(SPACE_2),
                )
                .padding(SPACE_5)
                .width(Length::Fixed(480.0))
                .style(card_container_style),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_backdrop_style);

            root_stack = root_stack.push(unsaved_modal);
        }

        if self.show_validation_modal {
            let report = self.validation_report.clone().unwrap_or_default();
            let err_count = report.error_count();
            let warn_count = report.warning_count();
            let summary_text = if report.is_clean() {
                "All checks passed! Your project is ready to play and export.".to_string()
            } else {
                format!(
                    "Found {} critical error(s) (block export) and {} warning(s).",
                    err_count, warn_count
                )
            };

            let mut issue_list = column![].spacing(SPACE_2);
            if report.is_clean() {
                issue_list = issue_list.push(
                    text("No missing assets, broken links, or config issues detected.")
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                );
            } else {
                for issue in &report.issues {
                    let (badge, color) = match issue.severity {
                        blanc_core::validator::ValidationSeverity::Error => ("ERROR", STATUS_ERROR),
                        blanc_core::validator::ValidationSeverity::Warning => ("WARN", STATUS_WARNING),
                    };
                    issue_list = issue_list.push(
                        container(
                            column![
                                row![
                                    text(format!("[{}] [{}]", badge, issue.category.label()))
                                        .size(LABEL_SM)
                                        .color(iced::Color::from_rgb8(color.r, color.g, color.b)),
                                    text(issue.title.clone())
                                        .size(BODY_SM)
                                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                                ]
                                .spacing(SPACE_2),
                                text(issue.detail.clone())
                                    .size(LABEL_SM)
                                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                            ]
                            .spacing(2.0),
                        )
                        .padding(SPACE_2)
                        .width(Length::Fill)
                        .style(sidebar_container_style),
                    );
                }
            }

            let val_modal = container(
                container(
                    column![
                        text("Project Validation Report")
                            .size(HEADING_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        text(summary_text)
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(
                                if err_count > 0 {
                                    STATUS_ERROR.r
                                } else if warn_count > 0 {
                                    STATUS_WARNING.r
                                } else {
                                    STATUS_SUCCESS.r
                                },
                                if err_count > 0 {
                                    STATUS_ERROR.g
                                } else if warn_count > 0 {
                                    STATUS_WARNING.g
                                } else {
                                    STATUS_SUCCESS.g
                                },
                                if err_count > 0 {
                                    STATUS_ERROR.b
                                } else if warn_count > 0 {
                                    STATUS_WARNING.b
                                } else {
                                    STATUS_SUCCESS.b
                                },
                            )),
                        scrollable(issue_list).height(Length::Fixed(280.0)),
                        Space::with_height(SPACE_2),
                        row![
                            Space::with_width(Length::Fill),
                            button(text("Re-check").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_3])
                                .style(secondary_button_style)
                                .on_press(DashboardMessage::ValidateProject),
                            button(text("Close").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_4])
                                .style(primary_button_style)
                                .on_press(DashboardMessage::CloseValidationModal),
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(SPACE_2),
                )
                .padding(SPACE_5)
                .width(Length::Fixed(580.0))
                .style(card_container_style),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_backdrop_style);

            root_stack = root_stack.push(val_modal);
        }

        if self.show_format_guide_modal {
            let guide_section = |title: &'static str, syntax: &'static str, desc: &'static str| -> Element<'a, DashboardMessage> {
                container(
                    column![
                        text(title)
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                        text(syntax)
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        text(desc)
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                    ]
                    .spacing(3.0),
                )
                .padding(SPACE_3)
                .width(Length::Fill)
                .style(sidebar_container_style)
                .into()
            };

            let guide_modal = container(
                container(
                    column![
                        text("Story Script Format Guide")
                            .size(HEADING_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        text("Supported syntax when converting .txt / .md / .fountain scripts in Story Setup:")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        scrollable(
                            column![
                                guide_section("1. Scene Separators", "[Scene: Scene_Name]   |   # Scene_Name   |   ## Scene_Name   |   INT. / EXT.", "Splits the script into separate .bscene files automatically."),
                                guide_section("2. Dialogue & Emotion", "Rowan (Happy): \"Let's go!\"   or   Mina: Wait up!", "Quotes and (Emotion) tag are optional."),
                                guide_section("3. Narration", "The afternoon bell rang across the empty hallway.", "Any normal line without a colon (:) becomes a Narration block."),
                                guide_section("4. Audio Directives", "[BGM: School_Afternoon]   |   [SFX: Door_Slide]", "Plays background music or sound effects."),
                                guide_section("5. Interactive Choices", "[Choice: Where to go?]\n- \"Rooftop\" -> Scene_2A\n- \"Cafe\" -> Scene_2B\n]", "Creates branching choice options jumping to target scenes."),
                                guide_section("6. Comments", "// Writer note here", "Lines starting with // are ignored by the parser."),
                            ]
                            .spacing(SPACE_2),
                        )
                        .height(Length::Fixed(320.0)),
                        Space::with_height(SPACE_1),
                        row![
                            Space::with_width(Length::Fill),
                            button(text("Close").size(LABEL_SM))
                                .padding([SPACE_2, SPACE_4])
                                .style(primary_button_style)
                                .on_press(DashboardMessage::CloseFormatGuide),
                        ],
                    ]
                    .spacing(SPACE_2),
                )
                .padding(SPACE_5)
                .width(Length::Fixed(600.0))
                .style(card_container_style),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_backdrop_style);

            root_stack = root_stack.push(guide_modal);
        }
        root_stack.into()
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

        let saved_badge: Element<'a, DashboardMessage> = if self.has_unsaved_changes {
            button(
                text("● Unsaved (Ctrl+S)")
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(STATUS_WARNING.r, STATUS_WARNING.g, STATUS_WARNING.b)),
            )
            .padding([2.0, 6.0])
            .style(subtle_button_style)
            .on_press(DashboardMessage::SaveAllProjectChanges)
            .into()
        } else {
            text("● Saved")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b))
                .into()
        };

        let info_col = column![
            project_name,
            row![manifest_label, Space::with_width(SPACE_3), saved_badge].align_y(Alignment::Center),
        ]
        .spacing(SPACE_1);

        let btn_undo = {
            let b = button(text("Undo").size(LABEL_SM))
                .padding([SPACE_2, SPACE_3])
                .style(if self.undo_stack.is_empty() { subtle_button_style } else { secondary_button_style });
            if self.undo_stack.is_empty() {
                b
            } else {
                b.on_press(DashboardMessage::Undo)
            }
        };

        let btn_redo = {
            let b = button(text("Redo").size(LABEL_SM))
                .padding([SPACE_2, SPACE_3])
                .style(if self.redo_stack.is_empty() { subtle_button_style } else { secondary_button_style });
            if self.redo_stack.is_empty() {
                b
            } else {
                b.on_press(DashboardMessage::Redo)
            }
        };

        let btn_folder = button(
            row![icon_folder(), text("Open Folder").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenProjectFolder);

        let btn_validate = button(text("Validate").size(LABEL_SM))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::ValidateProject);

        let btn_play = if self.is_preview_running {
            button(
                row![icon_close(), text("Close Preview").size(BODY_SM)]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_4])
            .style(danger_icon_button_style)
            .on_press(DashboardMessage::ClosePreview)
        } else {
            button(
                row![icon_play(), text("Play Preview").size(BODY_SM)]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_4])
            .style(primary_button_style)
            .on_press(DashboardMessage::PlayPreview)
        };

        let btn_close = button(text("Close Project").size(BODY_SM))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::CloseProject);

        let brand_logo: Element<'a, DashboardMessage> = if project.use_custom_logo {
            if let Some(logo_rel_path) = &project.custom_logo_path {
                let abs_path = if std::path::Path::new(logo_rel_path).is_absolute() {
                    std::path::PathBuf::from(logo_rel_path)
                } else {
                    _project_dir.join(logo_rel_path)
                };
                if abs_path.exists() {
                    iced::widget::image(iced::widget::image::Handle::from_path(abs_path))
                        .width(28.0)
                        .height(28.0)
                        .into()
                } else {
                    engine_logo(28.0).into()
                }
            } else {
                engine_logo(28.0).into()
            }
        } else {
            engine_logo(28.0).into()
        };

        let brand_and_info = row![
            brand_logo,
            info_col,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let actions = row![btn_undo, btn_redo, btn_validate, btn_folder, btn_play, btn_close]
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
            WorkflowTab::UiDesigner,
            WorkflowTab::Localization,
            WorkflowTab::Settings,
            WorkflowTab::Export,
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
    fn view_overview_content<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let overview_title = text("Project Overview")
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        // Merge active unsaved scene edits into a temporary slice for live validation
        let mut live_scenes = self.all_scenes.clone();
        if let Some(cur) = &self.current_scene {
            if let Some(existing) = live_scenes.iter_mut().find(|s| s.name == cur.name) {
                *existing = cur.clone();
            } else {
                live_scenes.push(cur.clone());
            }
        }
        let report = blanc_core::validator::validate_project(
            project_dir,
            &project.assets,
            &self.flow_graph,
            &live_scenes,
            &self.system_screens_config,
        );

        let total_assets = project.assets.assets.len();
        let missing_assets = project
            .assets
            .assets
            .values()
            .filter(|a| !project_dir.join(&a.relative_path).exists())
            .count();

        let scene_count = self.scenes_list.len();
        let total_blocks: usize = live_scenes.iter().map(|s| s.blocks.len()).sum();
        let empty_scenes = live_scenes.iter().filter(|s| s.blocks.is_empty()).count();
        let flow_nodes = self.flow_graph.nodes.len();
        let flow_wires = self.flow_graph.connections.len();

        let story_lang_code = self.locale_manifest.effective_story_language().code.clone();
        let all_langs = self.locale_manifest.all_languages();
        let lang_count = all_langs.len();

        // Check translation completion across languages (ignoring native strings)
        let mut trans_needed_total = 0usize;
        let mut trans_done_total = 0usize;
        for lang in &all_langs {
            let tbl = self.locale_tables.get(&lang.code.to_ascii_lowercase());
            for item in &self.locale_items {
                let needed = match &item.category {
                    LocaleCategory::Ui => !lang.code.eq_ignore_ascii_case("en"),
                    LocaleCategory::Speaker | LocaleCategory::Scene(_) => {
                        !lang.code.eq_ignore_ascii_case(&story_lang_code)
                    }
                };
                if needed {
                    trans_needed_total += 1;
                    if tbl
                        .and_then(|t| t.entries.get(&item.key))
                        .map(|v| !v.trim().is_empty())
                        .unwrap_or(false)
                    {
                        trans_done_total += 1;
                    }
                }
            }
        }

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

        let assets_sub = if total_assets == 0 {
            ("No assets yet".to_string(), TEXT_MUTED)
        } else if missing_assets > 0 {
            (format!("{} missing on disk", missing_assets), STATUS_ERROR)
        } else {
            ("All files verified".to_string(), STATUS_SUCCESS)
        };
        let card_assets = column![
            text("Registered Assets").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} files", total_assets))
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text(assets_sub.0)
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(assets_sub.1.r, assets_sub.1.g, assets_sub.1.b)),
        ]
        .spacing(SPACE_1);

        let card_scenes = column![
            text("Scenes & Story Flow").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} scenes", scene_count))
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text(format!("{} blocks · {} nodes · {} wires", total_blocks, flow_nodes, flow_wires))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let card_locales = column![
            text("Localization").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} language(s)", lang_count))
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text(format!("Story script: {}", story_lang_code.to_ascii_uppercase()))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let summary_cards = row![
            container(card_author).padding(SPACE_4).width(Length::FillPortion(1)).style(card_container_style),
            container(card_version).padding(SPACE_4).width(Length::FillPortion(1)).style(card_container_style),
            container(card_assets).padding(SPACE_4).width(Length::FillPortion(1)).style(card_container_style),
            container(card_scenes).padding(SPACE_4).width(Length::FillPortion(1)).style(card_container_style),
            container(card_locales).padding(SPACE_4).width(Length::FillPortion(1)).style(card_container_style),
        ]
        .spacing(SPACE_3)
        .width(Length::Fill);

        let checklist_row = |title: &'static str,
                             status_label: String,
                             status_color: iced::Color,
                             detail: String,
                             btn_label: &'static str,
                             msg: DashboardMessage|
         -> Element<'a, DashboardMessage> {
            container(
                row![
                    text("●")
                        .size(12)
                        .color(status_color),
                    column![
                        row![
                            text(title)
                                .size(BODY_SM)
                                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                            text(format!("[{}]", status_label))
                                .size(LABEL_SM)
                                .color(status_color),
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                        text(detail)
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                    ]
                    .spacing(2.0)
                    .width(Length::Fill),
                    button(text(btn_label).size(LABEL_SM))
                        .padding([SPACE_1, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(msg),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
            )
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_container_style)
            .into()
        };

        let asset_status = if total_assets == 0 {
            ("Not Started".to_string(), TEXT_MUTED, "Import backgrounds, character sprites, and audio.".to_string())
        } else if missing_assets > 0 {
            ("Missing Files".to_string(), STATUS_ERROR, format!("{} asset(s) missing on disk. Relink them in Asset Manager.", missing_assets))
        } else {
            ("Ready".to_string(), STATUS_SUCCESS, format!("{} asset(s) registered and verified on disk.", total_assets))
        };

        let scene_status = if scene_count == 0 {
            ("Not Started".to_string(), TEXT_MUTED, "Create scenes in Story Setup or Scene Editor.".to_string())
        } else if empty_scenes > 0 {
            ("Draft".to_string(), STATUS_WARNING, format!("{} scene(s) configured ({} empty scene(s)).", scene_count, empty_scenes))
        } else {
            ("Ready".to_string(), STATUS_SUCCESS, format!("{} scene(s) with {} total action blocks.", scene_count, total_blocks))
        };

        let flow_errors = report
            .issues
            .iter()
            .filter(|i| {
                i.severity == blanc_core::validator::ValidationSeverity::Error
                    && i.category == blanc_core::validator::ValidationCategory::StoryFlow
            })
            .count();
        let flow_warns = report
            .issues
            .iter()
            .filter(|i| {
                i.severity == blanc_core::validator::ValidationSeverity::Warning
                    && i.category == blanc_core::validator::ValidationCategory::StoryFlow
            })
            .count();
        let flow_status = if flow_errors > 0 {
            ("Broken Links".to_string(), STATUS_ERROR, format!("{} critical Story Flow error(s) found.", flow_errors))
        } else if flow_wires == 0 || flow_warns > 0 {
            ("Needs Attention".to_string(), STATUS_WARNING, format!("{} nodes, {} wires ({} unreachable warning(s)).", flow_nodes, flow_wires, flow_warns))
        } else {
            ("Connected".to_string(), STATUS_SUCCESS, format!("All {} nodes connected cleanly with {} wires.", flow_nodes, flow_wires))
        };

        let locale_status = if trans_needed_total == 0 {
            ("Native".to_string(), STATUS_SUCCESS, format!("Single-language project ({}). Add target languages anytime.", story_lang_code.to_ascii_uppercase()))
        } else if trans_done_total >= trans_needed_total {
            ("Complete".to_string(), STATUS_SUCCESS, format!("All {}/{} target strings translated.", trans_done_total, trans_needed_total))
        } else {
            let pct = (trans_done_total * 100) / trans_needed_total.max(1);
            ("In Progress".to_string(), STATUS_WARNING, format!("{}/{} strings translated ({}%) across {} languages.", trans_done_total, trans_needed_total, pct, lang_count))
        };

        let export_status = if report.has_errors() {
            ("Blocked".to_string(), STATUS_ERROR, format!("{} critical error(s) block export. Click Validate to inspect.", report.error_count()))
        } else if report.warning_count() > 0 {
            ("Ready (Warnings)".to_string(), STATUS_WARNING, format!("0 errors, {} warning(s). Ready to Play Preview & Export.", report.warning_count()))
        } else {
            ("Ready to Export".to_string(), STATUS_SUCCESS, "All project checks passed with 0 errors and 0 warnings.".to_string())
        };

        let checklist_card = container(
            column![
                text("Project Readiness Checklist")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                checklist_row("1. Assets", asset_status.0, iced::Color::from_rgb8(asset_status.1.r, asset_status.1.g, asset_status.1.b), asset_status.2, "Open Assets", DashboardMessage::SelectWorkflowTab(WorkflowTab::Assets)),
                checklist_row("2. Scenes & Story", scene_status.0, iced::Color::from_rgb8(scene_status.1.r, scene_status.1.g, scene_status.1.b), scene_status.2, "Open Scene Editor", DashboardMessage::SelectWorkflowTab(WorkflowTab::SceneEditor)),
                checklist_row("3. Story Flow", flow_status.0, iced::Color::from_rgb8(flow_status.1.r, flow_status.1.g, flow_status.1.b), flow_status.2, "Open Story Flow", DashboardMessage::SelectWorkflowTab(WorkflowTab::StoryFlow)),
                checklist_row("4. Localization", locale_status.0, iced::Color::from_rgb8(locale_status.1.r, locale_status.1.g, locale_status.1.b), locale_status.2, "Open Localization", DashboardMessage::SelectWorkflowTab(WorkflowTab::Localization)),
                checklist_row("5. Validation & Export", export_status.0, iced::Color::from_rgb8(export_status.1.r, export_status.1.g, export_status.1.b), export_status.2, "Validate Now", DashboardMessage::ValidateProject),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let (next_title, next_desc, next_actions): (&str, String, Element<'a, DashboardMessage>) = if missing_assets > 0 {
            (
                "Recommended: Relink Missing Assets",
                format!("{} registered asset(s) could not be found on disk. Relink them so scenes and exports stay intact.", missing_assets),
                button(text("Open Asset Manager").size(BODY_SM))
                    .padding([SPACE_2, SPACE_4])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::Assets))
                    .into(),
            )
        } else if total_assets == 0 {
            (
                "Recommended: Import Your Game Assets",
                "Start by importing backgrounds, character sprites, and BGM/SFX in Asset Manager.".to_string(),
                button(text("Open Asset Manager").size(BODY_SM))
                    .padding([SPACE_2, SPACE_4])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::Assets))
                    .into(),
            )
        } else if scene_count == 0 {
            (
                "Recommended: Create Your First Scene",
                "Draft your screenplay in Story Setup or build scenes directly in Scene Editor.".to_string(),
                button(text("Open Story Setup").size(BODY_SM))
                    .padding([SPACE_2, SPACE_4])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::Story))
                    .into(),
            )
        } else if report.has_errors() {
            (
                "Recommended: Fix Critical Validation Errors",
                format!("Found {} critical error(s) that block export. Review the Validation Report to resolve them.", report.error_count()),
                button(text("Open Validation Report").size(BODY_SM))
                    .padding([SPACE_2, SPACE_4])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::ValidateProject)
                    .into(),
            )
        } else {
            (
                "Project Ready to Play & Export!",
                "All core checks passed. Test your visual novel with Play Preview or package it in Export.".to_string(),
                row![
                    if self.is_preview_running {
                        button(
                            row![icon_close(), text("Close Preview").size(BODY_SM)]
                                .spacing(SPACE_2)
                                .align_y(Alignment::Center),
                        )
                        .padding([SPACE_2, SPACE_4])
                        .style(danger_icon_button_style)
                        .on_press(DashboardMessage::ClosePreview)
                    } else {
                        button(
                            row![icon_play(), text("Play Preview").size(BODY_SM)]
                                .spacing(SPACE_2)
                                .align_y(Alignment::Center),
                        )
                        .padding([SPACE_2, SPACE_4])
                        .style(primary_button_style)
                        .on_press(DashboardMessage::PlayPreview)
                    },
                    button(text("Go to Export").size(BODY_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::Export)),
                ]
                .spacing(SPACE_3)
                .into(),
            )
        };

        let next_step_box = container(
            column![
                text(next_title)
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(
                        if report.has_errors() || missing_assets > 0 { STATUS_WARNING.r } else { ACCENT_PRIMARY.r },
                        if report.has_errors() || missing_assets > 0 { STATUS_WARNING.g } else { ACCENT_PRIMARY.g },
                        if report.has_errors() || missing_assets > 0 { STATUS_WARNING.b } else { ACCENT_PRIMARY.b },
                    )),
                text(next_desc)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_1),
                next_actions,
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let content = column![overview_title, summary_cards, checklist_card, next_step_box]
            .spacing(SPACE_4)
            .padding(SPACE_6)
            .width(Length::Fill);

        scrollable(content).width(Length::Fill).height(Length::Fill).into()
    }

    // ==========================================
    // TAB 2: ASSET MANAGER
    // ==========================================
    fn view_assets_content<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let sidebar = self.view_asset_categories_sidebar(project);
        let main_pane = self.view_asset_list_pane(project, project_dir);
        let preview_pane = self.view_asset_preview_pane(project, project_dir);

        let split = row![sidebar, main_pane, preview_pane]
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
                AssetCategory::Sfx | AssetCategory::Voice => icon_volume(),
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

    fn view_asset_list_pane<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let (category_title, category_subtitle, current_cat) = match self.selected_category {
            Some(cat) => (
                cat.display_name(),
                match cat {
                    AssetCategory::Background => "Background illustrations (1920×1080 recommended, PNG/WebP/JPG)",
                    AssetCategory::Character => "Full character sprites (Transparent PNG/WebP)",
                    AssetCategory::CharacterExpression => "Facial expression variants for character sprites",
                    AssetCategory::Bgm => "Background music tracks (Seamless looping OGG/WAV/MP3)",
                    AssetCategory::Sfx => "Short sound effects (WAV/OGG/MP3)",
                    AssetCategory::Voice => "Character voice-over lines (OGG/WAV/MP3)",
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
            .width(Length::Fixed(190.0))
            .style(input_field_style);

        let btn_mode_list = button(text("List").size(LABEL_SM).wrapping(text::Wrapping::None))
            .padding([SPACE_2, SPACE_3])
            .style(if self.asset_view_mode == AssetViewMode::List { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::SetAssetViewMode(AssetViewMode::List));

        let btn_mode_grid = button(text("Grid").size(LABEL_SM).wrapping(text::Wrapping::None))
            .padding([SPACE_2, SPACE_3])
            .style(if self.asset_view_mode == AssetViewMode::Grid { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::SetAssetViewMode(AssetViewMode::Grid));

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
            btn_mode_list,
            btn_mode_grid,
            btn_open_folder,
            btn_import,
        ]
        .spacing(SPACE_2)
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
        } else if self.asset_view_mode == AssetViewMode::List {
            let mut asset_list = column![].spacing(SPACE_2).width(Length::Fill);

            for asset in filtered_assets {
                let card = self.view_asset_card(asset, project_dir);
                asset_list = asset_list.push(card);
            }

            container(scrollable(asset_list).width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .clip(true)
                .into()
        } else {
            let mut grid_col = column![].spacing(SPACE_3).width(Length::Fill);
            let mut current_row = row![].spacing(SPACE_3).width(Length::Fill);
            let mut in_row = 0;

            for asset in filtered_assets {
                current_row = current_row.push(self.view_asset_grid_card(asset, project_dir));
                in_row += 1;
                if in_row == 3 {
                    grid_col = grid_col.push(current_row);
                    current_row = row![].spacing(SPACE_3).width(Length::Fill);
                    in_row = 0;
                }
            }
            if in_row > 0 {
                while in_row < 3 {
                    current_row = current_row.push(Space::with_width(Length::FillPortion(1)));
                    in_row += 1;
                }
                grid_col = grid_col.push(current_row);
            }

            container(scrollable(grid_col).width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .clip(true)
                .into()
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

    fn asset_is_image(cat: AssetCategory) -> bool {
        matches!(
            cat,
            AssetCategory::Background
                | AssetCategory::Character
                | AssetCategory::CharacterExpression
                | AssetCategory::Ui
        )
    }

    fn asset_is_audio(cat: AssetCategory) -> bool {
        matches!(cat, AssetCategory::Bgm | AssetCategory::Sfx | AssetCategory::Voice)
    }

    fn view_asset_card<'a>(&'a self, asset: &'a Asset, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let is_selected = self.selected_preview_asset_id.as_deref() == Some(&asset.id);
        let full_path = project_dir.join(&asset.relative_path);

        let thumb_box: Element<'a, DashboardMessage> = if Self::asset_is_image(asset.category) && full_path.exists() {
            let fit = if asset.category == AssetCategory::Background {
                iced::ContentFit::Cover
            } else {
                iced::ContentFit::Contain
            };
            container(
                image(image::Handle::from_path(&full_path))
                    .width(Length::Fixed(52.0))
                    .height(Length::Fixed(52.0))
                    .content_fit(fit),
            )
            .width(Length::Fixed(52.0))
            .height(Length::Fixed(52.0))
            .clip(true)
            .style(code_container_style)
            .into()
        } else {
            let cat_icon = match asset.category {
                AssetCategory::Background => icon_image(),
                AssetCategory::Character | AssetCategory::CharacterExpression => icon_user(),
                AssetCategory::Bgm => icon_music(),
                AssetCategory::Sfx | AssetCategory::Voice => icon_volume(),
                AssetCategory::Font => icon_type(),
                AssetCategory::Ui | AssetCategory::Other => icon_blocks(),
            };
            container(cat_icon)
                .center_x(Length::Fixed(52.0))
                .center_y(Length::Fixed(52.0))
                .style(code_container_style)
                .into()
        };

        let size_str = format_bytes(asset.file_size);
        let name_display = if asset.name.chars().count() > 30 {
            let t: String = asset.name.chars().take(27).collect();
            format!("{}...", t)
        } else {
            asset.name.clone()
        };

        let is_missing = !full_path.exists();
        let mut title_badges = row![
            text(name_display)
                .size(HEADING_SM)
                .wrapping(text::Wrapping::None)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            container(
                text(asset.category.display_name())
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
            )
            .padding([2.0, 6.0])
            .style(badge_container_style),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        if is_missing {
            title_badges = title_badges.push(
                container(
                    text("Missing")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)),
                )
                .padding([2.0, 6.0])
                .style(badge_container_style),
            );
        }

        let details = column![
            title_badges,
            text(format!("{} • {}", asset.relative_path, size_str))
                .size(LABEL_SM)
                .wrapping(text::Wrapping::None)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        ]
        .spacing(SPACE_1);

        let mut actions = row![].spacing(SPACE_2).align_y(Alignment::Center);
        if is_missing {
            actions = actions.push(
                button(text("Relink...").size(LABEL_SM))
                    .padding([SPACE_1, SPACE_3])
                    .style(secondary_button_style)
                    .on_press(DashboardMessage::RelinkAssetClicked(asset.id.clone())),
            );
        } else if Self::asset_is_audio(asset.category) {
            let is_playing = self.playing_audio_asset_id.as_deref() == Some(&asset.id);
            actions = actions.push(
                button(text(if is_playing { "Stop" } else { "Play" }).size(LABEL_SM))
                    .padding([SPACE_1, SPACE_3])
                    .style(if is_playing { primary_button_style } else { secondary_button_style })
                    .on_press(if is_playing {
                        DashboardMessage::StopAssetAudioPreview
                    } else {
                        DashboardMessage::PlayAssetAudioPreview(asset.id.clone())
                    }),
            );
        }
        actions = actions.push(
            button(icon_trash())
                .padding([SPACE_2, SPACE_3])
                .style(subtle_button_style)
                .on_press(DashboardMessage::DeleteAssetClicked(asset.id.clone())),
        );

        let card_content = row![
            thumb_box,
            Space::with_width(SPACE_3),
            details,
            Space::with_width(Length::Fill),
            actions,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        button(card_content)
            .padding(SPACE_3)
            .width(Length::Fill)
            .style(asset_card_button_style(is_selected))
            .on_press(DashboardMessage::SelectAssetForPreview(asset.id.clone()))
            .into()
    }

    fn view_asset_grid_card<'a>(&'a self, asset: &'a Asset, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let is_selected = self.selected_preview_asset_id.as_deref() == Some(&asset.id);
        let full_path = project_dir.join(&asset.relative_path);

        let thumb: Element<'a, DashboardMessage> = if Self::asset_is_image(asset.category) && full_path.exists() {
            container(
                image(image::Handle::from_path(&full_path))
                    .width(Length::Fill)
                    .height(Length::Fixed(108.0))
                    .content_fit(iced::ContentFit::Contain),
            )
            .width(Length::Fill)
            .height(Length::Fixed(108.0))
            .clip(true)
            .style(code_container_style)
            .into()
        } else if Self::asset_is_audio(asset.category) {
            let is_playing = self.playing_audio_asset_id.as_deref() == Some(&asset.id);
            container(
                column![
                    if asset.category == AssetCategory::Bgm { icon_music() } else { icon_volume() },
                    text(if is_playing { "Playing..." } else { "Audio Track" })
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(
                            if is_playing { ACCENT_PRIMARY.r } else { TEXT_MUTED.r },
                            if is_playing { ACCENT_PRIMARY.g } else { TEXT_MUTED.g },
                            if is_playing { ACCENT_PRIMARY.b } else { TEXT_MUTED.b },
                        )),
                ]
                .spacing(SPACE_1)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fixed(108.0))
            .style(code_container_style)
            .into()
        } else {
            container(icon_blocks())
                .width(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fixed(108.0))
                .style(code_container_style)
                .into()
        };

        let name_display = if asset.name.chars().count() > 21 {
            let t: String = asset.name.chars().take(18).collect();
            format!("{}...", t)
        } else {
            asset.name.clone()
        };

        let body = column![
            thumb,
            text(name_display)
                .size(LABEL_MD)
                .wrapping(text::Wrapping::None)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            row![
                text(asset.category.display_name())
                    .size(LABEL_SM)
                    .wrapping(text::Wrapping::None)
                    .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                Space::with_width(Length::Fill),
                text(format_bytes(asset.file_size))
                    .size(LABEL_SM)
                    .wrapping(text::Wrapping::None)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(SPACE_2);

        button(body)
            .padding(SPACE_3)
            .width(Length::FillPortion(1))
            .style(asset_card_button_style(is_selected))
            .on_press(DashboardMessage::SelectAssetForPreview(asset.id.clone()))
            .into()
    }

    fn view_asset_preview_pane<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let header = row![
            text("Asset Preview")
                .size(HEADING_SM)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ];

        let content: Element<'a, DashboardMessage> = if let Some(sel_id) = &self.selected_preview_asset_id {
            if let Some(asset) = project.assets.assets.get(sel_id) {
                let full_path = project_dir.join(&asset.relative_path);

                let preview_box: Element<'a, DashboardMessage> = if Self::asset_is_image(asset.category) && full_path.exists() {
                    let light_bg = self.asset_preview_light_bg;
                    let dims = ::image::image_dimensions(&full_path)
                        .map(|(w, h)| format!("{} × {} px", w, h))
                        .unwrap_or_else(|_| "Unknown dimensions".to_string());

                    let img_container = container(
                        image(image::Handle::from_path(&full_path))
                            .width(Length::Fixed(282.0))
                            .height(Length::Fixed(210.0))
                            .content_fit(iced::ContentFit::Contain),
                    )
                    .width(Length::Fixed(282.0))
                    .height(Length::Fixed(210.0))
                    .clip(true)
                    .style(move |_| container::Style {
                        background: Some(iced::Background::Color(if light_bg {
                            iced::Color::from_rgb8(230, 232, 238)
                        } else {
                            iced::Color::from_rgb8(18, 20, 28)
                        })),
                        border: iced::Border {
                            radius: 8.0.into(),
                            width: 1.0,
                            color: iced::Color::from_rgb8(BORDER_DEFAULT.r, BORDER_DEFAULT.g, BORDER_DEFAULT.b),
                        },
                        ..Default::default()
                    });

                    let bg_toggle = row![
                        text("Backdrop:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        button(text("Dark").size(LABEL_SM))
                            .padding([2.0, 8.0])
                            .style(if !light_bg { primary_button_style } else { secondary_button_style })
                            .on_press(DashboardMessage::ToggleAssetPreviewBg),
                        button(text("Light").size(LABEL_SM))
                            .padding([2.0, 8.0])
                            .style(if light_bg { primary_button_style } else { secondary_button_style })
                            .on_press(DashboardMessage::ToggleAssetPreviewBg),
                        Space::with_width(Length::Fill),
                        text(dims).size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center);

                    column![img_container, bg_toggle].spacing(SPACE_2).into()
                } else if Self::asset_is_audio(asset.category) {
                    let is_playing = self.playing_audio_asset_id.as_deref() == Some(&asset.id);
                    container(
                        column![
                            if asset.category == AssetCategory::Bgm { icon_music() } else { icon_volume() },
                            text(if is_playing { "Playing Audio Preview..." } else { "Audio Ready" })
                                .size(BODY_SM)
                                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                            row![
                                button(text("Play").size(LABEL_SM))
                                    .padding([SPACE_1, SPACE_4])
                                    .style(if is_playing { primary_button_style } else { secondary_button_style })
                                    .on_press(DashboardMessage::PlayAssetAudioPreview(asset.id.clone())),
                                button(text("Stop").size(LABEL_SM))
                                    .padding([SPACE_1, SPACE_4])
                                    .style(secondary_button_style)
                                    .on_press(DashboardMessage::StopAssetAudioPreview),
                            ]
                            .spacing(SPACE_2),
                        ]
                        .spacing(SPACE_3)
                        .align_x(Alignment::Center),
                    )
                    .center_x(Length::Fixed(282.0))
                    .center_y(Length::Fixed(160.0))
                    .style(code_container_style)
                    .into()
                } else {
                    container(icon_blocks())
                        .center_x(Length::Fixed(282.0))
                        .center_y(Length::Fixed(140.0))
                        .style(code_container_style)
                        .into()
                };

                let is_missing = !full_path.exists();
                let mut meta = column![
                    text(&asset.name)
                        .size(HEADING_SM)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text(format!("Category: {}", asset.category.display_name()))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    text(format!("Size: {}", format_bytes(asset.file_size)))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!("Path: {}", asset.relative_path))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_1);

                if is_missing {
                    meta = meta.push(
                        text("File missing on disk. Click Relink to restore.")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)),
                    );
                }

                let mut actions = row![].spacing(SPACE_2);
                if is_missing {
                    actions = actions.push(
                        button(text("Relink...").size(LABEL_SM))
                            .padding([SPACE_2, SPACE_3])
                            .style(primary_button_style)
                            .on_press(DashboardMessage::RelinkAssetClicked(asset.id.clone())),
                    );
                } else {
                    actions = actions.push(
                        button(
                            row![icon_external_link(), text("Explorer").size(LABEL_SM)]
                                .spacing(SPACE_1)
                                .align_y(Alignment::Center),
                        )
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::RevealAssetInExplorer(asset.relative_path.clone())),
                    );
                }
                actions = actions.push(
                    button(
                        row![icon_trash(), text("Delete").size(LABEL_SM)]
                            .spacing(SPACE_1)
                            .align_y(Alignment::Center),
                    )
                    .padding([SPACE_2, SPACE_3])
                    .style(danger_icon_button_style)
                    .on_press(DashboardMessage::DeleteAssetClicked(asset.id.clone())),
                );

                column![preview_box, meta, actions].spacing(SPACE_4).into()
            } else {
                container(text("Asset not found.").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into()
            }
        } else {
            container(
                column![
                    icon_image(),
                    text("Select an asset to preview")
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(SPACE_2)
                .align_x(Alignment::Center),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        };

        container(
            column![header, content]
                .spacing(SPACE_4)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .padding(SPACE_4)
        .width(Length::Fixed(320.0))
        .height(Length::Fill)
        .style(sidebar_container_style)
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

        let btn_guide = button(text("? Format Guide").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .style(secondary_button_style)
            .on_press(DashboardMessage::OpenFormatGuide);

        let action_bar = row![
            title_col,
            Space::with_width(Length::Fill),
            btn_guide,
            btn_template,
            btn_import,
            btn_save,
            btn_convert,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        let story_lang = self.locale_manifest.effective_story_language();
        let story_lang_code = story_lang.code.clone();
        let story_lang_presets = [
            ("en", "English"),
            ("id", "Indonesia"),
            ("ja", "Japanese"),
            ("ko", "Korean"),
            ("zh", "Chinese"),
            ("es", "Spanish"),
        ];
        let mut story_lang_row = row![
            text("Story Script Language:")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        for (code, name) in story_lang_presets {
            let active = story_lang_code.eq_ignore_ascii_case(code);
            story_lang_row = story_lang_row.push(
                button(text(format!("{} ({})", name, code)).size(LABEL_SM))
                    .padding([3.0, 9.0])
                    .style(if active { primary_button_style } else { secondary_button_style })
                    .on_press(DashboardMessage::SetStoryScriptLanguage(
                        code.to_string(),
                        name.to_string(),
                    )),
            );
        }

        let custom_code_trimmed = self.story_lang_custom_input.trim().to_ascii_lowercase();
        let custom_apply_btn = {
            let b = button(text("Set").size(LABEL_SM))
                .padding([3.0, 9.0])
                .style(if custom_code_trimmed.is_empty() {
                    subtle_button_style
                } else {
                    secondary_button_style
                });
            if custom_code_trimmed.is_empty() {
                b
            } else {
                let code_upper = custom_code_trimmed.to_ascii_uppercase();
                b.on_press(DashboardMessage::SetStoryScriptLanguage(
                    custom_code_trimmed.clone(),
                    code_upper,
                ))
            }
        };

        let story_lang_bar = container(
            row![
                story_lang_row,
                Space::with_width(Length::Fill),
                text(format!("Active: {} ({})", story_lang.display_name, story_lang.code))
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                text_input("Other code (e.g. fr, de)", &self.story_lang_custom_input)
                    .on_input(DashboardMessage::StoryLangCustomCodeChanged)
                    .size(LABEL_SM)
                    .padding([3.0, 8.0])
                    .width(Length::Fixed(150.0))
                    .style(input_field_style),
                custom_apply_btn,
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_container_style);

        let left_editor_pane = self.view_script_editor_pane();
        let right_preview_pane = self.view_script_parsed_preview_pane();

        let split_view = row![
            left_editor_pane,
            Space::with_width(SPACE_4),
            right_preview_pane,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let mut pane = column![action_bar, story_lang_bar, Space::with_height(SPACE_2), split_view]
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
                        text(format!("-> {}", opt.target_scene))
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
        let main_canvas = self.view_scene_canvas_pane(project, project_dir);

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

        let mut scenes_col = column![].spacing(SPACE_1).width(Length::Fill);

        for (idx, scene_name) in self.scenes_list.iter().enumerate() {
            let is_selected = self.selected_scene_index == idx;
            let block_count = if is_selected {
                self.current_scene.as_ref().map(|s| s.blocks.len()).unwrap_or(0)
            } else {
                0
            };

            let display_name = if scene_name.chars().count() > 23 {
                let truncated: String = scene_name.chars().take(20).collect();
                format!("{}...", truncated)
            } else {
                scene_name.clone()
            };

            let title_line = row![
                icon_film(),
                text(display_name)
                    .size(LABEL_MD)
                    .wrapping(text::Wrapping::None),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center)
            .width(Length::Fill);

            let btn_content: Element<'a, DashboardMessage> = if is_selected {
                column![
                    title_line,
                    row![
                        Space::with_width(Length::Fixed(24.0)),
                        text(format!("{} blocks", block_count))
                            .size(LABEL_SM)
                            .wrapping(text::Wrapping::None)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    ]
                    .align_y(Alignment::Center),
                ]
                .spacing(SPACE_1)
                .width(Length::Fill)
                .into()
            } else {
                title_line.into()
            };

            let btn_scene = button(btn_content)
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

        let scenes_scroll = scrollable(scenes_col)
            .width(Length::Fill)
            .height(Length::Fill);

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
            title_row,
            scenes_scroll,
            btn_new_scene,
            btn_open_folder,
        ]
        .spacing(SPACE_3)
        .padding(SPACE_4)
        .width(Length::Fixed(280.0))
        .height(Length::Fill);

        container(sidebar_layout)
            .style(sidebar_container_style)
            .height(Length::Fill)
            .into()
    }

    fn view_scene_canvas_pane<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
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

        let btn_play_here = button(
            row![icon_play(), text("Play From Here").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style)
        .on_press(DashboardMessage::PlayFromCurrentScene);

        let top_action_bar = row![
            scene_header,
            Space::with_width(Length::Fill),
            btn_save,
            btn_play_here,
        ]
        .spacing(SPACE_2)
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
            nameplate_position: None,
        }));

        let btn_add_char = button(
            row![icon_user(), text("Character").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::Character {
            character_id: "Alice".to_string(),
            sprite_asset_id: None,
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

        let btn_add_effect = button(
            row![icon_film(), text("Effect").size(LABEL_SM)].spacing(SPACE_1).align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::AddBlockToScene(ActionBlock::VisualEffect {
            effect: blanc_core::scene::ScreenEffectType::FadeIn,
            duration: 0.6,
        }));

        let add_bar = container(
            row![
                bar_label,
                btn_add_dialogue,
                btn_add_char,
                btn_add_bg,
                btn_add_choice,
                btn_add_bgm,
                btn_add_effect,
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
            let card = self.view_action_block_card(idx, block, scene.blocks.len(), project, project_dir);
            blocks_col = blocks_col.push(card);
        }

        let scroll_pane = scrollable(blocks_col)
            .id(scrollable::Id::new("scene_editor_blocks_scroll"))
            .on_scroll(|vp| DashboardMessage::SceneEditorBlocksScrolled(vp.absolute_offset()))
            .width(Length::Fill)
            .height(Length::Fill);

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

    fn view_action_block_card<'a>(
        &'a self,
        idx: usize,
        block: &'a ActionBlock,
        total: usize,
        project: &'a Project,
        project_dir: &'a PathBuf,
    ) -> Element<'a, DashboardMessage> {
        let resolve_thumb = |key: &str| -> Option<PathBuf> {
            let asset = project
                .assets
                .get(key)
                .or_else(|| project.assets.assets.values().find(|a| a.name.eq_ignore_ascii_case(key)))?;
            let path = project_dir.join(&asset.relative_path);
            if path.exists() { Some(path) } else { None }
        };

        let order_badge = container(
            text(format!("#{}", idx + 1))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        )
        .padding([2.0, 6.0])
        .style(code_container_style);

        // Block Type Header & Custom Inline Controls
        let (block_type_label, block_icon, content_element): (&'static str, iced::widget::svg::Svg<'a, iced::Theme>, Element<'a, DashboardMessage>) = match block {
            ActionBlock::Character { character_id, sprite_asset_id, expression, position, transition, .. } => {
                let pos_buttons = row![
                    text("Position:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
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
                    Space::with_width(SPACE_3),
                    text("Anim:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("Cut").size(LABEL_SM))
                        .padding([2.0, 7.0])
                        .style(if matches!(transition, None | Some(blanc_core::scene::TransitionType::Cut)) { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockTransition(idx, Some(blanc_core::scene::TransitionType::Cut))),
                    button(text("Slide L").size(LABEL_SM))
                        .padding([2.0, 7.0])
                        .style(if *transition == Some(blanc_core::scene::TransitionType::SlideLeft) { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockTransition(idx, Some(blanc_core::scene::TransitionType::SlideLeft))),
                    button(text("Slide R").size(LABEL_SM))
                        .padding([2.0, 7.0])
                        .style(if *transition == Some(blanc_core::scene::TransitionType::SlideRight) { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockTransition(idx, Some(blanc_core::scene::TransitionType::SlideRight))),
                ]
                .spacing(SPACE_1)
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

                let sprite_label = match sprite_asset_id {
                    Some(id) => format!("Sprite Asset: {}", id),
                    None => "Sprite Asset: Auto-match by character/expression (or click Select Sprite...)".to_string(),
                };
                let sprite_row = text(sprite_label)
                    .size(LABEL_SM)
                    .color(if sprite_asset_id.is_some() {
                        iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
                    } else {
                        iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)
                    });

                let thumb_path = sprite_asset_id
                    .as_deref()
                    .and_then(&resolve_thumb)
                    .or_else(|| resolve_thumb(character_id));

                let thumb_box: Element<'a, DashboardMessage> = if let Some(path) = thumb_path {
                    container(
                        image(image::Handle::from_path(path))
                            .width(Length::Fixed(52.0))
                            .height(Length::Fixed(52.0))
                            .content_fit(iced::ContentFit::Contain),
                    )
                    .width(Length::Fixed(52.0))
                    .height(Length::Fixed(52.0))
                    .clip(true)
                    .style(code_container_style)
                    .into()
                } else {
                    container(icon_user())
                        .center_x(Length::Fixed(52.0))
                        .center_y(Length::Fixed(52.0))
                        .style(code_container_style)
                        .into()
                };

                (
                    "Character Sprite",
                    icon_user(),
                    row![
                        thumb_box,
                        column![info_row, sprite_row, pos_buttons].spacing(SPACE_2).width(Length::Fill),
                    ]
                    .spacing(SPACE_3)
                    .align_y(Alignment::Center)
                    .into(),
                )
            }
            ActionBlock::Dialogue { speaker, text: spoken_text, nameplate_position, voice_asset_id } => {
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

                let np_pos_btn = |label: &'static str, pos: Option<CharacterPosition>| {
                    let is_active = *nameplate_position == pos;
                    button(text(label).size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if is_active { primary_button_style } else { secondary_button_style })
                        .on_press(DashboardMessage::UpdateDialogueNameplatePosition(idx, pos))
                };

                let nameplate_row = row![
                    text("Nameplate:")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    np_pos_btn("Default", None),
                    np_pos_btn("Left", Some(CharacterPosition::Left)),
                    np_pos_btn("Center", Some(CharacterPosition::Center)),
                    np_pos_btn("Right", Some(CharacterPosition::Right)),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                let vo_label = voice_asset_id
                    .as_deref()
                    .map(|id| format!("VO: {}", id))
                    .unwrap_or_else(|| "Select Voice (VO)...".to_string());

                let btn_pick_vo = button(
                    row![icon_volume(), text(vo_label).size(LABEL_SM)]
                        .spacing(SPACE_1)
                        .align_y(Alignment::Center),
                )
                .padding([2.0, 8.0])
                .style(if voice_asset_id.is_some() { primary_button_style } else { secondary_button_style })
                .on_press(DashboardMessage::OpenAssetPickerModal {
                    target: AssetPickerTarget::Block(idx),
                    category: AssetCategory::Voice,
                });

                let mut vo_row = row![
                    text("Voice:")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    btn_pick_vo,
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                if voice_asset_id.is_some() {
                    vo_row = vo_row.push(
                        button(icon_close())
                            .padding([2.0, 6.0])
                            .style(danger_icon_button_style)
                            .on_press(DashboardMessage::ClearBlockVoice(idx)),
                    );
                }

                (
                    "Dialogue",
                    icon_message(),
                    column![inputs_row, nameplate_row, vo_row].spacing(SPACE_2).into(),
                )
            }
            ActionBlock::Background { asset_id, transition } => {
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

                let thumb_box: Element<'a, DashboardMessage> = if let Some(path) = resolve_thumb(asset_id) {
                    container(
                        image(image::Handle::from_path(path))
                            .width(Length::Fixed(76.0))
                            .height(Length::Fixed(46.0))
                            .content_fit(iced::ContentFit::Cover),
                    )
                    .width(Length::Fixed(76.0))
                    .height(Length::Fixed(46.0))
                    .clip(true)
                    .style(code_container_style)
                    .into()
                } else {
                    container(icon_image())
                        .center_x(Length::Fixed(76.0))
                        .center_y(Length::Fixed(46.0))
                        .style(code_container_style)
                        .into()
                };

                let trans_row = row![
                    text("Transition:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("Cut").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if matches!(transition, None | Some(blanc_core::scene::TransitionType::Cut)) { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockTransition(idx, Some(blanc_core::scene::TransitionType::Cut))),
                    button(text("Fade").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if matches!(transition, Some(blanc_core::scene::TransitionType::Fade | blanc_core::scene::TransitionType::Dissolve)) { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateBlockTransition(idx, Some(blanc_core::scene::TransitionType::Fade))),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center);

                let content = row![
                    thumb_box,
                    column![
                        row![
                            text(format!("Asset: {}", display_id)).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                            Space::with_width(Length::Fill),
                            btn_pick_bg,
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                        trans_row,
                    ]
                    .spacing(SPACE_2)
                    .width(Length::Fill),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center);

                (
                    "Background",
                    icon_image(),
                    content.into(),
                )
            }
            ActionBlock::VisualEffect { effect, duration } => {
                use blanc_core::scene::ScreenEffectType;
                let fx_row = row![
                    text("Effect:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("Fade In").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *effect == ScreenEffectType::FadeIn { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, ScreenEffectType::FadeIn, *duration)),
                    button(text("Fade Out").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *effect == ScreenEffectType::FadeOut { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, ScreenEffectType::FadeOut, *duration)),
                    button(text("Flash").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *effect == ScreenEffectType::FlashWhite { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, ScreenEffectType::FlashWhite, *duration)),
                    button(text("Shake").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if *effect == ScreenEffectType::Shake { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, ScreenEffectType::Shake, *duration)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center);

                let dur_row = row![
                    text("Duration:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    button(text("0.3s").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if (*duration - 0.3).abs() < 0.1 { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, *effect, 0.3)),
                    button(text("0.6s").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if (*duration - 0.6).abs() < 0.1 { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, *effect, 0.6)),
                    button(text("1.0s").size(LABEL_SM))
                        .padding([2.0, 8.0])
                        .style(if (*duration - 1.0).abs() < 0.1 { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::UpdateVisualEffect(idx, *effect, 1.0)),
                ]
                .spacing(SPACE_1)
                .align_y(Alignment::Center);

                (
                    "Visual Effect",
                    icon_film(),
                    column![fx_row, dur_row].spacing(SPACE_2).into(),
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
                        text(format!("-> Jump to: {}", opt.target_scene_id))
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
            AssetCategory::Voice => ("Select Voice Over (VO)", "Voice"),
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
                    || modal.category == AssetCategory::Ui
                {
                    if full_path.exists() {
                        let fit = if modal.category == AssetCategory::Background {
                            iced::ContentFit::Cover
                        } else {
                            iced::ContentFit::Contain
                        };
                        container(
                            image(image::Handle::from_path(&full_path))
                                .content_fit(fit)
                                .width(Length::Fixed(190.0))
                                .height(Length::Fixed(110.0)),
                        )
                        .width(Length::Fixed(190.0))
                        .height(Length::Fixed(110.0))
                        .clip(true)
                        .style(code_container_style)
                        .into()
                    } else {
                        container(icon_image())
                            .center_x(Length::Fixed(190.0))
                            .center_y(Length::Fixed(110.0))
                            .style(code_container_style)
                            .into()
                    }
                } else {
                    container(icon_music())
                        .center_x(Length::Fixed(190.0))
                        .center_y(Length::Fixed(80.0))
                        .style(code_container_style)
                        .into()
                };

                let asset_display = if asset.name.chars().count() > 24 {
                    let truncated: String = asset.name.chars().take(21).collect();
                    format!("{}...", truncated)
                } else {
                    asset.name.clone()
                };

                let path_display = if asset.relative_path.chars().count() > 18 {
                    let tail: String = asset.relative_path.chars().rev().take(15).collect::<String>().chars().rev().collect();
                    format!("...{}", tail)
                } else {
                    asset.relative_path.clone()
                };

                let status_badge: Element<'a, DashboardMessage> = if is_selected {
                    text("Selected")
                        .size(LABEL_SM)
                        .wrapping(text::Wrapping::None)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b))
                        .into()
                } else {
                    text(format_bytes(asset.file_size))
                        .size(LABEL_SM)
                        .wrapping(text::Wrapping::None)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                        .into()
                };

                let card_body = column![
                    preview_element,
                    text(asset_display)
                        .size(LABEL_MD)
                        .wrapping(text::Wrapping::None)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    row![
                        text(path_display)
                            .size(LABEL_SM)
                            .wrapping(text::Wrapping::None)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        Space::with_width(Length::Fill),
                        status_badge,
                    ]
                    .align_y(Alignment::Center),
                ]
                .spacing(SPACE_1);

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

            container(scrollable(cards_col).width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .clip(true)
                .into()
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
                row![icon_sparkle_white(), text("Use Selected Asset").size(LABEL_MD)]
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
                text("! Unconnected Path(s)")
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

            let scene_label = container(
                text(scene_name)
                    .size(LABEL_SM)
                    .wrapping(iced::widget::text::Wrapping::None),
            )
            .width(Length::Fill)
            .clip(true);

            let row_item = if is_on_canvas {
                row![
                    icon_film(),
                    scene_label,
                    button(icon_close())
                        .padding([2.0, 6.0])
                        .style(danger_icon_button_style)
                        .on_press(DashboardMessage::FlowRemoveNode(node_id)),
                    container(
                        text("In Flow")
                            .size(LABEL_SM)
                            .wrapping(iced::widget::text::Wrapping::None)
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
                .width(Length::Fill)
            } else {
                row![
                    icon_film(),
                    scene_label,
                    button(
                        text("+ Add")
                            .size(LABEL_SM)
                            .wrapping(iced::widget::text::Wrapping::None),
                    )
                    .padding([2.0, 8.0])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::FlowAddScene(scene_name.clone())),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
                .width(Length::Fill)
            };

            scenes_list_col = scenes_list_col.push(
                container(row_item)
                    .width(Length::Fill)
                    .style(callout_container_style),
            );
        }

        // Section 2: Ending & Flow Outcome Nodes (built-in Pause/Settings/SaveLoad are automatic in runtime)
        let system_library_defs = [
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

            let sys_label = container(
                text(label)
                    .size(LABEL_SM)
                    .wrapping(iced::widget::text::Wrapping::None),
            )
            .width(Length::Fill)
            .clip(true);

            let row_item = if is_on_canvas {
                row![
                    icon_sparkle(),
                    sys_label,
                    container(
                        text("In Flow")
                            .size(LABEL_SM)
                            .wrapping(iced::widget::text::Wrapping::None)
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
                .width(Length::Fill)
            } else {
                row![
                    icon_sparkle(),
                    sys_label,
                    button(
                        text("+ Add")
                            .size(LABEL_SM)
                            .wrapping(iced::widget::text::Wrapping::None),
                    )
                    .padding([2.0, 8.0])
                    .style(primary_button_style)
                    .on_press(DashboardMessage::FlowAddSystemNode(sys_type.to_string())),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .padding([SPACE_1, SPACE_2])
                .width(Length::Fill)
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
            container(
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
                    text("Ending & Flow Nodes:")
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
            .padding(iced::Padding {
                top: 0.0,
                right: SPACE_3,
                bottom: 0.0,
                left: 0.0,
            })
            .width(Length::Fill),
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
        .width(Length::Fixed(340.0))
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
            .width(Length::Fixed(340.0))
            .height(Length::Fill)
            .clip(true)
            .style(sidebar_container_style);

        // --- 2. Right Canvas Area & Top Toolbar ---
        // Mode Buttons
        let is_sel = self.flow_mode == FlowInteractionMode::Select;
        let btn_mode_select = button(
            row![
                icon_cursor(),
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
                icon_link(),
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
                icon_cut(),
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
            .clip(true)
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

        // Left Column: Identity & Version
        let title_text_label = text("Game Title Text (Displayed on Main Menu):")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));
        let title_text_input = text_input("e.g. Echoes of Memories", &self.system_screens_config.title_text)
            .on_input(DashboardMessage::UpdateTitleText)
            .padding([SPACE_2, SPACE_3])
            .style(input_field_style);

        let version_col = column![
            text("Project / Game Version:")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text_input("e.g. 0.1.0", &project.version)
                .on_input(DashboardMessage::UpdateProjectVersion)
                .padding([SPACE_2, SPACE_3])
                .style(input_field_style),
        ]
        .spacing(SPACE_1)
        .width(Length::FillPortion(1));

        let author_col = column![
            text("Author / Studio:")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text_input("e.g. Studio Kharis", &project.author)
                .on_input(DashboardMessage::UpdateProjectAuthor)
                .padding([SPACE_2, SPACE_3])
                .style(input_field_style),
        ]
        .spacing(SPACE_1)
        .width(Length::FillPortion(1));

        let identity_row = row![version_col, author_col]
            .spacing(SPACE_3)
            .width(Length::Fill);

        let flow_hint = text("Starting scene is configured visually in Story Flow (Main Menu -> New Game).")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let left_col = column![
            title_text_label,
            title_text_input,
            Space::with_height(SPACE_1),
            identity_row,
            flow_hint,
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

        // Logo & Branding Selector Box
        let logo_badge: Element<'a, DashboardMessage> = if project.use_custom_logo {
            if let Some(logo_rel_path) = &project.custom_logo_path {
                let abs_path = if std::path::Path::new(logo_rel_path).is_absolute() {
                    std::path::PathBuf::from(logo_rel_path)
                } else {
                    _project_dir.join(logo_rel_path)
                };
                if abs_path.exists() {
                    iced::widget::image(iced::widget::image::Handle::from_path(abs_path))
                        .width(36.0)
                        .height(36.0)
                        .into()
                } else {
                    engine_logo_squircle(36.0).into()
                }
            } else {
                engine_logo_squircle(36.0).into()
            }
        } else {
            engine_logo_squircle(36.0).into()
        };

        let logo_status_text = if project.use_custom_logo {
            if let Some(p) = &project.custom_logo_path {
                format!("Custom Game Logo: {}", p)
            } else {
                "Custom Game Logo (No file selected)".to_string()
            }
        } else {
            "Default Engine Logo (Official Blanc Monogram)".to_string()
        };

        let btn_use_default = button(text("Default Logo").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if !project.use_custom_logo { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::SetUseCustomLogo(false));

        let btn_use_custom = button(text("Custom Game Logo").size(LABEL_SM))
            .padding([SPACE_1, SPACE_3])
            .style(if project.use_custom_logo { primary_button_style } else { secondary_button_style })
            .on_press(DashboardMessage::SetUseCustomLogo(true));

        let btn_choose_logo = button(
            row![icon_image(), text("Choose / Change Logo...").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_1, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::ChooseProjectLogo);

        let logo_box = container(
            row![
                logo_badge,
                column![
                    text("Project & Game Logo Badge").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text(logo_status_text).size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ].spacing(SPACE_1),
                Space::with_width(Length::Fill),
                row![btn_use_default, btn_use_custom, btn_choose_logo].spacing(SPACE_2).align_y(Alignment::Center),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        let card1 = container(
            column![
                card1_title,
                card1_body,
                logo_box,
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

        // Voice Volume Row
        let voice_vol = self.system_screens_config.voice_volume;
        let voice_fill = (voice_vol.clamp(0.0, 1.0) * 140.0) as f32;
        let voice_bar = container(
            row![
                container(Space::with_width(Length::Fixed(voice_fill)))
                    .height(Length::Fixed(6.0))
                    .style(meter_fill_style)
            ]
        )
        .width(Length::Fixed(140.0))
        .height(Length::Fixed(6.0))
        .style(meter_bg_style);

        let voice_badge = container(
            text(format!("{:.0}%", voice_vol * 100.0))
                .size(LABEL_SM)
                .color(if voice_vol == 0.0 { iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b) } else { iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b) })
        )
        .width(48)
        .align_x(Alignment::Center);

        let is_v_100 = (voice_vol - 1.0).abs() < 0.01;
        let is_v_80 = (voice_vol - 0.8).abs() < 0.01;
        let is_v_50 = (voice_vol - 0.5).abs() < 0.01;
        let is_v_0 = voice_vol == 0.0;

        let voice_vol_row = row![
            container(row![icon_volume(), text("Voice Over (VO)").size(LABEL_SM)].spacing(SPACE_2).align_y(Alignment::Center)).width(170),
            voice_bar,
            voice_badge,
            Space::with_width(SPACE_2),
            button(text("100%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_v_100 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateVoiceVolume(1.0)),
            button(text("80%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_v_80 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateVoiceVolume(0.8)),
            button(text("50%").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_v_50 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateVoiceVolume(0.5)),
            button(text("Mute").size(LABEL_SM)).padding([SPACE_1, SPACE_3]).style(if is_v_0 { primary_button_style } else { secondary_button_style }).on_press(DashboardMessage::UpdateVoiceVolume(0.0)),
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
                voice_vol_row,
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

        let cur_trans = self.system_screens_config.scene_transition_mode;
        let trans_label = container(
            text("Scene Transition:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
        )
        .width(170);

        let trans_row = row![
            trans_label,
            button(text("Clean Fade (Hide UI & Chars -> Fade)").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(if cur_trans == SceneTransitionMode::CleanFade { primary_button_style } else { secondary_button_style })
                .on_press(DashboardMessage::UpdateSceneTransitionMode(SceneTransitionMode::CleanFade)),
            button(text("Direct (Instant)").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(if cur_trans == SceneTransitionMode::Direct { primary_button_style } else { secondary_button_style })
                .on_press(DashboardMessage::UpdateSceneTransitionMode(SceneTransitionMode::Direct)),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let card4 = container(
            column![
                card4_title,
                playback_desc,
                speed_row,
                delay_row,
                trans_row,
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
            text("Blanc Engine 0.1.0")
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
    // EXPORT WORKSPACE
    // ==========================================
    fn view_export_content<'a>(&'a self, project: &'a Project, project_dir: &'a PathBuf) -> Element<'a, DashboardMessage> {
        let default_exe_name: String = project
            .name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect();
        let default_exe_name = if default_exe_name.trim_matches('_').is_empty() {
            "BlancGame".to_string()
        } else {
            default_exe_name
        };

        let effective_exe = if self.export_exe_name.trim().is_empty() {
            default_exe_name.clone()
        } else {
            self.export_exe_name.trim().trim_end_matches(".exe").to_string()
        };

        let effective_output = if self.export_output_dir.trim().is_empty() {
            project_dir.join("Exports").to_string_lossy().to_string()
        } else {
            self.export_output_dir.clone()
        };

        let title_col = column![
            text("Export Standalone Game")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Package your visual novel into a standalone Windows executable with encrypted game assets.")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let btn_open_folder = button(
            row![icon_folder(), text("Open Output Folder").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::OpenExportFolder);

        let mut btn_export = button(
            row![
                icon_play(),
                text(if self.export_in_progress {
                    "Exporting..."
                } else {
                    "Export Game"
                })
                .size(LABEL_MD)
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(primary_button_style);
        if !self.export_in_progress {
            btn_export = btn_export.on_press(DashboardMessage::RunExportGame);
        }

        let header_row = row![
            title_col,
            Space::with_width(Length::Fill),
            btn_open_folder,
            btn_export,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        // Card 1: Executable & Output Folder
        let card1_title = row![
            icon_sparkle(),
            text("Executable & Output Location").size(HEADING_SM),
            Space::with_width(SPACE_2),
            container(text("WINDOWS BUILD").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                .padding([2.0, 6.0])
                .style(badge_container_style),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let exe_input = text_input(&default_exe_name, &self.export_exe_name)
            .on_input(DashboardMessage::SetExportExeName)
            .padding([SPACE_2, SPACE_3])
            .style(input_field_style);

        let output_input = text_input("Select output folder...", &self.export_output_dir)
            .on_input(DashboardMessage::SetExportOutputDir)
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(input_field_style);

        let btn_browse = button(
            row![icon_folder(), text("Browse...").size(LABEL_SM)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(secondary_button_style)
        .on_press(DashboardMessage::BrowseExportOutputDir);

        let card1 = container(
            column![
                card1_title,
                row![
                    column![
                        text("Executable Name (.exe will be appended):")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        exe_input,
                        text(format!("Output binary: {}.exe", effective_exe))
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                    ]
                    .spacing(SPACE_2)
                    .width(Length::FillPortion(1)),
                    column![
                        text("Export Output Directory:")
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        row![output_input, btn_browse].spacing(SPACE_2).align_y(Alignment::Center),
                        text(format!("Target path: {}/{}.exe", effective_output, effective_exe))
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                    ]
                    .spacing(SPACE_2)
                    .width(Length::FillPortion(2)),
                ]
                .spacing(SPACE_5),
            ]
            .spacing(SPACE_4),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // Card 2: Packaging & Security Mode
        let option_title_color = |selected: bool| {
            if selected {
                iced::Color::WHITE
            } else {
                iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)
            }
        };
        let option_desc_color = |selected: bool| {
            if selected {
                iced::Color::from_rgba(1.0, 1.0, 1.0, 0.88)
            } else {
                iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)
            }
        };

        let btn_packed = button(
            column![
                text("Encrypted .bpack Archive (Recommended)")
                    .size(LABEL_MD)
                    .color(option_title_color(self.export_packed)),
                text("Packs all scenes, scripts, config, and assets into a single encrypted game.bpack file.")
                    .size(BODY_SM)
                    .color(option_desc_color(self.export_packed)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if self.export_packed { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportPacked(true));

        let btn_loose = button(
            column![
                text("Loose Folder (GameData/)")
                    .size(LABEL_MD)
                    .color(option_title_color(!self.export_packed)),
                text("Copies project files into a readable GameData/ folder next to the executable for debugging/modding.")
                    .size(BODY_SM)
                    .color(option_desc_color(!self.export_packed)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if !self.export_packed { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportPacked(false));

        let card2 = container(
            column![
                row![
                    text("Packaging & Asset Protection").size(HEADING_SM),
                    Space::with_width(SPACE_2),
                    container(text("PROTECTION").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                        .padding([2.0, 6.0])
                        .style(badge_container_style),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![btn_packed, btn_loose].spacing(SPACE_3).width(Length::Fill),
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_4)
        .width(Length::Fill)
        .style(card_container_style);

        // Card 3: Distribution Type (Portable vs Windows Installer)
        let is_portable_build = self.export_build_kind == ExportBuildKind::Portable;
        let btn_dist_portable = button(
            column![
                text("Portable Build (No Install Required)")
                    .size(LABEL_MD)
                    .color(option_title_color(is_portable_build)),
                text("Outputs ready-to-play Game.exe + game.bpack. Players extract and run directly.")
                    .size(BODY_SM)
                    .color(option_desc_color(is_portable_build)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if is_portable_build { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportBuildKind(ExportBuildKind::Portable));

        let btn_dist_installer = button(
            column![
                text("Windows Installer (Setup.exe Wizard)")
                    .size(LABEL_MD)
                    .color(option_title_color(!is_portable_build)),
                text("Packages a standalone Setup.exe wizard with install folder, shortcuts, and Uninstaller.")
                    .size(BODY_SM)
                    .color(option_desc_color(!is_portable_build)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if !is_portable_build { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportBuildKind(ExportBuildKind::Installer));

        let card_dist = container(
            column![
                row![
                    text("Distribution Type").size(HEADING_SM),
                    Space::with_width(SPACE_2),
                    container(text("PORTABLE / INSTALLER").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                        .padding([2.0, 6.0])
                        .style(badge_container_style),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                row![btn_dist_portable, btn_dist_installer].spacing(SPACE_3).width(Length::Fill),
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_4)
        .width(Length::Fill)
        .style(card_container_style);

        // Card 3b: Save Data Mode (Portable vs AppData)
        let is_portable = self.export_save_mode == ExportSaveMode::Portable;
        let btn_portable = button(
            column![
                text("Portable (Saves/ next to .exe)")
                    .size(LABEL_MD)
                    .color(option_title_color(is_portable)),
                text("Saves stay in Saves/ beside the game executable.")
                    .size(BODY_SM)
                    .color(option_desc_color(is_portable)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if is_portable { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportSaveMode(ExportSaveMode::Portable));

        let btn_appdata = button(
            column![
                text("System AppData (%APPDATA%/BlancGames)")
                    .size(LABEL_MD)
                    .color(option_title_color(!is_portable)),
                text("Stores saves in the Windows user profile folder (automatically used by Installer).")
                    .size(BODY_SM)
                    .color(option_desc_color(!is_portable)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::FillPortion(1))
        .style(if !is_portable { primary_button_style } else { secondary_button_style })
        .on_press(DashboardMessage::SetExportSaveMode(ExportSaveMode::AppData));

        let card3 = container(
            column![
                text("Save Data Location").size(HEADING_SM),
                row![btn_portable, btn_appdata].spacing(SPACE_3).width(Length::Fill),
            ]
            .spacing(SPACE_3),
        )
        .padding(SPACE_4)
        .width(Length::Fill)
        .style(card_container_style);

        // Card 3c: Installer Wizard Customization (when Installer is selected)
        let card_installer_opt: Option<Element<'a, DashboardMessage>> = if !is_portable_build {
            let toggle_pill = |lbl: &'static str, active: bool, msg: DashboardMessage| {
                button(text(lbl).size(LABEL_SM))
                    .padding([SPACE_1, SPACE_3])
                    .style(if active { primary_button_style } else { secondary_button_style })
                    .on_press(msg)
            };

            Some(
                container(
                    column![
                        row![
                            text("Setup Wizard Customization").size(HEADING_SM),
                            Space::with_width(SPACE_2),
                            container(text("SETUP.EXE").size(LABEL_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)))
                                .padding([2.0, 6.0])
                                .style(badge_container_style),
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                        row![
                            column![
                                text("Version:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                                text_input("1.0.0", &self.installer_version)
                                    .on_input(DashboardMessage::SetInstallerVersion)
                                    .padding([SPACE_2, SPACE_3])
                                    .style(input_field_style),
                            ]
                            .spacing(SPACE_1)
                            .width(Length::FillPortion(1)),
                            column![
                                text("Publisher / Studio:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                                text_input(&project.author, &self.installer_publisher)
                                    .on_input(DashboardMessage::SetInstallerPublisher)
                                    .padding([SPACE_2, SPACE_3])
                                    .style(input_field_style),
                            ]
                            .spacing(SPACE_1)
                            .width(Length::FillPortion(2)),
                        ]
                        .spacing(SPACE_3),
                        column![
                            text("Welcome Message:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                            text_input("Welcome! Click Next to install...", &self.installer_welcome_text)
                                .on_input(DashboardMessage::SetInstallerWelcome)
                                .padding([SPACE_2, SPACE_3])
                                .style(input_field_style),
                        ]
                        .spacing(SPACE_1),
                        {
                            let mut lang_row = row(InstallerLang::ALL.iter().map(|&lang| {
                                let code = lang.code().to_string();
                                let has_text = self
                                    .installer_policy_texts
                                    .get(&code)
                                    .is_some_and(|t| !t.trim().is_empty());
                                let label = if has_text {
                                    format!("{} •", lang.short_label())
                                } else {
                                    lang.short_label().to_string()
                                };
                                let active = self.installer_policy_lang == code;
                                button(text(label).size(LABEL_SM))
                                    .padding([SPACE_1, SPACE_3])
                                    .style(if active { primary_button_style } else { subtle_button_style })
                                    .on_press(DashboardMessage::SelectInstallerPolicyLang(code))
                                    .into()
                            }))
                            .spacing(SPACE_1)
                            .align_y(Alignment::Center);

                            for custom in &self.installer_custom_policy_langs {
                                let code = custom.code.clone();
                                let has_text = self
                                    .installer_policy_texts
                                    .get(&code)
                                    .is_some_and(|t| !t.trim().is_empty());
                                let label = if has_text {
                                    format!("{} •", code.to_ascii_uppercase())
                                } else {
                                    code.to_ascii_uppercase()
                                };
                                let active = self.installer_policy_lang == code;
                                lang_row = lang_row.push(
                                    row![
                                        button(text(label).size(LABEL_SM))
                                            .padding([SPACE_1, SPACE_2])
                                            .style(if active { primary_button_style } else { subtle_button_style })
                                            .on_press(DashboardMessage::SelectInstallerPolicyLang(code.clone())),
                                        button(text("×").size(LABEL_SM))
                                            .padding([SPACE_1, SPACE_1])
                                            .style(subtle_button_style)
                                            .on_press(DashboardMessage::RemoveCustomPolicyLang(code)),
                                    ]
                                    .spacing(2.0),
                                );
                            }

                            lang_row = lang_row.push(
                                button(text("+").size(LABEL_SM))
                                    .padding([SPACE_1, SPACE_3])
                                    .style(if self.installer_add_lang_open {
                                        primary_button_style
                                    } else {
                                        subtle_button_style
                                    })
                                    .on_press(DashboardMessage::ToggleAddInstallerPolicyLang),
                            );

                            column![
                                row![
                                    text("Terms & Privacy Policy (per language — empty falls back to EN):").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                                    Space::with_width(Length::Fill),
                                    button(text("Reset Defaults").size(LABEL_SM))
                                        .padding([SPACE_1, SPACE_2])
                                        .style(subtle_button_style)
                                        .on_press(DashboardMessage::ResetInstallerPolicyDefault),
                                ]
                                .align_y(Alignment::Center),
                                lang_row,
                                if self.installer_add_lang_open {
                                    Element::from(
                                        container(
                                            row![
                                                text_input("Code (e.g. ko, fr)", &self.installer_new_lang_code)
                                                    .on_input(DashboardMessage::SetNewPolicyLangCode)
                                                    .padding([SPACE_1, SPACE_2])
                                                    .width(Length::Fixed(130.0))
                                                    .style(input_field_style),
                                                text_input("Display Name (e.g. 한국어 (Korean))", &self.installer_new_lang_name)
                                                    .on_input(DashboardMessage::SetNewPolicyLangName)
                                                    .padding([SPACE_1, SPACE_2])
                                                    .width(Length::Fill)
                                                    .style(input_field_style),
                                                button(text("Add").size(LABEL_SM))
                                                    .padding([SPACE_1, SPACE_3])
                                                    .style(primary_button_style)
                                                    .on_press(DashboardMessage::ConfirmAddInstallerPolicyLang),
                                                button(text("Cancel").size(LABEL_SM))
                                                    .padding([SPACE_1, SPACE_2])
                                                    .style(subtle_button_style)
                                                    .on_press(DashboardMessage::ToggleAddInstallerPolicyLang),
                                            ]
                                            .spacing(SPACE_2)
                                            .align_y(Alignment::Center),
                                        )
                                        .padding(SPACE_2)
                                        .style(badge_container_style),
                                    )
                                } else {
                                    Space::with_height(0.0).into()
                                },
                                text_editor(&self.installer_policy_editor)
                                    .on_action(DashboardMessage::InstallerPolicyEditorAction)
                                    .height(Length::Fixed(160.0))
                                    .style(text_editor_style),
                                text("Placeholders {GAME_TITLE} and {PUBLISHER} are replaced automatically. Leave all languages blank to skip.")
                                    .size(LABEL_SM)
                                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                            ]
                            .spacing(SPACE_1)
                        },
                        row![
                            toggle_pill(
                                if self.installer_include_logo { "Logo: ON" } else { "Logo: OFF" },
                                self.installer_include_logo,
                                DashboardMessage::SetInstallerIncludeLogo(!self.installer_include_logo),
                            ),
                            toggle_pill(
                                if self.installer_include_banner { "Title Banner: ON" } else { "Title Banner: OFF" },
                                self.installer_include_banner,
                                DashboardMessage::SetInstallerIncludeBanner(!self.installer_include_banner),
                            ),
                            toggle_pill(
                                if self.installer_desktop_shortcut { "Desktop Shortcut: ON" } else { "Desktop Shortcut: OFF" },
                                self.installer_desktop_shortcut,
                                DashboardMessage::SetInstallerDesktopShortcut(!self.installer_desktop_shortcut),
                            ),
                            toggle_pill(
                                if self.installer_start_menu_shortcut { "Start Menu: ON" } else { "Start Menu: OFF" },
                                self.installer_start_menu_shortcut,
                                DashboardMessage::SetInstallerStartMenuShortcut(!self.installer_start_menu_shortcut),
                            ),
                        ]
                        .spacing(SPACE_2),
                        row![
                            text(match &self.installer_custom_banner_path {
                                Some(p) => format!(
                                    "Wizard Sidebar Photo: {}",
                                    p.file_name()
                                        .map(|n| n.to_string_lossy().to_string())
                                        .unwrap_or_else(|| "Custom Image".to_string())
                                ),
                                None => "Wizard Sidebar Photo: Title Screen Background (Default)".to_string(),
                            })
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                            Space::with_width(Length::Fill),
                            button(text("Choose Image...").size(LABEL_SM))
                                .padding([SPACE_1, SPACE_2])
                                .style(subtle_button_style)
                                .on_press(DashboardMessage::PickInstallerCustomBanner),
                            if self.installer_custom_banner_path.is_some() {
                                Element::from(
                                    button(text("Use Default").size(LABEL_SM))
                                        .padding([SPACE_1, SPACE_2])
                                        .style(subtle_button_style)
                                        .on_press(DashboardMessage::ClearInstallerCustomBanner),
                                )
                            } else {
                                Space::with_width(0.0).into()
                            },
                        ]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(SPACE_3),
                )
                .padding(SPACE_4)
                .width(Length::Fill)
                .style(card_container_style)
                .into(),
            )
        } else {
            None
        };

        // Card 4: Pre-Export Checklist
        let runtime_exe_found = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|dir| dir.join("blanc_runtime.exe").exists()))
            .unwrap_or(false);
        let installer_exe_found = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|dir| dir.join("blanc_installer.exe").exists()))
            .unwrap_or(false);

        let check_line = |ok: bool, label: String| {
            let status_txt = if ok { "READY" } else { "MISSING" };
            let status_col = if ok {
                iced::Color::from_rgb8(110, 220, 140)
            } else {
                iced::Color::from_rgb8(240, 120, 120)
            };
            row![
                text(status_txt).size(LABEL_SM).color(status_col),
                text(label)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center)
        };

        let card4 = container(
            column![
                text("Pre-Export Checklist").size(HEADING_SM),
                check_line(!self.scenes_list.is_empty(), format!("Scenes available: {} scene(s)", self.scenes_list.len())),
                check_line(true, format!("Registered assets: {} asset(s)", project.assets.assets.len())),
                check_line(
                    runtime_exe_found,
                    if runtime_exe_found {
                        "Runtime template (blanc_runtime.exe) found next to editor.".to_string()
                    } else {
                        "Runtime template (blanc_runtime.exe) not found next to editor — build blanc_runtime first.".to_string()
                    }
                ),
                check_line(
                    is_portable_build || installer_exe_found,
                    if is_portable_build {
                        "Installer stub not needed for Portable build.".to_string()
                    } else if installer_exe_found {
                        "Installer wizard stub (blanc_installer.exe) found next to editor.".to_string()
                    } else {
                        "Installer wizard stub (blanc_installer.exe) not found — build blanc_installer first.".to_string()
                    }
                ),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_4)
        .width(Length::Fill)
        .style(card_container_style);

        let mut content_col = column![header_row, card1, card2, card_dist, card3].spacing(SPACE_4);
        if let Some(card_inst) = card_installer_opt {
            content_col = content_col.push(card_inst);
        }
        content_col = content_col.push(card4);

        let centered_content = container(content_col)
            .max_width(1120.0)
            .width(Length::Fill);

        let sticky_footer_content: Element<'_, DashboardMessage> = if self.export_in_progress {
            let pct = (self.export_progress * 100.0).clamp(0.0, 100.0).round() as u32;
            column![
                row![
                    text(&self.export_stage_label)
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    Space::with_width(Length::Fill),
                    text(format!("{}%", pct))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                ]
                .align_y(Alignment::Center),
                progress_bar(0.0..=1.0, self.export_progress)
                    .width(Length::Fill)
                    .height(Length::Fixed(8.0)),
            ]
            .spacing(SPACE_2)
            .into()
        } else if let Some(msg) = &self.export_status_message {
            let is_err = msg.starts_with("Export failed");
            let msg_color = if is_err {
                iced::Color::from_rgb8(248, 113, 113)
            } else {
                iced::Color::from_rgb8(110, 220, 140)
            };
            row![
                text(msg).size(BODY_SM).color(msg_color),
                Space::with_width(Length::Fill),
                button(text("Open Folder").size(LABEL_SM))
                    .padding([SPACE_1, SPACE_3])
                    .style(subtle_button_style)
                    .on_press(DashboardMessage::OpenExportFolder),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center)
            .into()
        } else {
            let exe_base = if self.export_exe_name.trim().is_empty() {
                project.name.replace(' ', "_")
            } else {
                self.export_exe_name.trim().to_string()
            };
            let target_label = if is_portable_build {
                format!("{}.exe (Portable)", exe_base)
            } else {
                format!("{}_Setup.exe (Windows Installer)", exe_base)
            };
            row![
                text(format!("Ready to export: {}", target_label))
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                Space::with_width(Length::Fill),
            ]
            .align_y(Alignment::Center)
            .into()
        };

        let sticky_footer = container(sticky_footer_content)
            .width(Length::Fill)
            .padding([SPACE_3, SPACE_6])
            .style(sidebar_container_style);

        column![
            container(
                scrollable(
                    container(centered_content)
                        .width(Length::Fill)
                        .padding([SPACE_4, SPACE_6])
                        .center_x(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true),
            sticky_footer,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    // ==========================================
    // LOCALIZATION TAB
    // ==========================================
    fn view_localization_content(&self) -> Element<'_, DashboardMessage> {
        let selected_key = self.locale_selected_code.to_ascii_lowercase();
        let active_table = self.locale_tables.get(&selected_key);

        let active_lang_entry = self
            .locale_manifest
            .all_languages()
            .into_iter()
            .find(|l| l.code.eq_ignore_ascii_case(&self.locale_selected_code));
        let active_lang_name = active_lang_entry
            .as_ref()
            .map(|l| l.display_name.clone())
            .unwrap_or_else(|| self.locale_selected_code.clone());
        let story_lang_code = self.locale_manifest.effective_story_language().code.clone();
        let needs_translation = |item: &blanc_core::localization::LocalizationItem, lang_code: &str| -> bool {
            match &item.category {
                LocaleCategory::Ui => !lang_code.eq_ignore_ascii_case("en"),
                LocaleCategory::Speaker | LocaleCategory::Scene(_) => {
                    !lang_code.eq_ignore_ascii_case(&story_lang_code)
                }
            }
        };

        let active_total = self
            .locale_items
            .iter()
            .filter(|it| needs_translation(it, &self.locale_selected_code))
            .count();
        let active_translated = active_table
            .map(|tbl| {
                self.locale_items
                    .iter()
                    .filter(|it| {
                        needs_translation(it, &self.locale_selected_code)
                            && tbl.entries.get(&it.key).map(|v| !v.trim().is_empty()).unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);
        let active_pct = if active_total == 0 {
            100
        } else {
            (active_translated * 100) / active_total
        };

        let header = container(
            row![
                column![
                    text("Localization & Multi-Language")
                        .size(HEADING_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("Manage game languages and translate UI labels, character names, and scene dialogue.")
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                ]
                .spacing(SPACE_1)
                .width(Length::Fill),
                row![
                    button(text("Sync Strings").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::LocaleSyncFromProject),
                    button(text("Export CSV").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::LocaleExportCsv),
                    button(text("Import CSV").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(DashboardMessage::LocaleImportCsv),
                    button(text("Save Locales").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(primary_button_style)
                        .on_press(DashboardMessage::LocaleSaveAll),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_4)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(card_container_style);

        // Left sidebar: Unified panel (Compact Default bar + Collapsible Add + Languages list)
        let mut default_section = column![
            row![
                column![
                    text("SOURCE LANGUAGE")
                        .size(10)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!(
                        "{} ({})",
                        self.locale_manifest.default_language.display_name,
                        self.locale_manifest.default_language.code
                    ))
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text(if self.locale_show_default_editor { "Close" } else { "Edit" }).size(LABEL_SM))
                    .padding([4, 10])
                    .style(subtle_button_style)
                    .on_press(DashboardMessage::LocaleToggleDefaultEditor),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(SPACE_2);

        if self.locale_show_default_editor {
            default_section = default_section.push(
                container(
                    column![
                        row![
                            text_input("Code (en)", &self.locale_default_code)
                                .on_input(DashboardMessage::LocaleDefaultCodeChanged)
                                .size(BODY_SM)
                                .padding(SPACE_2)
                                .width(Length::Fixed(76.0))
                                .style(input_field_style),
                            text_input("Display Name", &self.locale_default_name)
                                .on_input(DashboardMessage::LocaleDefaultNameChanged)
                                .size(BODY_SM)
                                .padding(SPACE_2)
                                .width(Length::Fill)
                                .style(input_field_style),
                        ]
                        .spacing(SPACE_2),
                        button(text("Save Default Language").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .width(Length::Fill)
                            .style(primary_button_style)
                            .on_press(DashboardMessage::LocaleSaveDefaultLanguage),
                    ]
                    .spacing(SPACE_2),
                )
                .padding(SPACE_2)
                .width(Length::Fill)
                .style(code_container_style),
            );
        }

        let mut languages_header_col = column![
            row![
                text("LANGUAGES")
                    .size(11)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                Space::new(Length::Fill, Length::Shrink),
                button(text(if self.locale_show_add_form { "Cancel" } else { "+ Add" }).size(LABEL_SM))
                    .padding([4, 10])
                    .style(if self.locale_show_add_form { subtle_button_style } else { primary_button_style })
                    .on_press(DashboardMessage::LocaleToggleAddForm),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(SPACE_2);

        if self.locale_show_add_form {
            let presets = [
                ("id", "Indonesian"),
                ("ja", "日本語"),
                ("zh", "中文"),
                ("ko", "한국어"),
                ("es", "Español"),
            ];
            let mut preset_row = row![].spacing(4);
            for (code, name) in presets {
                preset_row = preset_row.push(
                    button(text(code.to_ascii_uppercase()).size(10))
                        .padding([3, 7])
                        .style(subtle_button_style)
                        .on_press(DashboardMessage::LocaleFillPresetLanguage(
                            code.to_string(),
                            name.to_string(),
                        )),
                );
            }

            languages_header_col = languages_header_col.push(
                container(
                    column![
                        text("Quick presets:").size(10).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        preset_row,
                        row![
                            text_input("Code", &self.locale_new_code)
                                .on_input(DashboardMessage::LocaleNewCodeChanged)
                                .size(BODY_SM)
                                .padding(SPACE_2)
                                .width(Length::Fixed(76.0))
                                .style(input_field_style),
                            text_input("Name (e.g. Indonesian)", &self.locale_new_name)
                                .on_input(DashboardMessage::LocaleNewNameChanged)
                                .size(BODY_SM)
                                .padding(SPACE_2)
                                .width(Length::Fill)
                                .style(input_field_style),
                        ]
                        .spacing(SPACE_2),
                        button(text("Create Language").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .width(Length::Fill)
                            .style(primary_button_style)
                            .on_press(DashboardMessage::LocaleAddLanguage),
                    ]
                    .spacing(SPACE_2),
                )
                .padding(SPACE_2)
                .width(Length::Fill)
                .style(code_container_style),
            );
        }

        let mut lang_list_col = column![].spacing(SPACE_1).width(Length::Fill);
        for lang in self.locale_manifest.all_languages() {
            let is_default = lang
                .code
                .eq_ignore_ascii_case(&self.locale_manifest.default_language.code);
            let is_story_lang = lang.code.eq_ignore_ascii_case(&story_lang_code);
            let is_selected = lang.code.eq_ignore_ascii_case(&self.locale_selected_code);

            let lang_total = self
                .locale_items
                .iter()
                .filter(|it| needs_translation(it, &lang.code))
                .count();
            let translated_count = self
                .locale_tables
                .get(&lang.code.to_ascii_lowercase())
                .map(|tbl| {
                    self.locale_items
                        .iter()
                        .filter(|it| {
                            needs_translation(it, &lang.code)
                                && tbl.entries.get(&it.key).map(|v| !v.trim().is_empty()).unwrap_or(false)
                        })
                        .count()
                })
                .unwrap_or(0);
            let pct = if lang_total == 0 {
                100
            } else {
                (translated_count * 100) / lang_total
            };
            let bar_total_w = 124.0f32;
            let bar_fill_w = ((pct as f32 / 100.0) * bar_total_w).clamp(0.0, bar_total_w);

            let mut title_row = row![
                container(
                    text(lang.display_name.clone())
                        .size(LABEL_MD)
                        .wrapping(iced::widget::text::Wrapping::None)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                )
                .width(Length::Fill)
                .clip(true),
                container(
                    text(lang.code.to_ascii_uppercase())
                        .size(10)
                        .wrapping(iced::widget::text::Wrapping::None)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                )
                .padding([1, 6])
                .style(badge_container_style),
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center)
            .width(Length::Fill);

            if is_default {
                title_row = title_row.push(
                    container(
                        text("DEF")
                            .size(9)
                            .wrapping(iced::widget::text::Wrapping::None)
                            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                    )
                    .padding([1, 5])
                    .style(badge_container_style),
                );
            }
            if is_story_lang {
                title_row = title_row.push(
                    container(
                        text("STORY")
                            .size(9)
                            .wrapping(iced::widget::text::Wrapping::None)
                            .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                    )
                    .padding([1, 5])
                    .style(badge_container_style),
                );
            }

            let progress_bar = container(
                row![
                    container(Space::new(Length::Fixed(bar_fill_w), Length::Fixed(4.0)))
                        .height(Length::Fixed(4.0))
                        .style(meter_fill_style),
                ],
            )
            .width(Length::Fixed(bar_total_w))
            .height(Length::Fixed(4.0))
            .style(meter_bg_style);

            let count_text = if lang_total == 0 {
                "Native (no translation needed)".to_string()
            } else {
                format!("{} / {} strings", translated_count, lang_total)
            };

            let info_col = column![
                title_row,
                row![
                    progress_bar,
                    Space::new(Length::Fill, Length::Shrink),
                    text(format!("{}%", pct))
                        .size(10)
                        .wrapping(iced::widget::text::Wrapping::None)
                        .color(if pct == 100 {
                            iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
                        } else {
                            iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)
                        }),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .width(Length::Fill),
                text(count_text)
                    .size(10)
                    .wrapping(iced::widget::text::Wrapping::None)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(4)
            .width(Length::Fill);

            let mut card_row = row![
                button(info_col)
                    .padding([SPACE_2, SPACE_3])
                    .width(Length::Fill)
                    .style(sidebar_button_style(is_selected))
                    .on_press(DashboardMessage::LocaleSelectLanguage(lang.code.clone())),
            ]
            .spacing(SPACE_1)
            .align_y(Alignment::Center);

            if !is_default {
                card_row = card_row.push(
                    button(text("×").size(LABEL_MD))
                        .padding([SPACE_2, SPACE_2])
                        .style(danger_icon_button_style)
                        .on_press(DashboardMessage::LocaleRemoveLanguage(lang.code.clone())),
                );
            }

            lang_list_col = lang_list_col.push(card_row);
        }

        let story_lang_entry = self.locale_manifest.effective_story_language();
        let mut story_lang_section = column![
            row![
                column![
                    text("STORY LANGUAGE")
                        .size(10)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    text(format!(
                        "{} ({})",
                        story_lang_entry.display_name, story_lang_entry.code
                    ))
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                ]
                .spacing(2)
                .width(Length::Fill),
                button(text(if self.locale_story_lang_open { "Close" } else { "Edit" }).size(LABEL_SM))
                    .padding([4, 10])
                    .style(secondary_button_style)
                    .on_press(DashboardMessage::LocaleToggleStoryLangEditor),
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(SPACE_2);

        if self.locale_story_lang_open {
            let presets = [
                ("en", "English"),
                ("id", "Indonesia"),
                ("ja", "Japanese"),
                ("ko", "Korean"),
                ("zh", "Chinese"),
                ("es", "Spanish"),
            ];
            let mut preset_grid = column![].spacing(4);
            for chunk in presets.chunks(2) {
                let mut r = row![].spacing(4);
                for (code, name) in chunk {
                    let active = story_lang_entry.code.eq_ignore_ascii_case(code);
                    r = r.push(
                        button(text(format!("{} ({})", name, code)).size(10))
                            .padding([4, 6])
                            .width(Length::FillPortion(1))
                            .style(if active { primary_button_style } else { secondary_button_style })
                            .on_press(DashboardMessage::SetStoryScriptLanguage(
                                (*code).to_string(),
                                (*name).to_string(),
                            )),
                    );
                }
                preset_grid = preset_grid.push(r);
            }
            let custom_trim = self.story_lang_custom_input.trim().to_ascii_lowercase();
            let set_btn = {
                let b = button(text("Set").size(10))
                    .padding([4, 8])
                    .style(if custom_trim.is_empty() { subtle_button_style } else { secondary_button_style });
                if custom_trim.is_empty() {
                    b
                } else {
                    let upper = custom_trim.to_ascii_uppercase();
                    b.on_press(DashboardMessage::SetStoryScriptLanguage(custom_trim, upper))
                }
            };
            story_lang_section = story_lang_section.push(preset_grid);
            story_lang_section = story_lang_section.push(
                row![
                    text_input("Other code (e.g. fr)", &self.story_lang_custom_input)
                        .on_input(DashboardMessage::StoryLangCustomCodeChanged)
                        .size(11)
                        .padding(SPACE_1)
                        .width(Length::Fill)
                        .style(input_field_style),
                    set_btn,
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            );
        }

        let left_pane = container(
            column![
                container(default_section)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(card_container_style),
                container(story_lang_section)
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(card_container_style),
                languages_header_col,
                scrollable(lang_list_col).height(Length::Fill).width(Length::Fill),
            ]
            .spacing(SPACE_3)
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding(SPACE_3)
        .width(Length::Fixed(264.0))
        .height(Length::Fill)
        .style(sidebar_container_style);

        // Middle pane: Vertical Sections / Groups column with per-group progress
        let ui_needed = !self.locale_selected_code.eq_ignore_ascii_case("en");
        let story_needed = !self.locale_selected_code.eq_ignore_ascii_case(&story_lang_code);

        let mut scopes = Vec::new();
        if ui_needed {
            scopes.push(LocaleFilterScope::Ui);
        }
        if story_needed {
            scopes.push(LocaleFilterScope::Speaker);
            let mut seen_scenes: Vec<String> = Vec::new();
            for item in &self.locale_items {
                if let LocaleCategory::Scene(name) = &item.category {
                    if !seen_scenes.contains(name) {
                        seen_scenes.push(name.clone());
                    }
                }
            }
            for s in seen_scenes {
                scopes.push(LocaleFilterScope::Scene(s));
            }
        }
        if !scopes.is_empty() {
            scopes.push(LocaleFilterScope::All);
        }

        let effective_scope = if scopes.contains(&self.locale_filter_scope) {
            self.locale_filter_scope.clone()
        } else {
            scopes.first().cloned().unwrap_or(LocaleFilterScope::All)
        };

        let mut groups_col = column![].spacing(SPACE_1).width(Length::Fill);
        for scope in &scopes {
            let mut grp_total = 0usize;
            let mut grp_done = 0usize;
            for item in &self.locale_items {
                if !needs_translation(item, &self.locale_selected_code) {
                    continue;
                }
                let in_scope = match scope {
                    LocaleFilterScope::All => true,
                    LocaleFilterScope::Ui => item.category == LocaleCategory::Ui,
                    LocaleFilterScope::Speaker => item.category == LocaleCategory::Speaker,
                    LocaleFilterScope::Scene(name) => {
                        matches!(&item.category, LocaleCategory::Scene(s) if s == name)
                    }
                };
                if in_scope {
                    grp_total += 1;
                    let translated = active_table
                        .and_then(|t| t.entries.get(&item.key))
                        .map(|v| !v.trim().is_empty())
                        .unwrap_or(false);
                    if translated {
                        grp_done += 1;
                    }
                }
            }

            let is_selected = &effective_scope == scope;
            let is_complete = grp_total > 0 && grp_done == grp_total;
            let label_str = match scope {
                LocaleFilterScope::All => "All Strings".to_string(),
                LocaleFilterScope::Ui => "UI Labels".to_string(),
                LocaleFilterScope::Speaker => "Speakers".to_string(),
                LocaleFilterScope::Scene(name) => name.clone(),
            };
            let badge_str = format!("{}/{}", grp_done, grp_total);
            let badge_color = if is_complete {
                iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
            } else if is_selected {
                iced::Color::WHITE
            } else {
                iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)
            };

            let btn_content = row![
                container(
                    text(label_str)
                        .size(LABEL_SM)
                        .wrapping(iced::widget::text::Wrapping::None),
                )
                .width(Length::Fill)
                .clip(true),
                text(badge_str)
                    .size(10)
                    .wrapping(iced::widget::text::Wrapping::None)
                    .color(badge_color),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center)
            .width(Length::Fill);

            groups_col = groups_col.push(
                button(btn_content)
                    .padding([SPACE_2, SPACE_2])
                    .width(Length::Fill)
                    .style(if is_selected {
                        primary_button_style
                    } else {
                        subtle_button_style
                    })
                    .on_press(DashboardMessage::LocaleFilterScopeChanged(scope.clone())),
            );
        }

        let groups_pane = container(
            column![
                text("SECTIONS")
                    .size(11)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                scrollable(
                    container(groups_col)
                        .padding(iced::Padding {
                            top: 0.0,
                            right: SPACE_2,
                            bottom: 0.0,
                            left: 0.0,
                        })
                        .width(Length::Fill),
                )
                .height(Length::Fill)
                .width(Length::Fill),
            ]
            .spacing(SPACE_2)
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding(SPACE_3)
        .width(Length::Fixed(236.0))
        .height(Length::Fill)
        .style(sidebar_container_style);

        let filter_bar = row![
            text_input("Search across all groups (key, source, or translation)...", &self.locale_search_query)
                .on_input(DashboardMessage::LocaleSearchChanged)
                .size(BODY_SM)
                .padding(SPACE_2)
                .width(Length::Fill)
                .style(input_field_style),
            container(
                row![
                    button(text("All").size(LABEL_SM))
                        .padding([5, 10])
                        .style(if !self.locale_untranslated_only { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::LocaleToggleUntranslatedOnly(false)),
                    button(text("Untranslated").size(LABEL_SM))
                        .padding([5, 10])
                        .style(if self.locale_untranslated_only { primary_button_style } else { subtle_button_style })
                        .on_press(DashboardMessage::LocaleToggleUntranslatedOnly(true)),
                ]
                .spacing(2),
            )
            .padding(2)
            .style(code_container_style),
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        let source_header_code = match &effective_scope {
            LocaleFilterScope::Ui => "EN".to_string(),
            LocaleFilterScope::Speaker | LocaleFilterScope::Scene(_) => {
                story_lang_code.to_ascii_uppercase()
            }
            LocaleFilterScope::All => {
                if ui_needed && story_needed && !story_lang_code.eq_ignore_ascii_case("en") {
                    format!("EN / {}", story_lang_code.to_ascii_uppercase())
                } else if story_needed {
                    story_lang_code.to_ascii_uppercase()
                } else {
                    "EN".to_string()
                }
            }
        };

        let table_header = container(
            row![
                text(format!("SOURCE ({})", source_header_code))
                .size(10)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                .width(Length::FillPortion(1)),
                text(format!(
                    "TRANSLATION ({})",
                    self.locale_selected_code.to_ascii_uppercase()
                ))
                .size(10)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                .width(Length::FillPortion(1)),
            ]
            .spacing(SPACE_3),
        )
        .padding([SPACE_1, SPACE_3])
        .width(Length::Fill)
        .style(code_container_style);

        let query = self.locale_search_query.trim().to_ascii_lowercase();
        let mut rows_col = column![].spacing(SPACE_1).width(Length::Fill);
        let mut shown_count = 0usize;

        for item in &self.locale_items {
            if !needs_translation(item, &self.locale_selected_code) {
                continue;
            }
            let scope_ok = if !query.is_empty() {
                true
            } else {
                match &effective_scope {
                    LocaleFilterScope::All => true,
                    LocaleFilterScope::Ui => item.category == LocaleCategory::Ui,
                    LocaleFilterScope::Speaker => item.category == LocaleCategory::Speaker,
                    LocaleFilterScope::Scene(name) => {
                        matches!(&item.category, LocaleCategory::Scene(s) if s == name)
                    }
                }
            };
            if !scope_ok {
                continue;
            }

            let trans_val = active_table
                .and_then(|t| t.entries.get(&item.key))
                .map(String::as_str)
                .unwrap_or("");

            if self.locale_untranslated_only && !trans_val.is_empty() {
                continue;
            }

            if !query.is_empty() {
                let matches = item.key.to_ascii_lowercase().contains(&query)
                    || item.source_text.to_ascii_lowercase().contains(&query)
                    || trans_val.to_ascii_lowercase().contains(&query)
                    || item.context_label.to_ascii_lowercase().contains(&query);
                if !matches {
                    continue;
                }
            }

            shown_count += 1;
            let key_clone = item.key.clone();
            let is_translated = !trans_val.trim().is_empty();
            let status_color = if is_translated {
                iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
            } else {
                iced::Color::from_rgb8(STATUS_WARNING.r, STATUS_WARNING.g, STATUS_WARNING.b)
            };

            let row_item = container(
                column![
                    row![
                        text("●").size(10).color(status_color),
                        container(
                            text(&item.context_label)
                                .size(10)
                                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                        )
                        .padding([1, 7])
                        .style(badge_container_style),
                        text(&item.key)
                            .size(11)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                        Space::new(Length::Fill, Length::Shrink),
                        text(if is_translated { "Translated" } else { "Fallback" })
                            .size(10)
                            .color(status_color),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
                    row![
                        container(
                            text(&item.source_text)
                                .size(BODY_SM)
                                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                        )
                        .padding([SPACE_2, SPACE_2])
                        .width(Length::FillPortion(1))
                        .style(code_container_style),
                        text_input("Enter translation (empty = fallback to source)...", trans_val)
                            .on_input(move |v| DashboardMessage::LocaleTranslationChanged(key_clone.clone(), v))
                            .size(BODY_SM)
                            .padding(SPACE_2)
                            .width(Length::FillPortion(1))
                            .style(input_field_style),
                    ]
                    .spacing(SPACE_3)
                    .align_y(Alignment::Start),
                ]
                .spacing(SPACE_2),
            )
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(code_container_style);

            rows_col = rows_col.push(row_item);
        }

        if shown_count == 0 {
            let empty_msg = if scopes.is_empty() {
                "All strings in this language are already native — no translation needed."
            } else {
                "No strings match the current filter."
            };
            rows_col = rows_col.push(
                container(
                    text(empty_msg)
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                )
                .padding(SPACE_5)
                .width(Length::Fill)
                .center_x(Length::Fill),
            );
        }

        let right_pane = container(
            column![
                row![
                    row![
                        text(format!("{} ({})", active_lang_name, self.locale_selected_code))
                            .size(HEADING_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        container(
                            text(if active_total == 0 {
                                "Native · 100%".to_string()
                            } else {
                                format!("{}/{} translated · {}%", active_translated, active_total, active_pct)
                            })
                                .size(LABEL_SM)
                                .color(if active_pct == 100 {
                                    iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
                                } else {
                                    iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)
                                }),
                        )
                        .padding([2, 8])
                        .style(badge_container_style),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
                    Space::new(Length::Fill, Length::Shrink),
                    text(if !query.is_empty() {
                        format!("Searching all groups · {} match(es)", shown_count)
                    } else if scopes.is_empty() {
                        "Native · 0 string(s) to translate".to_string()
                    } else {
                        format!("{} · {} string(s)", effective_scope, shown_count)
                    })
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill),
                filter_bar,
                table_header,
                scrollable(rows_col).height(Length::Fill).width(Length::Fill),
            ]
            .spacing(SPACE_2)
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(card_container_style);

        let mut root = column![header].spacing(SPACE_3).padding(SPACE_4).height(Length::Fill);
        if let Some(status) = &self.locale_status_message {
            root = root.push(
                container(
                    text(status)
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                )
                .padding([SPACE_2, SPACE_3])
                .width(Length::Fill)
                .style(callout_container_style),
            );
        }
        root = root.push(
            row![left_pane, groups_pane, right_pane]
                .spacing(SPACE_3)
                .width(Length::Fill)
                .height(Length::Fill),
        );

        root.into()
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
