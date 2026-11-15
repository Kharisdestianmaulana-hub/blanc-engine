use iced::mouse::{self, Cursor};
use iced::widget::canvas::{self, Canvas, Event, Fill, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

use blanc_core::theme::colors::*;
use blanc_core::theme::spacing::*;
use blanc_core::ui_config::{parse_hex_rgb, RuntimeUiConfig};

fn rgb(c: blanc_core::theme::ColorRgb) -> Color {
    Color::from_rgb8(c.r, c.g, c.b)
}

use crate::screens::dashboard::DashboardMessage;
use crate::screens::ui_designer::UiDesignerMessage;
use crate::theme::{input_field_style, secondary_button_style, subtle_button_style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiColorTarget {
    Accent,
    Panel,
    GlobalText,
    ButtonText,
    ButtonFocusText,
    SliderTrackBg,
    SliderFill,
    SliderHandle,
    MainMenuText,
    MainMenuButtonText,
    MainMenuButtonFocusText,
    DialogueText,
    NameplateText,
    ChoiceText,
    HudText,
    PauseText,
    PauseButtonText,
    SettingsText,
    SettingsButtonText,
    SaveLoadTitleText,
    SaveLoadSlotText,
    SaveLoadButtonText,
    HistoryTitleText,
    HistorySpeakerText,
    HistoryEntryText,
    HistoryButtonText,
    EndingTitleText,
    EndingSubtitleText,
    EndingButtonText,
}

impl UiColorTarget {
    pub fn allows_global(self) -> bool {
        !matches!(self, Self::Accent | Self::Panel | Self::GlobalText)
    }

    pub fn get_hex<'a>(self, ui: &'a RuntimeUiConfig) -> &'a str {
        match self {
            Self::Accent => &ui.theme.accent_hex,
            Self::Panel => &ui.theme.panel_hex,
            Self::GlobalText => &ui.theme.text_hex,
            Self::ButtonText => &ui.theme.button_text_hex,
            Self::ButtonFocusText => &ui.theme.button_focus_text_hex,
            Self::SliderTrackBg => &ui.theme.slider_track_bg_hex,
            Self::SliderFill => &ui.theme.slider_fill_hex,
            Self::SliderHandle => &ui.theme.slider_handle_hex,
            Self::MainMenuText => &ui.main_menu.text_hex,
            Self::MainMenuButtonText => &ui.main_menu.button_text_hex,
            Self::MainMenuButtonFocusText => &ui.main_menu.button_focus_text_hex,
            Self::DialogueText => &ui.gameplay.dialogue_text_hex,
            Self::NameplateText => &ui.gameplay.nameplate_text_hex,
            Self::ChoiceText => &ui.gameplay.choice_text_hex,
            Self::HudText => &ui.gameplay.hud_text_hex,
            Self::PauseText => &ui.pause_settings.pause_text_hex,
            Self::PauseButtonText => &ui.pause_settings.pause_button_text_hex,
            Self::SettingsText => &ui.pause_settings.settings_text_hex,
            Self::SettingsButtonText => &ui.pause_settings.settings_button_text_hex,
            Self::SaveLoadTitleText => &ui.save_load.title_text_hex,
            Self::SaveLoadSlotText => &ui.save_load.slot_text_hex,
            Self::SaveLoadButtonText => &ui.save_load.button_text_hex,
            Self::HistoryTitleText => &ui.history.title_text_hex,
            Self::HistorySpeakerText => &ui.history.speaker_text_hex,
            Self::HistoryEntryText => &ui.history.entry_text_hex,
            Self::HistoryButtonText => &ui.history.button_text_hex,
            Self::EndingTitleText => &ui.ending.title_text_hex,
            Self::EndingSubtitleText => &ui.ending.subtitle_text_hex,
            Self::EndingButtonText => &ui.ending.button_text_hex,
        }
    }

    pub fn set_hex(self, ui: &mut RuntimeUiConfig, hex: String) {
        match self {
            Self::Accent => ui.theme.accent_hex = hex,
            Self::Panel => ui.theme.panel_hex = hex,
            Self::GlobalText => ui.theme.text_hex = hex,
            Self::ButtonText => ui.theme.button_text_hex = hex,
            Self::ButtonFocusText => ui.theme.button_focus_text_hex = hex,
            Self::SliderTrackBg => ui.theme.slider_track_bg_hex = hex,
            Self::SliderFill => ui.theme.slider_fill_hex = hex,
            Self::SliderHandle => ui.theme.slider_handle_hex = hex,
            Self::MainMenuText => ui.main_menu.text_hex = hex,
            Self::MainMenuButtonText => ui.main_menu.button_text_hex = hex,
            Self::MainMenuButtonFocusText => ui.main_menu.button_focus_text_hex = hex,
            Self::DialogueText => ui.gameplay.dialogue_text_hex = hex,
            Self::NameplateText => ui.gameplay.nameplate_text_hex = hex,
            Self::ChoiceText => ui.gameplay.choice_text_hex = hex,
            Self::HudText => ui.gameplay.hud_text_hex = hex,
            Self::PauseText => ui.pause_settings.pause_text_hex = hex,
            Self::PauseButtonText => ui.pause_settings.pause_button_text_hex = hex,
            Self::SettingsText => ui.pause_settings.settings_text_hex = hex,
            Self::SettingsButtonText => ui.pause_settings.settings_button_text_hex = hex,
            Self::SaveLoadTitleText => ui.save_load.title_text_hex = hex,
            Self::SaveLoadSlotText => ui.save_load.slot_text_hex = hex,
            Self::SaveLoadButtonText => ui.save_load.button_text_hex = hex,
            Self::HistoryTitleText => ui.history.title_text_hex = hex,
            Self::HistorySpeakerText => ui.history.speaker_text_hex = hex,
            Self::HistoryEntryText => ui.history.entry_text_hex = hex,
            Self::HistoryButtonText => ui.history.button_text_hex = hex,
            Self::EndingTitleText => ui.ending.title_text_hex = hex,
            Self::EndingSubtitleText => ui.ending.subtitle_text_hex = hex,
            Self::EndingButtonText => ui.ending.button_text_hex = hex,
        }
    }
}

