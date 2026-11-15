use std::path::PathBuf;

use blanc_core::asset::AssetCategory;
use blanc_core::config::AspectScalingMode;
use blanc_core::localization::LanguageEntry;
use blanc_core::scene::CharacterPosition;
use blanc_core::ui_config::{
    parse_hex_rgb, ButtonDirection, ButtonVisualStyle, HistoryLayoutTemplate, NameplatePosition,
    PauseLayoutTemplate, RuntimeUiConfig, SaveLoadLayoutTemplate, SettingsLayoutTemplate, UiAnchor,
};
use iced::widget::{
    button, column, container, image, mouse_area, pick_list, row, scrollable, slider, stack, text, Space,
};
use iced::{Alignment, Background, Border, Color, ContentFit, Element, Length, Padding};

use crate::engine::{
    ActiveCharacter, RuntimeEngine, RuntimeScreen, SaveLoadMode, SettingsCategory,
    SettingsFocusItem, SkipMode,
};

#[derive(Debug, Clone)]
pub enum RuntimeMessage {
    Tick,
    Advance,
    SelectChoice(usize),
    ContinueGame,
    NewGame,
    OpenSaveScreen,
    OpenLoadScreen { return_to_pause: bool },
    CloseSaveLoad,
    SaveToSlot(usize),
    LoadFromSlot(usize),
    DeleteSaveSlot(usize),
    QuickSave,
    QuickLoad,
    OpenBacklog,
    CloseBacklog,
    OpenSettings,
    CloseSettings,
    TogglePause,
    ToggleHideUi,
    ReturnToTitle,
    FollowEndingTitle,
    SetMasterVolume(f32),
    SetBgmVolume(f32),
    SetSfxVolume(f32),
    SetVoiceVolume(f32),
    SetTextSpeed(f32),
    ToggleAutoPlay,
    ToggleSkip,
    SetSkipMode(SkipMode),
    SetAutoForwardDelay(f32),
    SetLanguage(String),
    SelectSettingsCategory(SettingsCategory),
    CycleSettingsCategory(isize),
    FocusPrev,
    FocusNext,
    NavLeft,
    NavRight,
    FocusConfirm,
    DeleteFocused,
    BackOrPause,
    ToggleFullscreen,
    SetFullscreen(bool),
    QuitGame,
}

const BG_DARK: Color = Color::from_rgb(0.055, 0.063, 0.086);
const TEXT_MUTED: Color = Color::from_rgb(0.62, 0.66, 0.75);

#[derive(Clone, Copy)]
struct ResolvedTheme {
    accent: Color,
    panel_bg: Color,
    text_main: Color,
    button_text_override: Option<Color>,
    button_focus_text_override: Option<Color>,
    radius: f32,
    btn_style: ButtonVisualStyle,
}

fn resolve_theme(ui: &RuntimeUiConfig) -> ResolvedTheme {
    let (ar, ag, ab) = parse_hex_rgb(&ui.theme.accent_hex).unwrap_or((99, 102, 241));
    let (pr, pg, pb) = parse_hex_rgb(&ui.theme.panel_hex).unwrap_or((13, 15, 23));
    let (tr, tg, tb) = parse_hex_rgb(&ui.theme.text_hex).unwrap_or((248, 250, 252));
    ResolvedTheme {
        accent: Color::from_rgb8(ar, ag, ab),
        panel_bg: Color::from_rgba8(pr, pg, pb, ui.theme.panel_opacity),
        text_main: Color::from_rgb8(tr, tg, tb),
        button_text_override: None,
        button_focus_text_override: None,
        radius: ui.theme.border_radius,
        btn_style: ui.theme.button_style,
    }
}

fn with_button_color(mut theme: ResolvedTheme, normal_hex: &str, focus_hex: &str) -> ResolvedTheme {
    if !normal_hex.trim().is_empty() {
        if let Some((r, g, b)) = parse_hex_rgb(normal_hex) {
            theme.button_text_override = Some(Color::from_rgb8(r, g, b));
        }
    }
    if !focus_hex.trim().is_empty() {
        if let Some((r, g, b)) = parse_hex_rgb(focus_hex) {
            theme.button_focus_text_override = Some(Color::from_rgb8(r, g, b));
        }
    }
    theme
}

fn resolve_text_color(hex: &str, fallback: Color) -> Color {
    let trimmed = hex.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        parse_hex_rgb(trimmed)
            .map(|(r, g, b)| Color::from_rgb8(r, g, b))
            .unwrap_or(fallback)
    }
}

fn resolve_ui_asset(engine: &RuntimeEngine, asset_id: Option<&str>) -> Option<PathBuf> {
    let id = asset_id?;
    engine
        .loaded
        .as_ref()
        .and_then(|l| l.resolve_asset_path(id, Some(AssetCategory::Ui)))
}

fn default_logo_widget(width: f32) -> Element<'static, RuntimeMessage> {
    static BYTES: &[u8] = include_bytes!("../../../assets/icons/icon_transparent_128.png");
    image(image::Handle::from_bytes(BYTES))
        .width(Length::Fixed(width))
        .content_fit(ContentFit::Contain)
        .into()
}

fn load_image_handle(engine: &RuntimeEngine, path: impl AsRef<std::path::Path>) -> image::Handle {
    let path = path.as_ref();
    if let Some(loaded) = &engine.loaded {
        if loaded.pack.is_some() {
            if let Some(bytes) = loaded.read_path_bytes(path) {
                return image::Handle::from_bytes(bytes);
            }
        }
    }
    image::Handle::from_path(path)
}

fn project_logo_widget<'a>(engine: &RuntimeEngine, width: f32) -> Element<'a, RuntimeMessage> {
    if let Some(loaded) = &engine.loaded {
        if let Some(custom_path) = loaded.resolve_custom_logo() {
            return image(load_image_handle(engine, &custom_path))
                .width(Length::Fixed(width))
                .content_fit(ContentFit::Contain)
                .into();
        }
    }
    default_logo_widget(width)
}

fn themed_panel_style(bg: Color, radius: f32) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            radius: radius.into(),
            width: 1.0,
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.16),
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

struct ButtonTextureSet<'a> {
    normal: Option<&'a str>,
    focus: Option<&'a str>,
    disabled: Option<&'a str>,
}

fn themed_button_with_dims<'a>(
    engine: &RuntimeEngine,
    theme: ResolvedTheme,
    style_override: Option<ButtonVisualStyle>,
    label: impl Into<String>,
    width: f32,
    height: f32,
    font_size: u16,
    is_focused: bool,
    is_disabled: bool,
    tex_set: ButtonTextureSet<'_>,
    on_press: RuntimeMessage,
) -> Element<'a, RuntimeMessage> {
    let ui = engine.ui_config();
    let style_kind = style_override.unwrap_or(theme.btn_style);
    let label_str: String = label.into();

    let btn_normal_color = if let Some(override_col) = theme.button_text_override {
        override_col
    } else if !ui.theme.button_text_hex.trim().is_empty() {
        parse_hex_rgb(&ui.theme.button_text_hex)
            .map(|(r, g, b)| Color::from_rgb8(r, g, b))
            .unwrap_or(theme.text_main)
    } else {
        theme.text_main
    };
    let btn_focus_color = if let Some(focus_override) = theme.button_focus_text_override {
        focus_override
    } else if !ui.theme.button_focus_text_hex.trim().is_empty() {
        parse_hex_rgb(&ui.theme.button_focus_text_hex)
            .map(|(r, g, b)| Color::from_rgb8(r, g, b))
            .unwrap_or(btn_normal_color)
    } else {
        btn_normal_color
    };

    let tex_path = if is_disabled {
        resolve_ui_asset(engine, tex_set.disabled)
            .or_else(|| resolve_ui_asset(engine, ui.theme.button_disabled_asset_id.as_deref()))
            .or_else(|| resolve_ui_asset(engine, tex_set.normal))
            .or_else(|| resolve_ui_asset(engine, ui.theme.button_normal_asset_id.as_deref()))
    } else if is_focused {
        resolve_ui_asset(engine, tex_set.focus)
            .or_else(|| resolve_ui_asset(engine, ui.theme.button_focus_asset_id.as_deref()))
            .or_else(|| resolve_ui_asset(engine, tex_set.normal))
            .or_else(|| resolve_ui_asset(engine, ui.theme.button_normal_asset_id.as_deref()))
    } else {
        resolve_ui_asset(engine, tex_set.normal)
            .or_else(|| resolve_ui_asset(engine, ui.theme.button_normal_asset_id.as_deref()))
    };

    if let Some(path) = tex_path {
        let tex_layer = image(load_image_handle(engine, path))
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .content_fit(ContentFit::Fill);

        let text_color = if is_disabled {
            TEXT_MUTED
        } else if is_focused {
            btn_focus_color
        } else {
            btn_normal_color
        };

        let text_layer = container(text(label_str).size(font_size).color(text_color))
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .center_x(Length::Fixed(width))
            .center_y(Length::Fixed(height));

        let focus_border = if is_focused && !is_disabled { 2.0 } else { 0.0 };
        let body = container(stack![tex_layer, text_layer])
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .style(move |_| container::Style {
                border: Border {
                    radius: (theme.radius * 0.75).into(),
                    width: focus_border,
                    color: theme.accent,
                },
                ..Default::default()
            });

        let btn = button(body).padding(0).style(move |_, status| {
            let hovered = !is_disabled && matches!(status, button::Status::Hovered);
            button::Style {
                background: None,
                border: Border {
                    radius: (theme.radius * 0.75).into(),
                    width: if hovered && !is_focused { 1.5 } else { 0.0 },
                    color: if hovered { theme.accent } else { Color::TRANSPARENT },
                },
                ..Default::default()
            }
        });
        return if is_disabled { btn.into() } else { btn.on_press(on_press).into() };
    }

    let accent = theme.accent;
    let panel_bg = theme.panel_bg;
    let radius = theme.radius * 0.75;
    let pad_v = if height <= 36.0 { 6.0 } else { 12.0 };
    let pad_h = if height <= 36.0 { 12.0 } else { 18.0 };

    let btn = button(
        container(text(label_str).size(font_size))
            .width(Length::Fixed(width))
            .center_x(Length::Fixed(width)),
    )
    .padding([pad_v, pad_h])
    .style(move |_t, status| {
        if is_disabled {
            return button::Style {
                background: Some(Background::Color(Color::from_rgba(0.15, 0.16, 0.2, 0.45))),
                text_color: TEXT_MUTED,
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                },
                ..Default::default()
            };
        }

        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        let active_text = if is_focused || hovered || pressed {
            btn_focus_color
        } else {
            btn_normal_color
        };

        let (bg, border_color, border_w) = match style_kind {
            ButtonVisualStyle::SolidAccent => {
                if pressed {
                    (Color::from_rgb(accent.r * 0.78, accent.g * 0.78, accent.b * 0.78), active_text, 2.0)
                } else if is_focused {
                    (accent, active_text, 2.0)
                } else if hovered {
                    (Color::from_rgba(0.14, 0.16, 0.23, 0.95), accent, 1.5)
                } else {
                    (Color::from_rgba(0.05, 0.06, 0.09, 0.90), Color::from_rgba(1.0, 1.0, 1.0, 0.16), 1.0)
                }
            }
            ButtonVisualStyle::GlassDark => {
                if pressed {
                    (Color::from_rgba(accent.r, accent.g, accent.b, 0.45), accent, 2.0)
                } else if is_focused {
                    (accent, active_text, 2.0)
                } else if hovered {
                    (Color::from_rgba(accent.r, accent.g, accent.b, 0.25), accent, 1.5)
                } else {
                    (panel_bg, Color::from_rgba(1.0, 1.0, 1.0, 0.18), 1.0)
                }
            }
            ButtonVisualStyle::Outlined => {
                if pressed {
                    (Color::from_rgba(accent.r, accent.g, accent.b, 0.35), active_text, 2.0)
                } else if is_focused {
                    (accent, active_text, 2.0)
                } else if hovered {
                    (Color::from_rgba(accent.r, accent.g, accent.b, 0.20), accent, 2.0)
                } else {
                    (Color::TRANSPARENT, Color::from_rgba(1.0, 1.0, 1.0, 0.35), 1.5)
                }
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: active_text,
            border: Border {
                radius: radius.into(),
                width: border_w,
                color: border_color,
            },
            ..Default::default()
        }
    });

    if is_disabled {
        btn.into()
    } else {
        btn.on_press(on_press).into()
    }
}

