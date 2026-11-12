use std::path::PathBuf;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length};

use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::theme::typography::*;

use crate::theme::{card_container_style, input_field_style, primary_button_style, secondary_button_style};

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    ShowCreateDialog,
    CancelCreateDialog,
    ProjectNameChanged(String),
    AuthorNameChanged(String),
    BrowseProjectLocation,
    CreateProjectConfirmed,
    OpenProjectClicked,
}

#[derive(Debug, Clone, Default)]
pub struct WelcomeState {
    pub is_creating_project: bool,
    pub input_name: String,
    pub input_author: String,
    pub selected_folder: Option<PathBuf>,
    pub error_message: Option<String>,
}

impl WelcomeState {
    pub fn new() -> Self {
        Self {
            input_name: "My Visual Novel".to_string(),
            input_author: "Creator".to_string(),
            ..Default::default()
        }
    }

    pub fn view(&self) -> Element<'_, WelcomeMessage> {
        let header = column![
            text("BLANC ENGINE")
                .size(HEADING_XL)
                .color(iced::Color::from_rgb8(ACCENT_PRIMARY.r, ACCENT_PRIMARY.g, ACCENT_PRIMARY.b)),
            text("No-Code Visual Novel Creation Engine")
                .size(BODY_MD)
                .color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b)),
        ]
        .spacing(SPACE_2)
        .align_x(Alignment::Center);

        let content: Element<'_, WelcomeMessage> = if self.is_creating_project {
            self.view_create_dialog()
        } else {
            self.view_welcome_actions()
        };

        let layout = column![header, Space::with_height(SPACE_8), content]
            .spacing(SPACE_4)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn view_welcome_actions(&self) -> Element<'_, WelcomeMessage> {
        let btn_create = button(
            text("+ Create New Project")
                .size(BODY_MD)
                .align_x(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_6])
        .width(Length::Fixed(260.0))
        .style(primary_button_style)
        .on_press(WelcomeMessage::ShowCreateDialog);

        let btn_open = button(
            text("Open Existing Project")
                .size(BODY_MD)
                .align_x(Alignment::Center),
        )
        .padding([SPACE_3, SPACE_6])
        .width(Length::Fixed(260.0))
        .style(secondary_button_style)
        .on_press(WelcomeMessage::OpenProjectClicked);

        let actions = column![btn_create, btn_open]
            .spacing(SPACE_4)
            .align_x(Alignment::Center);

        container(actions)
            .padding(SPACE_6)
            .style(card_container_style)
            .into()
    }

    fn view_create_dialog(&self) -> Element<'_, WelcomeMessage> {
        let title = text("Create New Project")
            .size(HEADING_LG)
            .color(iced::Color::from_rgb8(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b));

        let name_label = text("Project Name").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let name_input = text_input("e.g. Echoes of Memories", &self.input_name)
            .on_input(WelcomeMessage::ProjectNameChanged)
            .padding(SPACE_2)
            .style(input_field_style);

        let author_label = text("Author / Studio").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let author_input = text_input("e.g. Your Name", &self.input_author)
            .on_input(WelcomeMessage::AuthorNameChanged)
            .padding(SPACE_2)
            .style(input_field_style);

        let folder_label = text("Project Location").size(LABEL_MD).color(iced::Color::from_rgb8(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b));
        let folder_display_text = self
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
                text(folder_display_text)
                    .size(BODY_SM)
                    .color(iced::Color::from_rgb8(TEXT_MUTED.r, TEXT_MUTED.g, TEXT_MUTED.b))
            )
            .width(Length::Fill)
            .padding(SPACE_2)
            .style(card_container_style),
            btn_browse
        ]
        .spacing(SPACE_2)
        .align_y(Alignment::Center);

        let mut form = column![
            title,
            Space::with_height(SPACE_2),
            name_label,
            name_input,
            author_label,
            author_input,
            folder_label,
            folder_row,
        ]
        .spacing(SPACE_3)
        .width(Length::Fixed(460.0));

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
            .on_press(WelcomeMessage::CancelCreateDialog);

        let btn_submit = button(text("Create Project").size(BODY_SM))
            .padding([SPACE_2, SPACE_5])
            .style(primary_button_style)
            .on_press(WelcomeMessage::CreateProjectConfirmed);

        let actions = row![Space::with_width(Length::Fill), btn_cancel, btn_submit]
            .spacing(SPACE_3)
            .align_y(Alignment::Center);

        form = form.push(Space::with_height(SPACE_4)).push(actions);

        container(form)
            .padding(SPACE_6)
            .style(card_container_style)
            .into()
    }
}