pub fn format_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta < 1e-5 {
        0.0
    } else if (max - rf).abs() < 1e-5 {
        60.0 * (((gf - bf) / delta).rem_euclid(6.0))
    } else if (max - gf).abs() < 1e-5 {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };
    let s = if max < 1e-5 { 0.0 } else { delta / max };
    (h, s, max)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0);
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelDragMode {
    #[default]
    None,
    Wheel,
    ValueBar,
}

struct ColorWheelProgram {
    target: UiColorTarget,
    h: f32,
    s: f32,
    v: f32,
}

impl ColorWheelProgram {
    const WHEEL_CX: f32 = 66.0;
    const WHEEL_CY: f32 = 66.0;
    const WHEEL_R: f32 = 60.0;
    const BAR_X: f32 = 148.0;
    const BAR_Y: f32 = 6.0;
    const BAR_W: f32 = 20.0;
    const BAR_H: f32 = 120.0;

    fn pick_wheel(&self, pos: Point) -> (f32, f32) {
        let dx = pos.x - Self::WHEEL_CX;
        let dy = pos.y - Self::WHEEL_CY;
        let dist = (dx * dx + dy * dy).sqrt().min(Self::WHEEL_R);
        let sat = (dist / Self::WHEEL_R).clamp(0.0, 1.0);
        let hue = dy.atan2(dx).to_degrees().rem_euclid(360.0);
        (hue, sat)
    }

    fn pick_value(&self, pos: Point) -> f32 {
        (1.0 - (pos.y - Self::BAR_Y) / Self::BAR_H).clamp(0.0, 1.0)
    }
}

