use std::path::PathBuf;

use blanc_core::asset::AssetCategory;
use blanc_core::config::AspectScalingMode;
use blanc_core::scene::{CharacterPosition, ChoiceLayoutPosition};
use iced::widget::{button, column, container, image, mouse_area, row, stack, text, Space};
use iced::{Alignment, Background, Border, Color, ContentFit, Element, Length, Padding};

use crate::engine::{ActiveCharacter, RuntimeEngine, RuntimeScreen};

#[derive(Debug, Clone)]
pub enum RuntimeMessage {
    Tick,
    Advance,
    SelectChoice(usize),
    NewGame,
    OpenSettings,
    CloseSettings,
    TogglePause,
    ToggleHideUi,
    ReturnToTitle,
    FollowEndingTitle,
    SetMasterVolume(f32),
    SetBgmVolume(f32),
    SetSfxVolume(f32),
    SetTextSpeed(f32),
    QuitGame,
}

const ACCENT: Color = Color::from_rgb(0.486, 0.549, 1.0);
const BG_DARK: Color = Color::from_rgb(0.055, 0.063, 0.086);
const PANEL_BG: Color = Color::from_rgba(0.07, 0.082, 0.115, 0.90);
const BORDER_SUBTLE: Color = Color::from_rgba(0.486, 0.549, 1.0, 0.35);
const TEXT_MAIN: Color = Color::from_rgb(0.94, 0.95, 0.98);
const TEXT_MUTED: Color = Color::from_rgb(0.62, 0.66, 0.75);

fn default_logo_widget(size: f32) -> Element<'static, RuntimeMessage> {
    static BYTES: &[u8] = include_bytes!("../../../assets/icons/icon_transparent_128.png");
    image(image::Handle::from_bytes(BYTES))
        .width(size)
        .height(size)
        .into()
}

fn project_logo_widget<'a>(engine: &RuntimeEngine, size: f32) -> Element<'a, RuntimeMessage> {
    if let Some(loaded) = &engine.loaded {
        if let Some(custom_path) = loaded.resolve_custom_logo() {
            return image(image::Handle::from_path(custom_path))
                .width(size)
                .height(size)
                .into();
        }
    }
    default_logo_widget(size)
}

fn primary_btn_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Color::from_rgb(0.56, 0.62, 1.0),
        button::Status::Pressed => Color::from_rgb(0.40, 0.46, 0.92),
        _ => ACCENT,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::WHITE,
        border: Border {
            radius: 8.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

fn glass_btn_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (bg, border_color) = match status {
        button::Status::Hovered => (
            Color::from_rgba(0.16, 0.18, 0.25, 0.95),
            ACCENT,
        ),
        button::Status::Pressed => (
            Color::from_rgba(0.20, 0.23, 0.32, 0.98),
            ACCENT,
        ),
        _ => (
            Color::from_rgba(0.09, 0.105, 0.145, 0.88),
            BORDER_SUBTLE,
        ),
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: TEXT_MAIN,
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border_color,
        },
        ..Default::default()
    }
}

fn panel_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(PANEL_BG)),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: BORDER_SUBTLE,
        },
        ..Default::default()
    }
}

fn dark_backdrop_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_DARK)),
        ..Default::default()
    }
}

fn modal_overlay_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.06, 0.78))),
        ..Default::default()
    }
}

