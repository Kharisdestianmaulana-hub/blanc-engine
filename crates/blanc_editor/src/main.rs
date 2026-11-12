mod app;
mod icons;
mod screens;
mod theme;

use app::EditorApp;
use blanc_core::theme::colors::BACKGROUND_PRIMARY;

fn main() -> iced::Result {
    let title = "Blanc Engine — Visual Novel Creation";

    iced::application(title, EditorApp::update, EditorApp::view)
        .theme(|_state| {
            iced::Theme::custom(
                "BlancDark".to_string(),
                iced::theme::Palette {
                    background: iced::Color::from_rgb8(
                        BACKGROUND_PRIMARY.r,
                        BACKGROUND_PRIMARY.g,
                        BACKGROUND_PRIMARY.b,
                    ),
                    text: iced::Color::from_rgb8(230, 233, 237),
                    primary: iced::Color::from_rgb8(124, 140, 255),
                    success: iced::Color::from_rgb8(85, 184, 138),
                    danger: iced::Color::from_rgb8(225, 111, 120),
                },
            )
        })
        .window(iced::window::Settings {
            min_size: Some(iced::Size::new(960.0, 600.0)),
            size: iced::Size::new(1280.0, 800.0),
            ..Default::default()
        })
        .run()
}