fn themed_button<'a>(
    engine: &RuntimeEngine,
    theme: ResolvedTheme,
    style_override: Option<ButtonVisualStyle>,
    label: impl Into<String>,
    width: f32,
    is_focused: bool,
    is_disabled: bool,
    custom_override_tex: Option<&str>,
    on_press: RuntimeMessage,
) -> Element<'a, RuntimeMessage> {
    themed_button_with_dims(
        engine,
        theme,
        style_override,
        label,
        width,
        46.0,
        16,
        is_focused,
        is_disabled,
        ButtonTextureSet {
            normal: custom_override_tex,
            focus: None,
            disabled: None,
        },
        on_press,
    )
}

fn themed_small_button<'a>(
    engine: &RuntimeEngine,
    theme: ResolvedTheme,
    label: impl Into<String>,
    width: f32,
    is_active: bool,
    on_press: RuntimeMessage,
) -> Element<'a, RuntimeMessage> {
    let ui = engine.ui_config();
    themed_button_with_dims(
        engine,
        theme,
        None,
        label,
        width,
        32.0,
        12,
        is_active,
        false,
        ButtonTextureSet {
            normal: ui.theme.small_button_normal_asset_id.as_deref(),
            focus: ui.theme.small_button_focus_asset_id.as_deref(),
            disabled: ui.theme.small_button_disabled_asset_id.as_deref(),
        },
        on_press,
    )
}

fn themed_choice_button<'a>(
    engine: &RuntimeEngine,
    theme: ResolvedTheme,
    style_override: Option<ButtonVisualStyle>,
    label: impl Into<String>,
    width: f32,
    is_focused: bool,
    on_press: RuntimeMessage,
) -> Element<'a, RuntimeMessage> {
    let ui = engine.ui_config();
    themed_button_with_dims(
        engine,
        theme,
        style_override,
        label,
        width,
        46.0,
        16,
        is_focused,
        false,
        ButtonTextureSet {
            normal: ui.gameplay.choice_button_asset_id.as_deref(),
            focus: ui.gameplay.choice_button_hover_asset_id.as_deref(),
            disabled: ui.gameplay.choice_button_disabled_asset_id.as_deref(),
        },
        on_press,
    )
}

fn place_with_anchor<'a>(
    content: Element<'a, RuntimeMessage>,
    anchor: UiAnchor,
    offset_x: f32,
    offset_y: f32,
) -> Element<'a, RuntimeMessage> {
    let (ax, ay) = match anchor {
        UiAnchor::TopLeft => (Alignment::Start, Alignment::Start),
        UiAnchor::TopCenter => (Alignment::Center, Alignment::Start),
        UiAnchor::TopRight => (Alignment::End, Alignment::Start),
        UiAnchor::CenterLeft => (Alignment::Start, Alignment::Center),
        UiAnchor::Center => (Alignment::Center, Alignment::Center),
        UiAnchor::CenterRight => (Alignment::End, Alignment::Center),
        UiAnchor::BottomLeft => (Alignment::Start, Alignment::End),
        UiAnchor::BottomCenter => (Alignment::Center, Alignment::End),
        UiAnchor::BottomRight => (Alignment::End, Alignment::End),
    };

    let mx = if ax == Alignment::Center { 2.0 } else { 1.0 };
    let my = if ay == Alignment::Center { 2.0 } else { 1.0 };

    let pad = Padding {
        top: if offset_y > 0.0 { offset_y * my } else { 0.0 },
        bottom: if offset_y < 0.0 { -offset_y * my } else { 0.0 },
        left: if offset_x > 0.0 { offset_x * mx } else { 0.0 },
        right: if offset_x < 0.0 { -offset_x * mx } else { 0.0 },
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(ax)
        .align_y(ay)
        .padding(pad)
        .into()
}

fn render_background<'a>(
    engine: &'a RuntimeEngine,
    path: Option<&PathBuf>,
    scaling: AspectScalingMode,
) -> Element<'a, RuntimeMessage> {
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
        image(load_image_handle(engine, bg_path))
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
        RuntimeScreen::SaveLoad { mode, .. } => view_save_load(engine, *mode),
        RuntimeScreen::Backlog => view_backlog(engine),
        RuntimeScreen::Ending { title, .. } => view_ending(engine, title),
        RuntimeScreen::Error(err) => view_error(err),
    }
}

fn view_title_screen(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    let ui = engine.ui_config();
    let theme = resolve_theme(&ui);
    let m = &ui.main_menu;

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
        ("Blanc Runtime".to_string(), String::new(), None, AspectScalingMode::Cover)
    };

    let bg_layer = render_background(engine, bg_path.as_ref(), scaling);
    let dim_alpha = m.backdrop_dimming;
    let dim_layer = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
            ..Default::default()
        });

    let logo = project_logo_widget(engine, m.logo_width);
    let brand_align = match m.brand_anchor {
        UiAnchor::TopLeft | UiAnchor::CenterLeft | UiAnchor::BottomLeft => Alignment::Start,
        UiAnchor::TopRight | UiAnchor::CenterRight | UiAnchor::BottomRight => Alignment::End,
        _ => Alignment::Center,
    };

    let main_text_col = resolve_text_color(&m.text_hex, theme.text_main);
    let menu_theme = with_button_color(theme, &m.button_text_hex, &m.button_focus_text_hex);
    let mut brand_col = column![logo].spacing(6.0).align_x(brand_align);
    if m.show_title_text {
        brand_col = brand_col.push(text(title_text).size(38).color(main_text_col));
    }
    if m.show_author_text && !author_text.trim().is_empty() {
        brand_col = brand_col.push(text(format!("Created by {}", author_text)).size(15).color(main_text_col));
    }

    let continue_enabled = m.show_continue_button && engine.has_save_data();
    let total_focus = if continue_enabled { 5 } else { 4 };
    let focused = engine.focus_index % total_focus;
    let offset = if continue_enabled { 1 } else { 0 };

    let mut menu_btns: Vec<Element<'_, RuntimeMessage>> = Vec::new();
    if m.show_continue_button {
        menu_btns.push(themed_button(
            engine,
            menu_theme,
            None,
            engine.tr_or("ui.title.continue", &m.continue_label),
            m.buttons_width,
            continue_enabled && focused == 0,
            !continue_enabled,
            None,
            RuntimeMessage::ContinueGame,
        ));
    }
    menu_btns.push(themed_button(
        engine, menu_theme, None, engine.tr_or("ui.title.new_game", &m.new_game_label), m.buttons_width, focused == offset, false, None, RuntimeMessage::NewGame,
    ));
    menu_btns.push(themed_button(
        engine, menu_theme, None, engine.tr_or("ui.title.load_game", "Load Game"), m.buttons_width, focused == offset + 1, false, None, RuntimeMessage::OpenLoadScreen { return_to_pause: false },
    ));
    menu_btns.push(themed_button(
        engine, menu_theme, None, engine.tr_or("ui.title.settings", &m.settings_label), m.buttons_width, focused == offset + 2, false, None, RuntimeMessage::OpenSettings,
    ));
    menu_btns.push(themed_button(
        engine, menu_theme, None, engine.tr_or("ui.title.quit", &m.quit_label), m.buttons_width, focused == offset + 3, false, None, RuntimeMessage::QuitGame,
    ));

    let buttons_group: Element<'_, RuntimeMessage> = match m.buttons_direction {
        ButtonDirection::Vertical => column(menu_btns).spacing(12.0).into(),
        ButtonDirection::Horizontal => row(menu_btns).spacing(12.0).into(),
    };

    let buttons_panel: Element<'_, RuntimeMessage> = if let Some(panel_path) = resolve_ui_asset(engine, m.panel_asset_id.as_deref()) {
        container(
            stack![
                image(load_image_handle(engine, panel_path)).content_fit(ContentFit::Fill),
                container(buttons_group).padding(20.0),
            ],
        )
        .into()
    } else {
        buttons_group
    };

    let (brand_oy, btns_oy) = if m.brand_anchor == UiAnchor::Center
        && m.buttons_anchor == UiAnchor::Center
        && m.buttons_direction == ButtonDirection::Vertical
        && (m.buttons_offset_y - m.brand_offset_y) < 250.0
    {
        (m.brand_offset_y.min(-175.0), m.buttons_offset_y.max(115.0))
    } else {
        (m.brand_offset_y, m.buttons_offset_y)
    };

    let brand_placed = place_with_anchor(brand_col.into(), m.brand_anchor, m.brand_offset_x, brand_oy);
    let buttons_placed = place_with_anchor(buttons_panel, m.buttons_anchor, m.buttons_offset_x, btns_oy);

    stack![bg_layer, dim_layer, brand_placed, buttons_placed].into()
}

fn view_character_slot<'a>(
    engine: &'a RuntimeEngine,
    ch: &'a ActiveCharacter,
    theme: ResolvedTheme,
) -> Element<'a, RuntimeMessage> {
    let active_speaker = engine
        .stage
        .dialogue
        .as_ref()
        .and_then(|d| d.speaker_id.as_deref());
    let any_speaker_on_stage = active_speaker.is_some_and(|spk| {
        engine
            .stage
            .characters
            .iter()
            .any(|c| c.character_id.eq_ignore_ascii_case(spk))
    });
    let is_dimmed = any_speaker_on_stage
        && active_speaker.is_some_and(|spk| !ch.character_id.eq_ignore_ascii_case(spk));
    let char_opacity = if is_dimmed { 0.52_f32 } else { 1.0_f32 };

    if let Some(img_path) = &ch.image_path {
        let pad = Padding {
            top: 40.0,
            bottom: 40.0,
            left: 12.0 + if ch.anim_offset_x > 0.0 { ch.anim_offset_x } else { 0.0 },
            right: 12.0 + if ch.anim_offset_x < 0.0 { -ch.anim_offset_x } else { 0.0 },
        };
        container(
            image(load_image_handle(engine, img_path))
                .height(Length::FillPortion(1))
                .content_fit(ContentFit::Contain)
                .opacity(char_opacity),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::End)
        .padding(pad)
        .into()
    } else {
        let expr_line = if ch.expression.trim().is_empty() {
            String::new()
        } else {
            format!("({})", ch.expression)
        };
        let badge = container(
            column![
                text(&ch.character_id).size(18).color(theme.text_main),
                text(expr_line).size(13).color(theme.accent),
            ]
            .spacing(4.0)
            .align_x(Alignment::Center),
        )
        .padding([18.0, 28.0])
        .style(themed_panel_style(theme.panel_bg, theme.radius));

        container(badge)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding([0.0, 12.0])
            .into()
    }
}