fn render_background<'a>(path: Option<&PathBuf>, scaling: AspectScalingMode) -> Element<'a, RuntimeMessage> {
    let Some(bg_path) = path else {
        return container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(dark_backdrop_style)
            .into();
    };

    let fit = match scaling {
        AspectScalingMode::Cover | AspectScalingMode::AmbientBlur => ContentFit::Cover,
        AspectScalingMode::Fit => ContentFit::Contain,
    };

    container(
        image(image::Handle::from_path(bg_path))
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(fit),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(dark_backdrop_style)
    .into()
}

pub fn view_runtime(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    match &engine.screen {
        RuntimeScreen::TitleScreen => view_title_screen(engine),
        RuntimeScreen::Playing => view_stage(engine, false),
        RuntimeScreen::PauseMenu => view_stage(engine, true),
        RuntimeScreen::Settings { .. } => view_settings(engine),
        RuntimeScreen::Ending { title, .. } => view_ending(engine, title),
        RuntimeScreen::Error(err) => view_error(err),
    }
}

fn view_title_screen(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    let (title_text, author_text, bg_path, scaling) = if let Some(loaded) = &engine.loaded {
        let bg = loaded
            .config
            .title_background_asset_id
            .as_deref()
            .and_then(|id| loaded.resolve_asset_path(id, Some(AssetCategory::Background)));
        (
            loaded.config.title_text.clone(),
            loaded.project.author.clone(),
            bg,
            loaded.config.aspect_scaling_mode,
        )
    } else {
        (
            "Blanc Runtime".to_string(),
            String::new(),
            None,
            AspectScalingMode::Cover,
        )
    };

    let bg_layer = render_background(bg_path.as_ref(), scaling);

    let logo = project_logo_widget(engine, 72.0);
    let title_lbl = text(title_text).size(40).color(TEXT_MAIN);

    let subtitle = if author_text.trim().is_empty() {
        text("Visual Novel").size(15).color(TEXT_MUTED)
    } else {
        text(format!("Created by {}", author_text))
            .size(15)
            .color(TEXT_MUTED)
    };

    let btn_new_game = button(
        container(text("New Game").size(16))
            .width(Length::Fixed(220.0))
            .center_x(Length::Fixed(220.0)),
    )
    .padding([12.0, 20.0])
    .style(primary_btn_style)
    .on_press(RuntimeMessage::NewGame);

    let btn_settings = button(
        container(text("Settings").size(16))
            .width(Length::Fixed(220.0))
            .center_x(Length::Fixed(220.0)),
    )
    .padding([12.0, 20.0])
    .style(glass_btn_style)
    .on_press(RuntimeMessage::OpenSettings);

    let btn_quit = button(
        container(text("Quit").size(16))
            .width(Length::Fixed(220.0))
            .center_x(Length::Fixed(220.0)),
    )
    .padding([12.0, 20.0])
    .style(glass_btn_style)
    .on_press(RuntimeMessage::QuitGame);

    let card = container(
        column![
            logo,
            Space::with_height(4.0),
            title_lbl,
            subtitle,
            Space::with_height(20.0),
            btn_new_game,
            btn_settings,
            btn_quit,
        ]
        .spacing(12.0)
        .align_x(Alignment::Center),
    )
    .padding([40.0, 56.0])
    .style(panel_style);

    let overlay = container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    stack![bg_layer, overlay].into()
}

fn view_character_slot<'a>(ch: &'a ActiveCharacter) -> Element<'a, RuntimeMessage> {
    if let Some(img_path) = &ch.image_path {
        container(
            image(image::Handle::from_path(img_path))
                .height(Length::FillPortion(1))
                .content_fit(ContentFit::Contain),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::End)
        .padding([40.0, 12.0])
        .into()
    } else {
        // Clean visual placeholder card when sprite image is not yet imported
        let expr_line = if ch.expression.trim().is_empty() {
            String::new()
        } else {
            format!("({})", ch.expression)
        };
        let badge = container(
            column![
                text(&ch.character_id).size(18).color(TEXT_MAIN),
                text(expr_line).size(13).color(ACCENT),
            ]
            .spacing(4.0)
            .align_x(Alignment::Center),
        )
        .padding([18.0, 28.0])
        .style(panel_style);

        container(badge)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding([0.0, 12.0])
            .into()
    }
}

