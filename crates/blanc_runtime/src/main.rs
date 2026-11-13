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
        let logo_bytes: &[u8] = include_bytes!("../../../assets/icons/icon_transparent_128.png");
        let logo_widget = iced::widget::image(iced::widget::image::Handle::from_bytes(logo_bytes))
            .width(64.0)
            .height(64.0);

        let content = column![
            logo_widget,
            text("Blanc Runtime").size(24),
            text("Standalone Visual Novel Player").size(14),
        ]
        .spacing(12)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn load_window_icon() -> Option<iced::window::Icon> {
    let img = image::load_from_memory(include_bytes!("../../../assets/icons/icon_64.png")).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    let project_path = args.get(1).cloned();

    println!("Starting Blanc Runtime with project: {:?}", project_path);

    iced::application("Blanc Runtime Player", RuntimeApp::update, RuntimeApp::view)
        .window(iced::window::Settings {
            min_size: Some(iced::Size::new(960.0, 540.0)),
            size: iced::Size::new(1280.0, 720.0),
            icon: load_window_icon(),
            ..Default::default()
        })
        .run()
}