fn view_stage_visual(engine: &RuntimeEngine, theme: ResolvedTheme) -> Element<'_, RuntimeMessage> {
    let scaling = engine
        .loaded
        .as_ref()
        .map(|l| l.config.aspect_scaling_mode)
        .unwrap_or(AspectScalingMode::Cover);

    let bg_layer = render_background(engine, engine.stage.background_path.as_ref(), scaling);

    let left_char = engine.stage.characters.iter().find(|c| c.position == CharacterPosition::Left);
    let center_char = engine.stage.characters.iter().find(|c| c.position == CharacterPosition::Center);
    let right_char = engine.stage.characters.iter().find(|c| c.position == CharacterPosition::Right);

    let slot_left: Element<'_, RuntimeMessage> = match left_char {
        Some(ch) => view_character_slot(engine, ch, theme),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };
    let slot_center: Element<'_, RuntimeMessage> = match center_char {
        Some(ch) => view_character_slot(engine, ch, theme),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };
    let slot_right: Element<'_, RuntimeMessage> = match right_char {
        Some(ch) => view_character_slot(engine, ch, theme),
        None => Space::new(Length::Fill, Length::Fill).into(),
    };

    let chars_layer: Element<'_, RuntimeMessage> = if engine.stage.transition_hide_until_fade_in {
        Space::new(Length::Fill, Length::Fill).into()
    } else {
        container(
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
        .padding([0.0, 32.0])
        .into()
    };

    if engine.stage.shake_remaining > 0.0 {
        let sx = engine.stage.shake_phase.sin() * 14.0;
        let sy = (engine.stage.shake_phase * 1.3).cos() * 8.0;
        container(stack![bg_layer, chars_layer])
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding {
                top: if sy > 0.0 { sy } else { 0.0 },
                bottom: if sy < 0.0 { -sy } else { 0.0 },
                left: if sx > 0.0 { sx } else { 0.0 },
                right: if sx < 0.0 { -sx } else { 0.0 },
            })
            .into()
    } else {
        stack![bg_layer, chars_layer].into()
    }
}

