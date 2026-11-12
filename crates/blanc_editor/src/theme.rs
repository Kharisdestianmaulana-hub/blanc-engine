use iced::border::Radius;
use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow, Vector};

use blanc_core::theme::colors::*;
use blanc_core::theme::radius::*;

fn to_iced_color(c: blanc_core::theme::ColorRgb, a: f32) -> Color {
    Color::from_rgba8(c.r, c.g, c.b, a)
}

/// Custom button styling matching Design_System.md
pub fn primary_button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let base = button::Style {
        background: Some(Background::Color(to_iced_color(ACCENT_PRIMARY, 1.0))),
        text_color: to_iced_color(TEXT_ON_ACCENT, 1.0),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(to_iced_color(ACCENT_HOVER, 1.0))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(to_iced_color(ACCENT_PRESSED, 1.0))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 0.5))),
            text_color: to_iced_color(TEXT_DISABLED, 1.0),
            shadow: Shadow::default(),
            ..base
        },
    }
}

pub fn secondary_button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let base = button::Style {
        background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 1.0))),
        text_color: to_iced_color(TEXT_PRIMARY, 1.0),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 1.0),
        },
        shadow: Shadow::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_HOVER, 1.0))),
            border: Border {
                color: to_iced_color(ACCENT_PRIMARY, 0.8),
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_ELEVATED, 1.0))),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: to_iced_color(TEXT_DISABLED, 1.0),
            ..base
        },
    }
}

pub fn card_container_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 1.0))),
        text_color: Some(to_iced_color(TEXT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_LG),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 1.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        },
    }
}

pub fn input_field_style(theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let _ = theme;
    let base = text_input::Style {
        background: Background::Color(to_iced_color(BACKGROUND_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 1.0),
        },
        icon: to_iced_color(TEXT_MUTED, 1.0),
        placeholder: to_iced_color(TEXT_MUTED, 1.0),
        value: to_iced_color(TEXT_PRIMARY, 1.0),
        selection: to_iced_color(ACCENT_SUBTLE, 1.0),
    };

    match status {
        text_input::Status::Active => base,
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: to_iced_color(TEXT_SECONDARY, 0.8),
                ..base.border
            },
            ..base
        },
        text_input::Status::Focused => text_input::Style {
            border: Border {
                color: to_iced_color(ACCENT_PRIMARY, 1.0),
                width: 1.5,
                ..base.border
            },
            ..base
        },
        text_input::Status::Disabled => text_input::Style {
            value: to_iced_color(TEXT_DISABLED, 1.0),
            ..base
        },
    }
}

pub fn sidebar_container_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 1.0))),
        text_color: Some(to_iced_color(TEXT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(0.0),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 1.0),
        },
        shadow: Shadow::default(),
    }
}

pub fn sidebar_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, text_col, border_color) = if is_active {
            (
                to_iced_color(ACCENT_SUBTLE, 1.0),
                to_iced_color(TEXT_PRIMARY, 1.0),
                to_iced_color(ACCENT_PRIMARY, 1.0),
            )
        } else {
            (
                Color::TRANSPARENT,
                to_iced_color(TEXT_SECONDARY, 1.0),
                Color::TRANSPARENT,
            )
        };

        let base = button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border {
                radius: Radius::from(RADIUS_MD),
                width: if is_active { 1.0 } else { 0.0 },
                color: border_color,
            },
            shadow: Shadow::default(),
        };

        match status {
            button::Status::Active => base,
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(to_iced_color(SURFACE_HOVER, 0.9))),
                text_color: to_iced_color(TEXT_PRIMARY, 1.0),
                ..base
            },
            button::Status::Pressed => button::Style {
                background: Some(Background::Color(to_iced_color(SURFACE_ELEVATED, 1.0))),
                ..base
            },
            button::Status::Disabled => button::Style {
                text_color: to_iced_color(TEXT_DISABLED, 1.0),
                ..base
            },
        }
    }
}

pub fn icon_button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: to_iced_color(TEXT_SECONDARY, 1.0),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_HOVER, 1.0))),
            text_color: to_iced_color(TEXT_PRIMARY, 1.0),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_ELEVATED, 1.0))),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: to_iced_color(TEXT_DISABLED, 1.0),
            ..base
        },
    }
}

pub fn subtle_button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: to_iced_color(TEXT_SECONDARY, 1.0),
        border: Border {
            radius: Radius::from(RADIUS_SM),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 0.6),
        },
        shadow: Shadow::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_HOVER, 1.0))),
            text_color: to_iced_color(TEXT_PRIMARY, 1.0),
            border: Border {
                color: to_iced_color(TEXT_MUTED, 1.0),
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(to_iced_color(SURFACE_ELEVATED, 1.0))),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: to_iced_color(TEXT_DISABLED, 1.0),
            ..base
        },
    }
}

pub fn badge_container_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(ACCENT_SUBTLE, 1.0))),
        text_color: Some(to_iced_color(ACCENT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_FULL),
            width: 1.0,
            color: to_iced_color(ACCENT_PRIMARY, 0.4),
        },
        shadow: Shadow::default(),
    }
}

pub fn code_container_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(BACKGROUND_PRIMARY, 1.0))),
        text_color: Some(to_iced_color(TEXT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 1.0),
        },
        shadow: Shadow::default(),
    }
}

pub fn project_card_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 0.95))),
        text_color: Some(to_iced_color(TEXT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 1.0,
            color: to_iced_color(BORDER_DEFAULT, 0.9),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    }
}

pub fn callout_container_style(theme: &iced::Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(to_iced_color(SURFACE_PRIMARY, 0.8))),
        text_color: Some(to_iced_color(TEXT_PRIMARY, 1.0)),
        border: Border {
            radius: Radius::from(RADIUS_MD),
            width: 1.0,
            color: to_iced_color(ACCENT_PRIMARY, 0.7),
        },
        shadow: Shadow::default(),
    }
}
