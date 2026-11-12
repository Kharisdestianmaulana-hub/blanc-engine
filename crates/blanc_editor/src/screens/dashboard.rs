use std::path::PathBuf;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};

use blanc_core::asset::{Asset, AssetCategory};
use blanc_core::project::Project;
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::icons::*;
use crate::theme::{
    badge_container_style, callout_container_style, card_container_style, input_field_style,
    primary_button_style, project_card_style, secondary_button_style, sidebar_button_style,
    sidebar_container_style, subtle_button_style,
};

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
    pub selected_category: Option<AssetCategory>,
    pub asset_search_query: String,
    pub status_message: Option<String>,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            active_tab: WorkflowTab::Dashboard,
            selected_category: Some(AssetCategory::Background),
            asset_search_query: String::new(),
            status_message: None,
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
            WorkflowTab::Story => self.view_placeholder_content("Story Setup & Screenplay Importer", "Script editor (.bstory) and Fountain/Markdown importer are ready for Phase 3."),
            WorkflowTab::SceneEditor => self.view_placeholder_content("Block-Based Scene Editor", "Action block timeline and branching dialogue visual graph are coming in Phase 4."),
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
                text("Step 2: Manage and Import Project Assets").size(HEADING_SM).color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
                text("Head over to the 'Asset Manager' tab to import background illustrations, character sprites, BGM tracks, and sound effects.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
                Space::with_height(SPACE_2),
                button(
                    row![icon_image(), text("Open Asset Manager").size(BODY_SM)]
                        .spacing(SPACE_2)
                        .align_y(Alignment::Center),
                )
                .padding([SPACE_2, SPACE_4])
                .style(primary_button_style)
                .on_press(DashboardMessage::SelectWorkflowTab(WorkflowTab::Assets)),
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

        // Filtered Asset List
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
