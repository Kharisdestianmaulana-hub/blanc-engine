use std::path::PathBuf;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};

use blanc_core::project::Project;
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::theme::{
    badge_container_style, callout_container_style, card_container_style, code_container_style,
    input_field_style, primary_button_style, project_card_style, secondary_button_style,
    sidebar_button_style, sidebar_container_style, subtle_button_style,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HubTab {
    #[default]
    Projects,
    LearnDocs,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DocsTopic {
    #[default]
    QuickStart,
    StoryScript,
    ActionBlocks,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HubConfig {
    pub default_folder: Option<PathBuf>,
    pub recent_projects: Vec<RecentProject>,
}

impl HubConfig {
    pub fn config_path() -> PathBuf {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        base.join("BlancEngine").join("hub_config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str(&content) {
                return cfg;
            }
        }

        // Default to C:\Users\<user>\Documents
        let default_folder = std::env::var("USERPROFILE")
            .map(|p| PathBuf::from(p).join("Documents"))
            .ok()
            .filter(|p| p.exists());

        Self {
            default_folder,
            recent_projects: Vec::new(),
        }
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
}

#[derive(Debug, Clone)]
pub struct WelcomeState {
    pub active_tab: HubTab,
    pub active_docs_topic: DocsTopic,
    pub search_query: String,

    // New project modal state
    pub is_creating_project: bool,
    pub input_name: String,
    pub input_author: String,
    pub selected_folder: Option<PathBuf>,
    pub error_message: Option<String>,

    // Hub config & recents
    pub config: HubConfig,
}

impl Default for WelcomeState {
    fn default() -> Self {
        let config = HubConfig::load();
        let initial_folder = config.default_folder.clone();

        Self {
            active_tab: HubTab::Projects,
            active_docs_topic: DocsTopic::QuickStart,
            search_query: String::new(),
            is_creating_project: false,
            input_name: "Echoes of Memories".to_string(),
            input_author: "Creator".to_string(),
            selected_folder: initial_folder,
            error_message: None,
            config,
        }
    }
}

impl WelcomeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<'_, WelcomeMessage> {
        let sidebar = self.view_sidebar();
        let main_content = match self.active_tab {
            HubTab::Projects => self.view_projects_tab(),
            HubTab::LearnDocs => self.view_learn_docs_tab(),
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
    // SIDEBAR
    // ==========================================
    fn view_sidebar(&self) -> Element<'_, WelcomeMessage> {
        // App brand header
        let brand_icon = text("✦")
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b));

        let brand_title = text("BLANC ENGINE")
            .size(HEADING_SM)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let brand_row = row![brand_icon, brand_title]
            .spacing(SPACE_2)
            .align_y(Alignment::Center);

        let badge = container(
            text("v0.1.0-alpha")
                .size(LABEL_SM)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
        )
        .padding([2.0, 8.0])
        .style(badge_container_style);

        let brand_subtitle = text("Visual Novel Creation Hub")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let brand_section = column![brand_row, row![badge, brand_subtitle].spacing(SPACE_2).align_y(Alignment::Center)]
            .spacing(SPACE_2);

        // Sidebar Navigation Buttons
        let btn_projects = button(
            row![
                text("📁").size(BODY_MD),
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
                text("📚").size(BODY_MD),
                text("Learn & Docs").size(BODY_MD),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::LearnDocs))
        .on_press(WelcomeMessage::SelectTab(HubTab::LearnDocs));

        let btn_settings = button(
            row![
                text("⚙️").size(BODY_MD),
                text("Settings").size(BODY_MD),
            ]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_4])
        .width(Length::Fill)
        .style(sidebar_button_style(self.active_tab == HubTab::Settings))
        .on_press(WelcomeMessage::SelectTab(HubTab::Settings));

        let nav_list = column![btn_projects, btn_learn, btn_settings].spacing(SPACE_2);

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
            text("Create or manage your visual novel projects")
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
            row![text("📂").size(LABEL_MD), text("Open...").size(LABEL_MD)]
                .spacing(SPACE_2)
                .align_y(Alignment::Center),
        )
        .padding([SPACE_2, SPACE_4])
        .style(secondary_button_style)
        .on_press(WelcomeMessage::OpenProjectClicked);

        let btn_new = button(
            row![text("+").size(LABEL_MD), text("New Project").size(LABEL_MD)]
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
            let empty_icon = text(if self.search_query.is_empty() { "🎮" } else { "🔍" }).size(48);
            let empty_title = text(if self.search_query.is_empty() {
                "No Visual Novel Projects Found"
            } else {
                "No matching projects"
            })
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

            let empty_sub = text(if self.search_query.is_empty() {
                "Get started by creating a new project or open an existing .blanc manifest."
            } else {
                "Try searching with a different name or path."
            })
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));

            let mut empty_col = column![empty_icon, empty_title, empty_sub]
                .spacing(SPACE_3)
                .align_x(Alignment::Center);

            if self.search_query.is_empty() {
                let btn_first_create = button(text("+ Create Your First Project").size(BODY_MD))
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
        let icon_box = container(text("📖").size(24))
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
        let btn_reveal = button(text("Explorer").size(LABEL_SM))
            .padding([SPACE_2, SPACE_3])
            .style(subtle_button_style)
            .on_press(WelcomeMessage::RevealInExplorer(parent_dir));

        let btn_remove = button(text("✕").size(LABEL_SM))
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
        let title = text("Create New Visual Novel Project")
            .size(HEADING_LG)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let subtitle = text("Setup project details and directory structure")
            .size(BODY_SM)
            .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));

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
        let folder_label = text("Destination Parent Folder").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
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
                text("Project Target Path Preview:").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
                text(preview_path).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
            ]
            .spacing(SPACE_1),
        )
        .padding(SPACE_3)
        .width(Length::Fill)
        .style(code_container_style);

        // Preset badge summary
        let specs_info = text("⚙️ Default Specs: 1920×1080 (16:9 Full HD) • Dual-process Runtime • GPLv3 Engine")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let mut form = column![
            title,
            subtitle,
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
        .width(Length::Fixed(560.0));

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
    // LEARN & DOCS TAB
    // ==========================================
    fn view_learn_docs_tab(&self) -> Element<'_, WelcomeMessage> {
        let header = column![
            text("Learn & Documentation")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Panduan lengkap dan referensi pengembangan Visual Novel di Blanc Engine")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        // Subtopics navigation
        let topic_quick = button(text("🚀 Panduan Cepat").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(self.active_docs_topic == DocsTopic::QuickStart))
            .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::QuickStart));

        let topic_script = button(text("✍️ Format Naskah Cerita").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(self.active_docs_topic == DocsTopic::StoryScript))
            .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::StoryScript));

        let topic_blocks = button(text("🎬 Sistem Action Blocks").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(self.active_docs_topic == DocsTopic::ActionBlocks))
            .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::ActionBlocks));

        let topic_assets = button(text("🎨 Standar Aset & Audio").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(self.active_docs_topic == DocsTopic::AssetGuidelines))
            .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::AssetGuidelines));

        let topic_controls = button(text("🎮 Kontrol & Shortcut").size(LABEL_MD))
            .padding([SPACE_2, SPACE_3])
            .width(Length::Fill)
            .style(sidebar_button_style(self.active_docs_topic == DocsTopic::RuntimeControls))
            .on_press(WelcomeMessage::SelectDocsTopic(DocsTopic::RuntimeControls));

        let topics_menu = column![
            text("TOPIK PANDUAN").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            topic_quick,
            topic_script,
            topic_blocks,
            topic_assets,
            topic_controls,
        ]
        .spacing(SPACE_2)
        .width(Length::Fixed(220.0));

        let content_body = match self.active_docs_topic {
            DocsTopic::QuickStart => self.doc_quick_start(),
            DocsTopic::StoryScript => self.doc_story_script(),
            DocsTopic::ActionBlocks => self.doc_action_blocks(),
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
            text("🚀 Panduan Cepat: Membuat Visual Novel dalam 5 Menit")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Selamat datang di Blanc Engine! Panduan ini akan memandu Anda membuat proyek visual novel pertama Anda.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),
            
            self.create_step_card(
                "Langkah 1: Buat Proyek Baru",
                "Klik tab 'Projects' di bilah kiri, lalu klik tombol '+ New Project'. Beri nama visual novel dan tentukan studio pembuatnya. Lokasi folder bawaan telah disiapkan di folder Documents Anda."
            ),
            
            self.create_step_card(
                "Langkah 2: Tulis Dialog & Naskah Cerita",
                "Di Editor Workspace, masuk ke tab 'Story Script'. Anda dapat mengetik naskah langsung dengan format sederhana seperti:\n\nAlice (Happy): \"Halo! Selamat datang di Blanc Engine!\"\nBob: \"Luar biasa, pembuatan VN jadi sangat mudah!\""
            ),
            
            self.create_step_card(
                "Langkah 3: Kelola Aset & Background",
                "Di tab 'Asset Registry', impor gambar latar belakang (Backgrounds) dan sprite karakter (Characters). Cukup drag-and-drop atau salin gambar ke folder proyek Anda."
            ),
            
            self.create_step_card(
                "Langkah 4: Jalankan Permainan Instan",
                "Klik tombol hijau '▶ Play Preview' di pojok kanan atas editor. Standalone Runtime (blanc_runtime.exe) akan langsung muncul sebagai proses terpisah yang mulus tanpa delay!"
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_story_script(&self) -> Element<'_, WelcomeMessage> {
        let code_sample = r#"// Contoh Naskah Cerita Blanc Engine (.txt / .md / .fountain)
[Scene: Ruang_Kelas]
[BGM: santai_siang]

Alice (Happy): "Selamat pagi! Kamu sudah sarapan belum?"
Bob (Thinking): "Hmm... baru saja minum kopi sih."

Alice: "Kalau begitu, ayo ke kantin bareng!"

[Choice:
  - "Ikut ke kantin" -> Scene_Kantin
  - "Tolak dengan sopan" -> Scene_Belajar
]"#;

        column![
            text("✍️ Format Naskah Cerita (Story Script)")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Blanc Engine mendukung format teks naskah alami. Anda dapat menulis seperti menulis naskah teater atau film, dan engine akan otomatis mengonversinya menjadi action blocks!")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_2),

            text("Sintaks Dialog:").size(HEADING_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Gunakan format: Karakter (Emosi): \"Teks dialog\". Emosi dalam kurung bersifat opsional dan akan otomatis memilih sprite karakter yang sesuai.").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),

            container(
                text(code_sample).size(BODY_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b))
            )
            .padding(SPACE_4)
            .width(Length::Fill)
            .style(code_container_style),

            Space::with_height(SPACE_2),
            text("💡 Tips Impor Eksternal:")
                .size(LABEL_MD)
                .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
            text("Anda juga dapat menulis skenario menggunakan aplikasi favorit Anda (Notepad, Obsidian, VS Code, atau WriterDuet) dan langsung mengimpor file .txt, .md, atau .fountain ke dalam Blanc Engine.")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_3)
        .into()
    }

    fn doc_action_blocks(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("🎬 Sistem Action Blocks Tanpa Koding")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Setiap adegan (Scene) di Blanc Engine tersusun atas blok-blok aksi modular. Anda dapat menyusun alur cerita semudah menyusun balok puzzle.")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "1. Dialog Block",
                "Menampilkan kotak dialog, nama karakter pembicara, emosi wajah, teks cerita yang dapat diatur kecepatan ketiknya (typewriter effect), serta pemutaran voice acting."
            ),
            self.create_step_card(
                "2. Choice & Branching Block",
                "Menyajikan pilihan interaktif di layar kepada pemain. Setiap pilihan dapat mengarahkan cerita ke adegan baru (jump scene), menyetel variabel flags (misalnya relasi atau poin cinta)."
            ),
            self.create_step_card(
                "3. Audio & Music Block",
                "Mengontrol musik latar (BGM Play/Stop/Fade-In/Fade-Out) dan efek suara (SFX Play/Volume). Mendukung pemutaran looping audio yang mulus."
            ),
            self.create_step_card(
                "4. Character & Camera Block",
                "Mengatur kemunculan sprite karakter pada preset posisi (Left, Center, Right), transisi kemunculan, dan efek transisi latar belakang (Fade to Black, Cross Dissolve)."
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_asset_guidelines(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("🎨 Standar Aset Visual & Audio")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Agar visual novel Anda tampil optimal di berbagai layar dengan performa mulus, ikuti panduan format berikut:")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "📐 Resolusi Kanvas Standar: 1920 × 1080 (16:9 Full HD)",
                "Semua background dianjurkan berukuran 1920x1080 px. Runtime otomatis melakukan aspect-ratio scaling letterbox jika dimainkan pada rasio layar yang berbeda."
            ),
            self.create_step_card(
                "🖼 Format Gambar: PNG & WebP",
                "Gunakan PNG dengan transparansi untuk Sprite Karakter dan UI. WebP sangat direkomendasikan untuk Backgrounds karena ukuran file jauh lebih kecil tanpa mengurangi kualitas visual."
            ),
            self.create_step_card(
                "🎵 Format Audio: OGG, WAV, MP3",
                "Format OGG sangat direkomendasikan untuk BGM karena efisiensi memori dan mendukung seamless looping audio. WAV cocok untuk SFX pendek yang responsif."
            ),
            self.create_step_card(
                "📁 Struktur Folder Proyek",
                "Aset tersimpan rapi dalam folder proyek Anda:\n• Assets/Backgrounds/\n• Assets/Characters/\n• Assets/BGM/\n• Assets/SFX/\n• Assets/Fonts/"
            ),
        ]
        .spacing(SPACE_4)
        .into()
    }

    fn doc_runtime_controls(&self) -> Element<'_, WelcomeMessage> {
        column![
            text("🎮 Kontrol Runtime & Shortcut")
                .size(HEADING_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Kontrol standar bawaan saat menjalankan permainan di Blanc Runtime:")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            Space::with_height(SPACE_3),

            self.create_step_card(
                "Space / Enter / Klik Kiri",
                "Maju ke baris dialog berikutnya atau mengonfirmasi pilihan interaktif."
            ),
            self.create_step_card(
                "Ctrl (Tahan)",
                "Fitur Skip / Fast-Forward untuk mempercepat dialog yang sudah pernah dibaca sebelumnya."
            ),
            self.create_step_card(
                "H (Hide UI)",
                "Menyembunyikan teks box dialog untuk menikmati ilustrasi CG atau latar belakang secara penuh."
            ),
            self.create_step_card(
                "Escape (Esc)",
                "Membuka menu jeda dalam game (In-Game Pause Menu) untuk Save, Load, atau kembali ke Main Menu."
            ),
            self.create_step_card(
                "F5 (Quick Save) & F8 (Quick Load)",
                "Menyimpan permainan secara instan ke Quick Slot dan memuatnya kembali kapan saja."
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
    // SETTINGS TAB
    // ==========================================
    fn view_settings_tab(&self) -> Element<'_, WelcomeMessage> {
        let title_col = column![
            text("Engine Settings & Preferences")
                .size(HEADING_LG)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
            text("Konfigurasi penyimpanan proyek bawaan dan informasi engine")
                .size(BODY_SM)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_1);

        // Section 1: Default Project Location
        let current_dir_text = self
            .config
            .default_folder
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Not configured (User Documents default)".to_string());

        let btn_change_folder = button(text("Ganti Folder...").size(LABEL_MD))
            .padding([SPACE_2, SPACE_4])
            .style(secondary_button_style)
            .on_press(WelcomeMessage::ChangeDefaultFolder);

        let folder_card = container(
            column![
                text("📁 Folder Penyimpanan Proyek Bawaan")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Lokasi default saat Anda membuat proyek visual novel baru.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_2),
                row![
                    container(
                        text(current_dir_text)
                            .size(BODY_SM)
                            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b))
                    )
                    .padding(SPACE_3)
                    .width(Length::Fill)
                    .style(code_container_style),
                    btn_change_folder,
                ]
                .spacing(SPACE_3)
                .align_y(Alignment::Center),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        // Section 2: Engine Specifications & License
        let info_card = container(
            column![
                text("ℹ️ Informasi Engine & Lisensi")
                    .size(HEADING_SM)
                    .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                Space::with_height(SPACE_2),
                row![
                    text("Versi Engine:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(160.0)),
                    text("v0.1.0-alpha (Milestone 1)").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Lisensi Engine:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(160.0)),
                    text("GNU General Public License v3 (GPLv3)").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Kepemilikan Game:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(160.0)),
                    text("100% Hak Milik Kreator (Bebas komersial / gratis)").size(BODY_SM).color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b)),
                ].spacing(SPACE_2),
                row![
                    text("Teknologi:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(160.0)),
                    text("Rust 2021, Iced GUI, wgpu hardware acceleration").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
                row![
                    text("Arsitektur Runtime:").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)).width(Length::Fixed(160.0)),
                    text("Dual-process (Child Process blanc_runtime.exe)").size(BODY_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                ].spacing(SPACE_2),
            ]
            .spacing(SPACE_2),
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let content = column![title_col, Space::with_height(SPACE_4), folder_card, info_card]
            .spacing(SPACE_4)
            .width(Length::Fill);

        scrollable(content).width(Length::Fill).height(Length::Fill).into()
    }
}
