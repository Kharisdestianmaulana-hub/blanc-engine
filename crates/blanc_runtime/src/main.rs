use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

#[derive(Default)]
struct RuntimeApp {
    #[allow(dead_code)]
    project_path: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    #[allow(dead_code)]
    NoOp,
}

impl RuntimeApp {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Blanc Runtime").size(24),
            text("Standalone Visual Novel Player").size(14),
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
    let args: Vec<String> = std::env::args().collect();
    let project_path = args.get(1).cloned();

    println!("Starting Blanc Runtime with project: {:?}", project_path);

    iced::application("Blanc Runtime Player", RuntimeApp::update, RuntimeApp::view)
        .run()
}
