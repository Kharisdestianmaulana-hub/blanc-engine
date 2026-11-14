#![cfg_attr(all(target_os = "windows", not(test)), windows_subsystem = "windows")]

mod i18n;

use std::path::{Path, PathBuf};
use std::process::Command;

use blanc_core::installer::{
    extract_embedded_files, read_embedded_payload, sanitize_installer_name, uninstall_exe_filename,
    InstallerConfig, InstallerLang, InstallerPayloadManifest, UninstallMetadata,
    UNINSTALL_META_FILENAME,
};
use blanc_core::ui_config::parse_hex_rgb;
use iced::widget::{
    button, checkbox, column, container, image, pick_list, progress_bar, row, scrollable, text,
    text_input, Space,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolicyLangOption {
    code: String,
    label: String,
}

impl std::fmt::Display for PolicyLangOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}
use iced::{Alignment, Background, Border, Color, Element, Length, Size, Task};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
enum WizardStep {
    SelectLanguage { is_uninstall: bool },
    Welcome,
    License,
    Location,
    Installing,
    Complete,
    Error(String),
    UninstallConfirm,
    UninstallProgress,
    UninstallDone,
}

#[derive(Debug, Clone)]
enum Message {
    SelectLanguage(InstallerLang),
    ConfirmLanguage,
    SelectPolicyLanguage(String),
    Next,
    Back,
    ToggleAcceptLicense(bool),
    InstallDirChanged(String),
    BrowseInstallDir,
    ToggleDesktopShortcut(bool),
    ToggleStartMenuShortcut(bool),
    RunInstall,
    InstallProgress(f32, String),
    InstallDone(Result<(), String>),
    ToggleLaunchAfter(bool),
    Finish,
    Cancel,
    ToggleRemoveSaveData(bool),
    RunUninstall,
    UninstallStep(u8),
    FinishUninstall,
}

struct InstallerApp {
    step: WizardStep,
    lang: InstallerLang,
    policy_lang_code: String,
    exe_path: PathBuf,
    config: InstallerConfig,
    game_exe_name: String,
    install_dir: String,
    accepted_license: bool,
    desktop_shortcut: bool,
    start_menu_shortcut: bool,
    launch_after: bool,
    uninstall_dir: Option<PathBuf>,
    uninstall_title: String,
    remove_save_data: bool,
    install_progress: f32,
    install_status: String,
    uninstall_progress: f32,
    uninstall_status: String,
}

impl InstallerApp {
    fn init_uninstall_state(exe_path: PathBuf, dir: PathBuf, arg_title: Option<String>) -> Self {
        let meta = std::fs::read(dir.join(UNINSTALL_META_FILENAME))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<UninstallMetadata>(&bytes).ok());

        let fallback_title = arg_title.unwrap_or_else(|| {
            dir.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Game".to_string())
        });

        let (title, version, publisher, game_exe_name, accent_hex, logo_png) = match meta {
            Some(m) => (
                m.game_title,
                m.version,
                m.publisher,
                m.game_exe_name,
                m.accent_hex,
                m.logo_png,
            ),
            None => (
                fallback_title,
                "1.0.0".to_string(),
                "Blanc Studio".to_string(),
                String::new(),
                "#6366F1".to_string(),
                None,
            ),
        };