fn view_stage(engine: &RuntimeEngine, is_paused: bool) -> Element<'_, RuntimeMessage> {
    let ui = engine.ui_config();
    let theme = resolve_theme(&ui);
    let g = &ui.gameplay;

    let stage_visual = view_stage_visual(engine, theme);

    let clickable_stage = mouse_area(stage_visual).on_press(RuntimeMessage::Advance);

    let show_ui = !engine.stage.ui_hidden && !engine.stage.transition_hide_until_fade_in;

    // HUD buttons bar (anchored & movable)
    let hud_theme = with_button_color(theme, &g.hud_text_hex, "");
    let mut hud_row = row![].spacing(10.0).align_y(Alignment::Center);
    if g.show_quick_save_button {
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            engine.tr_or("ui.hud.qsave", "Q.Save (F5)"),
            104.0,
            false,
            RuntimeMessage::QuickSave,
        ));
    }
    if g.show_quick_load_button {
        let ui_cfg = engine.ui_config();
        hud_row = hud_row.push(themed_button_with_dims(
            engine,
            hud_theme,
            None,
            engine.tr_or("ui.hud.qload", "Q.Load (F9)"),
            104.0,
            32.0,
            12,
            false,
            !engine.has_quicksave(),
            ButtonTextureSet {
                normal: ui_cfg.theme.small_button_normal_asset_id.as_deref(),
                focus: ui_cfg.theme.small_button_focus_asset_id.as_deref(),
                disabled: ui_cfg.theme.small_button_disabled_asset_id.as_deref(),
            },
            RuntimeMessage::QuickLoad,
        ));
    }
    if g.show_auto_button {
        let auto_lbl = if engine.auto_play {
            engine.tr_or("ui.hud.auto_active", "Auto ●")
        } else {
            engine.tr_or("ui.hud.auto", "Auto (A)")
        };
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            auto_lbl,
            96.0,
            engine.auto_play,
            RuntimeMessage::ToggleAutoPlay,
        ));
    }
    if g.show_skip_button {
        let skip_lbl = if engine.skip_mode_active {
            engine.tr_or("ui.hud.skip_active", "Skip ▶▶")
        } else {
            engine.tr_or("ui.hud.skip", "Skip (S)")
        };
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            skip_lbl,
            96.0,
            engine.skip_mode_active,
            RuntimeMessage::ToggleSkip,
        ));
    }
    if g.show_history_button {
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            engine.tr_or("ui.hud.history", "History (B)"),
            104.0,
            false,
            RuntimeMessage::OpenBacklog,
        ));
    }
    if g.show_hide_ui_button {
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            engine.tr_or("ui.hud.hide_ui", "Hide UI (H)"),
            104.0,
            false,
            RuntimeMessage::ToggleHideUi,
        ));
    }
    if g.show_menu_button {
        hud_row = hud_row.push(themed_small_button(
            engine,
            hud_theme,
            engine.tr_or("ui.hud.menu", "Menu (ESC)"),
            104.0,
            false,
            RuntimeMessage::TogglePause,
        ));
    }
    let hud_is_bottom = matches!(
        g.hud_anchor,
        UiAnchor::BottomLeft | UiAnchor::BottomCenter | UiAnchor::BottomRight
    );
    let hud_pad_v = if hud_is_bottom { 10.0 } else { 14.0 };
    let hud_placed = place_with_anchor(
        container(hud_row).padding([hud_pad_v, 20.0]).into(),
        g.hud_anchor,
        g.hud_offset_x,
        g.hud_offset_y,
    );

    // Dialogue Box & Nameplate
    let (speaker_opt, nameplate_override, visible_text, is_complete) = if let Some(dlg) = &engine.stage.dialogue {
        let shown: String = dlg.full_text.chars().take(dlg.visible_chars).collect();
        (dlg.speaker.clone(), dlg.nameplate_position, shown, dlg.visible_chars >= dlg.total_chars)
    } else {
        (None, None, String::new(), true)
    };

    let badge_bg = if g.nameplate_use_accent { theme.accent } else { theme.panel_bg };
    let badge_text_col = resolve_text_color(&g.nameplate_text_hex, theme.text_main);
    let build_badge = |name: String| -> Element<'_, RuntimeMessage> {
        if let Some(np_path) = resolve_ui_asset(engine, g.nameplate_asset_id.as_deref()) {
            stack![
                image(load_image_handle(engine, np_path)).content_fit(ContentFit::Fill),
                container(text(name).size(15).color(badge_text_col)).padding([6.0, 18.0]),
            ]
            .into()
        } else {
            container(text(name).size(15).color(badge_text_col))
                .padding([6.0, 18.0])
                .style(move |_| container::Style {
                    background: Some(Background::Color(badge_bg)),
                    border: Border { radius: (theme.radius * 0.65).into(), width: 0.0, color: Color::TRANSPARENT },
                    ..Default::default()
                })
                .into()
        }
    };

    let (pr, pg, pb) = parse_hex_rgb(&ui.theme.panel_hex).unwrap_or((13, 15, 23));
    let dlg_bg = Color::from_rgba8(pr, pg, pb, g.dialogue_opacity);
    let box_width = if g.dialogue_full_width { Length::Fill } else { Length::Fixed(g.dialogue_width) };
    let dlg_text_color = if !g.dialogue_text_hex.trim().is_empty() {
        parse_hex_rgb(&g.dialogue_text_hex)
            .map(|(r, g, b)| Color::from_rgb8(r, g, b))
            .unwrap_or(theme.text_main)
    } else {
        theme.text_main
    };

    let mut dlg_inner = column![].spacing(8.0).width(Length::Fill).height(Length::Fill);
    if g.nameplate_position == NameplatePosition::InsideBox {
        if let Some(speaker) = speaker_opt.clone() {
            let inside_align = match nameplate_override {
                Some(CharacterPosition::Center) => Alignment::Center,
                Some(CharacterPosition::Right) => Alignment::End,
                _ => Alignment::Start,
            };
            dlg_inner = dlg_inner.push(
                container(build_badge(speaker))
                    .width(Length::Fill)
                    .align_x(inside_align),
            );
        }
    }
    dlg_inner = dlg_inner.push(
        text(visible_text)
            .size(g.dialogue_font_size)
            .color(dlg_text_color)
            .width(Length::Fill),
    );
    dlg_inner = dlg_inner.push(Space::with_height(Length::Fill));

    let mut prompt_row = row![Space::with_width(Length::Fill)]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .width(Length::Fill);
    if is_complete {
        let mut prompt_frames = Vec::new();
        if let Some(p1) = resolve_ui_asset(engine, g.advance_prompt_asset_id.as_deref()) {
            prompt_frames.push(p1);
        }
        if let Some(p2) = resolve_ui_asset(engine, g.advance_prompt_frame2_asset_id.as_deref()) {
            prompt_frames.push(p2);
        }
        if let Some(p3) = resolve_ui_asset(engine, g.advance_prompt_frame3_asset_id.as_deref()) {
            prompt_frames.push(p3);
        }
        if !prompt_frames.is_empty() {
            let frame_idx = ((engine.ui_anim_time / 0.28) as usize) % prompt_frames.len();
            prompt_row = prompt_row.push(
                image(load_image_handle(engine, &prompt_frames[frame_idx]))
                    .height(Length::Fixed(20.0))
                    .content_fit(ContentFit::Contain),
            );
        }
        if !g.advance_prompt_label.trim().is_empty() {
            prompt_row = prompt_row.push(
                text(g.advance_prompt_label.clone())
                    .size(12)
                    .color(resolve_text_color(&g.hud_text_hex, theme.accent)),
            );
        }
    } else {
        prompt_row = prompt_row.push(text("...").size(12).color(theme.accent));
    }
    dlg_inner = dlg_inner.push(prompt_row);

    let dlg_box: Element<'_, RuntimeMessage> = if let Some(box_path) = resolve_ui_asset(engine, g.dialogue_box_asset_id.as_deref()) {
        mouse_area(
            container(
                stack![
                    image(load_image_handle(engine, box_path))
                        .width(box_width)
                        .height(Length::Fixed(160.0))
                        .content_fit(ContentFit::Fill),
                    container(dlg_inner)
                        .width(box_width)
                        .height(Length::Fixed(160.0))
                        .padding([18.0, 24.0]),
                ],
            )
            .width(box_width)
            .height(Length::Fixed(160.0)),
        )
        .on_press(RuntimeMessage::Advance)
        .into()
    } else {
        mouse_area(
            container(dlg_inner)
                .width(box_width)
                .height(Length::Fixed(150.0))
                .padding([18.0, 24.0])
                .style(themed_panel_style(dlg_bg, theme.radius)),
        )
        .on_press(RuntimeMessage::Advance)
        .into()
    };

    let mut dialogue_stack = column![].spacing(6.0);
    if g.nameplate_position != NameplatePosition::InsideBox {
        if let Some(speaker) = speaker_opt {
            let align = match nameplate_override {
                Some(CharacterPosition::Left) => Alignment::Start,
                Some(CharacterPosition::Center) => Alignment::Center,
                Some(CharacterPosition::Right) => Alignment::End,
                None => match g.nameplate_position {
                    NameplatePosition::TopCenter => Alignment::Center,
                    NameplatePosition::TopRight => Alignment::End,
                    _ => Alignment::Start,
                },
            };
            dialogue_stack = dialogue_stack.push(
                container(build_badge(speaker))
                    .width(box_width)
                    .align_x(align),
            );
        }
    }
    dialogue_stack = dialogue_stack.push(dlg_box);

    let dlg_is_bottom = matches!(
        g.dialogue_anchor,
        UiAnchor::BottomLeft | UiAnchor::BottomCenter | UiAnchor::BottomRight
    );
    let auto_hud_clearance = if hud_is_bottom && dlg_is_bottom { -42.0 } else { 0.0 };

    let dialogue_layer = place_with_anchor(
        dialogue_stack.into(),
        g.dialogue_anchor,
        g.dialogue_offset_x,
        g.dialogue_offset_y + auto_hud_clearance,
    );

    let mut layers: Vec<Element<'_, RuntimeMessage>> = vec![clickable_stage.into()];
    if show_ui {
        layers.push(dialogue_layer);
        layers.push(hud_placed);
    }

    if engine.stage.fade_alpha > 0.005 {
        let alpha = engine.stage.fade_alpha.clamp(0.0, 1.0);
        layers.push(
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, alpha))),
                    ..Default::default()
                })
                .into(),
        );
    }

    if engine.stage.flash_alpha > 0.005 {
        let alpha = engine.stage.flash_alpha.clamp(0.0, 1.0);
        layers.push(
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha))),
                    ..Default::default()
                })
                .into(),
        );
    }

    // Choice Buttons Overlay
    if let Some(choice) = &engine.stage.active_choice {
        let mut choice_col = column![].spacing(10.0).align_x(Alignment::Center);
        if !choice.prompt.trim().is_empty() {
            choice_col = choice_col.push(
                container(text(&choice.prompt).size(16).color(theme.text_main))
                    .padding([8.0, 20.0])
                    .style(themed_panel_style(theme.panel_bg, theme.radius)),
            );
        }

        let choice_theme = with_button_color(theme, &g.choice_text_hex, "");
        let focused = engine.focus_index % choice.options.len().max(1);
        for (idx, opt) in choice.options.iter().enumerate() {
            choice_col = choice_col.push(themed_choice_button(
                engine,
                choice_theme,
                Some(g.choices_style),
                &opt.text,
                g.choices_width,
                focused == idx,
                RuntimeMessage::SelectChoice(idx),
            ));
        }

        layers.push(place_with_anchor(
            choice_col.into(),
            g.choices_anchor,
            g.choices_offset_x,
            g.choices_offset_y,
        ));
    }

    if let Some((toast_msg, _)) = &engine.toast {
        let toast_pill = container(text(toast_msg.clone()).size(14).color(theme.text_main))
            .padding([8.0, 18.0])
            .style(themed_panel_style(theme.panel_bg, theme.radius));
        layers.push(place_with_anchor(toast_pill.into(), UiAnchor::TopCenter, 0.0, 18.0));
    }

    // Pause Overlay
    if is_paused {
        let p = &ui.pause_settings;
        let dim_alpha = p.pause_overlay_dimming.clamp(0.0, 0.95);
        let dim = container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
                ..Default::default()
            });

        let pause_theme = with_button_color(theme, &p.pause_button_text_hex, "");
        let mut btns: Vec<Element<'_, RuntimeMessage>> = Vec::new();
        let mut visible_idx = 0usize;
        let focused = engine.focus_index;
        let bw = p.pause_button_width.max(110.0);

        if p.show_resume_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.resume", &p.resume_label), bw, focused == visible_idx, false, None, RuntimeMessage::TogglePause,
            ));
            visible_idx += 1;
        }
        if p.show_save_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.save", &p.save_label), bw, focused == visible_idx, false, None, RuntimeMessage::OpenSaveScreen,
            ));
            visible_idx += 1;
        }
        if p.show_load_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.load", &p.load_label), bw, focused == visible_idx, false, None, RuntimeMessage::OpenLoadScreen { return_to_pause: true },
            ));
            visible_idx += 1;
        }
        if p.show_settings_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.settings", &p.settings_label), bw, focused == visible_idx, false, None, RuntimeMessage::OpenSettings,
            ));
            visible_idx += 1;
        }
        if p.show_return_title_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.return_title", &p.return_title_label), bw, focused == visible_idx, false, None, RuntimeMessage::ReturnToTitle,
            ));
            visible_idx += 1;
        }
        if p.show_quit_button {
            btns.push(themed_button(
                engine, pause_theme, None, engine.tr_or("ui.pause.quit", &p.quit_label), bw, focused == visible_idx, false, None, RuntimeMessage::QuitGame,
            ));
        }

        let effective_btn_layout = match p.pause_layout_template {
            PauseLayoutTemplate::CenteredCard => p.pause_layout,
            PauseLayoutTemplate::LeftSidebar | PauseLayoutTemplate::RightSidebar => {
                blanc_core::ui_config::PauseButtonsLayout::Vertical
            }
            PauseLayoutTemplate::BottomBar | PauseLayoutTemplate::TopBar => {
                blanc_core::ui_config::PauseButtonsLayout::Horizontal
            }
        };

        let btn_group: Element<'_, RuntimeMessage> = match effective_btn_layout {
            blanc_core::ui_config::PauseButtonsLayout::Vertical => {
                column(btns).spacing(10.0).align_x(Alignment::Center).into()
            }
            blanc_core::ui_config::PauseButtonsLayout::Horizontal => {
                row(btns).spacing(10.0).align_y(Alignment::Center).into()
            }
            blanc_core::ui_config::PauseButtonsLayout::TwoRowGrid => {
                let mut rows_col = column![].spacing(10.0).align_x(Alignment::Center);
                let split_at = (btns.len() + 1) / 2;
                let second_half = btns.split_off(split_at.min(btns.len()));
                if !btns.is_empty() {
                    rows_col = rows_col.push(row(btns).spacing(10.0).align_y(Alignment::Center));
                }
                if !second_half.is_empty() {
                    rows_col = rows_col.push(row(second_half).spacing(10.0).align_y(Alignment::Center));
                }
                rows_col.into()
            }
        };

        let btns_widget: Element<'_, RuntimeMessage> = if p.pause_show_panel {
            if let Some(pp_path) = resolve_ui_asset(engine, p.pause_panel_asset_id.as_deref()) {
                container(
                    stack![
                        image(load_image_handle(engine, pp_path)).content_fit(ContentFit::Fill),
                        container(btn_group).padding([24.0, 28.0]),
                    ],
                )
                .into()
            } else {
                container(btn_group)
                    .padding([24.0, 28.0])
                    .style(themed_panel_style(theme.panel_bg, theme.radius))
                    .into()
            }
        } else {
            btn_group
        };

        let mut pause_layers: Vec<Element<'_, RuntimeMessage>> = vec![dim.into()];
        let pause_title_col = resolve_text_color(&p.pause_text_hex, theme.text_main);
        match p.pause_layout_template {
            PauseLayoutTemplate::CenteredCard => {
                pause_layers.push(place_with_anchor(
                    btns_widget,
                    p.pause_anchor,
                    p.pause_offset_x,
                    p.pause_offset_y,
                ));
                if p.pause_show_title && !p.pause_title.trim().is_empty() {
                    let title_el = text(p.pause_title.clone()).size(34).color(pause_title_col);
                    pause_layers.push(place_with_anchor(
                        title_el.into(),
                        p.pause_title_anchor,
                        p.pause_title_offset_x,
                        p.pause_title_offset_y,
                    ));
                }
            }
            PauseLayoutTemplate::LeftSidebar | PauseLayoutTemplate::RightSidebar => {
                let mut col_items = column![].spacing(18.0).align_x(Alignment::Center);
                if p.pause_show_title && !p.pause_title.trim().is_empty() {
                    col_items = col_items.push(text(p.pause_title.clone()).size(30).color(pause_title_col));
                }
                col_items = col_items.push(btns_widget);
                let (anchor, offset_x) = if p.pause_layout_template == PauseLayoutTemplate::LeftSidebar {
                    (UiAnchor::CenterLeft, 36.0)
                } else {
                    (UiAnchor::CenterRight, -36.0)
                };
                pause_layers.push(place_with_anchor(col_items.into(), anchor, offset_x, 0.0));
            }
            PauseLayoutTemplate::BottomBar | PauseLayoutTemplate::TopBar => {
                let mut col_items = column![].spacing(14.0).align_x(Alignment::Center);
                if p.pause_show_title && !p.pause_title.trim().is_empty() {
                    col_items = col_items.push(text(p.pause_title.clone()).size(28).color(pause_title_col));
                }
                col_items = col_items.push(btns_widget);
                let (anchor, oy) = if p.pause_layout_template == PauseLayoutTemplate::BottomBar {
                    (UiAnchor::BottomCenter, -28.0)
                } else {
                    (UiAnchor::TopCenter, 28.0)
                };
                pause_layers.push(place_with_anchor(col_items.into(), anchor, 0.0, oy));
            }
        }
        layers.push(stack(pause_layers).into());
    }

    stack(layers).into()
}

