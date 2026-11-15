#![cfg_attr(all(target_os = "windows", not(test)), windows_subsystem = "windows")]

mod engine;
mod loader;
mod view;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use gilrs::{Axis, Button, EventType, Gilrs};
use iced::keyboard::{self, key::Named, Key};
use iced::{Element, Subscription, Task};

use engine::{RuntimeEngine, RuntimeScreen, SaveLoadMode, SettingsFocusItem, SkipMode};
use view::{view_runtime, RuntimeMessage};

struct RuntimeApp {
    engine: RuntimeEngine,
    gilrs: Option<Gilrs>,
    last_stick_nav: Instant,
    is_fullscreen: bool,
}

impl RuntimeApp {
    fn new(project_dir: Option<PathBuf>, start_scene: Option<String>) -> (Self, Task<RuntimeMessage>) {
        let engine = RuntimeEngine::new(project_dir, start_scene);
        let is_fullscreen = engine.is_fullscreen;
        let startup_task = if is_fullscreen {
            iced::window::get_latest()
                .and_then(|id| iced::window::change_mode(id, iced::window::Mode::Fullscreen))
        } else {
            Task::none()
        };
        (
            Self {
                engine,
                gilrs: Gilrs::new().ok(),
                last_stick_nav: Instant::now(),
                is_fullscreen,
            },
            startup_task,
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
                self.engine.expire_toast();
                self.poll_gamepad();
            }
            RuntimeMessage::FocusPrev => {
                self.engine.move_focus(-1);
                self.engine.save_load_action_index = 0;
            }
            RuntimeMessage::FocusNext => {
                self.engine.move_focus(1);
                self.engine.save_load_action_index = 0;
            }
            RuntimeMessage::NavLeft => {
                return self.handle_horizontal_nav(-1);
            }
            RuntimeMessage::NavRight => {
                return self.handle_horizontal_nav(1);
            }
            RuntimeMessage::FocusConfirm => {
                return self.confirm_focused();
            }
            RuntimeMessage::DeleteFocused => {
                if let RuntimeScreen::SaveLoad { .. } = self.engine.screen {
                    let slots = self.engine.list_save_slots();
                    if !slots.is_empty() {
                        let idx = self.engine.focus_index % (slots.len() + 1);
                        if idx < slots.len() {
                            self.engine.delete_save_slot(idx + 1);
                        }
                    }
                }
            }
            RuntimeMessage::BackOrPause => {
                return self.update(match self.engine.screen {
                    RuntimeScreen::Settings { .. } => RuntimeMessage::CloseSettings,
                    RuntimeScreen::SaveLoad { .. } => RuntimeMessage::CloseSaveLoad,
                    RuntimeScreen::Backlog => RuntimeMessage::CloseBacklog,
                    RuntimeScreen::Playing | RuntimeScreen::PauseMenu => RuntimeMessage::TogglePause,
                    _ => return Task::none(),
                });
            }
            RuntimeMessage::Advance => {
                self.engine.advance();
            }
            RuntimeMessage::ContinueGame => {
                self.engine.continue_game();
            }
            RuntimeMessage::SelectChoice(idx) => {
                self.engine.select_choice(idx);
            }
            RuntimeMessage::NewGame => {
                self.engine.start_new_game();
            }
            RuntimeMessage::OpenSaveScreen => {
                self.engine.focus_index = 0;
                self.engine.save_load_action_index = 0;
                self.engine.screen = RuntimeScreen::SaveLoad {
                    mode: SaveLoadMode::Save,
                    return_to_pause: true,
                };
            }
            RuntimeMessage::OpenLoadScreen { return_to_pause } => {
                self.engine.focus_index = 0;
                self.engine.save_load_action_index = 0;
                self.engine.screen = RuntimeScreen::SaveLoad {
                    mode: SaveLoadMode::Load,
                    return_to_pause,
                };
            }
            RuntimeMessage::CloseSaveLoad => {
                if let RuntimeScreen::SaveLoad { return_to_pause, .. } = self.engine.screen {
                    self.engine.focus_index = 0;
                    self.engine.screen = if return_to_pause {
                        RuntimeScreen::PauseMenu
                    } else {
                        RuntimeScreen::TitleScreen
                    };
                }
            }
            RuntimeMessage::SaveToSlot(slot) => {
                self.engine.save_to_slot(slot);
            }
            RuntimeMessage::LoadFromSlot(slot) => {
                self.engine.load_from_slot(slot);
            }
            RuntimeMessage::DeleteSaveSlot(slot) => {
                self.engine.delete_save_slot(slot);
            }
            RuntimeMessage::QuickSave => {
                if self.engine.screen == RuntimeScreen::Playing {
                    self.engine.quick_save();
                }
            }
            RuntimeMessage::QuickLoad => {
                if matches!(
                    self.engine.screen,
                    RuntimeScreen::Playing | RuntimeScreen::PauseMenu | RuntimeScreen::TitleScreen
                ) {
                    self.engine.quick_load();
                }
            }
            RuntimeMessage::OpenBacklog => {
                if self.engine.screen == RuntimeScreen::Playing {
                    self.engine.focus_index = 0;
                    self.engine.screen = RuntimeScreen::Backlog;
                } else if self.engine.screen == RuntimeScreen::Backlog {
                    self.engine.screen = RuntimeScreen::Playing;
                }
            }
            RuntimeMessage::CloseBacklog => {
                if self.engine.screen == RuntimeScreen::Backlog {
                    self.engine.screen = RuntimeScreen::Playing;
                }
            }
            RuntimeMessage::OpenSettings => {
                let return_to_pause = matches!(self.engine.screen, RuntimeScreen::PauseMenu);
                self.engine.focus_index = 0;
                self.engine.screen = RuntimeScreen::Settings { return_to_pause };
            }
            RuntimeMessage::CloseSettings => {
                if let RuntimeScreen::Settings { return_to_pause } = self.engine.screen {
                    self.engine.focus_index = 0;
                    self.engine.screen = if return_to_pause {
                        RuntimeScreen::PauseMenu
                    } else {
                        RuntimeScreen::TitleScreen
                    };
                }
            }
            RuntimeMessage::TogglePause => {
                self.engine.focus_index = 0;
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
            RuntimeMessage::SetVoiceVolume(v) => {
                self.engine.set_voice_volume(v);
            }
            RuntimeMessage::SetTextSpeed(s) => {
                self.engine.text_speed = s;
            }
            RuntimeMessage::ToggleAutoPlay => {
                self.engine.toggle_auto_play();
            }
            RuntimeMessage::ToggleSkip => {
                self.engine.toggle_skip();
            }
            RuntimeMessage::SetSkipMode(mode) => {
                self.engine.set_skip_rule(mode);
            }
            RuntimeMessage::SetAutoForwardDelay(delay) => {
                self.engine.set_auto_forward_delay(delay);
            }
            RuntimeMessage::SetLanguage(code) => {
                self.engine.set_language(code);
            }
            RuntimeMessage::SelectSettingsCategory(cat) => {
                self.engine.settings_category = cat;
                self.engine.focus_index = 0;
            }
            RuntimeMessage::CycleSettingsCategory(delta) => {
                if matches!(self.engine.screen, RuntimeScreen::Settings { .. })
                    && self
                        .engine
                        .ui_config()
                        .pause_settings
                        .settings_layout_template
                        .uses_tabs()
                {
                    self.engine.settings_category = self.engine.settings_category.cycle(delta);
                    self.engine.focus_index = 0;
                }
            }
            RuntimeMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
                self.engine.set_fullscreen(self.is_fullscreen);
                let mode = if self.is_fullscreen {
                    iced::window::Mode::Fullscreen
                } else {
                    iced::window::Mode::Windowed
                };
                return iced::window::get_latest()
                    .and_then(move |id| iced::window::change_mode(id, mode));
            }
            RuntimeMessage::SetFullscreen(fullscreen) => {
                self.is_fullscreen = fullscreen;
                self.engine.set_fullscreen(fullscreen);
                let mode = if fullscreen {
                    iced::window::Mode::Fullscreen
                } else {
                    iced::window::Mode::Windowed
                };
                return iced::window::get_latest()
                    .and_then(move |id| iced::window::change_mode(id, mode));
            }
            RuntimeMessage::QuitGame => {
                return iced::exit();
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<RuntimeMessage> {
        let needs_tick = self.engine.screen == RuntimeScreen::Playing
            || self.engine.toast.is_some()
            || self.gilrs.is_some();
        let tick_sub = if needs_tick {
            iced::time::every(Duration::from_millis(30)).map(|_| RuntimeMessage::Tick)
        } else {
            Subscription::none()
        };

        let key_sub = keyboard::on_key_press(|key, _modifiers| match key {
            Key::Named(Named::ArrowUp) => Some(RuntimeMessage::FocusPrev),
            Key::Named(Named::ArrowDown) | Key::Named(Named::Tab) => Some(RuntimeMessage::FocusNext),
            Key::Named(Named::ArrowLeft) => Some(RuntimeMessage::NavLeft),
            Key::Named(Named::ArrowRight) => Some(RuntimeMessage::NavRight),
            Key::Character(ref c) if c.eq_ignore_ascii_case("q") => Some(RuntimeMessage::CycleSettingsCategory(-1)),
            Key::Character(ref c) if c.eq_ignore_ascii_case("e") => Some(RuntimeMessage::CycleSettingsCategory(1)),
            Key::Character(ref c) if c.eq_ignore_ascii_case("w") => Some(RuntimeMessage::FocusPrev),
            Key::Character(ref c) if c.eq_ignore_ascii_case("d") => Some(RuntimeMessage::NavRight),
            Key::Character(ref c) if c.eq_ignore_ascii_case("a") => Some(RuntimeMessage::ToggleAutoPlay),
            Key::Character(ref c) if c.eq_ignore_ascii_case("s") => Some(RuntimeMessage::ToggleSkip),
            Key::Named(Named::Space) | Key::Named(Named::Enter) => Some(RuntimeMessage::FocusConfirm),
            Key::Named(Named::Delete) | Key::Named(Named::Backspace) => Some(RuntimeMessage::DeleteFocused),
            Key::Named(Named::Escape) => Some(RuntimeMessage::BackOrPause),
            Key::Character(c) if c.eq_ignore_ascii_case("h") => Some(RuntimeMessage::ToggleHideUi),
            Key::Character(c) if c.eq_ignore_ascii_case("b") => Some(RuntimeMessage::OpenBacklog),
            Key::Named(Named::F5) => Some(RuntimeMessage::QuickSave),
            Key::Named(Named::F9) => Some(RuntimeMessage::QuickLoad),
            Key::Named(Named::F11) => Some(RuntimeMessage::ToggleFullscreen),
            _ => None,
        });

        Subscription::batch([tick_sub, key_sub])
    }

    fn confirm_focused(&mut self) -> Task<RuntimeMessage> {
        let ui = self.engine.ui_config();
        let idx = self.engine.focus_index;
        match &self.engine.screen {
            RuntimeScreen::TitleScreen => {
                let continue_enabled = ui.main_menu.show_continue_button && self.engine.has_save_data();
                if continue_enabled {
                    match idx % 5 {
                        0 => return self.update(RuntimeMessage::ContinueGame),
                        1 => return self.update(RuntimeMessage::NewGame),
                        2 => return self.update(RuntimeMessage::OpenLoadScreen { return_to_pause: false }),
                        3 => return self.update(RuntimeMessage::OpenSettings),
                        _ => return self.update(RuntimeMessage::QuitGame),
                    }
                } else {
                    match idx % 4 {
                        0 => return self.update(RuntimeMessage::NewGame),
                        1 => return self.update(RuntimeMessage::OpenLoadScreen { return_to_pause: false }),
                        2 => return self.update(RuntimeMessage::OpenSettings),
                        _ => return self.update(RuntimeMessage::QuitGame),
                    }
                }
            }
            RuntimeScreen::Playing => {
                if let Some(choices) = &self.engine.stage.active_choice {
                    if !choices.options.is_empty() {
                        let chosen = idx.min(choices.options.len() - 1);
                        self.engine.select_choice(chosen);
                        self.engine.focus_index = 0;
                        return Task::none();
                    }
                }
                self.engine.advance();
            }
            RuntimeScreen::PauseMenu => {
                let p = &ui.pause_settings;
                let mut actions: Vec<RuntimeMessage> = Vec::new();
                if p.show_resume_button {
                    actions.push(RuntimeMessage::TogglePause);
                }
                if p.show_save_button {
                    actions.push(RuntimeMessage::OpenSaveScreen);
                }
                if p.show_load_button {
                    actions.push(RuntimeMessage::OpenLoadScreen { return_to_pause: true });
                }
                if p.show_settings_button {
                    actions.push(RuntimeMessage::OpenSettings);
                }
                if p.show_return_title_button {
                    actions.push(RuntimeMessage::ReturnToTitle);
                }
                if p.show_quit_button {
                    actions.push(RuntimeMessage::QuitGame);
                }
                if let Some(msg) = actions.get(idx % actions.len().max(1)).cloned() {
                    return self.update(msg);
                }
            }
            RuntimeScreen::Settings { .. } => {
                let items = self.engine.settings_focus_items();
                if let Some(item) = items.get(idx % items.len().max(1)).copied() {
                    match item {
                        SettingsFocusItem::Back => return self.update(RuntimeMessage::CloseSettings),
                        SettingsFocusItem::TextSpeed
                        | SettingsFocusItem::SkipMode
                        | SettingsFocusItem::DisplayMode
                        | SettingsFocusItem::Language => return self.handle_horizontal_nav(1),
                        _ => {}
                    }
                }
            }
            RuntimeScreen::SaveLoad { mode, .. } => {
                let slots = self.engine.list_save_slots();
                let total = slots.len() + 1;
                let focused = idx % total.max(1);
                if focused < slots.len() {
                    let slot_num = focused + 1;
                    let has_data = slots.get(focused).and_then(|s| s.as_ref()).is_some();
                    if has_data && self.engine.save_load_action_index == 1 {
                        self.engine.save_load_action_index = 0;
                        return self.update(RuntimeMessage::DeleteSaveSlot(slot_num));
                    }
                    match mode {
                        SaveLoadMode::Save => return self.update(RuntimeMessage::SaveToSlot(slot_num)),
                        SaveLoadMode::Load => {
                            if has_data {
                                return self.update(RuntimeMessage::LoadFromSlot(slot_num));
                            }
                        }
                    }
                } else {
                    return self.update(RuntimeMessage::CloseSaveLoad);
                }
            }
            RuntimeScreen::Backlog => {
                return self.update(RuntimeMessage::CloseBacklog);
            }
            RuntimeScreen::Ending { .. } => {
                return self.update(RuntimeMessage::FollowEndingTitle);
            }
            RuntimeScreen::Error(_) => {}
        }
        Task::none()
    }

    fn handle_horizontal_nav(&mut self, delta: isize) -> Task<RuntimeMessage> {
        match &self.engine.screen {
            RuntimeScreen::Settings { .. } => {
                let items = self.engine.settings_focus_items();
                let idx = self.engine.focus_index % items.len().max(1);
                let Some(item) = items.get(idx).copied() else {
                    return Task::none();
                };
                match item {
                    SettingsFocusItem::MasterVol => {
                        let next = (self.engine.master_volume + delta as f32 * 0.05).clamp(0.0, 1.0);
                        return self.update(RuntimeMessage::SetMasterVolume(next));
                    }
                    SettingsFocusItem::BgmVol => {
                        let next = (self.engine.bgm_volume + delta as f32 * 0.05).clamp(0.0, 1.0);
                        return self.update(RuntimeMessage::SetBgmVolume(next));
                    }
                    SettingsFocusItem::SfxVol => {
                        let next = (self.engine.sfx_volume + delta as f32 * 0.05).clamp(0.0, 1.0);
                        return self.update(RuntimeMessage::SetSfxVolume(next));
                    }
                    SettingsFocusItem::VoiceVol => {
                        let next = (self.engine.voice_volume + delta as f32 * 0.05).clamp(0.0, 1.0);
                        return self.update(RuntimeMessage::SetVoiceVolume(next));
                    }
                    SettingsFocusItem::AutoDelay => {
                        let next = (self.engine.auto_forward_delay + delta as f32 * 0.5).clamp(0.5, 5.0);
                        return self.update(RuntimeMessage::SetAutoForwardDelay(next));
                    }
                    SettingsFocusItem::TextSpeed => {
                        let presets = [0.5_f32, 1.0, 1.5, 2.0];
                        let cur = if self.engine.text_speed >= 2.0 {
                            3
                        } else if self.engine.text_speed >= 1.4 {
                            2
                        } else if self.engine.text_speed >= 0.9 {
                            1
                        } else {
                            0
                        };
                        let next_idx = (cur as isize + delta).rem_euclid(presets.len() as isize) as usize;
                        return self.update(RuntimeMessage::SetTextSpeed(presets[next_idx]));
                    }
                    SettingsFocusItem::SkipMode => {
                        let next = match self.engine.skip_rule {
                            SkipMode::ReadOnly => SkipMode::All,
                            SkipMode::All => SkipMode::ReadOnly,
                        };
                        return self.update(RuntimeMessage::SetSkipMode(next));
                    }
                    SettingsFocusItem::DisplayMode => {
                        return self.update(RuntimeMessage::SetFullscreen(!self.is_fullscreen));
                    }
                    SettingsFocusItem::Language => {
                        let langs = self.engine.available_languages();
                        if langs.len() > 1 {
                            let cur = langs
                                .iter()
                                .position(|l| l.code == self.engine.active_language)
                                .unwrap_or(0);
                            let next_idx = (cur as isize + delta).rem_euclid(langs.len() as isize) as usize;
                            return self.update(RuntimeMessage::SetLanguage(langs[next_idx].code.clone()));
                        }
                    }
                    SettingsFocusItem::Back => {}
                }
            }
            RuntimeScreen::SaveLoad { .. } => {
                let slots = self.engine.list_save_slots();
                let total = slots.len() + 1;
                let focused = self.engine.focus_index % total.max(1);
                let has_data = focused < slots.len()
                    && slots.get(focused).and_then(|s| s.as_ref()).is_some();
                if delta > 0 {
                    if has_data && self.engine.save_load_action_index == 0 {
                        self.engine.save_load_action_index = 1;
                    } else {
                        self.engine.move_focus(1);
                        self.engine.save_load_action_index = 0;
                    }
                } else if delta < 0 {
                    if has_data && self.engine.save_load_action_index == 1 {
                        self.engine.save_load_action_index = 0;
                    } else {
                        self.engine.move_focus(-1);
                        self.engine.save_load_action_index = 0;
                    }
                }
            }
            RuntimeScreen::TitleScreen
            | RuntimeScreen::PauseMenu
            | RuntimeScreen::Playing => {
                self.engine.move_focus(delta);
            }
            RuntimeScreen::Backlog | RuntimeScreen::Ending { .. } | RuntimeScreen::Error(_) => {}
        }
        Task::none()
    }

    fn poll_gamepad(&mut self) {
        let mut nav_actions: Vec<RuntimeMessage> = Vec::new();
        if let Some(gilrs) = &mut self.gilrs {
            while let Some(event) = gilrs.next_event() {
                match event.event {
                    EventType::ButtonPressed(btn, _) => match btn {
                        Button::DPadUp => nav_actions.push(RuntimeMessage::FocusPrev),
                        Button::DPadDown => nav_actions.push(RuntimeMessage::FocusNext),
                        Button::DPadLeft => nav_actions.push(RuntimeMessage::NavLeft),
                        Button::DPadRight => nav_actions.push(RuntimeMessage::NavRight),
                        Button::South => nav_actions.push(RuntimeMessage::FocusConfirm),
                        Button::West => nav_actions.push(RuntimeMessage::DeleteFocused),
                        Button::LeftTrigger => nav_actions.push(RuntimeMessage::CycleSettingsCategory(-1)),
                        Button::RightTrigger => nav_actions.push(RuntimeMessage::CycleSettingsCategory(1)),
                        Button::East | Button::Start => nav_actions.push(RuntimeMessage::BackOrPause),
                        _ => {}
                    },
                    EventType::AxisChanged(Axis::LeftStickY, val, _) => {
                        if self.last_stick_nav.elapsed() > Duration::from_millis(220) {
                            if val > 0.65 {
                                self.last_stick_nav = Instant::now();
                                nav_actions.push(RuntimeMessage::FocusPrev);
                            } else if val < -0.65 {
                                self.last_stick_nav = Instant::now();
                                nav_actions.push(RuntimeMessage::FocusNext);
                            }
                        }
                    }
                    EventType::AxisChanged(Axis::LeftStickX, val, _) => {
                        if self.last_stick_nav.elapsed() > Duration::from_millis(220) {
                            if val < -0.65 {
                                self.last_stick_nav = Instant::now();
                                nav_actions.push(RuntimeMessage::NavLeft);
                            } else if val > 0.65 {
                                self.last_stick_nav = Instant::now();
                                nav_actions.push(RuntimeMessage::NavRight);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        for act in nav_actions {
            let _ = self.update(act);
        }
    }

    fn view(&self) -> Element<'_, RuntimeMessage> {
        view_runtime(&self.engine)
    }
}

fn discover_project_path(cli_arg: Option<String>) -> Option<PathBuf> {
    if let Some(arg) = cli_arg {
        return Some(PathBuf::from(arg));
    }
    let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let bpack = exe_dir.join("game.bpack");
    if bpack.is_file() {
        return Some(bpack);
    }
    let game_data = exe_dir.join("GameData");
    if game_data.is_dir() {
        return Some(game_data);
    }
    if let Ok(entries) = std::fs::read_dir(&exe_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("blanc") {
                return Some(exe_dir);
            }
        }
    }
    None
}

fn load_project_for_boot(path: &PathBuf) -> Result<loader::LoadedProject, String> {
    if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("bpack") {
        let exe_dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        loader::LoadedProject::load_from_bpack(exe_dir, path)
    } else if path.join("game.bpack").is_file() {
        loader::LoadedProject::load_from_bpack(path, &path.join("game.bpack"))
    } else {
        loader::LoadedProject::load_from_dir(path)
    }
}

fn load_window_icon(project_dir: Option<&PathBuf>) -> Option<iced::window::Icon> {
    if let Some(dir) = project_dir {
        if let Ok(loaded) = load_project_for_boot(dir) {
            if let Some(custom_logo) = loaded.resolve_custom_logo() {
                if let Some(bytes) = loaded.read_path_bytes(&custom_logo) {
                    if let Ok(img) = image::load_from_memory(&bytes) {
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
    }

    let img = image::load_from_memory(include_bytes!("../../../assets/icons/icon_64.png")).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    let project_path = discover_project_path(args.get(1).cloned());
    let start_scene = args
        .windows(2)
        .find(|w| w[0] == "--scene")
        .map(|w| w[1].clone());

    let (win_w, win_h, font_family, font_bytes) = if let Some(dir) = &project_path {
        if let Ok(loaded) = load_project_for_boot(dir) {
            let mut family = loaded.config.ui_config.theme.font_family.trim().to_string();
            let mut bytes_opt: Option<Vec<u8>> = None;
            if let Some(asset_id) = loaded.config.ui_config.theme.font_asset_id.as_deref() {
                if let Some(asset) = loaded.project.assets.get(asset_id) {
                    let full = loaded.project_dir.join(&asset.relative_path);
                    if let Some(bytes) = loaded.read_path_bytes(&full) {
                        if let Some(parsed) = blanc_core::ui_config::read_ttf_family_name(&bytes) {
                            family = parsed;
                        }
                        bytes_opt = Some(bytes);
                    }
                }
            }
            (
                (loaded.config.default_resolution_width as f32).min(1280.0).max(960.0),
                (loaded.config.default_resolution_height as f32).min(720.0).max(540.0),
                family,
                bytes_opt,
            )
        } else {
            (1280.0, 720.0, String::new(), None)
        }
    } else {
        (1280.0, 720.0, String::new(), None)
    };

    let default_font = if font_family.is_empty() {
        iced::Font::DEFAULT
    } else {
        let leaked: &'static str = Box::leak(font_family.into_boxed_str());
        iced::Font::with_name(leaked)
    };

    let icon = load_window_icon(project_path.as_ref());

    let mut app = iced::application(RuntimeApp::title, RuntimeApp::update, RuntimeApp::view)
        .default_font(default_font)
        .subscription(RuntimeApp::subscription)
        .window(iced::window::Settings {
            min_size: Some(iced::Size::new(960.0, 540.0)),
            size: iced::Size::new(win_w, win_h),
            icon,
            ..Default::default()
        });
    if let Some(bytes) = font_bytes {
        let leaked_bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        app = app.font(leaked_bytes);
    }
    #[cfg(target_os = "windows")]
    {
        for font_path in [
            r"C:\Windows\Fonts\msyh.ttc",
            r"C:\Windows\Fonts\YuGothR.ttc",
            r"C:\Windows\Fonts\YuGothM.ttc",
            r"C:\Windows\Fonts\malgun.ttf",
        ] {
            if let Ok(bytes) = std::fs::read(font_path) {
                let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                app = app.font(leaked);
            }
        }
    }
    app.run_with(move || RuntimeApp::new(project_path, start_scene))
}