        Self {
            step: WizardStep::SelectLanguage { is_uninstall: true },
            lang: InstallerLang::En,
            policy_lang_code: InstallerLang::En.code().to_string(),
            exe_path,
            config: InstallerConfig {
                game_title: title.clone(),
                version,
                publisher,
                accent_hex,
                logo_png,
                ..Default::default()
            },
            game_exe_name,
            install_dir: dir.to_string_lossy().to_string(),
            accepted_license: false,
            desktop_shortcut: false,
            start_menu_shortcut: false,
            launch_after: false,
            uninstall_dir: Some(dir),
            uninstall_title: title,
            remove_save_data: false,
            install_progress: 0.0,
            install_status: String::new(),
            uninstall_progress: 0.0,
            uninstall_status: String::new(),
        }
    }

    fn new() -> (Self, Task<Message>) {
        let exe_path = std::env::current_exe().unwrap_or_default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(pos) = args.iter().position(|a| a == "--uninstall") {
            let dir = args
                .get(pos + 1)
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    exe_path
                        .parent()
                        .map(|d| d.to_path_buf())
                        .unwrap_or_else(|| PathBuf::from("."))
                });
            let arg_title = args
                .iter()
                .position(|a| a == "--title")
                .and_then(|i| args.get(i + 1))
                .cloned();
            return (Self::init_uninstall_state(exe_path, dir, arg_title), Task::none());
        }

        match read_embedded_payload(&exe_path) {
            Ok((manifest, _)) => {
                let default_dir = default_install_path(&manifest.config.default_install_subdir);
                let desktop_shortcut = manifest.config.create_desktop_shortcut_default;
                let start_menu_shortcut = manifest.config.create_start_menu_shortcut_default;
                (
                    Self {
                        step: WizardStep::SelectLanguage { is_uninstall: false },
                        lang: InstallerLang::En,
                        policy_lang_code: InstallerLang::En.code().to_string(),
                        exe_path,
                        game_exe_name: manifest.exe_name,
                        config: manifest.config,
                        install_dir: default_dir.to_string_lossy().to_string(),
                        accepted_license: false,
                        desktop_shortcut,
                        start_menu_shortcut,
                        launch_after: true,
                        uninstall_dir: None,
                        uninstall_title: String::new(),
                        remove_save_data: false,
                        install_progress: 0.0,
                        install_status: String::new(),
                        uninstall_progress: 0.0,
                        uninstall_status: String::new(),
                    },
                    Task::none(),
                )
            }
            Err(err) => {
                let exe_dir = exe_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."));
                let is_uninstall_exe = exe_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| {
                        let lower = s.to_ascii_lowercase();
                        lower == "uninstall" || lower.starts_with("uninstall ") || lower.starts_with("uninstall_")
                    })
                    .unwrap_or(false);
                if is_uninstall_exe || exe_dir.join(UNINSTALL_META_FILENAME).exists() {
                    return (
                        Self::init_uninstall_state(exe_path, exe_dir, None),
                        Task::none(),
                    );
                }

                (
                    Self {
                        step: WizardStep::Error(format!(
                            "No game installer payload found in this executable.\n\nDetails: {}",
                            err
                        )),
                        lang: InstallerLang::En,
                        policy_lang_code: InstallerLang::En.code().to_string(),
                        exe_path,
                        config: InstallerConfig::default(),
                        game_exe_name: String::new(),
                        install_dir: String::new(),
                        accepted_license: false,
                        desktop_shortcut: true,
                        start_menu_shortcut: true,
                        launch_after: false,
                        uninstall_dir: None,
                        uninstall_title: String::new(),
                        remove_save_data: false,
                        install_progress: 0.0,
                        install_status: String::new(),
                        uninstall_progress: 0.0,
                        uninstall_status: String::new(),
                    },
                    Task::none(),
                )
            }
        }
    }

    fn title(&self) -> String {
        match self.step {
            WizardStep::SelectLanguage { is_uninstall: true }
            | WizardStep::UninstallConfirm
            | WizardStep::UninstallProgress
            | WizardStep::UninstallDone => {
                format!("Uninstall — {}", self.uninstall_title)
            }
            _ => format!("{} Setup", self.config.game_title),
        }
    }

    fn accent_color(&self) -> Color {
        parse_hex_rgb(&self.config.accent_hex)
            .map(|(r, g, b)| Color::from_rgb8(r, g, b))
            .unwrap_or(Color::from_rgb8(99, 102, 241))
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        let t = i18n::strings(self.lang);
        match msg {
            Message::SelectLanguage(lang) => {
                self.lang = lang;
            }
            Message::ConfirmLanguage => match self.step {
                WizardStep::SelectLanguage { is_uninstall: true } => {
                    self.step = WizardStep::UninstallConfirm;
                }
                WizardStep::SelectLanguage { is_uninstall: false } => {
                    self.policy_lang_code = if self.config.has_direct_license_for(self.lang.code()) {
                        self.lang.code().to_string()
                    } else if self.config.has_direct_license_for("en") {
                        "en".to_string()
                    } else {
                        self.config
                            .available_policy_languages()
                            .into_iter()
                            .next()
                            .map(|(c, _)| c)
                            .unwrap_or_else(|| "en".to_string())
                    };
                    self.step = WizardStep::Welcome;
                }
                _ => {}
            },
            Message::SelectPolicyLanguage(code) => {
                self.policy_lang_code = code;
            }
            Message::Next => match self.step {
                WizardStep::Welcome => {
                    if self.config.resolve_license_text(self.lang).is_none() {
                        self.step = WizardStep::Location;
                    } else {
                        self.step = WizardStep::License;
                    }
                }
                WizardStep::License => {
                    if self.accepted_license {
                        self.step = WizardStep::Location;
                    }
                }
                _ => {}
            },
            Message::Back => match self.step {
                WizardStep::Welcome => {
                    self.step = WizardStep::SelectLanguage { is_uninstall: false };
                }
                WizardStep::License => self.step = WizardStep::Welcome,
                WizardStep::Location => {
                    if self.config.resolve_license_text(self.lang).is_none() {
                        self.step = WizardStep::Welcome;
                    } else {
                        self.step = WizardStep::License;
                    }
                }
                _ => {}
            },
            Message::ToggleAcceptLicense(v) => self.accepted_license = v,
            Message::InstallDirChanged(v) => self.install_dir = v,
            Message::BrowseInstallDir => {
                if let Some(folder) = rfd::FileDialog::new()
                    .set_title("Choose Installation Folder")
                    .pick_folder()
                {
                    let sub = sanitize_name(&self.config.default_install_subdir);
                    self.install_dir = folder.join(sub).to_string_lossy().to_string();
                }
            }
            Message::ToggleDesktopShortcut(v) => self.desktop_shortcut = v,
            Message::ToggleStartMenuShortcut(v) => self.start_menu_shortcut = v,
            Message::RunInstall => {
                let dest = PathBuf::from(self.install_dir.trim());
                if dest.as_os_str().is_empty() {
                    self.step = WizardStep::Error("Installation folder cannot be empty.".to_string());
                    return Task::none();
                }
                self.step = WizardStep::Installing;
                self.install_progress = 0.05;
                self.install_status = format!("{}...", t.installing_title_prefix);

                let exe_path = self.exe_path.clone();
                let desktop_shortcut = self.desktop_shortcut;
                let start_menu_shortcut = self.start_menu_shortcut;
                let extracting_lbl = t.status_extracting;
                let registering_lbl = t.status_registering;
                let selected_lang = self.lang.code().to_string();

                return Task::run(
                    iced::stream::channel(16, move |mut sender| async move {
                        let (done_tx, done_rx) = std::sync::mpsc::channel();
                        let mut prog_tx = sender.clone();
                        std::thread::spawn(move || {
                            let res = match extract_embedded_files(&exe_path, &dest, |cur, total, name| {
                                let frac = if total > 0 {
                                    0.08 + (cur as f32 / total as f32) * 0.72
                                } else {
                                    0.5
                                };
                                let _ = prog_tx.try_send(Message::InstallProgress(
                                    frac,
                                    format!("{} {}", extracting_lbl, name),
                                ));
                            }) {
                                Ok(manifest) => {
                                    let meta_path = dest.join(UNINSTALL_META_FILENAME);
                                    if let Ok(raw) = std::fs::read(&meta_path) {
                                        if let Ok(mut meta) = serde_json::from_slice::<UninstallMetadata>(&raw) {
                                            meta.selected_lang = Some(selected_lang);
                                            if let Ok(updated) = serde_json::to_vec_pretty(&meta) {
                                                let _ = std::fs::write(&meta_path, updated);
                                            }
                                        }
                                    }
                                    let _ = prog_tx.try_send(Message::InstallProgress(
                                        0.88,
                                        registering_lbl.to_string(),
                                    ));
                                    register_windows_install(
                                        &dest,
                                        &manifest,
                                        desktop_shortcut,
                                        start_menu_shortcut,
                                    )
                                }
                                Err(err) => Err(format!("Installation failed:\n{}", err)),
                            };
                            let _ = done_tx.send(res);
                        });

                        let res = loop {
                            match done_rx.try_recv() {
                                Ok(r) => break r,
                                Err(std::sync::mpsc::TryRecvError::Empty) => {
                                    std::thread::sleep(std::time::Duration::from_millis(20));
                                }
                                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                    break Err("Installer worker thread disconnected.".to_string());
                                }
                            }
                        };
                        let _ = sender.try_send(Message::InstallDone(res));
                    }),
                    |m| m,
                );
            }
            Message::InstallProgress(p, label) => {
                self.install_progress = p;
                self.install_status = label;
            }
            Message::InstallDone(res) => match res {
                Ok(()) => {
                    self.install_progress = 1.0;
                    self.step = WizardStep::Complete;
                }
                Err(err) => {
                    self.step = WizardStep::Error(err);
                }
            },
            Message::ToggleLaunchAfter(v) => self.launch_after = v,
            Message::Finish => {
                if self.launch_after && !self.game_exe_name.is_empty() {
                    let game_exe = PathBuf::from(self.install_dir.trim()).join(&self.game_exe_name);
                    if game_exe.exists() {
                        let _ = Command::new(&game_exe).spawn();
                    }
                }
                std::process::exit(0);
            }
            Message::Cancel => {
                std::process::exit(0);
            }
            Message::ToggleRemoveSaveData(v) => {
                self.remove_save_data = v;
            }
            Message::RunUninstall => {
                self.step = WizardStep::UninstallProgress;
                self.uninstall_progress = 0.15;
                self.uninstall_status = t.status_check_running.to_string();
                return Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_millis(80));
                    },
                    |_| Message::UninstallStep(1),
                );
            }
            Message::UninstallStep(stage) => match stage {
                1 => {
                    stop_running_game_process(&self.game_exe_name);
                    self.uninstall_progress = 0.45;
                    self.uninstall_status = t.status_remove_shortcuts.to_string();
                    return Task::perform(
                        async {
                            std::thread::sleep(std::time::Duration::from_millis(90));
                        },
                        |_| Message::UninstallStep(2),
                    );
                }
                2 => {
                    remove_windows_shortcuts_and_registry(&self.uninstall_title);
                    self.uninstall_progress = 0.75;
                    self.uninstall_status = t.status_remove_files.to_string();
                    return Task::perform(
                        async {
                            std::thread::sleep(std::time::Duration::from_millis(90));
                        },
                        |_| Message::UninstallStep(3),
                    );
                }
                _ => {
                    if let Some(dir) = &self.uninstall_dir {
                        remove_installed_files_except_self(dir, &self.exe_path);
                    }
                    if self.remove_save_data {
                        if let Some(save_root) = app_data_game_dir(&self.uninstall_title) {
                            let _ = std::fs::remove_dir_all(save_root);
                        }
                    }
                    self.uninstall_progress = 1.0;
                    self.uninstall_status = t.status_cleanup_done.to_string();
                    self.step = WizardStep::UninstallDone;
                }
            },
            Message::FinishUninstall => {
                if let Some(dir) = &self.uninstall_dir {
                    schedule_self_folder_cleanup(dir, &self.exe_path);
                }
                std::process::exit(0);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let t = i18n::strings(self.lang);
        let ui_font = if self.lang == InstallerLang::Ko {
            iced::Font::with_name("Malgun Gothic")
        } else {
            iced::Font::with_name("Microsoft YaHei")
        };
        let accent = self.accent_color();
        let bg_dark = Color::from_rgb8(17, 19, 26);
        let sidebar_bg = Color::from_rgb8(24, 27, 37);
        let text_main = Color::from_rgb8(241, 245, 249);
        let text_muted = Color::from_rgb8(148, 163, 184);

        let mut side_col = column![].spacing(12.0).width(Length::Fill);

        if let Some(logo_bytes) = &self.config.logo_png {
            side_col = side_col.push(
                container(
                    image(image::Handle::from_bytes(logo_bytes.clone()))
                        .width(Length::Fixed(72.0))
                        .height(Length::Fixed(72.0)),
                )
                .padding([4.0, 0.0]),
            );
        }

        side_col = side_col
            .push(text(&self.config.game_title).size(20).color(text_main))
            .push(
                text(format!("v{} • {}", self.config.version, self.config.publisher))
                    .size(12)
                    .color(text_muted),
            );

        if let Some(banner_bytes) = &self.config.banner_png {
            side_col = side_col.push(Space::with_height(8.0)).push(
                container(
                    image(image::Handle::from_bytes(banner_bytes.clone()))
                        .width(Length::Fill)
                        .height(Length::Fixed(110.0))
                        .content_fit(iced::ContentFit::Cover),
                )
                .width(Length::Fill)
                .clip(true),
            );
        }

        let step_badge = |label: &'static str, active: bool| {
            let col = if active { text_main } else { text_muted };
            let prefix = if active { "● " } else { "○ " };
            text(format!("{}{}", prefix, label))
                .font(ui_font)
                .size(13)
                .color(col)
        };

        if matches!(
            self.step,
            WizardStep::SelectLanguage { is_uninstall: true }
                | WizardStep::UninstallConfirm
                | WizardStep::UninstallProgress
                | WizardStep::UninstallDone
        ) {
            side_col = side_col
                .push(Space::with_height(Length::Fill))
                .push(step_badge(
                    t.step_language,
                    matches!(self.step, WizardStep::SelectLanguage { .. }),
                ))
                .push(step_badge(t.step_confirm, self.step == WizardStep::UninstallConfirm))
                .push(step_badge(t.step_removing, self.step == WizardStep::UninstallProgress))
                .push(step_badge(t.step_finished, self.step == WizardStep::UninstallDone));
        } else {
            side_col = side_col
                .push(Space::with_height(Length::Fill))
                .push(step_badge(
                    t.step_language,
                    matches!(self.step, WizardStep::SelectLanguage { .. }),
                ))
                .push(step_badge(t.step_welcome, self.step == WizardStep::Welcome));
            if self.config.has_any_license_text() {
                side_col = side_col.push(step_badge(t.step_terms, self.step == WizardStep::License));
            }
            side_col = side_col
                .push(step_badge(t.step_location, self.step == WizardStep::Location))
                .push(step_badge(t.step_installing, self.step == WizardStep::Installing))
                .push(step_badge(t.step_complete, self.step == WizardStep::Complete));
        }

        let sidebar = container(side_col)
            .width(Length::Fixed(230.0))
            .height(Length::Fill)
            .padding(20.0)
            .style(move |_| container::Style {
                background: Some(Background::Color(sidebar_bg)),
                border: Border {
                    width: 1.0,
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                    ..Default::default()
                },
                ..Default::default()
            });

        let primary_btn_style = move |_: &iced::Theme, status: button::Status| {
            let bg = match status {
                button::Status::Hovered => Color {
                    a: 0.88,
                    ..accent
                },
                button::Status::Disabled => Color::from_rgb8(55, 60, 75),
                _ => accent,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: Color::WHITE,
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        };

        let secondary_btn_style = |_: &iced::Theme, status: button::Status| {
            let bg = match status {
                button::Status::Hovered => Color::from_rgb8(45, 50, 66),
                _ => Color::from_rgb8(33, 37, 50),
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: Color::from_rgb8(226, 232, 240),
                border: Border {
                    radius: 6.0.into(),
                    width: 1.0,
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.14),
                },
                ..Default::default()
            }
        };

        let body: Element<'_, Message> = match &self.step {
            WizardStep::SelectLanguage { .. } => {
                let mut lang_col = column![
                    text(t.select_lang_title).font(ui_font).size(22).color(text_main),
                    text(t.select_lang_subtitle).font(ui_font).size(14).color(text_muted),
                    Space::with_height(4.0),
                ]
                .spacing(10.0);

                for &lang_opt in &InstallerLang::ALL {
                    let selected = self.lang == lang_opt;
                    let prefix = if selected { "● " } else { "○ " };
                    let mut label_widget = text(format!("{}{}", prefix, lang_opt.display_name()))
                        .size(14)
                        .color(if selected { Color::WHITE } else { text_main });
                    if lang_opt == InstallerLang::Ko {
                        label_widget = label_widget.font(iced::Font::with_name("Malgun Gothic"));
                    }
                    let btn = button(label_widget)
                    .width(Length::Fill)
                    .padding([10.0, 14.0])
                    .style(move |_, status| {
                        let bg = if selected {
                            accent
                        } else {
                            match status {
                                button::Status::Hovered => Color::from_rgb8(42, 47, 62),
                                _ => Color::from_rgb8(27, 30, 42),
                            }
                        };
                        button::Style {
                            background: Some(Background::Color(bg)),
                            text_color: if selected { Color::WHITE } else { text_main },
                            border: Border {
                                radius: 6.0.into(),
                                width: 1.0,
                                color: if selected {
                                    accent
                                } else {
                                    Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                                },
                            },
                            ..Default::default()
                        }
                    })
                    .on_press(Message::SelectLanguage(lang_opt));
                    lang_col = lang_col.push(btn);
                }

                lang_col = lang_col.push(Space::with_height(Length::Fill)).push(
                    row![
                        Space::with_width(Length::Fill),
                        button(text(t.cancel_btn).font(ui_font).size(14))
                            .padding([8.0, 18.0])
                            .style(secondary_btn_style)
                            .on_press(Message::Cancel),
                        button(text(t.continue_btn).font(ui_font).size(14))
                            .padding([8.0, 22.0])
                            .style(primary_btn_style)
                            .on_press(Message::ConfirmLanguage),
                    ]
                    .spacing(10.0),
                );

                lang_col.height(Length::Fill).into()
            }

            WizardStep::Welcome => column![
                text(format!("{} {}", t.welcome_title_prefix, self.config.game_title))
                    .font(ui_font)
                    .size(22)
                    .color(text_main),
                text(&self.config.welcome_text).font(ui_font).size(14).color(text_muted),
                text(t.welcome_hint).font(ui_font).size(13).color(text_muted),
                Space::with_height(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    button(text(t.back_btn).font(ui_font).size(14))
                        .padding([8.0, 18.0])
                        .style(secondary_btn_style)
                        .on_press(Message::Back),
                    button(text(t.next_btn).font(ui_font).size(14))
                        .padding([8.0, 22.0])
                        .style(primary_btn_style)
                        .on_press(Message::Next),
                ]
                .spacing(10.0),
            ]
            .spacing(16.0)
            .height(Length::Fill)
            .into(),

            WizardStep::License => {
                let policy_body = self
                    .config
                    .resolve_license_text_by_code(&self.policy_lang_code)
                    .unwrap_or_default();
                let available_langs: Vec<PolicyLangOption> = self
                    .config
                    .available_policy_languages()
                    .into_iter()
                    .map(|(code, label)| PolicyLangOption { code, label })
                    .collect();
                let selected_opt = available_langs
                    .iter()
                    .find(|o| o.code == self.policy_lang_code)
                    .cloned();

                let mut next_btn = button(text(t.next_btn).font(ui_font).size(14))
                    .padding([8.0, 22.0])
                    .style(primary_btn_style);
                if self.accepted_license {
                    next_btn = next_btn.on_press(Message::Next);
                }

                let lang_picker_row: Element<'_, Message> = if available_langs.len() > 1 {
                    row![
                        text(format!("{}:", t.step_language))
                            .font(ui_font)
                            .size(12)
                            .color(text_muted),
                        pick_list(available_langs, selected_opt, |opt: PolicyLangOption| {
                            Message::SelectPolicyLanguage(opt.code)
                        })
                        .font(ui_font)
                        .text_size(12)
                        .width(Length::Fixed(240.0))
                        .padding([5.0, 10.0]),
                    ]
                    .spacing(8.0)
                    .align_y(Alignment::Center)
                    .into()
                } else {
                    Space::with_height(0.0).into()
                };

                column![
                    text(t.terms_title).font(ui_font).size(22).color(text_main),
                    text(t.terms_subtitle).font(ui_font).size(13).color(text_muted),
                    container(
                        scrollable(
                            {
                                let mut ptxt = text(policy_body).size(13).color(text_main);
                                if self.policy_lang_code == "ko" {
                                    ptxt = ptxt.font(iced::Font::with_name("Malgun Gothic"));
                                }
                                ptxt
                            },
                        )
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(12.0)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgb8(12, 14, 20))),
                        border: Border {
                            radius: 6.0.into(),
                            width: 1.0,
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                        },
                        ..Default::default()
                    }),
                    lang_picker_row,
                    checkbox(t.terms_accept, self.accepted_license)
                        .font(ui_font)
                        .on_toggle(Message::ToggleAcceptLicense),
                    row![
                        Space::with_width(Length::Fill),
                        button(text(t.back_btn).font(ui_font).size(14))
                            .padding([8.0, 18.0])
                            .style(secondary_btn_style)
                            .on_press(Message::Back),
                        next_btn,
                    ]
                    .spacing(10.0),
                ]
                .spacing(12.0)
                .height(Length::Fill)
                .into()
            }

            WizardStep::Location => column![
                text(t.location_title).font(ui_font).size(22).color(text_main),
                text(t.location_subtitle).font(ui_font).size(13).color(text_muted),
                text(t.install_folder_label).font(ui_font).size(12).color(text_muted),
                row![
                    text_input("Install directory...", &self.install_dir)
                        .on_input(Message::InstallDirChanged)
                        .padding(8.0)
                        .width(Length::Fill),
                    button(text(t.browse_btn).font(ui_font).size(13))
                        .padding([8.0, 14.0])
                        .style(secondary_btn_style)
                        .on_press(Message::BrowseInstallDir),
                ]
                .spacing(8.0)
                .align_y(Alignment::Center),
                checkbox(t.desktop_shortcut_label, self.desktop_shortcut)
                    .font(ui_font)
                    .on_toggle(Message::ToggleDesktopShortcut),
                checkbox(t.start_menu_shortcut_label, self.start_menu_shortcut)
                    .font(ui_font)
                    .on_toggle(Message::ToggleStartMenuShortcut),
                Space::with_height(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    button(text(t.back_btn).font(ui_font).size(14))
                        .padding([8.0, 18.0])
                        .style(secondary_btn_style)
                        .on_press(Message::Back),
                    button(text(t.install_btn).font(ui_font).size(14))
                        .padding([8.0, 24.0])
                        .style(primary_btn_style)
                        .on_press(Message::RunInstall),
                ]
                .spacing(10.0),
            ]
            .spacing(12.0)
            .height(Length::Fill)
            .into(),

            WizardStep::Installing => {
                let pct = (self.install_progress * 100.0).clamp(0.0, 100.0).round() as u32;
                column![
                    text(format!("{} {}", t.installing_title_prefix, self.config.game_title))
                        .font(ui_font)
                        .size(22)
                        .color(text_main),
                    text(t.installing_subtitle)
                        .font(ui_font)
                        .size(14)
                        .color(text_muted),
                    Space::with_height(12.0),
                    row![
                        text(&self.install_status).font(ui_font).size(13).color(text_main),
                        Space::with_width(Length::Fill),
                        text(format!("{}%", pct)).font(ui_font).size(13).color(accent),
                    ]
                    .align_y(Alignment::Center),
                    progress_bar(0.0..=1.0, self.install_progress)
                        .width(Length::Fill)
                        .height(Length::Fixed(10.0)),
                    Space::with_height(Length::Fill),
                ]
                .spacing(14.0)
                .height(Length::Fill)
                .into()
            }

            WizardStep::Complete => column![
                text(t.complete_title).font(ui_font).size(22).color(text_main),
                text(format!(
                    "{} {}\n{}",
                    self.config.game_title, t.complete_subtitle_prefix, self.install_dir
                ))
                .font(ui_font)
                .size(14)
                .color(text_muted),
                checkbox(t.complete_launch_label, self.launch_after)
                    .font(ui_font)
                    .on_toggle(Message::ToggleLaunchAfter),
                Space::with_height(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    button(text(t.finish_btn).font(ui_font).size(14))
                        .padding([8.0, 24.0])
                        .style(primary_btn_style)
                        .on_press(Message::Finish),
                ],
            ]
            .spacing(16.0)
            .height(Length::Fill)
            .into(),

            WizardStep::Error(err) => column![
                text(t.setup_error_title).font(ui_font).size(22).color(Color::from_rgb8(248, 113, 113)),
                text(err).size(14).color(text_main),
                Space::with_height(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    button(text(t.close_btn).font(ui_font).size(14))
                        .padding([8.0, 22.0])
                        .style(secondary_btn_style)
                        .on_press(Message::Cancel),
                ],
            ]
            .spacing(16.0)
            .height(Length::Fill)
            .into(),

            WizardStep::UninstallConfirm => {
                let save_dir_hint = app_data_game_dir(&self.uninstall_title)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "%APPDATA%\\BlancGames".to_string());
                column![
                    text(format!("{} {}", t.uninstall_title_prefix, self.uninstall_title))
                        .font(ui_font)
                        .size(22)
                        .color(text_main),
                    text(format!(
                        "{}\n{}",
                        t.uninstall_subtitle_prefix, self.uninstall_title
                    ))
                    .font(ui_font)
                    .size(14)
                    .color(text_muted),
                    container(
                        column![
                            text(t.uninstall_loc_label).font(ui_font).size(12).color(text_muted),
                            text(&self.install_dir).size(13).color(text_main),
                            Space::with_height(6.0),
                            text(t.uninstall_items_label).font(ui_font).size(12).color(text_muted),
                            text(t.uninstall_items_list)
                                .font(ui_font)
                                .size(13)
                                .color(text_main),
                        ]
                        .spacing(4.0)
                    )
                    .width(Length::Fill)
                    .padding(14.0)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgb8(24, 27, 37))),
                        border: Border {
                            radius: 8.0.into(),
                            width: 1.0,
                            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                        },
                        ..Default::default()
                    }),
                    checkbox(t.uninstall_remove_saves, self.remove_save_data)
                        .font(ui_font)
                        .on_toggle(Message::ToggleRemoveSaveData)
                        .size(16)
                        .text_size(14)
                        .style(move |_, _| checkbox::Style {
                            background: Background::Color(if self.remove_save_data {
                                accent
                            } else {
                                Color::from_rgb8(30, 34, 46)
                            }),
                            icon_color: Color::WHITE,
                            border: Border {
                                radius: 4.0.into(),
                                width: 1.0,
                                color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
                            },
                            text_color: Some(text_main),
                        }),
                    text(format!("{} {}", t.uninstall_save_folder_prefix, save_dir_hint))
                        .font(ui_font)
                        .size(11)
                        .color(text_muted),
                    Space::with_height(Length::Fill),
                    row![
                        Space::with_width(Length::Fill),
                        button(text(t.cancel_btn).font(ui_font).size(14))
                            .padding([8.0, 18.0])
                            .style(secondary_btn_style)
                            .on_press(Message::Cancel),
                        button(text(t.uninstall_btn).font(ui_font).size(14))
                            .padding([8.0, 22.0])
                            .style(primary_btn_style)
                            .on_press(Message::RunUninstall),
                    ]
                    .spacing(10.0),
                ]
                .spacing(12.0)
                .height(Length::Fill)
                .into()
            }

            WizardStep::UninstallProgress => column![
                text(format!("{} {}...", t.uninstalling_title_prefix, self.uninstall_title))
                    .font(ui_font)
                    .size(22)
                    .color(text_main),
                text(t.uninstalling_subtitle)
                    .font(ui_font)
                    .size(14)
                    .color(text_muted),
                Space::with_height(12.0),
                text(&self.uninstall_status).font(ui_font).size(13).color(text_main),
                progress_bar(0.0..=1.0, self.uninstall_progress)
                    .width(Length::Fill)
                    .height(Length::Fixed(10.0)),
                Space::with_height(Length::Fill),
            ]
            .spacing(14.0)
            .height(Length::Fill)
            .into(),

            WizardStep::UninstallDone => {
                let save_note = if self.remove_save_data {
                    t.saves_removed_note
                } else {
                    t.saves_kept_note
                };
                column![
                    text(t.uninstall_done_title).font(ui_font).size(22).color(text_main),
                    text(format!(
                        "{} {}",
                        self.uninstall_title, t.uninstall_done_msg_suffix
                    ))
                    .font(ui_font)
                    .size(14)
                    .color(text_main),
                    text(format!("{}\n{}", t.uninstall_done_detail, save_note))
                        .font(ui_font)
                        .size(13)
                        .color(text_muted),
                    Space::with_height(Length::Fill),
                    row![
                        Space::with_width(Length::Fill),
                        button(text(t.finish_btn).font(ui_font).size(14))
                            .padding([8.0, 24.0])
                            .style(primary_btn_style)
                            .on_press(Message::FinishUninstall),
                    ],
                ]
                .spacing(14.0)
                .height(Length::Fill)
                .into()
            }
        };

        let content_area = container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(26.0);

        container(row![sidebar, content_area].width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(bg_dark)),
                ..Default::default()
            })
            .into()
    }
}