fn themed_slider<'a>(
    engine: &'a RuntimeEngine,
    theme: ResolvedTheme,
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    step: f32,
    on_change: impl Fn(f32) -> RuntimeMessage + 'a,
) -> Element<'a, RuntimeMessage> {
    let ui_theme = &engine.ui_config().theme;
    let track_asset = resolve_ui_asset(engine, ui_theme.slider_track_asset_id.as_deref());
    let fill_asset = resolve_ui_asset(engine, ui_theme.slider_fill_asset_id.as_deref());
    let handle_asset = resolve_ui_asset(engine, ui_theme.slider_handle_asset_id.as_deref());

    let track_col = parse_hex_rgb(&ui_theme.slider_track_bg_hex)
        .map(|(r, g, b)| Color::from_rgb8(r, g, b))
        .unwrap_or_else(|| Color::from_rgba(1.0, 1.0, 1.0, 0.18));
    let fill_col = parse_hex_rgb(&ui_theme.slider_fill_hex)
        .map(|(r, g, b)| Color::from_rgb8(r, g, b))
        .unwrap_or(theme.accent);
    let handle_col = parse_hex_rgb(&ui_theme.slider_handle_hex)
        .map(|(r, g, b)| Color::from_rgb8(r, g, b))
        .unwrap_or(theme.accent);
    let track_h = ui_theme.slider_track_height.clamp(2.0, 128.0);
    let handle_sz = ui_theme.slider_handle_size.clamp(8.0, 128.0);

    if track_asset.is_none() && fill_asset.is_none() && handle_asset.is_none() {
        return slider(range, value, on_change)
            .step(step)
            .width(Length::Fill)
            .style(move |_, status| {
                let h_col = match status {
                    slider::Status::Hovered | slider::Status::Dragged => theme.text_main,
                    slider::Status::Active => handle_col,
                };
                slider::Style {
                    rail: slider::Rail {
                        backgrounds: (Background::Color(fill_col), Background::Color(track_col)),
                        width: track_h,
                        border: Border {
                            radius: 99.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle {
                            radius: handle_sz * 0.5,
                        },
                        background: Background::Color(h_col),
                        border_width: 2.0,
                        border_color: theme.text_main,
                    },
                }
            })
            .into();
    }

    let min_v = *range.start();
    let max_v = *range.end();
    let pct = ((value - min_v) / (max_v - min_v).max(0.0001)).clamp(0.0, 1.0);
    let left_p = (pct * 1000.0).round() as u16;
    let right_p = 1000u16.saturating_sub(left_p);
    let box_h = handle_sz.max(track_h).max(20.0);

    let has_track_asset = track_asset.is_some();
    let track_layer: Element<'a, RuntimeMessage> = if let Some(tp) = track_asset {
        image(load_image_handle(engine, tp))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .content_fit(ContentFit::Fill)
            .into()
    } else {
        container(Space::new(Length::Fill, Length::Fixed(track_h)))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .style(move |_| container::Style {
                background: Some(Background::Color(track_col)),
                border: Border {
                    radius: 99.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    };

    let show_fill_layer = fill_asset.is_some()
        || !has_track_asset
        || parse_hex_rgb(&ui_theme.slider_fill_hex).is_some();

    let fill_inner: Element<'a, RuntimeMessage> = if left_p == 0 || !show_fill_layer {
        Space::new(Length::Fixed(0.0), Length::Fixed(track_h)).into()
    } else if let Some(fp) = fill_asset {
        image(load_image_handle(engine, fp))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .content_fit(ContentFit::Fill)
            .into()
    } else {
        container(Space::new(Length::Fill, Length::Fixed(track_h)))
            .width(Length::Fill)
            .height(Length::Fixed(track_h))
            .style(move |_| container::Style {
                background: Some(Background::Color(fill_col)),
                border: Border {
                    radius: 99.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    };

    let fill_layer = row![
        container(fill_inner).width(if left_p == 0 {
            Length::Fixed(0.0)
        } else {
            Length::FillPortion(left_p)
        }),
        Space::new(
            if right_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(right_p)
            },
            Length::Fixed(track_h),
        ),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let handle_widget: Element<'a, RuntimeMessage> = if let Some(hp) = handle_asset {
        image(load_image_handle(engine, hp))
            .width(Length::Shrink)
            .height(Length::Fixed(handle_sz))
            .content_fit(ContentFit::Contain)
            .into()
    } else {
        container(Space::new(Length::Fixed(handle_sz), Length::Fixed(handle_sz)))
            .width(Length::Fixed(handle_sz))
            .height(Length::Fixed(handle_sz))
            .style(move |_| container::Style {
                background: Some(Background::Color(handle_col)),
                border: Border {
                    radius: 99.0.into(),
                    width: 2.0,
                    color: theme.text_main,
                },
                ..Default::default()
            })
            .into()
    };

    let handle_layer = row![
        Space::new(
            if left_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(left_p)
            },
            Length::Fixed(box_h),
        ),
        handle_widget,
        Space::new(
            if right_p == 0 {
                Length::Fixed(0.0)
            } else {
                Length::FillPortion(right_p)
            },
            Length::Fixed(box_h),
        ),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let interactive = slider(range, value, on_change)
        .step(step)
        .width(Length::Fill)
        .height(box_h)
        .style(|_, _| slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color::TRANSPARENT),
                    Background::Color(Color::TRANSPARENT),
                ),
                width: 6.0,
                border: Border::default(),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 8.0 },
                background: Background::Color(Color::TRANSPARENT),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        });

    stack![
        container(track_layer)
            .width(Length::Fill)
            .height(box_h)
            .center_y(Length::Fixed(box_h)),
        container(fill_layer)
            .width(Length::Fill)
            .height(box_h)
            .center_y(Length::Fixed(box_h)),
        container(handle_layer)
            .width(Length::Fill)
            .height(box_h)
            .center_y(Length::Fixed(box_h)),
        container(interactive)
            .width(Length::Fill)
            .height(box_h)
            .center_y(Length::Fixed(box_h)),
    ]
    .width(Length::Fill)
    .height(Length::Fixed(box_h))
    .into()
}

fn settings_focus_row<'a>(
    is_focused: bool,
    accent: Color,
    content: Element<'a, RuntimeMessage>,
) -> Element<'a, RuntimeMessage> {
    container(content)
        .width(Length::Fill)
        .padding([6.0, 10.0])
        .style(move |_| container::Style {
            background: if is_focused {
                Some(Background::Color(Color::from_rgba(accent.r, accent.g, accent.b, 0.14)))
            } else {
                None
            },
            border: Border {
                radius: 8.0.into(),
                width: if is_focused { 1.5 } else { 0.0 },
                color: if is_focused { accent } else { Color::TRANSPARENT },
            },
            ..Default::default()
        })
        .into()
}

fn wrap_settings_panel_box<'a>(
    engine: &'a RuntimeEngine,
    theme: ResolvedTheme,
    show_panel: bool,
    panel_asset_path: Option<&PathBuf>,
    inner: Element<'a, RuntimeMessage>,
    panel_w: f32,
) -> Element<'a, RuntimeMessage> {
    if show_panel {
        if let Some(sp_path) = panel_asset_path {
            container(stack![
                image(load_image_handle(engine, sp_path)).content_fit(ContentFit::Fill),
                container(inner).width(Length::Fill).padding([24.0, 32.0]),
            ])
            .width(Length::Fixed(panel_w))
            .into()
        } else {
            container(inner)
                .width(Length::Fixed(panel_w))
                .padding([24.0, 32.0])
                .style(themed_panel_style(theme.panel_bg, theme.radius))
                .into()
        }
    } else {
        container(inner)
            .width(Length::Fixed(panel_w))
            .padding([12.0, 18.0])
            .into()
    }
}