fn view_stage(engine: &RuntimeEngine, is_paused: bool) -> Element<'_, RuntimeMessage> {
    let scaling = engine
        .loaded
        .as_ref()
        .map(|l| l.config.aspect_scaling_mode)
        .unwrap_or(AspectScalingMode::Cover);

    let bg_layer = render_background(engine.stage.background_path.as_ref(), scaling);

    // Character Slots (Left, Center, Right)
    let left_char = engine
        .stage
        .characters
        .iter()
        .find(|c| c.position == CharacterPosition::Left);
    let center_char = engine
        .stage
        .characters
        .iter()
        .find(|c| c.position == CharacterPosition::Center);
    let right_char = engine
        .stage
        .characters
        .iter()
        .find(|c| c.position == CharacterPosition::Right);

    let slot_left: Element<'_, RuntimeMessage> = match left_char {
        Some(ch) => view_character_slot(ch),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };
    let slot_center: Element<'_, RuntimeMessage> = match center_char {
        Some(ch) => view_character_slot(ch),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };
    let slot_right: Element<'_, RuntimeMessage> = match right_char {
        Some(ch) => view_character_slot(ch),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };

    let chars_layer = container(
        row![
            container(slot_left).width(Length::FillPortion(1)).height(Length::Fill),
            container(slot_center).width(Length::FillPortion(1)).height(Length::Fill),
            container(slot_right).width(Length::FillPortion(1)).height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([0.0, 32.0]);

    // Click anywhere on stage to advance dialogue
    let clickable_stage = mouse_area(stack![bg_layer, chars_layer])
        .on_press(RuntimeMessage::Advance);

    if engine.stage.ui_hidden && !is_paused {
        return clickable_stage.into();
    }

    // Top-right HUD bar
    let hud_top = container(
        row![
            container(
                text(&engine.stage.current_scene_name)
                    .size(12)
                    .color(TEXT_MUTED),
            )
            .padding([6.0, 12.0])
            .style(panel_style),
            Space::with_width(Length::Fill),
            button(text("Hide UI (H)").size(12))
                .padding([6.0, 12.0])
                .style(glass_btn_style)
                .on_press(RuntimeMessage::ToggleHideUi),
            button(text("Menu (ESC)").size(12))
                .padding([6.0, 12.0])
                .style(glass_btn_style)
                .on_press(RuntimeMessage::TogglePause),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    )
    .padding(16.0)
    .width(Length::Fill);

    // Dialogue Box at bottom
    let dialogue_area: Element<'_, RuntimeMessage> = if let Some(dlg) = &engine.stage.dialogue {
        let speaker_badge: Element<'_, RuntimeMessage> = if let Some(speaker) = &dlg.speaker {
            container(text(speaker).size(16).color(Color::WHITE))
                .padding([6.0, 16.0])
                .style(|_t| container::Style {
                    background: Some(Background::Color(ACCENT)),
                    border: Border {
                        radius: 6.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                })
                .into()
        } else {
            Space::with_height(0.0).into()
        };

        let next_hint = if engine.stage.active_choice.is_some() {
            text("Choose an option...").size(12).color(ACCENT)
        } else if dlg.is_complete() {
            text("Click / Space to continue ->").size(12).color(ACCENT)
        } else {
            text("...").size(12).color(TEXT_MUTED)
        };

        let box_panel = container(
            column![
                text(dlg.visible_slice()).size(19).color(TEXT_MAIN),
                Space::with_height(Length::Fill),
                row![Space::with_width(Length::Fill), next_hint],
            ]
            .spacing(8.0)
            .height(Length::Fixed(128.0)),
        )
        .padding([18.0, 24.0])
        .width(Length::Fill)
        .style(panel_style);

        let wrapped = column![speaker_badge, box_panel].spacing(6.0);

        container(wrapped)
            .width(Length::Fill)
            .max_width(980.0)
            .into()
    } else {
        Space::with_height(0.0).into()
    };

    let bottom_dialogue_layer = container(dialogue_area)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding { top: 0.0, right: 32.0, bottom: 24.0, left: 32.0 })
        .align_x(Alignment::Center)
        .align_y(Alignment::End);

    let mut layers: Vec<Element<'_, RuntimeMessage>> = vec![
        clickable_stage.into(),
        hud_top.into(),
        bottom_dialogue_layer.into(),
    ];

    // Choice Buttons Overlay
    if let Some(choice) = &engine.stage.active_choice {
        let mut choice_col = column![].spacing(10.0).align_x(Alignment::Center);

        if !choice.prompt.trim().is_empty() {
            choice_col = choice_col.push(
                container(text(&choice.prompt).size(16).color(TEXT_MAIN))
                    .padding([8.0, 20.0])
                    .style(panel_style),
            );
        }

        for (idx, opt) in choice.options.iter().enumerate() {
            let btn = button(
                container(text(&opt.text).size(16))
                    .width(Length::Fixed(420.0))
                    .center_x(Length::Fixed(420.0)),
            )
            .padding([14.0, 22.0])
            .style(glass_btn_style)
            .on_press(RuntimeMessage::SelectChoice(idx));
            choice_col = choice_col.push(btn);
        }

        let (align_x, align_y, bottom_pad) = match choice.position {
            ChoiceLayoutPosition::Center => (Alignment::Center, Alignment::Center, 40.0),
            ChoiceLayoutPosition::AboveDialogueBox => (Alignment::Center, Alignment::End, 210.0),
            ChoiceLayoutPosition::Left => (Alignment::Start, Alignment::Center, 40.0),
            ChoiceLayoutPosition::Right => (Alignment::End, Alignment::Center, 40.0),
        };

        let choice_overlay = container(choice_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding { top: 40.0, right: 48.0, bottom: bottom_pad, left: 48.0 })
            .align_x(align_x)
            .align_y(align_y);

        layers.push(choice_overlay.into());
    }

    // Pause Overlay
    if is_paused {
        let pause_card = container(
            column![
                text("Game Paused").size(26).color(TEXT_MAIN),
                Space::with_height(8.0),
                button(
                    container(text("Resume").size(15))
                        .width(Length::Fixed(200.0))
                        .center_x(Length::Fixed(200.0)),
                )
                .padding([10.0, 16.0])
                .style(primary_btn_style)
                .on_press(RuntimeMessage::TogglePause),
                button(
                    container(text("Settings").size(15))
                        .width(Length::Fixed(200.0))
                        .center_x(Length::Fixed(200.0)),
                )
                .padding([10.0, 16.0])
                .style(glass_btn_style)
                .on_press(RuntimeMessage::OpenSettings),
                button(
                    container(text("Return to Title").size(15))
                        .width(Length::Fixed(200.0))
                        .center_x(Length::Fixed(200.0)),
                )
                .padding([10.0, 16.0])
                .style(glass_btn_style)
                .on_press(RuntimeMessage::ReturnToTitle),
                button(
                    container(text("Quit").size(15))
                        .width(Length::Fixed(200.0))
                        .center_x(Length::Fixed(200.0)),
                )
                .padding([10.0, 16.0])
                .style(glass_btn_style)
                .on_press(RuntimeMessage::QuitGame),
            ]
            .spacing(10.0)
            .align_x(Alignment::Center),
        )
        .padding([32.0, 44.0])
        .style(panel_style);

        let pause_backdrop = container(pause_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(modal_overlay_style);

        layers.push(pause_backdrop.into());
    }

    stack(layers).into()
}

