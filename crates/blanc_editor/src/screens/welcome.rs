use std::path::PathBuf;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};

use blanc_core::project::Project;
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::icons::*;
use crate::theme::{
    badge_container_style, callout_container_style, card_container_style, code_container_style,
    icon_button_style, input_field_style, primary_button_style, project_card_style,
    secondary_button_style, sidebar_button_style, sidebar_container_style, subtle_button_style,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HubTab {
    #[default]
    Projects,
    LearnDocs,
    SubmitGames,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSubmitStatus {
    #[default]
    Released,
    Demo,
    InDevelopment,
}

impl GameSubmitStatus {
    pub fn as_form_value(&self) -> &'static str {
        match self {
            Self::Released => "Released",
            Self::Demo => "Demo",
            Self::InDevelopment => "In Development",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DocsTopic {
    #[default]
    QuickStart,
    StoryScript,
    ActionBlocks,
    StoryFlow,
    UiDesigner,
    AssetGuidelines,
    RuntimeControls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProject {
    pub name: String,
    pub author: String,
    pub path: PathBuf,
    pub last_opened: chrono::DateTime<chrono::Utc>,
}

fn default_author_value() -> String {
    "Creator".to_string()
}

fn default_resolution_value() -> (u32, u32) {
    (1280, 720)
}

fn default_language_value() -> String {
    "en".to_string()
}

fn default_true() -> bool {
    true
}

fn default_auto_save_interval_minutes() -> u32 {
    3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubConfig {
    pub default_folder: Option<PathBuf>,
    pub recent_projects: Vec<RecentProject>,
    #[serde(default)]
    pub default_export_folder: Option<PathBuf>,
    #[serde(default = "default_author_value")]
    pub default_author: String,
    #[serde(default = "default_resolution_value")]
    pub default_resolution: (u32, u32),
    #[serde(default = "default_language_value")]
    pub default_language_code: String,
    #[serde(default = "default_true")]
    pub auto_save_on_play_preview: bool,
    #[serde(default = "default_true")]
    pub periodic_auto_save: bool,
    #[serde(default = "default_auto_save_interval_minutes")]
    pub auto_save_interval_minutes: u32,
    #[serde(default)]
    pub reopen_last_project: bool,
    #[serde(default = "default_true")]
    pub confirm_before_delete: bool,
    #[serde(default)]
    pub onboarding_completed: bool,
}

impl Default for HubConfig {
    fn default() -> Self {
        let default_folder = std::env::var("USERPROFILE")
            .map(|p| PathBuf::from(p).join("Documents"))
            .ok()
            .filter(|p| p.exists());

        Self {
            default_folder,
            recent_projects: Vec::new(),
            default_export_folder: None,
            default_author: default_author_value(),
            default_resolution: default_resolution_value(),
            default_language_code: default_language_value(),
            auto_save_on_play_preview: true,
            periodic_auto_save: true,
            auto_save_interval_minutes: default_auto_save_interval_minutes(),
            reopen_last_project: false,
            confirm_before_delete: true,
            onboarding_completed: false,
        }
    }
}

impl HubConfig {
    pub fn config_path() -> PathBuf {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        base.join("BlancEngine").join("hub_config.bcfg")
    }

    fn legacy_config_path() -> PathBuf {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        base.join("BlancEngine").join("hub_config.json")
    }

    pub fn load() -> Self {
        for path in [Self::config_path(), Self::legacy_config_path()] {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str(&content) {
                    return cfg;
                }
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, content);
        }
    }

    pub fn add_recent(&mut self, name: &str, author: &str, path: PathBuf) {
        self.recent_projects.retain(|p| p.path != path);
        self.recent_projects.insert(
            0,
            RecentProject {
                name: name.to_string(),
                author: author.to_string(),
                path,
                last_opened: chrono::Utc::now(),
            },
        );
        if self.recent_projects.len() > 25 {
            self.recent_projects.truncate(25);
        }
        self.save();
    }

    pub fn remove_recent(&mut self, path: &PathBuf) {
        self.recent_projects.retain(|p| &p.path != path);
        self.save();
    }
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    ToggleSidebar,
    SelectTab(HubTab),
    SelectDocsTopic(DocsTopic),
    SearchQueryChanged(String),
    ShowCreateModal,
    CancelCreateModal,
    ProjectNameChanged(String),
    AuthorNameChanged(String),
    BrowseProjectLocation,
    CreateProjectConfirmed,
    OpenProjectClicked,
    OpenRecentProject(PathBuf),
    RemoveRecentProject(PathBuf),
    RevealInExplorer(PathBuf),
    ChangeDefaultFolder,
    OpenDefaultFolder,
    ChangeDefaultExportFolder,
    ClearDefaultExportFolder,
    OpenConfigFolder,
    ClearRecentProjects,
    ResetHubPreferences,
    DefaultAuthorChanged(String),
    SetDefaultResolution(u32, u32),
    SetDefaultLanguageCode(String),
    ToggleAutoSaveOnPlayPreview(bool),
    TogglePeriodicAutoSave(bool),
    SetAutoSaveIntervalMinutes(u32),
    ToggleReopenLastProject(bool),
    ToggleConfirmBeforeDelete(bool),
    OnboardingNextStep,
    OnboardingPrevStep,
    OnboardingSkip,
    OnboardingFinishToHub,
    OnboardingFinishCreateProject,
    ReplayOnboarding,
    SubmitTitleChanged(String),
    SubmitAuthorChanged(String),
    SubmitStatusChanged(GameSubmitStatus),
    SubmitGenreChanged(String),
    SubmitCoverUrlChanged(String),
    SubmitGameUrlChanged(String),
    SubmitDescriptionChanged(String),
    SubmitFillFromRecent,
    SubmitResetForm,
    SubmitGameClicked,
    SubmitGameCompleted(Result<(), String>),
}

#[derive(Debug, Clone)]
pub struct WelcomeState {
    pub active_tab: HubTab,
    pub active_docs_topic: DocsTopic,
    pub is_sidebar_expanded: bool,
    pub search_query: String,

    // New project modal state
    pub is_creating_project: bool,
    pub input_name: String,
    pub input_author: String,
    pub selected_folder: Option<PathBuf>,
    pub error_message: Option<String>,

    // Hub config & recents
    pub config: HubConfig,
    pub settings_status_message: Option<String>,

    // First-time onboarding
    pub show_onboarding: bool,
    pub onboarding_step: u8,

    // Submit Your Games tab state
    pub submit_title: String,
    pub submit_author: String,
    pub submit_status: GameSubmitStatus,
    pub submit_genre: String,
    pub submit_cover_url: String,
    pub submit_game_url: String,
    pub submit_description: String,
    pub submit_in_progress: bool,
    pub submit_feedback: Option<(bool, String)>,
}

impl Default for WelcomeState {
    fn default() -> Self {
        let config = HubConfig::load();
        let initial_folder = config.default_folder.clone();
        let initial_author = if config.default_author.trim().is_empty() {
            "Creator".to_string()
        } else {
            config.default_author.clone()
        };
        let show_onboarding = !config.onboarding_completed;

        Self {
            active_tab: HubTab::Projects,
            active_docs_topic: DocsTopic::QuickStart,
            is_sidebar_expanded: true,
            search_query: String::new(),
            is_creating_project: false,
            input_name: "Echoes of Memories".to_string(),
            input_author: initial_author.clone(),
            selected_folder: initial_folder,
            error_message: None,
            config,
            settings_status_message: None,
            show_onboarding,
            onboarding_step: 0,
            submit_title: String::new(),
            submit_author: initial_author,
            submit_status: GameSubmitStatus::Released,
            submit_genre: String::new(),
            submit_cover_url: String::new(),
            submit_game_url: String::new(),
            submit_description: String::new(),
            submit_in_progress: false,
            submit_feedback: None,
        }
    }
}

impl WelcomeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<'_, WelcomeMessage> {
        if self.show_onboarding {
            return self.view_onboarding_screen();
        }

        let sidebar = self.view_sidebar();
        let main_content = match self.active_tab {
            HubTab::Projects => self.view_projects_tab(),
            HubTab::LearnDocs => self.view_learn_docs_tab(),
            HubTab::SubmitGames => self.view_submit_games_tab(),
            HubTab::Settings => self.view_settings_tab(),
        };

        let content_container = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(SPACE_6);

        row![sidebar, content_container]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // ==========================================
    // SIDEBAR (COLLAPSIBLE)
    // ==========================================
    fn view_sidebar(&self) -> Element<'_, WelcomeMessage> {
        if self.is_sidebar_expanded {
            self.view_expanded_sidebar()
        } else {
            self.view_collapsed_sidebar()
        }
    }

    fn view_expanded_sidebar(&self) -> Element<'_, WelcomeMessage> {
        // Brand header with collapse button
        let brand_icon = engine_logo(28.0);

        let brand_title = text("BLANC ENGINE")
            .size(HEADING_SM)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let btn_collapse = button(icon_chevron_left())
            .padding([SPACE_1, SPACE_2])
            .style(icon_button_style)
            .on_press(WelcomeMessage::ToggleSidebar);

        let brand_row = row![
            brand_icon,
            brand_title,
            Space::with_width(Length::Fill),
            btn_collapse,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let badge = container(
            text("v0.1.0")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        )
        .padding([2.0, 8.0])
        .style(badge_container_style);

        let brand_subtitle = text("Visual Novel Creation Hub")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let brand_section = column![
            brand_row,
            row![badge, brand_subtitle].spacing(SPACE_2).align_y(Alignment::Center),
        ]
        .spacing(SPACE_2);

        // Sidebar Navigation Buttons
        let btn_projects = button(
            row![
                icon_folder(),
                text("Projects").size(BODY_MD),
                Space::with_width(Length::Fill),
                text(format!("{}", self.config.recent_projects.len()))
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::Projects))
        .on_press(WelcomeMessage::SelectTab(HubTab::Projects));

        let btn_learn = button(
            row![
                icon_book(),
                text("Learn & Docs").size(BODY_MD),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::LearnDocs))
        .on_press(WelcomeMessage::SelectTab(HubTab::LearnDocs));

        let btn_submit = button(
            row![
                icon_rocket(),
                text("Submit Your Games").size(BODY_MD),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::SubmitGames))
        .on_press(WelcomeMessage::SelectTab(HubTab::SubmitGames));

        let btn_settings = button(
            row![
                icon_settings(),
                text("Settings").size(BODY_MD),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::Settings))
        .on_press(WelcomeMessage::SelectTab(HubTab::Settings));

        let nav_list = column![btn_projects, btn_learn, btn_submit, btn_settings].spacing(SPACE_2);

        // Footer info
        let footer = column![
            text("GNU GPLv3 Open Source")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text("Rust + Iced Engine Core")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_DISABLED.r, TEXT_DISABLED.g, TEXT_DISABLED.b)),
        ]
        .spacing(SPACE_1);

        let sidebar_layout = column![
            brand_section,
            Space::with_height(SPACE_6),
            nav_list,
            Space::with_height(Length::Fill),
            footer,
        ]
        .padding(SPACE_5)
        .width(Length::Fixed(240.0))
        .height(Length::Fill);

        container(sidebar_layout)
            .style(sidebar_container_style)
            .height(Length::Fill)
            .into()
    }

    fn view_collapsed_sidebar(&self) -> Element<'_, WelcomeMessage> {
        let btn_expand = button(icon_menu())
            .padding(SPACE_2)
            .style(icon_button_style)
            .on_press(WelcomeMessage::ToggleSidebar);

        let btn_projects = button(icon_folder())
            .padding(SPACE_3)
            .style(sidebar_button_style(self.active_tab == HubTab::Projects))
            .on_press(WelcomeMessage::SelectTab(HubTab::Projects));

        let btn_learn = button(icon_book())
            .padding(SPACE_3)
            .style(sidebar_button_style(self.active_tab == HubTab::LearnDocs))
            .on_press(WelcomeMessage::SelectTab(HubTab::LearnDocs));

        let btn_submit = button(icon_rocket())
            .padding(SPACE_3)
            .style(sidebar_button_style(self.active_tab == HubTab::SubmitGames))
            .on_press(WelcomeMessage::SelectTab(HubTab::SubmitGames));

        let btn_settings = button(icon_settings())
            .padding(SPACE_3)
            .style(sidebar_button_style(self.active_tab == HubTab::Settings))
            .on_press(WelcomeMessage::SelectTab(HubTab::Settings));

        let nav_icons = column![btn_projects, btn_learn, btn_submit, btn_settings]
            .spacing(SPACE_3)
            .align_x(Alignment::Center);

        let layout = column![
            btn_expand,
            Space::with_height(SPACE_6),
            nav_icons,
            Space::with_height(Length::Fill),
            engine_logo(24.0),
        ]
        .spacing(SPACE_2)
        .align_x(Alignment::Center)
        .padding(SPACE_3)
        .width(Length::Fixed(64.0))
        .height(Length::Fill);

        container(layout)
            .style(sidebar_container_style)
            .height(Length::Fill)
            .into()
    }

    // ==========================================
    // PROJECTS TAB
    // ==========================================
    fn view_projects_tab(&self) -> Element<'_, WelcomeMessage> {
        if self.is_creating_project {
            return self.view_create_project_modal();
        }

        // Top Action Bar
        let title_col = column![
            text("Projects")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Manage and create your visual novel projects")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        let search_input = text_input("Search projects...", &self.search_query)
            .on_input(WelcomeMessage::SearchQueryChanged)
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fixed(260.0))
            .style(input_field_style);

        let btn_open = button(
            row![icon_folder(), text("Open...").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(secondary_button_style)
        .on_press(WelcomeMessage::OpenProjectClicked);

        let btn_new = button(
            row![icon_plus(), text("New Project").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_5])
        .style(primary_button_style)
        .on_press(WelcomeMessage::ShowCreateModal);

        let action_bar = row![
            title_col,
            Space::with_width(Length::Fill),
            search_input,
            btn_open,
            btn_new,
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        // Project List or Empty State
        let query_lower = self.search_query.to_lowercase();
        let filtered_projects: Vec<&RecentProject> = self
            .config
            .recent_projects
            .iter()
            .filter(|p| {
                if query_lower.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&query_lower)
                        || p.author.to_lowercase().contains(&query_lower)
                        || p.path.to_string_lossy().to_lowercase().contains(&query_lower)
                }
            })
            .collect();

        let list_content: Element<'_, WelcomeMessage> = if filtered_projects.is_empty() {
            let empty_icon = icon_gamepad();
            let empty_title = text(if self.search_query.is_empty() {
                "No Visual Novel Projects Found"
            } else {
                "No Matching Projects"
            })
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

            let empty_sub = text(if self.search_query.is_empty() {
                "Get started by creating a new visual novel project or open an existing .blanc manifest."
            } else {
                "Try searching with a different project name or path."
            })
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));

            let mut empty_col = column![empty_icon, empty_title, empty_sub]
                .spacing(SPACE_3)
                .align_x(Alignment::Center);

            if self.search_query.is_empty() {
                let btn_first_create = button(
                    row![icon_plus(), text("Create Your First Project").size(BODY_MD)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_3, SPACE_6])
                .style(primary_button_style)
                .on_press(WelcomeMessage::ShowCreateModal);
                empty_col = empty_col.push(Space::with_height(SPACE_4)).push(btn_first_create);
            }

            container(empty_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let mut list_col = column![].spacing(SPACE_3).width(Length::Fill);

            for project in filtered_projects {
                let card = self.view_project_card(project);
                list_col = list_col.push(card);
            }

            scrollable(list_col).width(Length::Fill).height(Length::Fill).into()
        };

        let mut layout = column![action_bar, Space::with_height(SPACE_5), list_content]
            .spacing(SPACE_2)
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(err) = &self.error_message {
            let error_banner = container(
                text(err)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)),
            )
            .padding(SPACE_3)
            .style(callout_container_style);
            layout = layout.push(error_banner);
        }

        layout.into()
    }

    fn view_project_card<'a>(&'a self, project: &'a RecentProject) -> Element<'a, WelcomeMessage> {
        let icon_box = container(icon_folder())
            .padding(SPACE_3)
            .style(card_container_style);

        let date_str = project
            .last_opened
            .format("%Y-%m-%d %H:%M")
            .to_string();

        let details = column![
            row![
                text(&project.name)
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text(format!("by {}", project.author))
                    .size(LABEL_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
            text(project.path.to_string_lossy().to_string())
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            text(format!("Last opened: {}", date_str))
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
        ]
        .spacing(SPACE_1);

        let btn_open = button(text("Open").size(LABEL_MD))
            .padding([SPACE_2, SPACE_4])
            .style(primary_button_style)
            .on_press(WelcomeMessage::OpenRecentProject(project.path.clone()));

        let parent_dir = project.path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| project.path.clone());
        let btn_reveal = button(
            row![icon_external_link(), text("Explorer").size(LABEL_SM)]
                .spacing(SPACE_1)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .style(subtle_button_style)
        .on_press(WelcomeMessage::RevealInExplorer(parent_dir));

        let btn_remove = button(icon_close())
            .padding([SPACE_2, SPACE_3])
            .style(subtle_button_style)
            .on_press(WelcomeMessage::RemoveRecentProject(project.path.clone()));

        let actions = row![btn_open, btn_reveal, btn_remove]
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
    // CREATE PROJECT MODAL
    // ==========================================
    fn view_create_project_modal(&self) -> Element<'_, WelcomeMessage> {
        let title_row = row![
            column![
                text("Create New Visual Novel Project")
                    .size(HEADING_LG)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Configure project details and directory structure")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_1),
            Space::with_width(Length::Fill),
            button(icon_close())
                .padding(SPACE_2)
                .style(icon_button_style)
                .on_press(WelcomeMessage::CancelCreateModal),
        ]
        .align_y(Alignment::Center);

        // Name field
        let name_label = text("Project Title").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let name_input = text_input("e.g. Echoes of Memories", &self.input_name)
            .on_input(WelcomeMessage::ProjectNameChanged)
            .padding(SPACE_2)
            .style(input_field_style);

        // Author field
        let author_label = text("Author / Studio").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let author_input = text_input("e.g. Studio Blanc", &self.input_author)
            .on_input(WelcomeMessage::AuthorNameChanged)
            .padding(SPACE_2)
            .style(input_field_style);

        // Folder selection
        let folder_label = text("Project Parent Directory").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let folder_text = self
            .selected_folder
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Select directory for project...".to_string());

        let btn_browse = button(text("Browse...").size(LABEL_MD))
            .padding([SPACE_2, SPACE_4])
            .style(secondary_button_style)
            .on_press(WelcomeMessage::BrowseProjectLocation);

        let folder_row = row![
            container(
                text(folder_text)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
            )
            .width(Length::Fill)
            .padding(SPACE_2)
            .style(card_container_style),
            btn_browse
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        // Calculated Full Path Preview
        let sanitized_folder = self.input_name.trim().replace(' ', "_");
        let preview_path = if let Some(base) = &self.selected_folder {
            if sanitized_folder.is_empty() {
                base.display().to_string()
            } else {
                let full = base.join(&sanitized_folder).join(format!("{}.{}", sanitized_folder, Project::MANIFEST_EXTENSION));
                full.display().to_string()
            }
        } else {
            "No folder selected yet".to_string()
        };

        let preview_box = container(
            column![
                text("Project Target File Preview:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text(preview_path).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        // Preset badge summary
        let specs_info = text("Specifications: 1920×1080 (16:9 Full HD) • Dual-process Architecture • GPLv3 Engine")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let mut form = column![
            title_row,
            Space::with_height(SPACE_3),
            name_label,
            name_input,
            author_label,
            author_input,
            folder_label,
            folder_row,
            Space::with_height(SPACE_2),
            preview_box,
            specs_info,
        ]
        .spacing(SPACE_3)
        .width(Length::Fixed(580.0));

        if let Some(err) = &self.error_message {
            form = form.push(
                text(err)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)),
            );
        }

        let btn_cancel = button(text("Cancel").size(BODY_SM))
            .padding([SPACE_2, SPACE_5])
            .style(secondary_button_style)
            .on_press(WelcomeMessage::CancelCreateModal);

        let btn_submit = button(text("Create & Open Project").size(BODY_SM))
            .padding([SPACE_2, SPACE_5])
            .style(primary_button_style)
            .on_press(WelcomeMessage::CreateProjectConfirmed);

        let actions = row![Space::with_width(Length::Fill), btn_cancel, btn_submit]
            .spacing(SPACE_3)
            .align_y(Alignment::Center);

        form = form.push(Space::with_height(SPACE_4)).push(actions);

        let modal_container = container(form)
            .padding(SPACE_6)
            .style(card_container_style);

        container(modal_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    // ==========================================
    // LEARN & DOCS TAB (ALL IN ENGLISH)
    // ==========================================
    fn view_learn_docs_tab(&self) -> Element<'_, WelcomeMessage> {
        let header = column![
            text("Learn & Documentation")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Complete guide and reference for Visual Novel development in Blanc Engine")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        // Subtopics navigation
        let topic_quick = button(
            row![icon_rocket(), text("Quick Start Guide").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::QuickStart))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::QuickStart));

        let topic_script = button(
            row![icon_edit(), text("Story Script Format").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::StoryScript))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::StoryScript));

        let topic_blocks = button(
            row![icon_blocks(), text("Action Block System").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::ActionBlocks))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::ActionBlocks));

        let topic_flow = button(
            row![icon_branch(), text("Story Flow & Nodes").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::StoryFlow))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::StoryFlow));

        let topic_ui = button(
            row![icon_palette(), text("UI Designer & Packs").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::UiDesigner))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::UiDesigner));

        let topic_assets = button(
            row![icon_palette(), text("Asset Guidelines").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::AssetGuidelines))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::AssetGuidelines));

        let topic_controls = button(
            row![icon_keyboard(), text("Runtime Controls").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_3])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_docs_topic == DocsTopic::RuntimeControls))
        .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::RuntimeControls));

        let topics_menu = column![
            text("DOCUMENTATION TOPICS").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            topic_quick,
            topic_script,
            topic_blocks,
            topic_flow,
            topic_ui,
            topic_assets,
            topic_controls,
        ]
        .spacing(SPACE_2)
        .width(Length::Fixed(240.0));

        let content_body = match self.active_docs_topic {
            DocsTopic::QuickStart => self.doc_quick_start(),
            DocsTopic::StoryScript => self.doc_story_script(),
            DocsTopic::ActionBlocks => self.doc_action_blocks(),
            DocsTopic::StoryFlow => self.doc_story_flow(),
            DocsTopic::UiDesigner => self.doc_ui_designer(),
            DocsTopic::AssetGuidelines => self.doc_asset_guidelines(),
            DocsTopic::RuntimeControls => self.doc_runtime_controls(),
        };

        let docs_viewer = container(scrollable(content_body).width(Length::Fill).height(Length::Fill))
            .padding(SPACE_5)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(card_container_style);

        let split_view = row![topics_menu, Space::with_width(SPACE_4), docs_viewer]
            .width(Length::Fill)
            .height(Length::Fill);

        column![header, Space::with_height(SPACE_4), split_view]
            .spacing(SPACE_2)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn doc_quick_start(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("Quick Start Guide: Create a Visual Novel in 5 Minutes")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Welcome to Blanc Engine! This guide walks you through setting up and running your first visual novel project.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "Step 1: Create a New Project",
                "Navigate to the 'Projects' tab in the left sidebar and click '+ New Project'. Enter your project title and author name. The engine automatically creates standard directories and initializes the project manifest."
            ),

            self.create_step_card(
                "Step 2: Write Story & Dialogue",
                "In the Editor Workspace, switch to the 'Story Script' tab. You can type screenplay-style scripts naturally:\n\nAlice (Happy): \"Hello! Welcome to Blanc Engine!\"\nBob: \"Writing visual novels has never been easier!\""
            ),

            self.create_step_card(
                "Step 3: Organize Assets & Characters",
                "Switch to the 'Asset Registry' tab or simply place your images and audio files inside your project's 'Assets/Backgrounds/', 'Assets/Characters/', and 'Assets/BGM/' folders."
            ),

            self.create_step_card(
                "Step 4: Launch Instant Play Preview",
                "Click the green 'Play Preview' button in the upper-right corner of the editor. The standalone runtime (blanc_runtime.exe) launches instantly as a smooth child process without recompilation delay!"
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_story_script(&self) -> Element<'_, WelcomeMessage> {
        let code_sample = r#"// Blanc Engine Story Script Sample (.txt / .md / .fountain)
[Scene: Classroom]
[BGM: peaceful_afternoon]

Alice (Happy): "Good morning! Have you had breakfast yet?"
Bob (Thinking): "Hmm... I only had a cup of coffee so far."

Alice: "In that case, let's head to the cafeteria together!"

[Choice:
  - "Go to the cafeteria" -> Scene_Cafeteria
  - "Politely decline" -> Scene_Study
]"#;

        column![
            text("Story Script Format & Dialogue Syntax")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Blanc Engine supports natural screenplay syntax. Each part of your script is parsed into visual action blocks when you click 'Convert to Scenes' in Story Setup.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_2),

            self.create_step_card(
                "1. Scene Separators  —  [Scene: Name]  |  # Name  |  ## Name  |  INT. / EXT.",
                "Splits your script into separate .bscene files. Examples:\n  [Scene: 01_Classroom]\n  ## Cafe Arrival\n  INT. SCHOOL ROOFTOP - DAY"
            ),
            self.create_step_card(
                "2. Dialogue & Emotion Tags  —  Speaker (Emotion): \"Text\"",
                "Creates a Dialogue block (quotes and emotion tags are optional). Examples:\n  Rowan (Happy): \"Let's head home together!\"\n  Mina: Wait, look at this notebook."
            ),
            self.create_step_card(
                "3. Narration  —  Plain Paragraph",
                "Any normal line without a colon (:) becomes a Narration block automatically. Example:\n  The afternoon bell rang across the empty hallway."
            ),
            self.create_step_card(
                "4. Audio Directives  —  [BGM: track]  |  [SFX: sound]",
                "Triggers background music or sound effects at that exact moment. Examples:\n  [BGM: School_Afternoon]\n  [SFX: Door_Slide]"
            ),
            self.create_step_card(
                "5. Interactive Choices  —  [Choice: Prompt] ... - \"Option\" -> TargetScene",
                "Presents branching choices to the player. Example:\n  [Choice: Where should we go?]\n  - \"Go to the rooftop\" -> Scene_2A_School_Rooftop\n  - \"Visit the cafe\" -> Scene_2B_Town_Cafe\n  ]"
            ),
            self.create_step_card(
                "6. Writer Comments  —  // Note",
                "Lines starting with // are ignored by the parser so you can keep personal notes inside your script."
            ),

            Space::with_height(SPACE_2),
            text("Complete Script Example:").size(HEADING_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            container(
                text(code_sample).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b))
            )
            .padding(SPACE_4)
            .width(Length::Fill)
            .style(code_container_style),

            Space::with_height(SPACE_2),
            text("External Script Import:")
                .size(LABEL_MD)
                .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
            text("You can draft your story scripts in any editor (Notepad, Obsidian, VS Code, or Fountain software) and import .txt, .md, or .fountain files into Story Setup — or skip Story Setup completely and build scenes block-by-block in Scene Editor.")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_3)
        .into()
    }

    fn doc_action_blocks(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("Modular No-Code Action Block System")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Every scene in Blanc Engine is composed of modular action blocks arranged sequentially. Build dynamic game mechanics with zero programming required.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "1. Dialogue Block",
                "Displays the dialogue box, speaker portrait, character name, emotion expression, typewriter text speed, and optional voice acting audio trigger."
            ),
            self.create_step_card(
                "2. Choice & Branching Block",
                "Presents interactive decision buttons on screen. Each choice can jump to a target scene, evaluate variables, and update story flags."
            ),
            self.create_step_card(
                "3. Audio & Music Block",
                "Controls background music (BGM Play/Stop/Fade-In/Fade-Out) and sound effects (SFX Play/Volume). Supports seamless looping playback."
            ),
            self.create_step_card(
                "4. Character & Camera Block",
                "Manages character sprite placement across presets (Left, Center, Right), appearance animations, and scene background transitions (Fade to Black, Cross Dissolve)."
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_story_flow(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("Story Flow & Visual Narrative Graph")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Orchestrate the entire narrative journey, branching choices, and game system screens with an intuitive node-based graph.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "1. Visual Narrative Mapping",
                "Story Flow provides a high-level bird's-eye view of your game's narrative structure. Instead of keeping complex branch logic in your head or in scripts, nodes and connecting wires visually define scene progression."
            ),
            self.create_step_card(
                "2. Dynamic Choice Branching Pins",
                "When a scene contains interactive Choice action blocks, the node dynamically reveals distinct colored output pins for each option. Connect each choice pin to a different branch scene to guide player outcomes."
            ),
            self.create_step_card(
                "3. System Nodes Library",
                "Standard game systems are ready to use from the sidebar:\n• Main Menu (Title): Root entry point and loop destination\n• In-Game Pause Menu: ESC menu for saving, options, or returning to title\n• In-Game Settings: Live player audio/video configuration\n• Save & Load Screen: Multi-slot player state persistence\n• Return to Title & Game Over: Dedicated arc conclusion screens"
            ),
            self.create_step_card(
                "4. Canvas Toolbar & Interaction Modes",
                "Use the docked canvas toolbar or fast keyboard shortcuts to switch modes:\n• Select & Move [V]: Pan canvas, drag and arrange node positions\n• Connect Wire [C]: Click source pin then target pin, or drag between pins\n• Cut Wire [X]: One-click removal of wire connections or pin links\n• Zoom Controls: Zoom In (+), Zoom Out (-), Reset View (0), Focus All (F)\n• Auto Align [A]: Automatically organize nodes into clean chronological columns\n• Grid Snap [S]: Toggle snap-to-grid for crisp alignment"
            ),
            self.create_step_card(
                "5. Narrative Loops & Return to Title",
                "Storylines can easily loop back. Connect the 'To Title Screen ->' output pin of Return to Title or any arc ending directly into the Main Menu's 'In / Loop' pin so players can replay your game smoothly."
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_ui_designer(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("UI Designer & Custom UI Packs")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Customize every runtime screen—from Main Menu and Dialogue HUD to Pause, Settings, and Save/Load—with live preview.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "1. Global Theme & Custom Fonts",
                "In Theme & Buttons, set your Accent color, Panel background, Panel opacity, Corner radius, Button style (Glass Dark, Solid Accent, Outlined), and optional TTF/OTF Custom Font. Default text and button colors defined here act as the fallback for all screens."
            ),
            self.create_step_card(
                "2. Per-Screen Font Color Overrides",
                "Main Menu, Dialogue Box, Speaker Nameplate, Pause Title, Settings, and Save/Load Title & Slot Cards each have their own Text Color override. Click 'Global' (or leave empty) to inherit Theme.text_hex, or pick/type any #RRGGBB hex color."
            ),
            self.create_step_card(
                "3. Panel-less Layouts & Quick Presets",
                "Main Menu, Pause, Settings, and Save/Load include one-click Quick Layout Presets plus independent 9-point Anchors and X/Y Offsets for titles, button groups/slots, and Back buttons. Toggle 'Panel Box: OFF' for clean floating text and buttons over the dimmed backdrop."
            ),
            self.create_step_card(
                "4. Gameplay HUD, Nameplate & Advance Indicator",
                "Position the Dialogue Box, Speaker Nameplate, and Choice buttons, and place the quick-action HUD (History, Hide UI, Menu) at Top-Left, Top-Right, Bottom-Left, Bottom-Center, or Bottom-Right. You can also assign a custom PNG icon and prompt text for the Advance Indicator."
            ),
            self.create_step_card(
                "5. Save & Load Thumbnails",
                "Choose 2-Column, 3-Column, or List slot layouts. When players save during gameplay, each slot automatically records a thumbnail preview of the active scene background and characters."
            ),
            self.create_step_card(
                "6. Batch Import UI Pack (Custom PNGs)",
                "Use 'Import UI Pack...' in the UI Designer to auto-map a folder of PNGs:\n• btn_normal.png, btn_hover.png, btn_disabled.png\n• dialogue_box.png, nameplate.png, advance_prompt.png\n• pause_panel.png, settings_panel.png, save_load_panel.png\n• slot_card.png, slot_card_hover.png, slot_card_disabled.png\nTip: Folders named with 'cream' or 'light' automatically apply dark-ink contrast for dialogue, buttons, and save slots."
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_asset_guidelines(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("Visual & Audio Asset Guidelines")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("To ensure crisp presentation and fast loading times across different display aspect ratios, follow these recommended asset standards:")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "Default Canvas: 1920 × 1080 (16:9 Full HD)",
                "All backgrounds are recommended at 1920x1080 px. The runtime player automatically handles letterboxing and aspect-ratio scaling on wider or taller screens."
            ),
            self.create_step_card(
                "Image Formats: PNG & WebP",
                "Use transparent PNG for Character Sprites and UI elements. WebP is highly recommended for Backgrounds to minimize file size while preserving high visual fidelity."
            ),
            self.create_step_card(
                "Audio Formats: OGG, WAV, MP3",
                "OGG is recommended for BGM tracks due to low memory footprint and seamless looping support. WAV is best for snappy, responsive sound effects."
            ),
            self.create_step_card(
                "Project Folder Structure",
                "Assets are organized cleanly within your project directory:\n• Assets/Backgrounds/\n• Assets/Characters/\n• Assets/BGM/\n• Assets/SFX/\n• Assets/Fonts/"
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_runtime_controls(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("Runtime Controls & Keyboard Shortcuts")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Built-in controls for playing visual novels in the standalone Blanc Runtime:")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "Space / Enter / Left Click",
                "Advances dialogue to the next line or confirms an on-screen interactive choice."
            ),
            self.create_step_card(
                "Ctrl (Hold)",
                "Fast-forward / Skip mode to quickly advance through dialogue that has already been read."
            ),
            self.create_step_card(
                "B (History / Backlog)",
                "Opens the dialogue history log so players can review previous lines and speaker names."
            ),
            self.create_step_card(
                "H (Hide UI)",
                "Toggles dialogue box visibility to view full-screen background artwork and character illustrations."
            ),
            self.create_step_card(
                "Escape (Esc)",
                "Opens the In-Game Pause Menu for Saving, Loading, Volume Settings, or returning to Title Screen."
            ),
            self.create_step_card(
                "F5 (Quick Save) & F8 (Quick Load)",
                "Instantly saves current progress to the quick save slot, and reloads it anytime."
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn create_step_card<'a>(&'a self, title: &'a str, desc: &'a str) -> Element<'a, WelcomeMessage> {
        container(
            column![
                text(title)
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                text(desc)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_4)
        .width(Length::Fill)
        .style(card_container_style)
        .into()
    }

    // ==========================================
    // SETTINGS TAB (ALL IN ENGLISH)
    // ==========================================
    fn view_settings_tab(&self) -> Element<'_, WelcomeMessage> {
        let title_col = column![
            text("Engine Settings & Preferences")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Configure global directories, new project defaults, editor workflow, and engine specifications.")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        // Section 1: Storage & Directories
        let current_dir_text = self
            .config
            .default_folder
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Not configured (User Documents default)".to_string());

        let current_export_text = self
            .config
            .default_export_folder
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Not configured (Ask per project / project Exports folder)".to_string());

        let storage_card = container(
            column![
                text("Storage & Directories")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Default Project Directory")
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                row![
                    container(
                        text(current_dir_text)
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                    )
                    .padding(SPACE_2)
                    .width(Length::Fill)
                    .style(code_container_style),
                    button(text("Change...").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::ChangeDefaultFolder),
                    button(text("Open").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(subtle_button_style)
                        .on_press(WelcomeMessage::OpenDefaultFolder),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                Space::with_height(SPACE_1),
                text("Default Export Output Directory")
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                row![
                    container(
                        text(current_export_text)
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                    )
                    .padding(SPACE_2)
                    .width(Length::Fill)
                    .style(code_container_style),
                    button(text("Change...").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::ChangeDefaultExportFolder),
                    button(text("Clear").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_3])
                        .style(subtle_button_style)
                        .on_press(WelcomeMessage::ClearDefaultExportFolder),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
                Space::with_height(SPACE_1),
                row![
                    text(format!("Global Config: {}", HubConfig::config_path().display()))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                        .width(Length::Fill),
                    button(text("Open Config Folder").size(LABEL_SM))
                        .padding([SPACE_1, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::OpenConfigFolder),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // Section 2: New Project Defaults
        let res = self.config.default_resolution;
        let lang = self.config.default_language_code.as_str();

        let defaults_card = container(
            column![
                text("New Project Defaults")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Pre-fill these values automatically whenever you create a new visual novel project.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_1),
                row![
                    text("Default Author / Studio")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b))
                        .width(Length::Fixed(190.0)),
                    text_input("Creator / Studio Name", &self.config.default_author)
                        .on_input(WelcomeMessage::DefaultAuthorChanged)
                        .size(BODY_SM)
                        .padding(SPACE_2)
                        .width(Length::Fixed(280.0))
                        .style(input_field_style),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
                row![
                    text("Default Resolution")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b))
                        .width(Length::Fixed(190.0)),
                    row![
                        button(text("1280 × 720 (HD)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1280, 720) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1280, 720)),
                        button(text("1600 × 900").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1600, 900) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1600, 900)),
                        button(text("1920 × 1080 (FHD)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1920, 1080) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1920, 1080)),
                    ]
                    .spacing(SPACE_2),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
                row![
                    text("Default Source Language")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b))
                        .width(Length::Fixed(190.0)),
                    row![
                        button(text("English (en)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("en") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("en".to_string())),
                        button(text("Indonesian (id)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("id") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("id".to_string())),
                        button(text("Japanese (ja)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("ja") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("ja".to_string())),
                    ]
                    .spacing(SPACE_2),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // Section 3: Editor Workflow & Maintenance
        let toggle_row = |label: &'static str, desc: &'static str, value: bool, on_yes: WelcomeMessage, on_no: WelcomeMessage| {
            row![
                column![
                    text(label)
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text(desc)
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                ]
                .spacing(2)
                .width(Length::Fill),
                row![
                    button(text("ON").size(LABEL_SM))
                        .padding([4, 12])
                        .style(if value { primary_button_style } else { secondary_button_style })
                        .on_press(on_yes),
                    button(text("OFF").size(LABEL_SM))
                        .padding([4, 12])
                        .style(if !value { primary_button_style } else { secondary_button_style })
                        .on_press(on_no),
                ]
                .spacing(4),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center)
            .width(Length::Fill)
        };

        let workflow_card = container(
            column![
                text("Editor Workflow & Maintenance")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                Space::with_height(SPACE_1),
                toggle_row(
                    "Auto-Save Scene Before Play Preview",
                    "Automatically save the active scene before launching blanc_runtime.exe.",
                    self.config.auto_save_on_play_preview,
                    WelcomeMessage::ToggleAutoSaveOnPlayPreview(true),
                    WelcomeMessage::ToggleAutoSaveOnPlayPreview(false),
                ),
                toggle_row(
                    "Periodic Auto-Save (Inside Project)",
                    "Automatically save unsaved project changes on a timer while editing.",
                    self.config.periodic_auto_save,
                    WelcomeMessage::TogglePeriodicAutoSave(true),
                    WelcomeMessage::TogglePeriodicAutoSave(false),
                ),
                row![
                    column![
                        text("Auto-Save Interval")
                            .size(LABEL_MD)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        text(if self.config.periodic_auto_save {
                            "How often unsaved changes are saved automatically."
                        } else {
                            "Periodic auto-save is currently disabled."
                        })
                            .size(LABEL_SM)
                            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    ]
                    .spacing(2.0)
                    .width(Length::Fill),
                    [1u32, 3, 5, 10]
                        .into_iter()
                        .fold(row![].spacing(SPACE_2), |acc, mins| {
                            let active = self.config.auto_save_interval_minutes == mins;
                            let btn = button(text(format!("{} min", mins)).size(LABEL_SM))
                                .padding([SPACE_1, SPACE_3])
                                .style(if active { primary_button_style } else { secondary_button_style });
                            let btn = if self.config.periodic_auto_save {
                                btn.on_press(WelcomeMessage::SetAutoSaveIntervalMinutes(mins))
                            } else {
                                btn
                            };
                            acc.push(btn)
                        }),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
                toggle_row(
                    "Reopen Last Project on Startup",
                    "Skip the Project Hub and open your most recent project automatically when launching.",
                    self.config.reopen_last_project,
                    WelcomeMessage::ToggleReopenLastProject(true),
                    WelcomeMessage::ToggleReopenLastProject(false),
                ),
                toggle_row(
                    "Confirm Before Delete",
                    "Keep safety prompts enabled when removing scenes, assets, or languages.",
                    self.config.confirm_before_delete,
                    WelcomeMessage::ToggleConfirmBeforeDelete(true),
                    WelcomeMessage::ToggleConfirmBeforeDelete(false),
                ),
                Space::with_height(SPACE_2),
                row![
                    text(format!("Recent projects stored: {}", self.config.recent_projects.len()))
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                        .width(Length::Fill),
                    button(text("Replay Onboarding").size(LABEL_SM))
                        .padding([SPACE_1, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::ReplayOnboarding),
                    button(text("Clear Recent Projects").size(LABEL_SM))
                        .padding([SPACE_1, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::ClearRecentProjects),
                    button(text("Reset Preferences").size(LABEL_SM))
                        .padding([SPACE_1, SPACE_3])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::ResetHubPreferences),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // Section 4: Engine Specifications & License
        let info_card = container(
            column![
                text("Engine Specifications & Licensing")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                Space::with_height(SPACE_2),
                row![
                    text("Engine Version:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(180.0)),
                    text("v0.1.0").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Engine License:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(180.0)),
                    text("GNU General Public License v3 (GPLv3)").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Created Games Ownership:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(180.0)),
                    text("100% Creator-Owned (Commercial / Free to distribute)").size(BODY_SM).color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                ].spacing(SPACE_2),
                row![
                    text("Technology Stack:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(180.0)),
                    text("Rust 2021, Iced GUI, wgpu hardware acceleration").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Runtime Architecture:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(180.0)),
                    text("Dual-process (Child Process blanc_runtime.exe)").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let mut content = column![title_col, Space::with_height(SPACE_2)].spacing(SPACE_4).width(Length::Fill);
        if let Some(status) = &self.settings_status_message {
            content = content.push(
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
        content = content
            .push(storage_card)
            .push(defaults_card)
            .push(workflow_card)
            .push(info_card);

        scrollable(content).width(Length::Fill).height(Length::Fill).into()
    }

    // ==========================================
    // FULL-PAGE FIRST-TIME ONBOARDING
    // ==========================================
    fn view_onboarding_screen(&self) -> Element<'_, WelcomeMessage> {
        let step = self.onboarding_step.min(2);

        let step_pill = |idx: u8, label: &'static str| {
            let is_active = step == idx;
            let is_done = step > idx;
            let badge_color = if is_active {
                iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)
            } else if is_done {
                iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
            } else {
                iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)
            };

            container(
                text(label).size(LABEL_SM).color(badge_color),
            )
            .padding([SPACE_1, SPACE_3])
            .style(if is_active { badge_container_style } else { code_container_style })
        };

        let top_bar = row![
            row![
                engine_logo(28.0),
                text("BLANC ENGINE")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                container(
                    text("v0.1.0")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                )
                .padding([2.0, 8.0])
                .style(badge_container_style),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
            Space::with_width(Length::Fill),
            row![
                step_pill(0, "1. Welcome"),
                step_pill(1, "2. Workspace"),
                step_pill(2, "3. Workflow"),
            ]
            .spacing(SPACE_2)
            .align_y(Alignment::Center),
            Space::with_width(Length::Fill),
            button(text("Skip Setup").size(LABEL_SM))
                .padding([SPACE_2, SPACE_3])
                .style(subtle_button_style)
                .on_press(WelcomeMessage::OnboardingSkip),
        ]
        .padding([SPACE_4, SPACE_6])
        .align_y(Alignment::Center)
        .width(Length::Fill);

        let feature_card = |title: &'static str, desc: &'static str| {
            container(
                column![
                    text(title)
                        .size(HEADING_SM)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text(desc)
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                ]
                .spacing(SPACE_1),
            )
            .padding(SPACE_4)
            .width(Length::FillPortion(1))
            .style(code_container_style)
        };

        let body_card: Element<'_, WelcomeMessage> = match step {
            0 => {
                let hero = column![
                    text("Welcome to Blanc Engine")
                        .size(28)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("Build, customize, localize, and export complete Visual Novels in one cohesive desktop studio.")
                        .size(BODY_MD)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                ]
                .spacing(SPACE_2);

                let grid_top = row![
                    feature_card(
                        "Visual Novel Workflow",
                        "Write dialogue in Scene Editor and connect branching paths visually in Story Flow."
                    ),
                    feature_card(
                        "UI Designer & 19 Templates",
                        "Pick ready-made layouts for Pause, Settings, Save/Load, and History, or skin everything with custom PNGs."
                    ),
                ]
                .spacing(SPACE_3);

                let grid_bottom = row![
                    feature_card(
                        "Multi-Language & Voice Acting",
                        "Translate UI, speakers, and scripts with CSV round-trip, plus Voice Acting & Active Speaker Highlight."
                    ),
                    feature_card(
                        "100% Creator-Owned",
                        "Export standalone builds and installers. Everything you create is yours to publish commercially or for free."
                    ),
                ]
                .spacing(SPACE_3);

                let footer = row![
                    text("Step 1 of 3")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    Space::with_width(Length::Fill),
                    button(text("Get Started ->").size(LABEL_MD))
                        .padding([SPACE_2, SPACE_5])
                        .style(primary_button_style)
                        .on_press(WelcomeMessage::OnboardingNextStep),
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill);

                container(
                    column![hero, Space::with_height(SPACE_2), grid_top, grid_bottom, Space::with_height(SPACE_2), footer]
                        .spacing(SPACE_3),
                )
                .padding(SPACE_6)
                .width(Length::Fixed(860.0))
                .style(card_container_style)
                .into()
            }
            1 => {
                let current_dir_text = self
                    .config
                    .default_folder
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Not configured (User Documents default)".to_string());
                let res = self.config.default_resolution;
                let lang = self.config.default_language_code.as_str();

                let header = column![
                    text("Quick Workspace Setup")
                        .size(26)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("Set your default project folder and starter preferences. You can change these anytime in Hub Settings.")
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                ]
                .spacing(SPACE_1);

                let folder_row = column![
                    text("Default Project Directory")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    row![
                        container(
                            text(current_dir_text)
                                .size(BODY_SM)
                                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                        )
                        .padding(SPACE_2)
                        .width(Length::Fill)
                        .style(code_container_style),
                        button(text("Change Folder...").size(LABEL_SM))
                            .padding([SPACE_2, SPACE_3])
                            .style(secondary_button_style)
                            .on_press(WelcomeMessage::ChangeDefaultFolder),
                    ]
                    .spacing(SPACE_2)
                    .align_y(Alignment::Center),
                ]
                .spacing(SPACE_1);

                let author_row = row![
                    text("Creator / Studio Name")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                        .width(Length::Fixed(200.0)),
                    text_input("Creator / Studio Name", &self.config.default_author)
                        .on_input(WelcomeMessage::DefaultAuthorChanged)
                        .size(BODY_SM)
                        .padding(SPACE_2)
                        .width(Length::Fill)
                        .style(input_field_style),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center);

                let res_row = row![
                    text("Default Game Resolution")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                        .width(Length::Fixed(200.0)),
                    row![
                        button(text("1280 x 720 (HD)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1280, 720) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1280, 720)),
                        button(text("1600 x 900").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1600, 900) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1600, 900)),
                        button(text("1920 x 1080 (FHD)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if res == (1920, 1080) { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultResolution(1920, 1080)),
                    ]
                    .spacing(SPACE_2),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center);

                let lang_row = row![
                    text("Default Source Language")
                        .size(LABEL_MD)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                        .width(Length::Fixed(200.0)),
                    row![
                        button(text("English (en)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("en") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("en".to_string())),
                        button(text("Indonesian (id)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("id") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("id".to_string())),
                        button(text("Japanese (ja)").size(LABEL_SM))
                            .padding([SPACE_1, SPACE_3])
                            .style(if lang.eq_ignore_ascii_case("ja") { primary_button_style } else { secondary_button_style })
                            .on_press(WelcomeMessage::SetDefaultLanguageCode("ja".to_string())),
                    ]
                    .spacing(SPACE_2),
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center);

                let footer = row![
                    button(text("<- Back").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::OnboardingPrevStep),
                    Space::with_width(Length::Fill),
                    text("Step 2 of 3")
                        .size(LABEL_SM)
                        .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                    Space::with_width(Length::Fixed(SPACE_3)),
                    button(text("Continue ->").size(LABEL_MD))
                        .padding([SPACE_2, SPACE_5])
                        .style(primary_button_style)
                        .on_press(WelcomeMessage::OnboardingNextStep),
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill);

                container(
                    column![
                        header,
                        Space::with_height(SPACE_1),
                        folder_row,
                        author_row,
                        res_row,
                        lang_row,
                        Space::with_height(SPACE_2),
                        footer,
                    ]
                    .spacing(SPACE_3),
                )
                .padding(SPACE_6)
                .width(Length::Fixed(860.0))
                .style(card_container_style)
                .into()
            }
            _ => {
                let header = column![
                    text("How Blanc Engine Works")
                        .size(26)
                        .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                    text("Follow these 4 stages to take your visual novel from idea to playable release.")
                        .size(BODY_SM)
                        .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                ]
                .spacing(SPACE_1);

                let row_1 = row![
                    feature_card(
                        "1. Import Assets",
                        "Drop in backgrounds, character sprites, BGM, SFX, and voice acting clips inside Asset Manager."
                    ),
                    feature_card(
                        "2. Build Story & Scenes",
                        "Author dialogue, expressions, choices, and branching paths in Scene Editor and Story Flow."
                    ),
                ]
                .spacing(SPACE_3);

                let row_2 = row![
                    feature_card(
                        "3. Design UI & Locales",
                        "Customize Title, Settings, Save/Load, and Gameplay UI with templates, PNG skins, and Localization."
                    ),
                    feature_card(
                        "4. Play Preview & Export",
                        "Test your game live with keyboard/gamepad support, then package a standalone build & installer."
                    ),
                ]
                .spacing(SPACE_3);

                let footer = row![
                    button(text("<- Back").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::OnboardingPrevStep),
                    Space::with_width(Length::Fill),
                    button(text("Go to Project Hub").size(LABEL_SM))
                        .padding([SPACE_2, SPACE_4])
                        .style(secondary_button_style)
                        .on_press(WelcomeMessage::OnboardingFinishToHub),
                    button(text("Create First Project ->").size(LABEL_MD))
                        .padding([SPACE_2, SPACE_5])
                        .style(primary_button_style)
                        .on_press(WelcomeMessage::OnboardingFinishCreateProject),
                ]
                .spacing(SPACE_2)
                .align_y(Alignment::Center)
                .width(Length::Fill);

                container(
                    column![header, Space::with_height(SPACE_2), row_1, row_2, Space::with_height(SPACE_2), footer]
                        .spacing(SPACE_3),
                )
                .padding(SPACE_6)
                .width(Length::Fixed(860.0))
                .style(card_container_style)
                .into()
            }
        };

        let center_area = container(body_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(SPACE_6);

        column![top_bar, center_area]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // ==========================================
    // SUBMIT YOUR GAMES TAB (MIRRORS WEBSITE FORM)
    // ==========================================
    fn view_submit_games_tab(&self) -> Element<'_, WelcomeMessage> {
        let mut header_row = row![
            column![
                text("Submit Your Games")
                    .size(HEADING_LG)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Submit your visual novel made with Blanc Engine to be featured in the official community showcase.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_1)
            .width(Length::Fill),
        ]
        .spacing(SPACE_3)
        .align_y(Alignment::Center);

        if !self.config.recent_projects.is_empty() {
            header_row = header_row.push(
                button(text("Auto-fill from Latest Project").size(LABEL_SM))
                    .padding([SPACE_2, SPACE_3])
                    .style(secondary_button_style)
                    .on_press(WelcomeMessage::SubmitFillFromRecent),
            );
        }

        let field_label = |label: &'static str| {
            row![
                text(label)
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text(" *")
                    .size(LABEL_MD)
                    .color(iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)),
            ]
        };

        let row_title_author = row![
            column![
                field_label("Game Title"),
                text_input("Echoes of Memories", &self.submit_title)
                    .on_input(WelcomeMessage::SubmitTitleChanged)
                    .size(BODY_SM)
                    .padding(SPACE_2)
                    .width(Length::Fill)
                    .style(input_field_style),
            ]
            .spacing(SPACE_1)
            .width(Length::FillPortion(1)),
            column![
                field_label("Author / Studio"),
                text_input("Your Name or Studio", &self.submit_author)
                    .on_input(WelcomeMessage::SubmitAuthorChanged)
                    .size(BODY_SM)
                    .padding(SPACE_2)
                    .width(Length::Fill)
                    .style(input_field_style),
            ]
            .spacing(SPACE_1)
            .width(Length::FillPortion(1)),
        ]
        .spacing(SPACE_4);

        let status_pills = row![
            button(text("Released").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(if self.submit_status == GameSubmitStatus::Released { primary_button_style } else { secondary_button_style })
                .on_press(WelcomeMessage::SubmitStatusChanged(GameSubmitStatus::Released)),
            button(text("Demo").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(if self.submit_status == GameSubmitStatus::Demo { primary_button_style } else { secondary_button_style })
                .on_press(WelcomeMessage::SubmitStatusChanged(GameSubmitStatus::Demo)),
            button(text("In Development").size(LABEL_SM))
                .padding([SPACE_1, SPACE_3])
                .style(if self.submit_status == GameSubmitStatus::InDevelopment { primary_button_style } else { secondary_button_style })
                .on_press(WelcomeMessage::SubmitStatusChanged(GameSubmitStatus::InDevelopment)),
        ]
        .spacing(SPACE_2);

        let row_status_genre = row![
            column![
                field_label("Status"),
                status_pills,
            ]
            .spacing(SPACE_2)
            .width(Length::FillPortion(1)),
            column![
                field_label("Genre / Tags"),
                text_input("Mystery, Romance, Slice of Life", &self.submit_genre)
                    .on_input(WelcomeMessage::SubmitGenreChanged)
                    .size(BODY_SM)
                    .padding(SPACE_2)
                    .width(Length::Fill)
                    .style(input_field_style),
            ]
            .spacing(SPACE_1)
            .width(Length::FillPortion(1)),
        ]
        .spacing(SPACE_4);

        let cover_field = column![
            field_label("Cover Image URL (16:9 recommended)"),
            text_input("https://example.com/cover.png", &self.submit_cover_url)
                .on_input(WelcomeMessage::SubmitCoverUrlChanged)
                .size(BODY_SM)
                .padding(SPACE_2)
                .width(Length::Fill)
                .style(input_field_style),
        ]
        .spacing(SPACE_1);

        let game_url_field = column![
            field_label("Game / Download URL"),
            text_input("https://yourname.itch.io/your-game", &self.submit_game_url)
                .on_input(WelcomeMessage::SubmitGameUrlChanged)
                .size(BODY_SM)
                .padding(SPACE_2)
                .width(Length::Fill)
                .style(input_field_style),
        ]
        .spacing(SPACE_1);

        let desc_field = column![
            field_label("Short Description"),
            text_input(
                "A brief 1-2 sentence summary of your visual novel...",
                &self.submit_description
            )
            .on_input(WelcomeMessage::SubmitDescriptionChanged)
            .size(BODY_SM)
            .padding(SPACE_2)
            .width(Length::Fill)
            .style(input_field_style),
        ]
        .spacing(SPACE_1);

        let mut form_col = column![
            row_title_author,
            row_status_genre,
            cover_field,
            game_url_field,
            desc_field,
        ]
        .spacing(SPACE_3);

        if let Some((is_ok, msg)) = &self.submit_feedback {
            let color = if *is_ok {
                iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)
            } else {
                iced::Color::from_rgb8(STATUS_ERROR.r, STATUS_ERROR.g, STATUS_ERROR.b)
            };
            form_col = form_col.push(
                container(text(msg).size(BODY_SM).color(color))
                    .padding([SPACE_2, SPACE_3])
                    .width(Length::Fill)
                    .style(callout_container_style),
            );
        }

        let submit_btn_label = if self.submit_in_progress {
            "Submitting..."
        } else {
            "Submit Game"
        };
        let mut submit_btn = button(text(submit_btn_label).size(LABEL_MD))
            .padding([SPACE_2, SPACE_5])
            .style(primary_button_style);
        if !self.submit_in_progress {
            submit_btn = submit_btn.on_press(WelcomeMessage::SubmitGameClicked);
        }

        let actions_row = row![
            text("Submitted entries are reviewed before appearing on the Blanc Engine Showcase.")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
                .width(Length::Fill),
            button(text("Clear Form").size(LABEL_SM))
                .padding([SPACE_2, SPACE_3])
                .style(secondary_button_style)
                .on_press(WelcomeMessage::SubmitResetForm),
            submit_btn,
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center)
        .width(Length::Fill);

        form_col = form_col.push(Space::with_height(SPACE_1)).push(actions_row);

        let form_card = container(form_col)
            .padding(SPACE_5)
            .width(Length::Fill)
            .style(card_container_style);

        let content = column![header_row, Space::with_height(SPACE_2), form_card]
            .spacing(SPACE_3)
            .width(Length::Fill);

        scrollable(content).width(Length::Fill).height(Length::Fill).into()
    }
}