fn view_settings(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    let ui = engine.ui_config();
    let p = &ui.pause_settings;
    let theme = with_button_color(resolve_theme(&ui), &p.settings_button_text_hex, "");
    let settings_text_col = resolve_text_color(&p.settings_text_hex, theme.text_main);

    let vol_slider_row = |label: String, current: f32, make_msg: fn(f32) -> RuntimeMessage| {
        let pct = (current * 100.0).round() as u32;
        row![
            text(label).size(15).color(settings_text_col).width(Length::Fixed(130.0)),
            themed_slider(engine, theme, 0.0..=100.0, current * 100.0, 1.0_f32, move |v| {
                make_msg(v / 100.0)
            }),
            text(format!("{}%", pct))
                .size(13)
                .color(TEXT_MUTED)
                .width(Length::Fixed(42.0))
                .align_x(Alignment::End),
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .align_y(Alignment::Center)
    };

    let speed_presets = row![
        themed_small_button(engine, theme, "Slow", 82.0, (engine.text_speed - 0.5).abs() < 0.1, RuntimeMessage::SetTextSpeed(0.5)),
        themed_small_button(engine, theme, "Normal", 82.0, (engine.text_speed - 1.0).abs() < 0.1, RuntimeMessage::SetTextSpeed(1.0)),
        themed_small_button(engine, theme, "Fast", 82.0, (engine.text_speed - 1.5).abs() < 0.1, RuntimeMessage::SetTextSpeed(1.5)),
        themed_small_button(engine, theme, "Instant", 82.0, engine.text_speed >= 2.0, RuntimeMessage::SetTextSpeed(2.0)),
    ]
    .spacing(8.0);

    let auto_delay_row = row![
        text(engine.tr_or("ui.settings.auto_delay", "Auto Delay"))
            .size(15)
            .color(settings_text_col)
            .width(Length::Fixed(130.0)),
        themed_slider(
            engine,
            theme,
            0.5..=5.0,
            engine.auto_forward_delay,
            0.1_f32,
            RuntimeMessage::SetAutoForwardDelay,
        ),
        text(format!("{:.1}s", engine.auto_forward_delay))
            .size(13)
            .color(TEXT_MUTED)
            .width(Length::Fixed(42.0))
            .align_x(Alignment::End),
    ]
    .spacing(12.0)
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let skip_mode_row = row![
        text(engine.tr_or("ui.settings.skip_mode", "Skip Mode"))
            .size(15)
            .color(settings_text_col)
            .width(Length::Fixed(130.0)),
        row![
            themed_small_button(
                engine,
                theme,
                engine.tr_or("ui.settings.skip_read", "Read Only"),
                118.0,
                engine.skip_rule == SkipMode::ReadOnly,
                RuntimeMessage::SetSkipMode(SkipMode::ReadOnly),
            ),
            themed_small_button(
                engine,
                theme,
                engine.tr_or("ui.settings.skip_all", "All Text"),
                118.0,
                engine.skip_rule == SkipMode::All,
                RuntimeMessage::SetSkipMode(SkipMode::All),
            ),
        ]
        .spacing(8.0),
    ]
    .spacing(12.0)
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let display_mode_row = row![
        text(engine.tr_or("ui.settings.display_mode", "Display Mode"))
            .size(15)
            .color(settings_text_col)
            .width(Length::Fixed(130.0)),
        row![
            themed_small_button(
                engine,
                theme,
                engine.tr_or("ui.settings.windowed", "Windowed"),
                118.0,
                !engine.is_fullscreen,
                RuntimeMessage::SetFullscreen(false),
            ),
            themed_small_button(
                engine,
                theme,
                engine.tr_or("ui.settings.fullscreen", "Fullscreen"),
                118.0,
                engine.is_fullscreen,
                RuntimeMessage::SetFullscreen(true),
            ),
        ]
        .spacing(8.0),
    ]
    .spacing(12.0)
    .width(Length::Fill)
    .align_y(Alignment::Center);

    let focus_items = engine.settings_focus_items();
    let focused_idx = engine.focus_index % focus_items.len().max(1);
    let focused_item = focus_items.get(focused_idx).copied();
    let wrap_focus_row = |item: SettingsFocusItem, content| {
        settings_focus_row(focused_item == Some(item), theme.accent, content)
    };

    let mut audio_rows: Vec<Element<'_, RuntimeMessage>> = vec![
        wrap_focus_row(
            SettingsFocusItem::MasterVol,
            vol_slider_row(
                engine.tr_or("ui.settings.master_vol", &p.master_volume_label),
                engine.master_volume,
                RuntimeMessage::SetMasterVolume,
            )
            .into(),
        ),
        wrap_focus_row(
            SettingsFocusItem::BgmVol,
            vol_slider_row(
                engine.tr_or("ui.settings.bgm_vol", &p.bgm_volume_label),
                engine.bgm_volume,
                RuntimeMessage::SetBgmVolume,
            )
            .into(),
        ),
        wrap_focus_row(
            SettingsFocusItem::SfxVol,
            vol_slider_row(
                engine.tr_or("ui.settings.sfx_vol", &p.sfx_volume_label),
                engine.sfx_volume,
                RuntimeMessage::SetSfxVolume,
            )
            .into(),
        ),
    ];
    if engine.has_voice_acting {
        audio_rows.push(wrap_focus_row(
            SettingsFocusItem::VoiceVol,
            vol_slider_row(
                engine.tr_or("ui.settings.voice_vol", "Voice Volume"),
                engine.voice_volume,
                RuntimeMessage::SetVoiceVolume,
            )
            .into(),
        ));
    }

    let gameplay_rows: Vec<Element<'_, RuntimeMessage>> = vec![
        wrap_focus_row(
            SettingsFocusItem::TextSpeed,
            row![
                text(engine.tr_or("ui.settings.text_speed", &p.text_speed_label))
                    .size(15)
                    .color(settings_text_col)
                    .width(Length::Fixed(130.0)),
                speed_presets,
            ]
            .spacing(12.0)
            .width(Length::Fill)
            .align_y(Alignment::Center)
            .into(),
        ),
        wrap_focus_row(SettingsFocusItem::AutoDelay, auto_delay_row.into()),
        wrap_focus_row(SettingsFocusItem::SkipMode, skip_mode_row.into()),
    ];

    let mut system_rows: Vec<Element<'_, RuntimeMessage>> = vec![wrap_focus_row(
        SettingsFocusItem::DisplayMode,
        display_mode_row.into(),
    )];
    let langs = engine.available_languages();
    if langs.len() > 1 {
        let current_lang = engine.current_language_entry();
        let lang_row = row![
            text(engine.tr_or("ui.settings.language", "Language"))
                .size(15)
                .color(settings_text_col)
                .width(Length::Fixed(130.0)),
            pick_list(langs, Some(current_lang), |entry: LanguageEntry| {
                RuntimeMessage::SetLanguage(entry.code)
            })
            .width(Length::Fixed(244.0))
            .padding([6, 12]),
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .align_y(Alignment::Center);
        system_rows.push(wrap_focus_row(SettingsFocusItem::Language, lang_row.into()));
    }

    let template = p.settings_layout_template;
    let settings_panel_path = resolve_ui_asset(engine, p.settings_panel_asset_id.as_deref());
    let wrap_settings_panel = |inner, panel_w: f32| {
        wrap_settings_panel_box(
            engine,
            theme,
            p.settings_show_panel,
            settings_panel_path.as_ref(),
            inner,
            panel_w,
        )
    };

    let bg_layer: Element<'_, RuntimeMessage> = if let Some(loaded) = &engine.loaded {
        if let Some(bg_path) = loaded
            .config
            .title_background_asset_id
            .as_deref()
            .and_then(|id| loaded.resolve_asset_path(id, Some(AssetCategory::Background)))
        {
            image(load_image_handle(engine, bg_path))
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(ContentFit::Cover)
                .into()
        } else {
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(dark_backdrop_style)
                .into()
        }
    } else {
        container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(dark_backdrop_style)
            .into()
    };

    let dim_alpha = p.settings_overlay_dimming.clamp(0.0, 0.95);
    let dim = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
            ..Default::default()
        });

    let back_btn = themed_button(
        engine,
        theme,
        None,
        p.back_label.clone(),
        p.settings_back_width.max(110.0),
        focused_item == Some(SettingsFocusItem::Back),
        false,
        None,
        RuntimeMessage::CloseSettings,
    );

    let mut layers: Vec<Element<'_, RuntimeMessage>> = vec![bg_layer, dim.into()];
    match template {
        SettingsLayoutTemplate::Classic => {
            let mut all_rows = Vec::new();
            all_rows.extend(audio_rows);
            all_rows.extend(gameplay_rows);
            all_rows.extend(system_rows);
            let controls_col = column(all_rows).spacing(10.0).width(Length::Fill);
            let controls_widget = wrap_settings_panel(controls_col.into(), p.settings_width);
            if p.settings_show_title && !p.settings_title.trim().is_empty() {
                let title_el = text(p.settings_title.clone()).size(34).color(settings_text_col);
                layers.push(place_with_anchor(
                    title_el.into(),
                    p.settings_title_anchor,
                    p.settings_title_offset_x,
                    p.settings_title_offset_y,
                ));
            }
            layers.push(place_with_anchor(
                controls_widget,
                p.settings_anchor,
                p.settings_offset_x,
                p.settings_offset_y,
            ));
            layers.push(place_with_anchor(
                back_btn,
                p.settings_back_anchor,
                p.settings_back_offset_x,
                p.settings_back_offset_y,
            ));
        }
        SettingsLayoutTemplate::TopTabs
        | SettingsLayoutTemplate::LeftSidebar
        | SettingsLayoutTemplate::RightSidebar => {
            let active_rows = match engine.settings_category {
                SettingsCategory::Audio => audio_rows,
                SettingsCategory::Gameplay => gameplay_rows,
                SettingsCategory::System => system_rows,
            };
            let content_col = column(active_rows).spacing(12.0).width(Length::Fill);
            let content_panel = wrap_settings_panel(content_col.into(), p.settings_width.max(540.0));

            let cat_btn = |cat: SettingsCategory, btn_w: f32| {
                themed_small_button(
                    engine,
                    theme,
                    engine.tr_or(cat.loc_key(), cat.default_label()),
                    btn_w,
                    engine.settings_category == cat,
                    RuntimeMessage::SelectSettingsCategory(cat),
                )
            };

            let title_el: Element<'_, RuntimeMessage> =
                if p.settings_show_title && !p.settings_title.trim().is_empty() {
                    text(p.settings_title.clone()).size(30).color(settings_text_col).into()
                } else {
                    Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
                };

            let layout_root: Element<'_, RuntimeMessage> = match template {
                SettingsLayoutTemplate::TopTabs => {
                    let tabs_row = row![
                        cat_btn(SettingsCategory::Audio, 130.0),
                        cat_btn(SettingsCategory::Gameplay, 130.0),
                        cat_btn(SettingsCategory::System, 130.0),
                    ]
                    .spacing(10.0)
                    .align_y(Alignment::Center);
                    let header = row![back_btn, title_el]
                        .spacing(18.0)
                        .width(Length::Fill)
                        .align_y(Alignment::Center);
                    column![
                        header,
                        container(tabs_row).width(Length::Fill).center_x(Length::Fill),
                        container(content_panel)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill),
                    ]
                    .spacing(18.0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(28.0)
                    .into()
                }
                SettingsLayoutTemplate::LeftSidebar => {
                    let header = row![back_btn, title_el]
                        .spacing(18.0)
                        .width(Length::Fill)
                        .align_y(Alignment::Center);
                    let sidebar = column![
                        cat_btn(SettingsCategory::Audio, 180.0),
                        cat_btn(SettingsCategory::Gameplay, 180.0),
                        cat_btn(SettingsCategory::System, 180.0),
                    ]
                    .spacing(10.0)
                    .width(Length::Fixed(200.0));
                    let body = row![
                        sidebar,
                        container(content_panel)
                            .width(Length::Fill)
                            .center_x(Length::Fill),
                    ]
                    .spacing(24.0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Alignment::Start);
                    column![header, body]
                        .spacing(22.0)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(28.0)
                        .into()
                }
                SettingsLayoutTemplate::RightSidebar => {
                    let header = row![title_el, Space::with_width(Length::Fill), back_btn]
                        .spacing(18.0)
                        .width(Length::Fill)
                        .align_y(Alignment::Center);
                    let sidebar = column![
                        cat_btn(SettingsCategory::Audio, 180.0),
                        cat_btn(SettingsCategory::Gameplay, 180.0),
                        cat_btn(SettingsCategory::System, 180.0),
                    ]
                    .spacing(10.0)
                    .width(Length::Fixed(200.0));
                    let body = row![
                        container(content_panel)
                            .width(Length::Fill)
                            .center_x(Length::Fill),
                        sidebar,
                    ]
                    .spacing(24.0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Alignment::Start);
                    column![header, body]
                        .spacing(22.0)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(28.0)
                        .into()
                }
                _ => Space::new(Length::Fill, Length::Fill).into(),
            };
            layers.push(layout_root);
        }
        SettingsLayoutTemplate::TwoColumnSplit => {
            let title_el: Element<'_, RuntimeMessage> =
                if p.settings_show_title && !p.settings_title.trim().is_empty() {
                    text(p.settings_title.clone()).size(30).color(settings_text_col).into()
                } else {
                    Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
                };
            let header = row![back_btn, title_el]
                .spacing(18.0)
                .width(Length::Fill)
                .align_y(Alignment::Center);
            let left_col = wrap_settings_panel(
                column(audio_rows).spacing(10.0).width(Length::Fill).into(),
                460.0,
            );
            let mut right_rows = Vec::new();
            right_rows.extend(gameplay_rows);
            right_rows.extend(system_rows);
            let right_col = wrap_settings_panel(
                column(right_rows).spacing(10.0).width(Length::Fill).into(),
                460.0,
            );
            let body = container(row![left_col, right_col].spacing(24.0).align_y(Alignment::Start))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill);
            layers.push(
                column![header, body]
                    .spacing(20.0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(28.0)
                    .into(),
            );
        }
    }

    stack(layers).into()
}

fn build_save_thumbnail<'a>(
    engine: &'a RuntimeEngine,
    data: Option<&crate::engine::SaveSlotData>,
    width: f32,
    height: f32,
    radius: f32,
    text_col: Color,
) -> Element<'a, RuntimeMessage> {
    let thumb_radius = (radius * 0.5).max(4.0);
    let mut layers: Vec<Element<'a, RuntimeMessage>> = Vec::new();

    if let Some(d) = data {
        if let Some(bg_path) = d
            .thumbnail_bg
            .as_deref()
            .and_then(|s| engine.resolve_save_asset_path(s))
        {
            layers.push(
                image(load_image_handle(engine, bg_path))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Cover)
                    .into(),
            );
        }

        if !d.thumbnail_characters.is_empty() {
            let mut left_slot: Option<PathBuf> = None;
            let mut center_slot: Option<PathBuf> = None;
            let mut right_slot: Option<PathBuf> = None;
            for ch in &d.thumbnail_characters {
                if let Some(p) = engine.resolve_save_asset_path(&ch.image_path) {
                    match ch.position {
                        blanc_core::scene::CharacterPosition::Left => left_slot = Some(p),
                        blanc_core::scene::CharacterPosition::Right => right_slot = Some(p),
                        _ => center_slot = Some(p),
                    }
                }
            }
            let make_char = |path: Option<PathBuf>| -> Element<'a, RuntimeMessage> {
                if let Some(p) = path {
                    image(load_image_handle(engine, p))
                        .width(Length::FillPortion(1))
                        .height(Length::Fill)
                        .content_fit(ContentFit::Contain)
                        .into()
                } else {
                    Space::with_width(Length::FillPortion(1)).into()
                }
            };
            layers.push(
                row![make_char(left_slot), make_char(center_slot), make_char(right_slot)]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Alignment::End)
                    .into(),
            );
        }
    }

    let content: Element<'a, RuntimeMessage> = if layers.is_empty() {
        let label = if data.is_some() { "No Preview" } else { "Empty" };
        container(
            text(label)
                .size(10)
                .color(Color::from_rgba(text_col.r, text_col.g, text_col.b, 0.45)),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        stack(layers).into()
    };

    container(content)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .clip(true)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.35))),
            border: Border {
                radius: thumb_radius.into(),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.16),
            },
            ..Default::default()
        })
        .into()
}