fn view_settings(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    let vol_presets = |current: f32, make_msg: fn(f32) -> RuntimeMessage| {
        row![
            button(text("100%").size(13))
                .padding([6.0, 12.0])
                .style(if (current - 1.0).abs() < 0.05 { primary_btn_style } else { glass_btn_style })
                .on_press(make_msg(1.0)),
            button(text("80%").size(13))
                .padding([6.0, 12.0])
                .style(if (current - 0.8).abs() < 0.05 { primary_btn_style } else { glass_btn_style })
                .on_press(make_msg(0.8)),
            button(text("50%").size(13))
                .padding([6.0, 12.0])
                .style(if (current - 0.5).abs() < 0.05 { primary_btn_style } else { glass_btn_style })
                .on_press(make_msg(0.5)),
            button(text("Mute").size(13))
                .padding([6.0, 12.0])
                .style(if current <= 0.01 { primary_btn_style } else { glass_btn_style })
                .on_press(make_msg(0.0)),
        ]
        .spacing(8.0)
    };

    let speed_presets = row![
        button(text("Slow (0.5x)").size(13))
            .padding([6.0, 12.0])
            .style(if (engine.text_speed - 0.5).abs() < 0.1 { primary_btn_style } else { glass_btn_style })
            .on_press(RuntimeMessage::SetTextSpeed(0.5)),
        button(text("Normal (1.0x)").size(13))
            .padding([6.0, 12.0])
            .style(if (engine.text_speed - 1.0).abs() < 0.1 { primary_btn_style } else { glass_btn_style })
            .on_press(RuntimeMessage::SetTextSpeed(1.0)),
        button(text("Fast (1.5x)").size(13))
            .padding([6.0, 12.0])
            .style(if (engine.text_speed - 1.5).abs() < 0.1 { primary_btn_style } else { glass_btn_style })
            .on_press(RuntimeMessage::SetTextSpeed(1.5)),
        button(text("Instant").size(13))
            .padding([6.0, 12.0])
            .style(if engine.text_speed >= 2.0 { primary_btn_style } else { glass_btn_style })
            .on_press(RuntimeMessage::SetTextSpeed(2.0)),
    ]
    .spacing(8.0);

    let card = container(
        column![
            text("Game Settings").size(26).color(TEXT_MAIN),
            Space::with_height(8.0),
            row![
                text("Master Volume").size(14).color(TEXT_MAIN).width(Length::Fixed(130.0)),
                vol_presets(engine.master_volume, RuntimeMessage::SetMasterVolume),
            ]
            .align_y(Alignment::Center),
            row![
                text("BGM Volume").size(14).color(TEXT_MAIN).width(Length::Fixed(130.0)),
                vol_presets(engine.bgm_volume, RuntimeMessage::SetBgmVolume),
            ]
            .align_y(Alignment::Center),
            row![
                text("SFX Volume").size(14).color(TEXT_MAIN).width(Length::Fixed(130.0)),
                vol_presets(engine.sfx_volume, RuntimeMessage::SetSfxVolume),
            ]
            .align_y(Alignment::Center),
            row![
                text("Text Speed").size(14).color(TEXT_MAIN).width(Length::Fixed(130.0)),
                speed_presets,
            ]
            .align_y(Alignment::Center),
            Space::with_height(12.0),
            button(text("Back").size(15))
                .padding([10.0, 28.0])
                .style(primary_btn_style)
                .on_press(RuntimeMessage::CloseSettings),
        ]
        .spacing(14.0)
        .align_x(Alignment::Center),
    )
    .padding([32.0, 40.0])
    .style(panel_style);

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(dark_backdrop_style)
        .into()
}

