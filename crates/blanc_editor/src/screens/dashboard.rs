use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

use blanc_core::project::Project;
use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::theme::{card_container_style, primary_button_style, secondary_button_style};

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    PlayPreview,
    CloseProject,
    SelectWorkflowTab(WorkflowTab),
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
            Self::Dashboard => "1. Overview",
            Self::Assets => "2. Asset Manager",
            Self::Story => "3. Story Setup",
            Self::SceneEditor => "4. Scene Editor",
            Self::Settings => "5. Project Settings",
        }
    }
}

pub struct DashboardState {
    pub active_tab: WorkflowTab,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            active_tab: WorkflowTab::Dashboard,
        }
    }
}

impl DashboardState {
    pub fn view<'a>(&'a self, project: &'a Project) -> Element<'a, DashboardMessage> {
        let top_bar = self.view_top_bar(project);
        let workflow_bar = self.view_workflow_bar();
        let main_content = self.view_overview_content(project);

        let layout = column![top_bar, workflow_bar, main_content]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_top_bar<'a>(&'a self, project: &'a Project) -> Element<'a, DashboardMessage> {
        let project_name = text(&project.name)
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let manifest_label = text(format!("{}.blanc", project.name.replace(' ', "_")))
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b));

        let saved_badge = text("● Saved")
            .size(LABEL_SM)
            .color(iced::Color::from_rgb8(STATUS_SUCCESS.r, STATUS_SUCCESS.g, STATUS_SUCCESS.b));

        let info_col = column![project_name, row![manifest_label, Space::with_width(SPACE_3), saved_badge].align_y(Alignment::Center)]
            .spacing(SPACE_1);

        let btn_play = button(text("▶ Run Preview").size(BODY_SM))
            .padding([SPACE_2, SPACE_4])
            .style(primary_button_style)
            .on_press(DashboardMessage::PlayPreview);

        let btn_close = button(text("Close Project").size(BODY_SM))
            .padding([SPACE_2, SPACE_4])
            .style(secondary_button_style)
            .on_press(DashboardMessage::CloseProject);

        let actions = row![btn_play, btn_close].spacing(SPACE_2).align_y(Alignment::Center);

        let bar_content = row![info_col, Space::with_width(Length::Fill), actions]
            .padding([SPACE_3, SPACE_4])
            .align_y(Alignment::Center)
            .width(Length::Fill);

        container(bar_content)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(BACKGROUND_SECONDARY.r, BACKGROUND_SECONDARY.g, BACKGROUND_SECONDARY.b))),
                border: iced::Border {
                    width: 1.0,
                    color: iced::Color::from_rgb8(BORDER_DEFAULT.r, BORDER_DEFAULT.g, BORDER_DEFAULT.b),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

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
                .padding([SPACE_2, SPACE_3])
                .style(if is_active { primary_button_style } else { secondary_button_style })
                .on_press(DashboardMessage::SelectWorkflowTab(tab));

            tab_row = tab_row.push(tab_btn);
        }

        container(tab_row.padding([SPACE_2, SPACE_4]).width(Length::Fill))
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(SURFACE_PRIMARY.r, SURFACE_PRIMARY.g, SURFACE_PRIMARY.b))),
                border: iced::Border {
                    width: 1.0,
                    color: iced::Color::from_rgb8(BORDER_SUBTLE.r, BORDER_SUBTLE.g, BORDER_SUBTLE.b),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }

    fn view_overview_content<'a>(&'a self, project: &'a Project) -> Element<'a, DashboardMessage> {
        let overview_title = text("Project Overview")
            .size(HEADING_MD)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let card_author = column![
            text("Author / Creator").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.author).size(BODY_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let card_version = column![
            text("Project Version").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(&project.version).size(BODY_MD).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let card_assets = column![
            text("Total Assets").size(LABEL_SM).color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b)),
            text(format!("{} assets registered", project.assets.assets.len()))
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
        ]
        .spacing(SPACE_1);

        let summary_cards = row![
            container(card_author).padding(SPACE_4).width(Length::Fixed(200.0)).style(card_container_style),
            container(card_version).padding(SPACE_4).width(Length::Fixed(200.0)).style(card_container_style),
            container(card_assets).padding(SPACE_4).width(Length::Fixed(200.0)).style(card_container_style),
        ]
        .spacing(SPACE_4);

        let next_step_box = container(
            column![
                text("Ready to create your story!").size(HEADING_SM).color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b)),
                text("Head over to Asset Manager to import characters and backgrounds, or open Scene Editor to start writing scenes.")
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
            ]
            .spacing(SPACE_2)
        )
        .padding(SPACE_5)
        .width(Length::Fill)
        .style(card_container_style);

        let content = column![overview_title, summary_cards, next_step_box]
            .spacing(SPACE_5)
            .padding(SPACE_6)
            .width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