fn sanitize_name(raw: &str) -> String {
    sanitize_installer_name(raw)
}

fn app_data_game_dir(project_name: &str) -> Option<PathBuf> {
    let safe_title: String = project_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let folder = if safe_title.trim_matches('_').is_empty() {
        "BlancGame".to_string()
    } else {
        safe_title
    };
    std::env::var_os("APPDATA")
        .map(|appdata| PathBuf::from(appdata).join("BlancGames").join(folder))
}

fn default_install_path(subdir: &str) -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|p| p.join("Programs"))
        .unwrap_or_else(|| PathBuf::from("C:\\Games"));
    base.join(sanitize_name(subdir))
}

#[cfg(windows)]
fn run_hidden_powershell(script: &str) -> Result<(), String> {
    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map_err(|e| format!("Failed to invoke PowerShell: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err("PowerShell helper returned non-zero status".to_string())
    }
}

fn ps_escape(s: &str) -> String {
    s.replace('\'', "''")
}

fn folder_size_kb(dir: &Path) -> u64 {
    fn walk(p: &Path, acc: &mut u64) {
        if let Ok(entries) = std::fs::read_dir(p) {
            for e in entries.flatten() {
                let path = e.path();
                if path.is_dir() {
                    walk(&path, acc);
                } else if let Ok(meta) = e.metadata() {
                    *acc = acc.saturating_add(meta.len());
                }
            }
        }
    }
    let mut bytes = 0u64;
    walk(dir, &mut bytes);
    (bytes / 1024).max(1)
}

fn register_windows_install(
    install_dir: &Path,
    manifest: &InstallerPayloadManifest,
    desktop_shortcut: bool,
    start_menu_shortcut: bool,
) -> Result<(), String> {
    #[cfg(windows)]
    {
        let title = &manifest.config.game_title;
        let clean = sanitize_name(title);
        let game_exe = install_dir.join(&manifest.exe_name);
        let uninstall_exe = install_dir.join(uninstall_exe_filename(title));
        let reg_key = format!(
            "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\BlancGame_{}",
            clean.replace(' ', "_")
        );
        let est_kb = folder_size_kb(install_dir);

        let mut script = String::new();
        if desktop_shortcut {
            script.push_str(&format!(
                "$ws = New-Object -ComObject WScript.Shell; $lnk = $ws.CreateShortcut([Environment]::GetFolderPath('Desktop') + '\\{}.lnk'); $lnk.TargetPath = '{}'; $lnk.WorkingDirectory = '{}'; $lnk.IconLocation = '{},0'; $lnk.Save();\n",
                ps_escape(&clean),
                ps_escape(&game_exe.to_string_lossy()),
                ps_escape(&install_dir.to_string_lossy()),
                ps_escape(&game_exe.to_string_lossy()),
            ));
        }
        if start_menu_shortcut {
            let uninstall_args = format!(
                "--uninstall \"{}\" --title \"{}\"",
                install_dir.to_string_lossy(),
                title
            );
            script.push_str(&format!(
                "$sm = [Environment]::GetFolderPath('Programs') + '\\{}'; New-Item -ItemType Directory -Force -Path $sm | Out-Null; $ws = New-Object -ComObject WScript.Shell; $lnk = $ws.CreateShortcut($sm + '\\{}.lnk'); $lnk.TargetPath = '{}'; $lnk.WorkingDirectory = '{}'; $lnk.IconLocation = '{},0'; $lnk.Save(); $ulnk = $ws.CreateShortcut($sm + '\\Uninstall {}.lnk'); $ulnk.TargetPath = '{}'; $ulnk.Arguments = '{}'; $ulnk.WorkingDirectory = '{}'; $ulnk.IconLocation = '{},0'; $ulnk.Save();\n",
                ps_escape(&clean),
                ps_escape(&clean),
                ps_escape(&game_exe.to_string_lossy()),
                ps_escape(&install_dir.to_string_lossy()),
                ps_escape(&game_exe.to_string_lossy()),
                ps_escape(&clean),
                ps_escape(&uninstall_exe.to_string_lossy()),
                ps_escape(&uninstall_args),
                ps_escape(&install_dir.to_string_lossy()),
                ps_escape(&uninstall_exe.to_string_lossy()),
            ));
        }

        let uninstall_cmd = format!(
            "\"{}\" --uninstall \"{}\" --title \"{}\"",
            uninstall_exe.to_string_lossy(),
            install_dir.to_string_lossy(),
            title
        );
        script.push_str(&format!(
            "New-Item -Path '{}' -Force | Out-Null; Set-ItemProperty -Path '{}' -Name DisplayName -Value '{}'; Set-ItemProperty -Path '{}' -Name DisplayVersion -Value '{}'; Set-ItemProperty -Path '{}' -Name Publisher -Value '{}'; Set-ItemProperty -Path '{}' -Name InstallLocation -Value '{}'; Set-ItemProperty -Path '{}' -Name DisplayIcon -Value '{}'; Set-ItemProperty -Path '{}' -Name UninstallString -Value '{}'; Set-ItemProperty -Path '{}' -Name EstimatedSize -Type DWord -Value {}; Set-ItemProperty -Path '{}' -Name NoModify -Type DWord -Value 1; Set-ItemProperty -Path '{}' -Name NoRepair -Type DWord -Value 1;",
            ps_escape(&reg_key),
            ps_escape(&reg_key), ps_escape(title),
            ps_escape(&reg_key), ps_escape(&manifest.config.version),
            ps_escape(&reg_key), ps_escape(&manifest.config.publisher),
            ps_escape(&reg_key), ps_escape(&install_dir.to_string_lossy()),
            ps_escape(&reg_key), ps_escape(&game_exe.to_string_lossy()),
            ps_escape(&reg_key), ps_escape(&uninstall_cmd),
            ps_escape(&reg_key), est_kb,
            ps_escape(&reg_key),
            ps_escape(&reg_key),
        ));

        let _ = run_hidden_powershell(&script);
    }
    Ok(())
}

fn stop_running_game_process(game_exe_name: &str) {
    #[cfg(windows)]
    {
        if !game_exe_name.trim().is_empty() {
            let _ = Command::new("taskkill")
                .args(["/IM", game_exe_name.trim(), "/F"])
                .creation_flags(CREATE_NO_WINDOW)
                .status();
        }
    }
    #[cfg(not(windows))]
    {
        let _ = game_exe_name;
    }
}

fn remove_windows_shortcuts_and_registry(title: &str) {
    #[cfg(windows)]
    {
        let clean = sanitize_name(title);
        let reg_key = format!(
            "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\BlancGame_{}",
            clean.replace(' ', "_")
        );
        let script = format!(
            "Remove-Item -Path ([Environment]::GetFolderPath('Desktop') + '\\{}.lnk') -Force -ErrorAction SilentlyContinue; Remove-Item -Path ([Environment]::GetFolderPath('Programs') + '\\{}') -Recurse -Force -ErrorAction SilentlyContinue; Remove-Item -Path '{}' -Force -ErrorAction SilentlyContinue;",
            ps_escape(&clean),
            ps_escape(&clean),
            ps_escape(&reg_key),
        );
        let _ = run_hidden_powershell(&script);
    }
    #[cfg(not(windows))]
    {
        let _ = title;
    }
}

fn remove_installed_files_except_self(install_dir: &Path, self_exe: &Path) {
    let self_name = self_exe.file_name().map(|n| n.to_os_string());
    if let Ok(entries) = std::fs::read_dir(install_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if self_name.as_ref().is_some_and(|n| p.file_name() == Some(n.as_os_str())) {
                continue;
            }
            if p.is_dir() {
                let _ = std::fs::remove_dir_all(&p);
            } else {
                let _ = std::fs::remove_file(&p);
            }
        }
    }
}