impl Program<DashboardMessage, Theme, Renderer> for ColorWheelProgram {
    type State = WheelDragMode;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<DashboardMessage>) {
        let Some(pos) = cursor.position_in(bounds) else {
            if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
                *state = WheelDragMode::None;
            }
            return (canvas::event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let dx = pos.x - Self::WHEEL_CX;
                let dy = pos.y - Self::WHEEL_CY;
                if dx * dx + dy * dy <= (Self::WHEEL_R + 6.0).powi(2) {
                    *state = WheelDragMode::Wheel;
                    let (nh, ns) = self.pick_wheel(pos);
                    let (r, g, b) = hsv_to_rgb(nh, ns, self.v.max(0.05));
                    return (
                        canvas::event::Status::Captured,
                        Some(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                            self.target,
                            format_hex(r, g, b),
                        ))),
                    );
                } else if pos.x >= Self::BAR_X - 4.0
                    && pos.x <= Self::BAR_X + Self::BAR_W + 4.0
                    && pos.y >= Self::BAR_Y - 4.0
                    && pos.y <= Self::BAR_Y + Self::BAR_H + 4.0
                {
                    *state = WheelDragMode::ValueBar;
                    let nv = self.pick_value(pos);
                    let (r, g, b) = hsv_to_rgb(self.h, self.s, nv);
                    return (
                        canvas::event::Status::Captured,
                        Some(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                            self.target,
                            format_hex(r, g, b),
                        ))),
                    );
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => match *state {
                WheelDragMode::Wheel => {
                    let (nh, ns) = self.pick_wheel(pos);
                    let (r, g, b) = hsv_to_rgb(nh, ns, self.v.max(0.05));
                    return (
                        canvas::event::Status::Captured,
                        Some(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                            self.target,
                            format_hex(r, g, b),
                        ))),
                    );
                }
                WheelDragMode::ValueBar => {
                    let nv = self.pick_value(pos);
                    let (r, g, b) = hsv_to_rgb(self.h, self.s, nv);
                    return (
                        canvas::event::Status::Captured,
                        Some(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                            self.target,
                            format_hex(r, g, b),
                        ))),
                    );
                }
                WheelDragMode::None => {}
            },
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if *state != WheelDragMode::None {
                    *state = WheelDragMode::None;
                    return (canvas::event::Status::Captured, None);
                }
            }
            _ => {}
        }

        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw HSV wheel using polar sectors
        let hue_steps = 48usize;
        let sat_steps = 8usize;
        for si in 0..sat_steps {
            let r0 = Self::WHEEL_R * (si as f32 / sat_steps as f32);
            let r1 = Self::WHEEL_R * ((si + 1) as f32 / sat_steps as f32);
            let sat = (si as f32 + 0.5) / sat_steps as f32;
            for hi in 0..hue_steps {
                let a0 = (hi as f32 / hue_steps as f32) * std::f32::consts::TAU;
                let a1 = ((hi + 1) as f32 / hue_steps as f32) * std::f32::consts::TAU + 0.015;
                let hue = (hi as f32 + 0.5) / hue_steps as f32 * 360.0;
                let (cr, cg, cb) = hsv_to_rgb(hue, sat, 1.0);
                let sector = Path::new(|b| {
                    b.move_to(Point::new(Self::WHEEL_CX + r0 * a0.cos(), Self::WHEEL_CY + r0 * a0.sin()));
                    b.line_to(Point::new(Self::WHEEL_CX + r1 * a0.cos(), Self::WHEEL_CY + r1 * a0.sin()));
                    b.line_to(Point::new(Self::WHEEL_CX + r1 * a1.cos(), Self::WHEEL_CY + r1 * a1.sin()));
                    b.line_to(Point::new(Self::WHEEL_CX + r0 * a1.cos(), Self::WHEEL_CY + r0 * a1.sin()));
                    b.close();
                });
                frame.fill(&sector, Fill::from(Color::from_rgb8(cr, cg, cb)));
            }
        }

        // Wheel outer ring
        let ring = Path::circle(Point::new(Self::WHEEL_CX, Self::WHEEL_CY), Self::WHEEL_R);
        frame.stroke(&ring, Stroke::default().with_width(1.5).with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.25)));

        // Current (H, S) marker on wheel
        let rad = self.h.to_radians();
        let dist = self.s * Self::WHEEL_R;
        let marker = Point::new(Self::WHEEL_CX + dist * rad.cos(), Self::WHEEL_CY + dist * rad.sin());
        frame.stroke(
            &Path::circle(marker, 5.5),
            Stroke::default().with_width(2.0).with_color(Color::BLACK),
        );
        frame.stroke(
            &Path::circle(marker, 4.0),
            Stroke::default().with_width(2.0).with_color(Color::WHITE),
        );

        // Brightness / Value vertical bar
        let steps = 24usize;
        let seg_h = Self::BAR_H / steps as f32;
        for i in 0..steps {
            let v = 1.0 - (i as f32 + 0.5) / steps as f32;
            let (vr, vg, vb) = hsv_to_rgb(self.h, self.s, v);
            let rect = Path::rectangle(
                Point::new(Self::BAR_X, Self::BAR_Y + i as f32 * seg_h),
                Size::new(Self::BAR_W, seg_h + 0.6),
            );
            frame.fill(&rect, Fill::from(Color::from_rgb8(vr, vg, vb)));
        }
        frame.stroke(
            &Path::rectangle(Point::new(Self::BAR_X, Self::BAR_Y), Size::new(Self::BAR_W, Self::BAR_H)),
            Stroke::default().with_width(1.0).with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.28)),
        );

        // Value bar indicator
        let vy = Self::BAR_Y + (1.0 - self.v) * Self::BAR_H;
        frame.stroke(
            &Path::line(Point::new(Self::BAR_X - 2.0, vy), Point::new(Self::BAR_X + Self::BAR_W + 2.0, vy)),
            Stroke::default().with_width(2.5).with_color(Color::WHITE),
        );

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone, Default)]
pub struct ActiveColorPickerState {
    pub target: Option<UiColorTarget>,
    pub r_input: String,
    pub g_input: String,
    pub b_input: String,
}