fn view_save_load(engine: &RuntimeEngine, mode: SaveLoadMode) -> Element<'_, RuntimeMessage> {
    let ui = engine.ui_config();
    let sl = &ui.save_load;
    let theme = with_button_color(resolve_theme(&ui), &sl.button_text_hex, "");
    let slots = engine.list_save_slots();

    let title_str = match mode {
        SaveLoadMode::Save => sl.save_title.clone(),
        SaveLoadMode::Load => sl.load_title.clone(),
    };
    let sl_title_col = resolve_text_color(&sl.title_text_hex, theme.text_main);
    let slot_text_col = resolve_text_color(&sl.slot_text_hex, theme.text_main);

    let card_asset_path = resolve_ui_asset(engine, sl.slot_card_asset_id.as_deref());
    let card_disabled_path = resolve_ui_asset(engine, sl.slot_card_disabled_asset_id.as_deref());
    let is_list_layout = matches!(sl.slot_layout, blanc_core::ui_config::SaveLoadSlotLayout::List);
    let card_height = if is_list_layout { 88.0 } else { 154.0 };
    let thumb_w = if is_list_layout { 96.0 } else { 104.0 };
    let thumb_h = if is_list_layout { 58.0 } else { 66.0 };

    let total_sl_focus = slots.len() + 1;
    let sl_focused = engine.focus_index % total_sl_focus.max(1);

    let build_slot_card = |idx: usize, data: Option<crate::engine::SaveSlotData>| -> Element<'_, RuntimeMessage> {
        let slot_num = idx + 1;
        let is_slot_focused = sl_focused == idx;
        let thumb_widget = build_save_thumbnail(
            engine,
            data.as_ref(),
            thumb_w,
            thumb_h,
            theme.radius,
            slot_text_col,
        );
        let header = row![
            text(format!("Slot {}", slot_num)).size(15).color(theme.accent),
            Space::with_width(Length::Fill),
            if let Some(ref d) = data {
                text(d.timestamp_label.clone()).size(10).color(slot_text_col)
            } else {
                text(sl.empty_slot_label.clone()).size(10).color(slot_text_col)
            },
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center);

        let body: Element<'_, RuntimeMessage> = if let Some(ref d) = data {
            column![
                text(d.scene_name.clone()).size(13).color(slot_text_col),
                text(d.preview_text.clone()).size(11).color(slot_text_col),
            ]
            .spacing(3.0)
            .into()
        } else {
            column![
                text(sl.empty_slot_label.clone()).size(12).color(slot_text_col),
            ]
            .spacing(3.0)
            .into()
        };

        let mut action_row = row![].spacing(8.0).align_y(Alignment::Center);
        let action_idx = if data.is_some() {
            engine.save_load_action_index.min(1)
        } else {
            0
        };
        match mode {
            SaveLoadMode::Save => {
                let lbl = if data.is_some() {
                    sl.overwrite_btn_label.as_str()
                } else {
                    sl.save_btn_label.as_str()
                };
                action_row = action_row.push(themed_button(
                    engine,
                    theme,
                    None,
                    lbl,
                    118.0,
                    is_slot_focused && action_idx == 0,
                    false,
                    None,
                    RuntimeMessage::SaveToSlot(slot_num),
                ));
            }
            SaveLoadMode::Load => {
                action_row = action_row.push(themed_button(
                    engine,
                    theme,
                    None,
                    &sl.load_btn_label,
                    118.0,
                    is_slot_focused && action_idx == 0,
                    data.is_none(),
                    None,
                    RuntimeMessage::LoadFromSlot(slot_num),
                ));
            }
        }
        if data.is_some() {
            action_row = action_row.push(themed_small_button(
                engine,
                theme,
                sl.delete_btn_label.clone(),
                76.0,
                is_slot_focused && action_idx == 1,
                RuntimeMessage::DeleteSaveSlot(slot_num),
            ));
        }

        let inner_content: Element<'_, RuntimeMessage> = if is_list_layout {
            row![
                thumb_widget,
                column![header, body].spacing(4.0).width(Length::Fill),
                action_row,
            ]
            .spacing(12.0)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            column![
                row![
                    thumb_widget,
                    column![header, body].spacing(3.0).width(Length::Fill),
                ]
                .spacing(10.0)
                .align_y(Alignment::Start)
                .width(Length::Fill),
                Space::with_height(Length::Fill),
                action_row,
            ]
            .spacing(8.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        };

        let slot_bg_path = if matches!(mode, SaveLoadMode::Load) && data.is_none() {
            card_disabled_path.as_ref().or(card_asset_path.as_ref())
        } else {
            card_asset_path.as_ref()
        };

        let card_el: Element<'_, RuntimeMessage> = if let Some(card_path) = slot_bg_path {
            container(
                stack![
                    image(load_image_handle(engine, card_path)).content_fit(ContentFit::Fill),
                    container(inner_content).width(Length::Fill).height(Length::Fill).padding(14.0),
                ],
            )
            .width(Length::FillPortion(1))
            .height(Length::Fixed(card_height))
            .into()
        } else {
            container(inner_content)
                .width(Length::FillPortion(1))
                .height(Length::Fixed(card_height))
                .padding(14.0)
                .style(themed_panel_style(theme.panel_bg, theme.radius))
                .into()
        };

        if is_slot_focused {
            let accent = theme.accent;
            let radius = theme.radius;
            container(card_el)
                .width(Length::FillPortion(1))
                .style(move |_| container::Style {
                    border: Border {
                        radius: radius.into(),
                        width: 2.0,
                        color: accent,
                    },
                    ..Default::default()
                })
                .into()
        } else {
            card_el
        }
    };

    let cols_per_row = match sl.slot_layout {
        blanc_core::ui_config::SaveLoadSlotLayout::TwoColumns => 2,
        blanc_core::ui_config::SaveLoadSlotLayout::ThreeColumns => 3,
        blanc_core::ui_config::SaveLoadSlotLayout::List => 1,
    };

    let mut rows_col = column![].spacing(if is_list_layout { 8.0 } else { 12.0 }).width(Length::Fill);
    for (chunk_idx, items) in slots.chunks(cols_per_row).enumerate() {
        if cols_per_row == 1 {
            rows_col = rows_col.push(build_slot_card(chunk_idx, items.first().cloned().flatten()));
        } else {
            let mut row_items: Vec<Element<'_, RuntimeMessage>> = Vec::with_capacity(cols_per_row);
            for offset in 0..cols_per_row {
                if let Some(item) = items.get(offset) {
                    row_items.push(build_slot_card(chunk_idx * cols_per_row + offset, item.clone()));
                } else {
                    row_items.push(Space::new(Length::FillPortion(1), Length::Shrink).into());
                }
            }
            rows_col = rows_col.push(row(row_items).spacing(12.0).width(Length::Fill));
        }
    }

    let panel_width = sl.panel_width.max(420.0);
    let slots_widget: Element<'_, RuntimeMessage> = if sl.show_panel {
        if let Some(pp_path) = resolve_ui_asset(engine, sl.panel_asset_id.as_deref()) {
            container(
                stack![
                    image(load_image_handle(engine, pp_path)).content_fit(ContentFit::Fill),
                    container(rows_col).width(Length::Fill).padding(22.0),
                ],
            )
            .width(Length::Fixed(panel_width))
            .into()
        } else {
            container(rows_col)
                .width(Length::Fixed(panel_width))
                .padding(22.0)
                .style(themed_panel_style(theme.panel_bg, theme.radius))
                .into()
        }
    } else {
        container(rows_col)
            .width(Length::Fixed(panel_width))
            .into()
    };

    let bg_layer: Element<'_, RuntimeMessage> = if let Some(loaded) = &engine.loaded {
        if let Some(bg_path) = loaded
            .config
            .title_background_asset_id
            .as_deref()
            .and_then(|id| loaded.resolve_asset_path(id, Some(AssetCategory::Background)))
        {
            image(load_image_handle(engine, bg_path))
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(ContentFit::Cover)
                .into()
        } else {
            container(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(dark_backdrop_style)
                .into()
        }
    } else {
        container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(dark_backdrop_style)
            .into()
    };

    let dim_alpha = sl.overlay_dimming.clamp(0.0, 0.95);
    let dim = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, dim_alpha))),
            ..Default::default()
        });

    let back_btn = themed_button(
        engine,
        theme,
        None,
        &sl.back_label,
        sl.back_width.max(110.0),
        sl_focused == slots.len(),
        false,
        None,
        RuntimeMessage::CloseSaveLoad,
    );

    let mut layers: Vec<Element<'_, RuntimeMessage>> = vec![bg_layer, dim.into()];
    match sl.layout_template {
        SaveLoadLayoutTemplate::Classic => {
            if sl.show_title && !title_str.trim().is_empty() {
                layers.push(place_with_anchor(
                    text(title_str).size(32).color(sl_title_col).into(),
                    sl.title_anchor,
                    sl.title_offset_x,
                    sl.title_offset_y,
                ));
            }
            layers.push(place_with_anchor(
                slots_widget,
                sl.panel_anchor,
                sl.panel_offset_x,
                sl.panel_offset_y,
            ));
            layers.push(place_with_anchor(
                back_btn,
                sl.back_anchor,
                sl.back_offset_x,
                sl.back_offset_y,
            ));
        }
        SaveLoadLayoutTemplate::TopBar => {
            let mut top_row = row![back_btn].spacing(18.0).align_y(Alignment::Center);
            if sl.show_title && !title_str.trim().is_empty() {
                top_row = top_row.push(text(title_str).size(30).color(sl_title_col));
            }
            let layout_col = column![
                top_row.width(Length::Fill),
                container(slots_widget).center_x(Length::Fill).center_y(Length::Fill),
            ]
            .spacing(16.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(28.0);
            layers.push(layout_col.into());
        }
        SaveLoadLayoutTemplate::LeftSidebar | SaveLoadLayoutTemplate::RightSidebar => {
            let mut sidebar = column![back_btn].spacing(18.0).align_x(Alignment::Start).width(Length::Fixed(220.0));
            if sl.show_title && !title_str.trim().is_empty() {
                sidebar = sidebar.push(text(title_str).size(28).color(sl_title_col));
            }
            let main_area = container(slots_widget).width(Length::Fill).center_x(Length::Fill).center_y(Length::Fill);
            let split = if sl.layout_template == SaveLoadLayoutTemplate::LeftSidebar {
                row![sidebar, main_area]
            } else {
                row![main_area, sidebar]
            }
            .spacing(24.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(28.0)
            .align_y(Alignment::Start);
            layers.push(split.into());
        }
        SaveLoadLayoutTemplate::BottomStrip => {
            let mut top_row = row![back_btn].spacing(18.0).align_y(Alignment::Center);
            if sl.show_title && !title_str.trim().is_empty() {
                top_row = top_row.push(text(title_str).size(30).color(sl_title_col));
            }
            let layout_col = column![
                top_row.width(Length::Fill),
                Space::with_height(Length::Fill),
                container(slots_widget).width(Length::Fill).center_x(Length::Fill),
            ]
            .spacing(16.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(28.0);
            layers.push(layout_col.into());
        }
    }

    stack(layers).into()
}

fn view_backlog(engine: &RuntimeEngine) -> Element<'_, RuntimeMessage> {
    let ui = engine.ui_config();
    let h = &ui.history;
    let theme = with_button_color(resolve_theme(&ui), &h.button_text_hex, "");

    let title_col = resolve_text_color(&h.title_text_hex, theme.text_main);
    let speaker_col = resolve_text_color(&h.speaker_text_hex, theme.accent);
    let entry_col = resolve_text_color(&h.entry_text_hex, theme.text_main);
    let card_skin = resolve_ui_asset(engine, h.entry_card_asset_id.as_deref());

    let mut list_col = column![].spacing(12.0).width(Length::Fill);
    if engine.stage.backlog.is_empty() {
        let empty_lbl = if h.empty_label.trim().is_empty() {
            "No dialogue history yet."
        } else {
            h.empty_label.as_str()
        };
        list_col = list_col.push(text(empty_lbl.to_string()).size(15).color(TEXT_MUTED));
    } else {
        let narr_lbl = if h.narration_label.trim().is_empty() {
            "Narration"
        } else {
            h.narration_label.as_str()
        };
        for entry in engine.stage.backlog.iter() {
            let speaker_line: Element<'_, RuntimeMessage> = match &entry.speaker {
                Some(spk) => text(spk.clone()).size(14).color(speaker_col).into(),
                None => text(narr_lbl.to_string()).size(12).color(TEXT_MUTED).into(),
            };
            let entry_body = column![speaker_line, text(entry.text.clone()).size(15).color(entry_col)]
                .spacing(4.0)
                .width(Length::Fill);
            let card: Element<'_, RuntimeMessage> = if let Some(skin_path) = &card_skin {
                container(stack![
                    image(load_image_handle(engine, skin_path))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Fill),
                    container(entry_body).width(Length::Fill).padding([10.0, 14.0]),
                ])
                .width(Length::Fill)
                .into()
            } else {
                container(entry_body)
                    .width(Length::Fill)
                    .padding([10.0, 14.0])
                    .style(themed_panel_style(
                        Color::from_rgba(0.08, 0.09, 0.13, 0.75),
                        theme.radius * 0.6,
                    ))
                    .into()
            };
            list_col = list_col.push(card);
        }
    }

    let title_lbl = if h.title_label.trim().is_empty() {
        "Dialogue History"
    } else {
        h.title_label.as_str()
    };
    let back_lbl = if h.back_label.trim().is_empty() {
        "Back"
    } else {
        h.back_label.as_str()
    };

    let back_btn = themed_button(
        engine,
        theme,
        None,
        back_lbl,
        130.0,
        true,
        false,
        None,
        RuntimeMessage::CloseBacklog,
    );

    let header_row = match h.layout_template {
        HistoryLayoutTemplate::TopBarReader => row![
            back_btn,
            text(title_lbl.to_string()).size(26).color(title_col),
            Space::with_width(Length::Fill),
        ]
        .spacing(16.0)
        .width(Length::Fill)
        .align_y(Alignment::Center),
        _ => row![
            text(title_lbl.to_string()).size(24).color(title_col),
            Space::with_width(Length::Fill),
            back_btn,
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center),
    };

    let inner_col = column![
        header_row,
        scrollable(list_col)
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .spacing(16.0)
    .width(Length::Fill)
    .height(Length::Fill);

    let (panel_w, panel_h_len): (Length, Length) = match h.layout_template {
        HistoryLayoutTemplate::CenteredModal => (
            Length::Fixed(h.panel_width.clamp(420.0, 1180.0)),
            Length::Fixed(h.panel_height.clamp(300.0, 680.0)),
        ),
        HistoryLayoutTemplate::TopBarReader => (Length::Fill, Length::Fill),
        HistoryLayoutTemplate::LeftDrawer | HistoryLayoutTemplate::RightDrawer => (
            Length::Fixed(h.panel_width.clamp(380.0, 560.0)),
            Length::Fill,
        ),
    };

    let panel: Element<'_, RuntimeMessage> = if h.show_panel {
        if let Some(pp_path) = resolve_ui_asset(engine, h.panel_asset_id.as_deref()) {
            container(stack![
                image(load_image_handle(engine, pp_path))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Fill),
                container(inner_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(26.0),
            ])
            .width(panel_w)
            .height(panel_h_len)
            .into()
        } else {
            container(inner_col)
                .width(panel_w)
                .height(panel_h_len)
                .padding(26.0)
                .style(themed_panel_style(theme.panel_bg, theme.radius))
                .into()
        }
    } else {
        container(inner_col)
            .width(panel_w)
            .height(panel_h_len)
            .padding(12.0)
            .into()
    };

    let dim_alpha = h.overlay_dimming.clamp(0.0, 0.95);
    let overlay_base = container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.07, dim_alpha))),
            ..Default::default()
        });
    let overlay: Element<'_, RuntimeMessage> = match h.layout_template {
        HistoryLayoutTemplate::CenteredModal => overlay_base.center_x(Length::Fill).center_y(Length::Fill).into(),
        HistoryLayoutTemplate::TopBarReader => overlay_base.padding(28.0).into(),
        HistoryLayoutTemplate::LeftDrawer => overlay_base.align_x(Alignment::Start).padding(18.0).into(),
        HistoryLayoutTemplate::RightDrawer => overlay_base.align_x(Alignment::End).padding(18.0).into(),
    };

    stack![view_stage_visual(engine, theme), overlay].into()
}