fn schedule_self_folder_cleanup(install_dir: &Path, self_exe: &Path) {
    #[cfg(windows)]
    {
        let pid = std::process::id();
        let dir_esc = ps_escape(&install_dir.to_string_lossy());
        let exe_esc = ps_escape(&self_exe.to_string_lossy());
        let script = format!(
            "$p = Get-Process -Id {} -ErrorAction SilentlyContinue; if ($p) {{ $p.WaitForExit(15000) | Out-Null }}; for ($i = 0; $i -lt 20; $i++) {{ Remove-Item -LiteralPath '{}' -Force -ErrorAction SilentlyContinue; Remove-Item -LiteralPath '{}' -Recurse -Force -ErrorAction SilentlyContinue; if (-not (Test-Path -LiteralPath '{}')) {{ break }}; Start-Sleep -Milliseconds 250 }}",
            pid, exe_esc, dir_esc, dir_esc
        );
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &script,
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = (install_dir, self_exe);
    }
}

fn load_installer_window_icon() -> Option<iced::window::Icon> {
    let exe_path = std::env::current_exe().ok()?;
    let logo_bytes = read_embedded_payload(&exe_path)
        .ok()
        .and_then(|(m, _)| m.config.logo_png)
        .or_else(|| {
            exe_path
                .parent()
                .and_then(|dir| std::fs::read(dir.join(UNINSTALL_META_FILENAME)).ok())
                .and_then(|bytes| serde_json::from_slice::<UninstallMetadata>(&bytes).ok())
                .and_then(|meta| meta.logo_png)
        })
        .unwrap_or_else(blanc_core::win_icon::default_blanc_logo_png);

    let (rgba, w, h) = blanc_core::win_icon::decode_window_icon_rgba(&logo_bytes)?;
    iced::window::icon::from_rgba(rgba, w, h).ok()
}

pub fn main() -> iced::Result {
    let icon = load_installer_window_icon();
    let mut app = iced::application(InstallerApp::title, InstallerApp::update, InstallerApp::view)
        .window(iced::window::Settings {
            size: Size::new(760.0, 500.0),
            resizable: false,
            icon,
            ..Default::default()
        });

    #[cfg(windows)]
    {
        if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
            app = app
                .font(bytes)
                .default_font(iced::Font::with_name("Microsoft YaHei"));
        }
        for font_path in [
            "C:\\Windows\\Fonts\\YuGothR.ttc",
            "C:\\Windows\\Fonts\\YuGothM.ttc",
            "C:\\Windows\\Fonts\\malgun.ttf",
        ] {
            if let Ok(bytes) = std::fs::read(font_path) {
                app = app.font(bytes);
            }
        }
    }

    app.run_with(InstallerApp::new)
}
