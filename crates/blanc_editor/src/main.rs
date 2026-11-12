use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

#[derive(Default)]
struct EditorApp {
    #[allow(dead_code)]
    project: Option<blanc_core::Project>,
}

#[derive(Debug, Clone)]
enum Message {
    #[allow(dead_code)]
    NoOp,
}

impl EditorApp {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Blanc Engine").size(24),
            text("No-Code Visual Novel Creation Engine").size(14),
            text("Welcome Screen - Initializing...").size(12),
        ]
        .spacing(12);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Blanc Engine", EditorApp::update, EditorApp::view)
        .run()
}