fn view_ending<'a>(engine: &'a RuntimeEngine, ending_title: &'a str) -> Element<'a, RuntimeMessage> {
    let ui = engine.ui_config();
    let e = &ui.ending;
    let theme = with_button_color(resolve_theme(&ui), &e.button_text_hex, &e.button_text_hex);

    let title_col = parse_hex_rgb(&e.title_text_hex)
        .map(|(r, g, b)| Color::from_rgb8(r, g, b))
        .unwrap_or(theme.text_main);
    let subtitle_col = parse_hex_rgb(&e.subtitle_text_hex)
        .map(|(r, g, b)| Color::from_rgb8(r, g, b))
        .unwrap_or(TEXT_MUTED);

    let title_lbl = if e.title_label.trim().is_empty() {
        ending_title.to_string()
    } else {
        e.title_label.clone()
    };
    let subtitle_lbl = if e.subtitle_label.trim().is_empty() {
        "End of Story Arc"
    } else {
        e.subtitle_label.as_str()
    };
    let btn_lbl = if e.button_label.trim().is_empty() {
        "Continue to Title Screen"
    } else {
        e.button_label.as_str()
    };

    let btn_w = e.button_width.clamp(160.0, 480.0);
    let panel_w = e.panel_width.clamp(320.0, 960.0);
    let content_align = match e.content_align {
        0 => Alignment::Start,
        2 => Alignment::End,
        _ => Alignment::Center,
    };

    let mut content_col = column![].spacing(12.0).align_x(content_align).width(Length::Fill);
    if e.show_logo {
        content_col = content_col.push(project_logo_widget(engine, 56.0));
    }
    content_col = content_col
        .push(text(title_lbl).size(28).color(title_col))
        .push(text(subtitle_lbl.to_string()).size(14).color(subtitle_col))
        .push(Space::with_height(14.0))
        .push(themed_button(
            engine,
            theme,
            None,
            btn_lbl,
            btn_w,
            true,
            false,
            None,
            RuntimeMessage::FollowEndingTitle,
        ));

    let card: Element<'_, RuntimeMessage> = if e.show_panel {
        if let Some(pp_path) = resolve_ui_asset(engine, e.panel_asset_id.as_deref()) {
            container(stack![
                image(load_image_handle(engine, &pp_path))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Fill),
                container(content_col)
                    .width(Length::Fill)
                    .padding([36.0, 48.0]),
            ])
            .width(Length::Fixed(panel_w))
            .into()
        } else {
            container(content_col)
                .width(Length::Fixed(panel_w))
                .padding([36.0, 48.0])
                .style(themed_panel_style(theme.panel_bg, theme.radius))
                .into()
        }
    } else {
        container(content_col)
            .width(Length::Fixed(panel_w))
            .padding([36.0, 48.0])
            .into()
    };

    let dim_alpha = e.overlay_dimming.clamp(0.0, 0.95);
    let dim_layer = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.03, 0.04, 0.07, dim_alpha))),
            ..Default::default()
        });
    let card_placed = place_with_anchor(card, e.panel_anchor, e.panel_offset_x, e.panel_offset_y);

    let custom_bg = e.background_asset_id.as_deref().and_then(|id| {
        engine
            .loaded
            .as_ref()
            .and_then(|l| l.resolve_asset_path(id, Some(AssetCategory::Background)))
    });
    let scaling = engine
        .loaded
        .as_ref()
        .map(|l| l.config.aspect_scaling_mode)
        .unwrap_or(AspectScalingMode::Cover);

    if let Some(bg_path) = custom_bg {
        stack![render_background(engine, Some(&bg_path), scaling), dim_layer, card_placed].into()
    } else if e.use_stage_backdrop {
        stack![view_stage_visual(engine, theme), dim_layer, card_placed].into()
    } else {
        let title_bg = engine.loaded.as_ref().and_then(|l| {
            l.config
                .title_background_asset_id
                .as_deref()
                .and_then(|id| l.resolve_asset_path(id, Some(AssetCategory::Background)))
        });
        stack![render_background(engine, title_bg.as_ref(), scaling), dim_layer, card_placed].into()
    }
}

fn view_error(err: &str) -> Element<'_, RuntimeMessage> {
    let card = container(
        column![
            default_logo_widget(56.0),
            text("Blanc Runtime").size(24).color(Color::WHITE),
            text(err).size(14).color(TEXT_MUTED),
            Space::with_height(10.0),
            button(text("Close").size(14))
                .padding([10.0, 24.0])
                .on_press(RuntimeMessage::QuitGame),
        ]
        .spacing(10.0)
        .align_x(Alignment::Center),
    )
    .padding([32.0, 44.0])
    .style(themed_panel_style(Color::from_rgba(0.07, 0.08, 0.12, 0.92), 12.0));

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(dark_backdrop_style)
        .into()
}