impl ActiveColorPickerState {
    pub fn sync_from_hex(&mut self, hex: &str) {
        let (r, g, b) = parse_hex_rgb(hex).unwrap_or((248, 250, 252));
        self.r_input = r.to_string();
        self.g_input = g.to_string();
        self.b_input = b.to_string();
    }

    pub fn toggle(&mut self, target: UiColorTarget, ui: &RuntimeUiConfig) {
        if self.target == Some(target) {
            self.target = None;
        } else {
            self.target = Some(target);
            self.sync_from_hex(target.get_hex(ui));
        }
    }
}

pub fn view_color_picker_panel<'a>(
    target: UiColorTarget,
    current_hex: &'a str,
    picker_state: &'a ActiveColorPickerState,
) -> Element<'a, DashboardMessage> {
    let (r, g, b) = parse_hex_rgb(current_hex).unwrap_or((248, 250, 252));
    let (h, s, v) = rgb_to_hsv(r, g, b);

    let wheel = Canvas::new(ColorWheelProgram { target, h, s, v })
        .width(Length::Fixed(178.0))
        .height(Length::Fixed(134.0));

    let rgb_field = |lbl: &'static str, val_str: &'a str, ch: u8| -> Element<'a, DashboardMessage> {
        row![
            text(lbl).size(11).color(rgb(TEXT_MUTED)).width(Length::Fixed(16.0)),
            text_input("0", val_str)
                .on_input(move |txt| {
                    DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorRgbChannel(
                        target,
                        ch,
                        txt,
                    ))
                })
                .padding([4.0, 6.0])
                .width(Length::Fill)
                .style(input_field_style),
        ]
        .spacing(4.0)
        .align_y(Alignment::Center)
        .into()
    };

    let mut action_row = row![Space::with_width(Length::Fill)].spacing(SPACE_1);
    if target.allows_global() {
        action_row = action_row.push(
            button(text("Use Global").size(11))
                .padding([4.0, 8.0])
                .style(secondary_button_style)
                .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(
                    target,
                    String::new(),
                ))),
        );
    }
    action_row = action_row.push(
        button(text("Done").size(11))
            .padding([4.0, 10.0])
            .style(subtle_button_style)
            .on_press(DashboardMessage::UiDesigner(UiDesignerMessage::ToggleColorPicker(target))),
    );

    let right_inputs = column![
        row![
            text("HEX").size(11).color(rgb(TEXT_MUTED)).width(Length::Fixed(28.0)),
            text_input("#ffffff", current_hex)
                .on_input(move |val| {
                    DashboardMessage::UiDesigner(UiDesignerMessage::SetTargetColorHex(target, val))
                })
                .padding([4.0, 6.0])
                .width(Length::Fill)
                .style(input_field_style),
        ]
        .spacing(4.0)
        .align_y(Alignment::Center),
        rgb_field("R", &picker_state.r_input, 0),
        rgb_field("G", &picker_state.g_input, 1),
        rgb_field("B", &picker_state.b_input, 2),
        action_row,
    ]
    .spacing(5.0)
    .width(Length::Fill);

    container(
        row![wheel, right_inputs]
            .spacing(SPACE_3)
            .align_y(Alignment::Center),
    )
    .padding(SPACE_2)
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.28))),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
        },
        ..Default::default()
    })
    .into()
}