fn view_ending<'a>(engine: &'a RuntimeEngine, ending_title: &'a str) -> Element<'a, RuntimeMessage> {
    let logo = project_logo_widget(engine, 56.0);
    let card = container(
        column![
            logo,
            text(ending_title).size(28).color(TEXT_MAIN),
            text("End of Story Arc").size(14).color(TEXT_MUTED),
            Space::with_height(14.0),
            button(text("Continue to Title Screen").size(15))
                .padding([12.0, 26.0])
                .style(primary_btn_style)
                .on_press(RuntimeMessage::FollowEndingTitle),
        ]
        .spacing(12.0)
        .align_x(Alignment::Center),
    )
    .padding([36.0, 48.0])
    .style(panel_style);

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(dark_backdrop_style)
        .into()
}

fn view_error(err: &str) -> Element<'_, RuntimeMessage> {
    let card = container(
        column![
            default_logo_widget(56.0),
            text("Blanc Runtime").size(24).color(TEXT_MAIN),
            text(err).size(14).color(TEXT_MUTED),
            Space::with_height(10.0),
            button(text("Close").size(14))
                .padding([10.0, 24.0])
                .style(primary_btn_style)
                .on_press(RuntimeMessage::QuitGame),
        ]
        .spacing(12.0)
        .align_x(Alignment::Center),
    )
    .padding([32.0, 44.0])
    .style(panel_style);

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(dark_backdrop_style)
        .into()
}
