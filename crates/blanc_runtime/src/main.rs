mod engine;
mod loader;
mod view;

use std::path::PathBuf;
use std::time::Duration;

use iced::keyboard::{self, key::Named, Key};
use iced::{Element, Subscription, Task};

use engine::{RuntimeEngine, RuntimeScreen};
use view::{view_runtime, RuntimeMessage};

struct RuntimeApp {
    engine: RuntimeEngine,
}

impl RuntimeApp {
    fn new(project_dir: Option<PathBuf>) -> (Self, Task<RuntimeMessage>) {
        (
            Self {
                engine: RuntimeEngine::new(project_dir),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        if let Some(loaded) = &self.engine.loaded {
            loaded.config.title_text.clone()
        } else {
            "Blanc Runtime Player".to_string()
        }
    }

    fn update(&mut self, message: RuntimeMessage) -> Task<RuntimeMessage> {
        match message {
            RuntimeMessage::Tick => {
                self.engine.tick(0.03);
            }
            RuntimeMessage::Advance => {
                self.engine.advance();
            }
            RuntimeMessage::SelectChoice(idx) => {
                self.engine.select_choice(idx);
            }
            RuntimeMessage::NewGame => {
                self.engine.start_new_game();
            }
            RuntimeMessage::OpenSettings => {
                let return_to_pause = matches!(self.engine.screen, RuntimeScreen::PauseMenu);
                self.engine.screen = RuntimeScreen::Settings { return_to_pause };
            }
            RuntimeMessage::CloseSettings => {
                if let RuntimeScreen::Settings { return_to_pause } = self.engine.screen {
                    self.engine.screen = if return_to_pause {
                        RuntimeScreen::PauseMenu
                    } else {
                        RuntimeScreen::TitleScreen
                    };
                }
            }
            RuntimeMessage::TogglePause => {
                match self.engine.screen {
                    RuntimeScreen::Playing => {
                        self.engine.screen = RuntimeScreen::PauseMenu;
                    }
                    RuntimeScreen::PauseMenu => {
                        self.engine.screen = RuntimeScreen::Playing;
                    }
                    _ => {}
                }
            }
            RuntimeMessage::ToggleHideUi => {
                if self.engine.screen == RuntimeScreen::Playing {
                    self.engine.stage.ui_hidden = !self.engine.stage.ui_hidden;
                }
            }
            RuntimeMessage::ReturnToTitle => {
                self.engine.enter_title_screen();
            }
            RuntimeMessage::FollowEndingTitle => {
                if let RuntimeScreen::Ending {
                    ending_node_id: Some(node_id),
                    ..
                } = self.engine.screen.clone()
                {
                    if self.engine.follow_flow_connection(&node_id, "title") {
                        return Task::none();
                    }
                }
                self.engine.enter_title_screen();
            }
            RuntimeMessage::SetMasterVolume(v) => {
                self.engine.master_volume = v;
                self.engine.update_volumes();
            }
            RuntimeMessage::SetBgmVolume(v) => {
                self.engine.bgm_volume = v;
                self.engine.update_volumes();
            }
            RuntimeMessage::SetSfxVolume(v) => {
                self.engine.sfx_volume = v;
            }
            RuntimeMessage::SetTextSpeed(s) => {
                self.engine.text_speed = s;
            }
            RuntimeMessage::QuitGame => {
                return iced::exit();
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<RuntimeMessage> {
        let tick_sub = if self.engine.screen == RuntimeScreen::Playing {
            iced::time::every(Duration::from_millis(30)).map(|_| RuntimeMessage::Tick)
        } else {
            Subscription::none()
        };

        let key_sub = keyboard::on_key_press(|key, _modifiers| match key {
            Key::Named(Named::Space) | Key::Named(Named::Enter) => Some(RuntimeMessage::Advance),
            Key::Named(Named::Escape) => Some(RuntimeMessage::TogglePause),
            Key::Character(c) if c.eq_ignore_ascii_case("h") => Some(RuntimeMessage::ToggleHideUi),
            _ => None,
        });

        Subscription::batch([tick_sub, key_sub])
    }

    fn view(&self) -> Element<'_, RuntimeMessage> {
        view_runtime(&self.engine)
    }
}

fn load_window_icon(project_dir: Option<&PathBuf>) -> Option<iced::window::Icon> {
    if let Some(dir) = project_dir {
        if let Ok(loaded) = loader::LoadedProject::load_from_dir(dir) {
            if let Some(custom_logo) = loaded.resolve_custom_logo() {
                if let Ok(img) = image::open(custom_logo) {
                    let resized = img.resize_exact(64, 64, image::imageops::FilterType::Lanczos3);
                    let rgba = resized.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    if let Ok(icon) = iced::window::icon::from_rgba(rgba.into_raw(), w, h) {
                        return Some(icon);
                    }
                }
            }
        }
    }

    let img = image::load_from_memory(include_bytes!("../../../assets/icons/icon_64.png")).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    let project_path = args.get(1).map(PathBuf::from);

    let (win_w, win_h) = if let Some(dir) = &project_path {
        if let Ok(loaded) = loader::LoadedProject::load_from_dir(dir) {
            (
                (loaded.config.default_resolution_width as f32).min(1280.0).max(960.0),
                (loaded.config.default_resolution_height as f32).min(720.0).max(540.0),
            )
        } else {
            (1280.0, 720.0)
        }
    } else {
        (1280.0, 720.0)
    };

    let icon = load_window_icon(project_path.as_ref());

    iced::application(RuntimeApp::title, RuntimeApp::update, RuntimeApp::view)
        .subscription(RuntimeApp::subscription)
        .window(iced::window::Settings {
            min_size: Some(iced::Size::new(960.0, 540.0)),
            size: iced::Size::new(win_w, win_h),
            icon,
            ..Default::default()
        })
        .run_with(move || RuntimeApp::new(project_path))
}
