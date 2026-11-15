use std::path::PathBuf;
use std::process::Command;
use iced::{Element, Task};

use blanc_core::asset::AssetCategory;
use blanc_core::flow::FlowNodeType;
use blanc_core::project::Project;
use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceOption as SceneChoiceOption, Scene};
use blanc_core::story::{Chapter, Story, StoryElement};
use crate::screens::{
    handle_ui_designer_message, AssetPickerModalState, AssetPickerTarget, DashboardMessage,
    DashboardState, FlowCanvasAction, UiAssetSlot, WelcomeMessage, WelcomeState, WorkflowTab,
    DEFAULT_SCRIPT_TEMPLATE,
};

#[derive(Debug, Clone)]
pub enum AppMessage {
    Welcome(WelcomeMessage),
    Dashboard(DashboardMessage),
}

struct ExportJobSnapshot {
    export_exe_name: String,
    export_output_dir: String,
    export_packed: bool,
    export_save_mode: blanc_core::pack::ExportSaveMode,
    export_build_kind: blanc_core::installer::ExportBuildKind,
    logo_asset_id: Option<String>,
    title_bg_asset_id: Option<String>,
    accent_hex: String,
    installer_include_logo: bool,
    installer_include_banner: bool,
    installer_custom_banner_path: Option<PathBuf>,
    installer_publisher: String,
    installer_version: String,
    installer_welcome_text: String,
    installer_desktop_shortcut: bool,
    installer_start_menu_shortcut: bool,
    license_translations: std::collections::BTreeMap<String, String>,
    custom_policy_languages: Vec<blanc_core::installer::CustomPolicyLanguage>,
    project: Project,
    project_dir: PathBuf,
}

pub enum CurrentScreen {
    Welcome(WelcomeState),
    Workspace {
        project: Project,
        project_dir: PathBuf,
        dashboard: DashboardState,
    },
}

impl Default for CurrentScreen {
    fn default() -> Self {
        let cfg = crate::screens::welcome::HubConfig::load();
        if cfg.reopen_last_project {
            if let Some(last) = cfg.recent_projects.first() {
                if last.path.exists() {
                    if let Ok(project) = Project::load_from_manifest(&last.path) {
                        let project_dir = last.path.parent().unwrap_or(&last.path).to_path_buf();
                        let mut dashboard = DashboardState::init_for_project(&project_dir);
                        dashboard.installer_publisher = project.author.clone();
                        if dashboard.export_output_dir.trim().is_empty() {
                            if let Some(exp) = &cfg.default_export_folder {
                                dashboard.export_output_dir = exp.to_string_lossy().to_string();
                            }
                        }
                        return Self::Workspace {
                            project,
                            project_dir,
                            dashboard,
                        };
                    }
                }
            }
        }
        Self::Welcome(WelcomeState::new())
    }
}

pub struct EditorApp {
    screen: CurrentScreen,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            screen: CurrentScreen::default(),
        }
    }
}

impl EditorApp {
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Welcome(w_msg) => self.update_welcome(w_msg),
            AppMessage::Dashboard(d_msg) => self.update_dashboard(d_msg),
        }
    }

    fn update_welcome(&mut self, message: WelcomeMessage) -> Task<AppMessage> {
        let CurrentScreen::Welcome(state) = &mut self.screen else {
            return Task::none();
        };

        match message {
            WelcomeMessage::ToggleSidebar => {
                state.is_sidebar_expanded = !state.is_sidebar_expanded;
            }
            WelcomeMessage::SelectTab(tab) => {
                state.active_tab = tab;
                state.error_message = None;
            }
            WelcomeMessage::SelectDocsTopic(topic) => {
                state.active_docs_topic = topic;
            }
            WelcomeMessage::SearchQueryChanged(query) => {
                state.search_query = query;
            }
            WelcomeMessage::ShowCreateModal => {
                state.is_creating_project = true;
                state.error_message = None;
                if state.selected_folder.is_none() {
                    state.selected_folder = state.config.default_folder.clone();
                }
                if !state.config.default_author.trim().is_empty() {
                    state.input_author = state.config.default_author.clone();
                }
            }
            WelcomeMessage::CancelCreateModal => {
                state.is_creating_project = false;
                state.error_message = None;
            }
            WelcomeMessage::ProjectNameChanged(name) => {
                state.input_name = name;
            }
            WelcomeMessage::AuthorNameChanged(author) => {
                state.input_author = author;
            }
            WelcomeMessage::BrowseProjectLocation => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    state.selected_folder = Some(folder);
                }
            }
            WelcomeMessage::CreateProjectConfirmed => {
                let name = state.input_name.trim();
                let author = state.input_author.trim();

                if name.is_empty() {
                    state.error_message = Some("Mohon masukkan nama proyek yang valid.".to_string());
                    return Task::none();
                }

                let Some(base_folder) = &state.selected_folder else {
                    state.error_message = Some("Mohon pilih folder tujuan penyimpanan proyek.".to_string());
                    return Task::none();
                };

                let project_folder_name = name.replace(' ', "_");
                let target_dir = base_folder.join(&project_folder_name);

                match Project::create_project_structure(&target_dir, name, author) {
                    Ok(project) => {
                        let manifest_filename = format!("{}.{}", project_folder_name, Project::MANIFEST_EXTENSION);
                        let manifest_path = target_dir.join(manifest_filename);
                        state.config.add_recent(name, author, manifest_path);

                        let mut dashboard = DashboardState::init_for_project(&target_dir);
                        dashboard.installer_publisher = project.author.clone();
                        let (res_w, res_h) = state.config.default_resolution;
                        dashboard.system_screens_config.default_resolution_width = res_w;
                        dashboard.system_screens_config.default_resolution_height = res_h;
                        let config_dir = target_dir.join("Settings");
                        let _ = std::fs::create_dir_all(&config_dir);
                        let _ = dashboard
                            .system_screens_config
                            .save_to_file(&config_dir.join("system_screens.bconfig"));

                        let def_lang = state.config.default_language_code.trim();
                        if !def_lang.is_empty() {
                            let lang_name = match def_lang.to_ascii_lowercase().as_str() {
                                "id" => "Indonesian",
                                "ja" => "日本語",
                                _ => "English",
                            };
                            dashboard.locale_manifest.default_language =
                                blanc_core::localization::LanguageEntry::new(def_lang, lang_name);
                            dashboard.locale_default_code = def_lang.to_string();
                            dashboard.locale_default_name = lang_name.to_string();
                            dashboard.locale_selected_code = def_lang.to_string();
                            let _ = Self::save_localization(&dashboard, &target_dir);
                        }
                        if dashboard.export_output_dir.trim().is_empty() {
                            if let Some(exp) = &state.config.default_export_folder {
                                dashboard.export_output_dir = exp.to_string_lossy().to_string();
                            }
                        }
                        self.screen = CurrentScreen::Workspace {
                            project,
                            project_dir: target_dir.clone(),
                            dashboard,
                        };
                    }
                    Err(err) => {
                        state.error_message = Some(format!("Gagal membuat proyek: {}", err));
                    }
                }
            }
            WelcomeMessage::OpenProjectClicked => {
                if let Some(file_path) = rfd::FileDialog::new()
                    .add_filter("Blanc Project", &[Project::MANIFEST_EXTENSION])
                    .pick_file()
                {
                    match Project::load_from_manifest(&file_path) {
                        Ok(project) => {
                            let project_dir = file_path.parent().unwrap_or(&file_path).to_path_buf();
                            state.config.add_recent(&project.name, &project.author, file_path);
                            let mut dashboard = DashboardState::init_for_project(&project_dir);
                            dashboard.installer_publisher = project.author.clone();
                            if dashboard.export_output_dir.trim().is_empty() {
                                if let Some(exp) = &state.config.default_export_folder {
                                    dashboard.export_output_dir = exp.to_string_lossy().to_string();
                                }
                            }
                            self.screen = CurrentScreen::Workspace {
                                project,
                                project_dir: project_dir.clone(),
                                dashboard,
                            };
                        }
                        Err(err) => {
                            state.error_message = Some(format!("Gagal membuka proyek: {}", err));
                        }
                    }
                }
            }
            WelcomeMessage::OpenRecentProject(path) => {
                match Project::load_from_manifest(&path) {
                    Ok(project) => {
                        let project_dir = path.parent().unwrap_or(&path).to_path_buf();
                        state.config.add_recent(&project.name, &project.author, path);
                        let mut dashboard = DashboardState::init_for_project(&project_dir);
                        dashboard.installer_publisher = project.author.clone();
                        if dashboard.export_output_dir.trim().is_empty() {
                            if let Some(exp) = &state.config.default_export_folder {
                                dashboard.export_output_dir = exp.to_string_lossy().to_string();
                            }
                        }
                        self.screen = CurrentScreen::Workspace {
                            project,
                            project_dir: project_dir.clone(),
                            dashboard,
                        };
                    }
                    Err(err) => {
                        state.error_message = Some(format!(
                            "Gagal membuka proyek '{}': {}. Pastikan file proyek masih ada.",
                            path.display(),
                            err
                        ));
                    }
                }
            }
            WelcomeMessage::RemoveRecentProject(path) => {
                state.config.remove_recent(&path);
            }
            WelcomeMessage::RevealInExplorer(path) => {
                #[cfg(target_os = "windows")]
                let _ = Command::new("explorer").arg(path).spawn();
                #[cfg(target_os = "macos")]
                let _ = Command::new("open").arg(path).spawn();
                #[cfg(target_os = "linux")]
                let _ = Command::new("xdg-open").arg(path).spawn();
            }
            WelcomeMessage::ChangeDefaultFolder => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    state.config.default_folder = Some(folder.clone());
                    state.selected_folder = Some(folder);
                    state.config.save();
                    state.settings_status_message = Some("Default project directory updated.".to_string());
                }
            }
            WelcomeMessage::OpenDefaultFolder => {
                let target = state.config.default_folder.clone().or_else(|| {
                    std::env::var("USERPROFILE")
                        .map(|p| PathBuf::from(p).join("Documents"))
                        .ok()
                });
                if let Some(dir) = target {
                    #[cfg(target_os = "windows")]
                    let _ = Command::new("explorer").arg(dir).spawn();
                    #[cfg(target_os = "macos")]
                    let _ = Command::new("open").arg(dir).spawn();
                    #[cfg(target_os = "linux")]
                    let _ = Command::new("xdg-open").arg(dir).spawn();
                }
            }
            WelcomeMessage::ChangeDefaultExportFolder => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    state.config.default_export_folder = Some(folder);
                    state.config.save();
                    state.settings_status_message = Some("Default export directory updated.".to_string());
                }
            }
            WelcomeMessage::ClearDefaultExportFolder => {
                state.config.default_export_folder = None;
                state.config.save();
                state.settings_status_message = Some("Default export directory cleared.".to_string());
            }
            WelcomeMessage::OpenConfigFolder => {
                let cfg_path = crate::screens::welcome::HubConfig::config_path();
                if let Some(dir) = cfg_path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                    #[cfg(target_os = "windows")]
                    let _ = Command::new("explorer").arg(dir).spawn();
                    #[cfg(target_os = "macos")]
                    let _ = Command::new("open").arg(dir).spawn();
                    #[cfg(target_os = "linux")]
                    let _ = Command::new("xdg-open").arg(dir).spawn();
                }
            }
            WelcomeMessage::ClearRecentProjects => {
                state.config.recent_projects.clear();
                state.config.save();
                state.settings_status_message = Some("Recent projects history cleared.".to_string());
            }
            WelcomeMessage::ResetHubPreferences => {
                let recents = state.config.recent_projects.clone();
                let onboarded = state.config.onboarding_completed;
                state.config = crate::screens::welcome::HubConfig::default();
                state.config.recent_projects = recents;
                state.config.onboarding_completed = onboarded;
                state.selected_folder = state.config.default_folder.clone();
                state.input_author = state.config.default_author.clone();
                state.config.save();
                state.settings_status_message = Some("Global preferences reset to defaults (recent projects kept).".to_string());
            }
            WelcomeMessage::DefaultAuthorChanged(val) => {
                state.config.default_author = val.clone();
                state.input_author = val;
                state.config.save();
            }
            WelcomeMessage::SetDefaultResolution(w, h) => {
                state.config.default_resolution = (w, h);
                state.config.save();
                state.settings_status_message = Some(format!("Default new-project resolution set to {}×{}.", w, h));
            }
            WelcomeMessage::SetDefaultLanguageCode(code) => {
                state.config.default_language_code = code.clone();
                state.config.save();
                state.settings_status_message = Some(format!("Default source language set to '{}'.", code));
            }
            WelcomeMessage::ToggleAutoSaveOnPlayPreview(val) => {
                state.config.auto_save_on_play_preview = val;
                state.config.save();
            }
            WelcomeMessage::ToggleReopenLastProject(val) => {
                state.config.reopen_last_project = val;
                state.config.save();
            }
            WelcomeMessage::ToggleConfirmBeforeDelete(val) => {
                state.config.confirm_before_delete = val;
                state.config.save();
            }
            WelcomeMessage::OnboardingNextStep => {
                if state.onboarding_step < 2 {
                    state.onboarding_step += 1;
                }
            }
            WelcomeMessage::OnboardingPrevStep => {
                if state.onboarding_step > 0 {
                    state.onboarding_step -= 1;
                }
            }
            WelcomeMessage::OnboardingSkip | WelcomeMessage::OnboardingFinishToHub => {
                state.config.onboarding_completed = true;
                state.config.save();
                state.show_onboarding = false;
                state.active_tab = crate::screens::welcome::HubTab::Projects;
            }
            WelcomeMessage::OnboardingFinishCreateProject => {
                state.config.onboarding_completed = true;
                state.config.save();
                state.show_onboarding = false;
                state.active_tab = crate::screens::welcome::HubTab::Projects;
                state.is_creating_project = true;
                state.error_message = None;
                if state.selected_folder.is_none() {
                    state.selected_folder = state.config.default_folder.clone();
                }
                if !state.config.default_author.trim().is_empty() {
                    state.input_author = state.config.default_author.clone();
                }
            }
            WelcomeMessage::ReplayOnboarding => {
                state.onboarding_step = 0;
                state.show_onboarding = true;
            }
            WelcomeMessage::SubmitTitleChanged(v) => {
                state.submit_title = v;
            }
            WelcomeMessage::SubmitAuthorChanged(v) => {
                state.submit_author = v;
            }
            WelcomeMessage::SubmitStatusChanged(s) => {
                state.submit_status = s;
            }
            WelcomeMessage::SubmitGenreChanged(v) => {
                state.submit_genre = v;
            }
            WelcomeMessage::SubmitCoverUrlChanged(v) => {
                state.submit_cover_url = v;
            }
            WelcomeMessage::SubmitGameUrlChanged(v) => {
                state.submit_game_url = v;
            }
            WelcomeMessage::SubmitDescriptionChanged(v) => {
                state.submit_description = v;
            }
            WelcomeMessage::SubmitFillFromRecent => {
                if let Some(recent) = state.config.recent_projects.first() {
                    state.submit_title = recent.name.clone();
                    state.submit_author = recent.author.clone();
                }
            }
            WelcomeMessage::SubmitResetForm => {
                state.submit_title.clear();
                state.submit_author = if state.config.default_author.trim().is_empty() {
                    "Creator".to_string()
                } else {
                    state.config.default_author.clone()
                };
                state.submit_status = crate::screens::welcome::GameSubmitStatus::Released;
                state.submit_genre.clear();
                state.submit_cover_url.clear();
                state.submit_game_url.clear();
                state.submit_description.clear();
                state.submit_feedback = None;
            }
            WelcomeMessage::SubmitGameClicked => {
                let title = state.submit_title.trim().to_string();
                let author = state.submit_author.trim().to_string();
                let status = state.submit_status.as_form_value();
                let genre = state.submit_genre.trim().to_string();
                let cover = state.submit_cover_url.trim().to_string();
                let game_url = state.submit_game_url.trim().to_string();
                let description = state.submit_description.trim().to_string();

                let is_http = |u: &str| u.starts_with("https://") || u.starts_with("http://");

                if title.is_empty()
                    || author.is_empty()
                    || genre.is_empty()
                    || cover.is_empty()
                    || game_url.is_empty()
                    || description.is_empty()
                {
                    state.submit_feedback = Some((false, "Please fill in all required (*) fields.".to_string()));
                    return Task::none();
                }
                if !is_http(&cover) {
                    state.submit_feedback = Some((false, "Cover Image URL must start with https:// or http://".to_string()));
                    return Task::none();
                }
                if !is_http(&game_url) {
                    state.submit_feedback = Some((false, "Game / Download URL must start with https:// or http://".to_string()));
                    return Task::none();
                }

                state.submit_in_progress = true;
                state.submit_feedback = None;

                return Task::perform(
                    async move {
                        let res = tokio::task::spawn_blocking(move || -> Result<(), String> {
                            const FORM_ACTION_URL: &str = "https://docs.google.com/forms/d/e/1FAIpQLSfX7ScWucadKsJ9BJ3rEfILWfAh6baSW9ISZy8YiWNd6KlzqQ/formResponse";
                            let resp = ureq::post(FORM_ACTION_URL)
                                .send_form(&[
                                    ("entry.897757391", title.as_str()),
                                    ("entry.203269822", author.as_str()),
                                    ("entry.771996666", status),
                                    ("entry.1232348967", genre.as_str()),
                                    ("entry.908982358", cover.as_str()),
                                    ("entry.153994697", game_url.as_str()),
                                    ("entry.255430852", description.as_str()),
                                ])
                                .map_err(|e| format!("Network error while submitting: {}", e))?;
                            if resp.status() >= 400 {
                                return Err(format!("Server returned status {}. Please try again.", resp.status()));
                            }
                            Ok(())
                        })
                        .await
                        .map_err(|e| format!("Submit task failed: {}", e))?;
                        res
                    },
                    |res| AppMessage::Welcome(WelcomeMessage::SubmitGameCompleted(res)),
                );
            }
            WelcomeMessage::SubmitGameCompleted(res) => {
                state.submit_in_progress = false;
                match res {
                    Ok(()) => {
                        state.submit_title.clear();
                        state.submit_genre.clear();
                        state.submit_cover_url.clear();
                        state.submit_game_url.clear();
                        state.submit_description.clear();
                        state.submit_status = crate::screens::welcome::GameSubmitStatus::Released;
                        state.submit_feedback = Some((
                            true,
                            "Game submitted successfully! It will appear in the showcase once approved.".to_string(),
                        ));
                    }
                    Err(err) => {
                        state.submit_feedback = Some((false, err));
                    }
                }
            }
        }

        Task::none()
    }

    fn save_localization(dashboard: &DashboardState, project_dir: &std::path::Path) -> Result<(), String> {
        dashboard.locale_manifest.save_to_dir(project_dir)?;
        for lang in dashboard.locale_manifest.all_languages() {
            if let Some(tbl) = dashboard.locale_tables.get(&lang.code.to_ascii_lowercase()) {
                tbl.save_to_dir(project_dir, &lang.code)?;
            }
        }
        Ok(())
    }

    fn auto_save_current_scene(dashboard: &DashboardState, project_dir: &std::path::Path) {
        if let Some(scene) = &dashboard.current_scene {
            if dashboard.selected_scene_index < dashboard.scenes_list.len() {
                let scene_name = &dashboard.scenes_list[dashboard.selected_scene_index];
                let chapter_dir = project_dir.join("Scenes").join(&dashboard.story_chapter_name);
                let _ = std::fs::create_dir_all(&chapter_dir);
                let scene_file = chapter_dir.join(format!("{}.bscene", scene_name));
                let _ = scene.save_to_file(&scene_file);
            }
        }
    }

    fn update_dashboard(&mut self, message: DashboardMessage) -> Task<AppMessage> {
        let CurrentScreen::Workspace {
            project,
            project_dir,
            dashboard,
        } = &mut self.screen
        else {
            return Task::none();
        };

        match message {
            DashboardMessage::CloseProject => {
                if let Some(sink) = dashboard.audio_sink.take() {
                    sink.stop();
                }
                dashboard.playing_audio_asset_id = None;
                Self::auto_save_current_scene(dashboard, project_dir);
                self.screen = CurrentScreen::Welcome(WelcomeState::new());
            }
            DashboardMessage::SelectWorkflowTab(tab) => {
                if tab != WorkflowTab::Assets {
                    if let Some(sink) = dashboard.audio_sink.take() {
                        sink.stop();
                    }
                    dashboard.playing_audio_asset_id = None;
                }
                if dashboard.active_tab == WorkflowTab::Localization && tab != WorkflowTab::Localization {
                    let _ = Self::save_localization(dashboard, project_dir);
                }
                Self::auto_save_current_scene(dashboard, project_dir);
                dashboard.active_tab = tab;
                dashboard.status_message = None;
                dashboard.story_status_message = None;
                if tab == WorkflowTab::Localization {
                    dashboard.locale_items = blanc_core::localization::collect_project_items(project_dir);
                }
                if tab == WorkflowTab::SceneEditor || tab == WorkflowTab::StoryFlow {
                    dashboard.refresh_scenes(project_dir);
                    if let Some(scene_name) = dashboard.scenes_list.get(dashboard.selected_scene_index) {
                        let scene_file = project_dir
                            .join("Scenes")
                            .join(&dashboard.story_chapter_name)
                            .join(format!("{}.bscene", scene_name));
                        if let Ok(loaded) = Scene::load_from_file(&scene_file) {
                            dashboard.current_scene = Some(loaded);
                        }
                    }
                }
            }
            DashboardMessage::OpenProjectFolder => {
                #[cfg(target_os = "windows")]
                let _ = Command::new("explorer").arg(project_dir).spawn();
                #[cfg(target_os = "macos")]
                let _ = Command::new("open").arg(project_dir).spawn();
                #[cfg(target_os = "linux")]
                let _ = Command::new("xdg-open").arg(project_dir).spawn();
            }
            DashboardMessage::SelectAssetCategory(cat) => {
                dashboard.selected_category = cat;
                dashboard.status_message = None;
            }
            DashboardMessage::AssetSearchChanged(q) => {
                dashboard.asset_search_query = q;
            }
            DashboardMessage::ImportAssetClicked(category) => {
                let mut dialog = rfd::FileDialog::new();
                match category {
                    AssetCategory::Background => {
                        dialog = dialog.add_filter("Images (*.png, *.jpg, *.webp)", &["png", "jpg", "jpeg", "webp"]);
                    }
                    AssetCategory::Character | AssetCategory::CharacterExpression => {
                        dialog = dialog.add_filter("Character Sprites (*.png, *.webp)", &["png", "webp"]);
                    }
                    AssetCategory::Bgm => {
                        dialog = dialog.add_filter("BGM Audio (*.ogg, *.mp3, *.wav)", &["ogg", "mp3", "wav"]);
                    }
                    AssetCategory::Sfx => {
                        dialog = dialog.add_filter("SFX Audio (*.wav, *.ogg, *.mp3)", &["wav", "ogg", "mp3"]);
                    }
                    AssetCategory::Voice => {
                        dialog = dialog.add_filter("Voice Audio (*.ogg, *.wav, *.mp3)", &["ogg", "wav", "mp3"]);
                    }
                    AssetCategory::Font => {
                        dialog = dialog.add_filter("Fonts (*.ttf, *.otf)", &["ttf", "otf"]);
                    }
                    AssetCategory::Ui | AssetCategory::Other => {
                        dialog = dialog.add_filter("Supported Assets", &["png", "jpg", "jpeg", "webp", "ogg", "wav", "mp3", "ttf", "otf"]);
                    }
                }

                if let Some(picked_paths) = dialog.pick_files() {
                    let mut imported_count = 0usize;
                    let mut last_imported_name: Option<String> = None;
                    let mut last_imported_id: Option<String> = None;
                    let mut first_error: Option<String> = None;
                    let mut error_count = 0usize;

                    for picked_path in picked_paths {
                        match project.import_asset(project_dir, &picked_path, category) {
                            Ok(asset) => {
                                imported_count += 1;
                                last_imported_name = Some(asset.name);
                                last_imported_id = Some(asset.id);
                            }
                            Err(err) => {
                                error_count += 1;
                                if first_error.is_none() {
                                    first_error = Some(err.to_string());
                                }
                            }
                        }
                    }

                    if let Some(id) = last_imported_id {
                        dashboard.selected_preview_asset_id = Some(id);
                    }

                    dashboard.status_message = Some(match (imported_count, error_count) {
                        (1, 0) => format!(
                            "Imported '{}' to {}.",
                            last_imported_name.unwrap_or_default(),
                            category.display_name()
                        ),
                        (n, 0) if n > 1 => format!(
                            "Imported {} assets to {}.",
                            n,
                            category.display_name()
                        ),
                        (n, e) if n > 0 => format!(
                            "Imported {} asset(s) to {} ({} failed: {}).",
                            n,
                            category.display_name(),
                            e,
                            first_error.unwrap_or_default()
                        ),
                        (0, _) => format!(
                            "Failed to import asset(s): {}",
                            first_error.unwrap_or_else(|| "Unknown error".to_string())
                        ),
                        _ => "No assets selected.".to_string(),
                    });
                }
            }
            DashboardMessage::DeleteAssetClicked(asset_id) => {
                if dashboard.playing_audio_asset_id.as_deref() == Some(asset_id.as_str()) {
                    if let Some(sink) = dashboard.audio_sink.take() {
                        sink.stop();
                    }
                    dashboard.playing_audio_asset_id = None;
                }
                if dashboard.selected_preview_asset_id.as_deref() == Some(asset_id.as_str()) {
                    dashboard.selected_preview_asset_id = None;
                }
                match project.delete_asset(project_dir, &asset_id, true) {
                    Ok(_) => {
                        dashboard.status_message = Some("Asset deleted successfully.".to_string());
                    }
                    Err(err) => {
                        dashboard.status_message = Some(format!("Failed to delete asset: {}", err));
                    }
                }
            }
            DashboardMessage::SetAssetViewMode(mode) => {
                dashboard.asset_view_mode = mode;
            }
            DashboardMessage::SelectAssetForPreview(asset_id) => {
                dashboard.selected_preview_asset_id = Some(asset_id);
            }
            DashboardMessage::ToggleAssetPreviewBg => {
                dashboard.asset_preview_light_bg = !dashboard.asset_preview_light_bg;
            }
            DashboardMessage::PlayAssetAudioPreview(asset_id) => {
                if let Some(sink) = dashboard.audio_sink.take() {
                    sink.stop();
                }
                dashboard.playing_audio_asset_id = None;

                if let Some(asset) = project.assets.assets.get(&asset_id) {
                    let full_path = project_dir.join(&asset.relative_path);
                    if dashboard.audio_stream.is_none() {
                        dashboard.audio_stream = rodio::OutputStream::try_default().ok();
                    }
                    if let Some((_, handle)) = &dashboard.audio_stream {
                        match std::fs::File::open(&full_path) {
                            Ok(file) => match rodio::Decoder::new(std::io::BufReader::new(file)) {
                                Ok(source) => match rodio::Sink::try_new(handle) {
                                    Ok(sink) => {
                                        sink.append(source);
                                        sink.play();
                                        dashboard.audio_sink = Some(sink);
                                        dashboard.playing_audio_asset_id = Some(asset_id);
                                    }
                                    Err(err) => {
                                        dashboard.status_message = Some(format!("Failed to create audio sink: {}", err));
                                    }
                                },
                                Err(err) => {
                                    dashboard.status_message = Some(format!("Failed to decode audio: {}", err));
                                }
                            },
                            Err(err) => {
                                dashboard.status_message = Some(format!("Failed to open audio file: {}", err));
                            }
                        }
                    } else {
                        dashboard.status_message = Some("No audio output device available.".to_string());
                    }
                }
            }
            DashboardMessage::StopAssetAudioPreview => {
                if let Some(sink) = dashboard.audio_sink.take() {
                    sink.stop();
                }
                dashboard.playing_audio_asset_id = None;
            }
            DashboardMessage::OpenAssetFolder(category) => {
                let cat_folder = project_dir.join(category.subfolder());
                let _ = std::fs::create_dir_all(&cat_folder);
                #[cfg(target_os = "windows")]
                let _ = Command::new("explorer").arg(&cat_folder).spawn();
                #[cfg(target_os = "macos")]
                let _ = Command::new("open").arg(&cat_folder).spawn();
                #[cfg(target_os = "linux")]
                let _ = Command::new("xdg-open").arg(&cat_folder).spawn();
            }
            DashboardMessage::RevealAssetInExplorer(relative_path) => {
                let full_path = project_dir.join(relative_path);
                let folder = full_path.parent().unwrap_or(project_dir);
                #[cfg(target_os = "windows")]
                let _ = Command::new("explorer").arg(folder).spawn();
                #[cfg(target_os = "macos")]
                let _ = Command::new("open").arg(folder).spawn();
                #[cfg(target_os = "linux")]
                let _ = Command::new("xdg-open").arg(folder).spawn();
            }
            DashboardMessage::ScriptEditorAction(action) => {
                dashboard.script_content.perform(action);
                dashboard.parsed_story_elements = Story::parse_script(&dashboard.script_content.text());
            }
            DashboardMessage::LoadScriptTemplate => {
                dashboard.script_content = iced::widget::text_editor::Content::with_text(DEFAULT_SCRIPT_TEMPLATE);
                dashboard.parsed_story_elements = Story::parse_script(DEFAULT_SCRIPT_TEMPLATE);
                dashboard.story_status_message = Some("Loaded sample screenplay template.".to_string());
            }
            DashboardMessage::ImportScriptFileClicked => {
                if let Some(picked_path) = rfd::FileDialog::new()
                    .add_filter("Screenplay (*.txt, *.md, *.fountain)", &["txt", "md", "fountain"])
                    .pick_file()
                {
                    match std::fs::read_to_string(&picked_path) {
                        Ok(content) => {
                            dashboard.script_content = iced::widget::text_editor::Content::with_text(&content);
                            dashboard.parsed_story_elements = Story::parse_script(&content);
                            dashboard.story_status_message = Some(format!(
                                "Imported screenplay from '{}' ({} parsed elements).",
                                picked_path.file_name().unwrap_or_default().to_string_lossy(),
                                dashboard.parsed_story_elements.len()
                            ));
                        }
                        Err(err) => {
                            dashboard.story_status_message = Some(format!("Failed to read file: {}", err));
                        }
                    }
                }
            }
            DashboardMessage::SaveStoryChapterClicked => {
                let mut chapter = Chapter::new(&dashboard.story_chapter_name);
                chapter.raw_script = dashboard.script_content.text();
                chapter.elements = dashboard.parsed_story_elements.clone();

                let target_path = project_dir
                    .join("Story/Chapters")
                    .join(format!("{}.{}", dashboard.story_chapter_name, Chapter::EXTENSION));

                match chapter.save_to_file(&target_path) {
                    Ok(_) => {
                        dashboard.story_status_message = Some(format!(
                            "Saved chapter to Story/Chapters/{}.{} ({} elements).",
                            dashboard.story_chapter_name,
                            Chapter::EXTENSION,
                            chapter.elements.len()
                        ));
                    }
                    Err(err) => {
                        dashboard.story_status_message = Some(format!("Failed to save story file: {}", err));
                    }
                }
            }
            DashboardMessage::ConvertScriptToScenesClicked => {
                let elements = &dashboard.parsed_story_elements;
                if elements.is_empty() {
                    dashboard.story_status_message = Some("Script is empty. Please enter story dialogue first.".to_string());
                    return Task::none();
                }

                let target_scenes_dir = project_dir.join("Scenes").join(&dashboard.story_chapter_name);
                let _ = std::fs::create_dir_all(&target_scenes_dir);

                let mut scenes_count = 0;
                let mut current_scene_name = "01_opening".to_string();
                let mut current_scene = Scene::new(&current_scene_name, &dashboard.story_chapter_name);

                for elem in elements {
                    match elem {
                        StoryElement::Scene(s) => {
                            if !current_scene.blocks.is_empty() {
                                let filename = format!("{}.bscene", current_scene.name.replace(' ', "_"));
                                let scene_json = serde_json::to_string_pretty(&current_scene).unwrap_or_default();
                                let _ = std::fs::write(target_scenes_dir.join(filename), scene_json);
                                scenes_count += 1;
                            }
                            current_scene_name = s.scene_name.clone();
                            current_scene = Scene::new(&current_scene_name, &dashboard.story_chapter_name);
                            current_scene.add_block(ActionBlock::Background {
                                asset_id: s.scene_name.to_lowercase(),
                                transition: Some(blanc_core::scene::TransitionType::Fade),
                            });
                        }
                        StoryElement::Dialogue(d) => {
                            if let Some(exp) = &d.expression {
                                current_scene.add_block(ActionBlock::Character {
                                    character_id: d.speaker.clone(),
                                    sprite_asset_id: None,
                                    expression: exp.clone(),
                                    position: CharacterPosition::Center,
                                    offset_x: 0.0,
                                    offset_y: 0.0,
                                    transition: Some(blanc_core::scene::TransitionType::Fade),
                                });
                            }
                            current_scene.add_block(ActionBlock::Dialogue {
                                speaker: d.speaker.clone(),
                                text: d.text.clone(),
                                voice_asset_id: None,
                                nameplate_position: None,
                            });
                        }
                        StoryElement::Narration(n) => {
                            current_scene.add_block(ActionBlock::Narration {
                                text: n.text.clone(),
                            });
                        }
                        StoryElement::Audio(a) => {
                            if a.is_bgm {
                                current_scene.add_block(ActionBlock::PlayBgm {
                                    asset_id: a.track_name.clone(),
                                    volume: 1.0,
                                    looping: true,
                                });
                            } else {
                                current_scene.add_block(ActionBlock::PlaySfx {
                                    asset_id: a.track_name.clone(),
                                    volume: 1.0,
                                });
                            }
                        }
                        StoryElement::Choice(c) => {
                            current_scene.add_block(ActionBlock::Choice {
                                prompt: c.prompt.clone().unwrap_or_else(|| "Make a choice".to_string()),
                                position: blanc_core::scene::ChoiceLayoutPosition::Center,
                                options: c.options.iter().map(|opt| SceneChoiceOption {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    text: opt.text.clone(),
                                    target_scene_id: opt.target_scene.clone(),
                                }).collect(),
                            });
                        }
                    }
                }

                // Save final scene
                if !current_scene.blocks.is_empty() || scenes_count == 0 {
                    let filename = format!("{}.bscene", current_scene.name.replace(' ', "_"));
                    let scene_json = serde_json::to_string_pretty(&current_scene).unwrap_or_default();
                    let _ = std::fs::write(target_scenes_dir.join(filename), scene_json);
                    scenes_count += 1;
                }

                // Save the story file as well
                let mut chapter = Chapter::new(&dashboard.story_chapter_name);
                chapter.raw_script = dashboard.script_content.text();
                chapter.elements = dashboard.parsed_story_elements.clone();
                let story_path = project_dir
                    .join("Story/Chapters")
                    .join(format!("{}.{}", dashboard.story_chapter_name, Chapter::EXTENSION));
                let _ = chapter.save_to_file(&story_path);

                dashboard.refresh_scenes(project_dir);
                dashboard.selected_scene_index = 0;
                if let Some(first_scene) = dashboard.scenes_list.first() {
                    let scene_file = project_dir
                        .join("Scenes")
                        .join(&dashboard.story_chapter_name)
                        .join(format!("{}.bscene", first_scene));
                    if let Ok(loaded) = Scene::load_from_file(&scene_file) {
                        dashboard.current_scene = Some(loaded);
                    }
                }

                dashboard.active_tab = WorkflowTab::SceneEditor;
                dashboard.story_status_message = Some(format!(
                    "Successfully generated {} scene(s) into Scenes/{}/ and saved story chapter!",
                    scenes_count,
                    dashboard.story_chapter_name
                ));
                dashboard.scene_status_message = Some(format!(
                    "Converted screenplay to {} scene(s)! Showing first scene.",
                    scenes_count
                ));
                return Task::perform(
                    tokio::time::sleep(std::time::Duration::from_secs(3)),
                    |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                );
            }
            DashboardMessage::SelectScene(idx) => {
                Self::auto_save_current_scene(dashboard, project_dir);
                if idx < dashboard.scenes_list.len() {
                    dashboard.selected_scene_index = idx;
                    let scene_name = &dashboard.scenes_list[idx];
                    let scene_file = project_dir
                        .join("Scenes")
                        .join(&dashboard.story_chapter_name)
                        .join(format!("{}.bscene", scene_name));
                    if let Ok(loaded) = Scene::load_from_file(&scene_file) {
                        dashboard.current_scene = Some(loaded);
                    }
                }
            }
            DashboardMessage::CreateNewScene => {
                Self::auto_save_current_scene(dashboard, project_dir);
                let mut num = dashboard.scenes_list.len() + 1;
                let mut new_name = format!("scene_{:02}", num);
                while dashboard.scenes_list.iter().any(|s| s == &new_name) {
                    num += 1;
                    new_name = format!("scene_{:02}", num);
                }
                let new_scene = Scene::new(&new_name, &dashboard.story_chapter_name);
                let scene_file = project_dir
                    .join("Scenes")
                    .join(&dashboard.story_chapter_name)
                    .join(format!("{}.bscene", new_name));
                let _ = new_scene.save_to_file(&scene_file);
                dashboard.scenes_list.push(new_name);
                dashboard.selected_scene_index = dashboard.scenes_list.len() - 1;
                dashboard.current_scene = Some(new_scene);
                dashboard.scene_status_message = Some("Created new scene.".to_string());
                return Task::perform(
                    tokio::time::sleep(std::time::Duration::from_secs(3)),
                    |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                );
            }
            DashboardMessage::DeleteScene(idx) => {
                if dashboard.scenes_list.len() <= 1 {
                    dashboard.scene_status_message = Some("Cannot delete the only scene. A chapter must have at least one scene.".to_string());
                    return Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(3)),
                        |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                    );
                } else if idx < dashboard.scenes_list.len() {
                    let scene_to_delete = dashboard.scenes_list.remove(idx);
                    let scene_file = project_dir
                        .join("Scenes")
                        .join(&dashboard.story_chapter_name)
                        .join(format!("{}.bscene", scene_to_delete));
                    let _ = std::fs::remove_file(scene_file);

                    if dashboard.selected_scene_index >= dashboard.scenes_list.len() {
                        dashboard.selected_scene_index = dashboard.scenes_list.len() - 1;
                    } else if dashboard.selected_scene_index > idx {
                        dashboard.selected_scene_index -= 1;
                    }

                    let new_scene_name = &dashboard.scenes_list[dashboard.selected_scene_index];
                    let new_scene_file = project_dir
                        .join("Scenes")
                        .join(&dashboard.story_chapter_name)
                        .join(format!("{}.bscene", new_scene_name));
                    if let Ok(loaded) = Scene::load_from_file(&new_scene_file) {
                        dashboard.current_scene = Some(loaded);
                    }
                    dashboard.scene_status_message = Some(format!("Deleted scene '{}'.", scene_to_delete));
                    return Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(3)),
                        |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                    );
                }
            }
            DashboardMessage::SaveCurrentScene => {
                if let Some(scene) = &dashboard.current_scene {
                    if dashboard.selected_scene_index < dashboard.scenes_list.len() {
                        let scene_name = &dashboard.scenes_list[dashboard.selected_scene_index];
                        let scene_file = project_dir
                            .join("Scenes")
                            .join(&dashboard.story_chapter_name)
                            .join(format!("{}.bscene", scene_name));
                        match scene.save_to_file(&scene_file) {
                            Ok(_) => {
                                dashboard.scene_status_message = Some(format!("Saved scene '{}'.", scene_name));
                                return Task::perform(
                                    tokio::time::sleep(std::time::Duration::from_secs(3)),
                                    |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                                );
                            }
                            Err(err) => {
                                dashboard.scene_status_message = Some(format!("Failed to save scene: {}", err));
                            }
                        }
                    }
                }
            }
            DashboardMessage::OpenScenesFolder => {
                let folder = project_dir.join("Scenes").join(&dashboard.story_chapter_name);
                let _ = std::fs::create_dir_all(&folder);
                #[cfg(target_os = "windows")]
                let _ = Command::new("explorer").arg(folder).spawn();
                #[cfg(target_os = "macos")]
                let _ = Command::new("open").arg(folder).spawn();
                #[cfg(target_os = "linux")]
                let _ = Command::new("xdg-open").arg(folder).spawn();
            }
            DashboardMessage::AddBlockToScene(block) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    scene.add_block(block);
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::RemoveBlockFromScene(idx) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    scene.remove_block(idx);
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::MoveBlockUp(idx) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    scene.move_block_up(idx);
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::MoveBlockDown(idx) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    scene.move_block_down(idx);
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateBlockCharacterPosition(idx, pos) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Character { position, .. }) = scene.blocks.get_mut(idx) {
                        *position = pos;
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateBlockTransition(idx, trans) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    match scene.blocks.get_mut(idx) {
                        Some(ActionBlock::Background { transition, .. })
                        | Some(ActionBlock::Character { transition, .. })
                        | Some(ActionBlock::HideCharacter { transition, .. }) => {
                            *transition = trans;
                        }
                        _ => {}
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateVisualEffect(idx, new_effect, new_duration) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::VisualEffect { effect, duration }) = scene.blocks.get_mut(idx) {
                        *effect = new_effect;
                        *duration = new_duration;
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateBlockChoicePosition(idx, pos) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Choice { position, .. }) = scene.blocks.get_mut(idx) {
                        *position = pos;
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateSpeakerName(idx, spk) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Dialogue { speaker, .. }) = scene.blocks.get_mut(idx) {
                        *speaker = spk;
                    }
                }
            }
            DashboardMessage::UpdateDialogueText(idx, txt) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    match scene.blocks.get_mut(idx) {
                        Some(ActionBlock::Dialogue { text, .. }) => *text = txt,
                        Some(ActionBlock::Narration { text, .. }) => *text = txt,
                        _ => {}
                    }
                }
            }
            DashboardMessage::UpdateDialogueNameplatePosition(idx, pos) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Dialogue { nameplate_position, .. }) = scene.blocks.get_mut(idx) {
                        *nameplate_position = pos;
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::ClearBlockVoice(idx) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Dialogue { voice_asset_id, .. }) = scene.blocks.get_mut(idx) {
                        *voice_asset_id = None;
                    }
                }
                Self::auto_save_current_scene(dashboard, project_dir);
            }
            DashboardMessage::UpdateChoicePrompt(idx, pmt) => {
                if let Some(scene) = &mut dashboard.current_scene {
                    if let Some(ActionBlock::Choice { prompt, .. }) = scene.blocks.get_mut(idx) {
                        *prompt = pmt;
                    }
                }
            }
            DashboardMessage::OpenAssetPickerModal { target, category } => {
                let initial_selected = match &target {
                    AssetPickerTarget::Block(block_index) => {
                        if let Some(scene) = &dashboard.current_scene {
                            match scene.blocks.get(*block_index) {
                                Some(ActionBlock::Background { asset_id, .. }) => Some(asset_id.clone()),
                                Some(ActionBlock::Character { character_id, sprite_asset_id, .. }) => {
                                    sprite_asset_id.clone().or_else(|| Some(character_id.clone()))
                                }
                                Some(ActionBlock::PlayBgm { asset_id, .. }) => Some(asset_id.clone()),
                                Some(ActionBlock::PlaySfx { asset_id, .. }) => Some(asset_id.clone()),
                                Some(ActionBlock::Dialogue { voice_asset_id, .. }) => voice_asset_id.clone(),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    AssetPickerTarget::TitleBackground => {
                        dashboard.system_screens_config.title_background_asset_id.clone()
                    }
                    AssetPickerTarget::TitleBgm => {
                        dashboard.system_screens_config.title_bgm_asset_id.clone()
                    }
                    AssetPickerTarget::UiAsset(slot) => {
                        let ui = &dashboard.system_screens_config.ui_config;
                        match slot {
                            UiAssetSlot::Font => ui.theme.font_asset_id.clone(),
                            UiAssetSlot::ButtonNormal => ui.theme.button_normal_asset_id.clone(),
                            UiAssetSlot::ButtonFocus => ui.theme.button_focus_asset_id.clone(),
                            UiAssetSlot::ButtonPressed => ui.theme.button_pressed_asset_id.clone(),
                            UiAssetSlot::ButtonDisabled => ui.theme.button_disabled_asset_id.clone(),
                            UiAssetSlot::SmallButtonNormal => ui.theme.small_button_normal_asset_id.clone(),
                            UiAssetSlot::SmallButtonFocus => ui.theme.small_button_focus_asset_id.clone(),
                            UiAssetSlot::SmallButtonPressed => ui.theme.small_button_pressed_asset_id.clone(),
                            UiAssetSlot::SmallButtonDisabled => ui.theme.small_button_disabled_asset_id.clone(),
                            UiAssetSlot::SliderTrack => ui.theme.slider_track_asset_id.clone(),
                            UiAssetSlot::SliderFill => ui.theme.slider_fill_asset_id.clone(),
                            UiAssetSlot::SliderHandle => ui.theme.slider_handle_asset_id.clone(),
                            UiAssetSlot::MainMenuLogo => dashboard.system_screens_config.logo_asset_id.clone(),
                            UiAssetSlot::MainMenuPanel => ui.main_menu.panel_asset_id.clone(),
                            UiAssetSlot::DialogueBox => ui.gameplay.dialogue_box_asset_id.clone(),
                            UiAssetSlot::Nameplate => ui.gameplay.nameplate_asset_id.clone(),
                            UiAssetSlot::ChoiceButton => ui.gameplay.choice_button_asset_id.clone(),
                            UiAssetSlot::ChoiceButtonHover => ui.gameplay.choice_button_hover_asset_id.clone(),
                            UiAssetSlot::ChoiceButtonDisabled => ui.gameplay.choice_button_disabled_asset_id.clone(),
                            UiAssetSlot::AdvancePrompt => ui.gameplay.advance_prompt_asset_id.clone(),
                            UiAssetSlot::AdvancePromptFrame2 => ui.gameplay.advance_prompt_frame2_asset_id.clone(),
                            UiAssetSlot::AdvancePromptFrame3 => ui.gameplay.advance_prompt_frame3_asset_id.clone(),
                            UiAssetSlot::PausePanel => ui.pause_settings.pause_panel_asset_id.clone(),
                            UiAssetSlot::SettingsPanel => ui.pause_settings.settings_panel_asset_id.clone(),
                            UiAssetSlot::SaveLoadPanel => ui.save_load.panel_asset_id.clone(),
                            UiAssetSlot::SaveLoadSlotCard => ui.save_load.slot_card_asset_id.clone(),
                            UiAssetSlot::SaveLoadSlotCardHover => ui.save_load.slot_card_hover_asset_id.clone(),
                            UiAssetSlot::SaveLoadSlotCardDisabled => ui.save_load.slot_card_disabled_asset_id.clone(),
                            UiAssetSlot::HistoryPanel => ui.history.panel_asset_id.clone(),
                            UiAssetSlot::HistoryEntryCard => ui.history.entry_card_asset_id.clone(),
                            UiAssetSlot::EndingPanel => ui.ending.panel_asset_id.clone(),
                            UiAssetSlot::EndingBackground => ui.ending.background_asset_id.clone(),
                        }
                    }
                };
                dashboard.asset_picker_modal = Some(AssetPickerModalState {
                    target,
                    category,
                    search_query: String::new(),
                    selected_asset_id: initial_selected,
                });
            }
            DashboardMessage::SceneEditorBlocksScrolled(offset) => {
                dashboard.scene_blocks_scroll_offset = offset;
            }
            DashboardMessage::LocaleSyncFromProject => {
                dashboard.locale_items = blanc_core::localization::collect_project_items(project_dir);
                dashboard.locale_status_message = Some(format!(
                    "Synced {} localizable string(s) from project.",
                    dashboard.locale_items.len()
                ));
            }
            DashboardMessage::LocaleSelectLanguage(code) => {
                dashboard.locale_selected_code = code;
                dashboard.locale_status_message = None;
            }
            DashboardMessage::LocaleNewCodeChanged(v) => {
                dashboard.locale_new_code = v;
            }
            DashboardMessage::LocaleNewNameChanged(v) => {
                dashboard.locale_new_name = v;
            }
            DashboardMessage::LocaleDefaultCodeChanged(v) => {
                dashboard.locale_default_code = v;
            }
            DashboardMessage::LocaleDefaultNameChanged(v) => {
                dashboard.locale_default_name = v;
            }
            DashboardMessage::LocaleSaveDefaultLanguage => {
                let code = dashboard.locale_default_code.trim().to_string();
                let name = dashboard.locale_default_name.trim().to_string();
                if code.is_empty() {
                    dashboard.locale_status_message = Some("Default language code cannot be empty.".to_string());
                } else {
                    let old_code = dashboard.locale_manifest.default_language.code.to_ascii_lowercase();
                    let new_code_key = code.to_ascii_lowercase();
                    if old_code != new_code_key {
                        if let Some(tbl) = dashboard.locale_tables.remove(&old_code) {
                            dashboard.locale_tables.insert(new_code_key.clone(), tbl);
                        }
                        if dashboard.locale_selected_code.eq_ignore_ascii_case(&old_code) {
                            dashboard.locale_selected_code = code.clone();
                        }
                    }
                    dashboard.locale_manifest.default_language =
                        blanc_core::localization::LanguageEntry::new(code.clone(), if name.is_empty() { code.clone() } else { name });
                    dashboard.locale_show_default_editor = false;
                    let _ = Self::save_localization(dashboard, project_dir);
                    dashboard.locale_status_message = Some(format!("Updated default language to '{}'.", code));
                }
            }
            DashboardMessage::LocaleAddLanguage => {
                let code = dashboard.locale_new_code.trim().to_string();
                let name = dashboard.locale_new_name.trim().to_string();
                if code.is_empty() {
                    dashboard.locale_status_message = Some("Please enter a language code (e.g. id, ja, fr).".to_string());
                } else if dashboard.locale_manifest.find_language_by_code(&code).is_some() {
                    dashboard.locale_status_message = Some(format!("Language '{}' already exists.", code));
                } else {
                    let entry = blanc_core::localization::LanguageEntry::new(
                        code.clone(),
                        if name.is_empty() { code.clone() } else { name },
                    );
                    dashboard.locale_manifest.languages.push(entry);
                    dashboard
                        .locale_tables
                        .entry(code.to_ascii_lowercase())
                        .or_default();
                    dashboard.locale_selected_code = code.clone();
                    dashboard.locale_new_code.clear();
                    dashboard.locale_new_name.clear();
                    dashboard.locale_show_add_form = false;
                    let _ = Self::save_localization(dashboard, project_dir);
                    dashboard.locale_status_message = Some(format!("Added language '{}'.", code));
                }
            }
            DashboardMessage::LocaleToggleDefaultEditor => {
                dashboard.locale_show_default_editor = !dashboard.locale_show_default_editor;
                if dashboard.locale_show_default_editor {
                    dashboard.locale_show_add_form = false;
                }
            }
            DashboardMessage::LocaleToggleAddForm => {
                dashboard.locale_show_add_form = !dashboard.locale_show_add_form;
                if dashboard.locale_show_add_form {
                    dashboard.locale_show_default_editor = false;
                }
            }
            DashboardMessage::LocaleFillPresetLanguage(code, name) => {
                dashboard.locale_new_code = code;
                dashboard.locale_new_name = name;
            }
            DashboardMessage::LocaleRemoveLanguage(code) => {
                dashboard
                    .locale_manifest
                    .languages
                    .retain(|l| !l.code.eq_ignore_ascii_case(&code));
                dashboard.locale_tables.remove(&code.to_ascii_lowercase());
                let file_path = blanc_core::localization::LocaleStrings::file_path(project_dir, &code);
                let _ = std::fs::remove_file(file_path);
                if dashboard.locale_selected_code.eq_ignore_ascii_case(&code) {
                    dashboard.locale_selected_code = dashboard
                        .locale_manifest
                        .languages
                        .first()
                        .map(|l| l.code.clone())
                        .unwrap_or_else(|| dashboard.locale_manifest.default_language.code.clone());
                }
                let _ = Self::save_localization(dashboard, project_dir);
                dashboard.locale_status_message = Some(format!("Removed language '{}'.", code));
            }
            DashboardMessage::LocaleSearchChanged(q) => {
                dashboard.locale_search_query = q;
            }
            DashboardMessage::LocaleFilterScopeChanged(scope) => {
                dashboard.locale_filter_scope = scope;
            }
            DashboardMessage::LocaleToggleUntranslatedOnly(val) => {
                dashboard.locale_untranslated_only = val;
            }
            DashboardMessage::LocaleTranslationChanged(key, val) => {
                let lang_key = dashboard.locale_selected_code.to_ascii_lowercase();
                let tbl = dashboard.locale_tables.entry(lang_key).or_default();
                if val.is_empty() {
                    tbl.entries.remove(&key);
                } else {
                    tbl.entries.insert(key, val);
                }
            }
            DashboardMessage::LocaleSaveAll => {
                match Self::save_localization(dashboard, project_dir) {
                    Ok(()) => {
                        dashboard.locale_status_message = Some("Saved localization manifest and all language files.".to_string());
                    }
                    Err(err) => {
                        dashboard.locale_status_message = Some(format!("Failed to save locales: {}", err));
                    }
                }
            }
            DashboardMessage::LocaleExportCsv => {
                let lang_code = dashboard.locale_selected_code.clone();
                let default_name = format!(
                    "localization_{}.csv",
                    blanc_core::localization::sanitize_lang_code(&lang_code)
                );
                if let Some(save_path) = rfd::FileDialog::new()
                    .set_file_name(&default_name)
                    .add_filter("CSV (*.csv)", &["csv"])
                    .save_file()
                {
                    let empty_tbl = blanc_core::localization::LocaleStrings::default();
                    let tbl = dashboard
                        .locale_tables
                        .get(&lang_code.to_ascii_lowercase())
                        .unwrap_or(&empty_tbl);
                    let csv = blanc_core::localization::export_csv(&dashboard.locale_items, tbl);
                    match std::fs::write(&save_path, csv) {
                        Ok(()) => {
                            dashboard.locale_status_message = Some(format!(
                                "Exported {} string(s) for '{}' to {}.",
                                dashboard.locale_items.len(),
                                lang_code,
                                save_path.display()
                            ));
                        }
                        Err(e) => {
                            dashboard.locale_status_message = Some(format!("Export CSV failed: {}", e));
                        }
                    }
                }
            }
            DashboardMessage::LocaleImportCsv => {
                if let Some(open_path) = rfd::FileDialog::new()
                    .add_filter("CSV (*.csv)", &["csv"])
                    .pick_file()
                {
                    match std::fs::read_to_string(&open_path) {
                        Ok(raw) => {
                            let lang_code = dashboard.locale_selected_code.clone();
                            let tbl = dashboard
                                .locale_tables
                                .entry(lang_code.to_ascii_lowercase())
                                .or_default();
                            let count = blanc_core::localization::import_csv(&raw, tbl);
                            let _ = Self::save_localization(dashboard, project_dir);
                            dashboard.locale_status_message = Some(format!(
                                "Imported {} translation(s) into '{}'.",
                                count, lang_code
                            ));
                        }
                        Err(e) => {
                            dashboard.locale_status_message = Some(format!("Import CSV failed: {}", e));
                        }
                    }
                }
            }
            DashboardMessage::CloseAssetPickerModal => {
                dashboard.asset_picker_modal = None;
                if dashboard.active_tab == WorkflowTab::SceneEditor {
                    return iced::widget::scrollable::scroll_to(
                        iced::widget::scrollable::Id::new("scene_editor_blocks_scroll"),
                        dashboard.scene_blocks_scroll_offset,
                    );
                }
            }
            DashboardMessage::AssetPickerSearchChanged(query) => {
                if let Some(modal) = &mut dashboard.asset_picker_modal {
                    modal.search_query = query;
                }
            }
            DashboardMessage::SelectAssetInModal(asset_id) => {
                if let Some(modal) = &mut dashboard.asset_picker_modal {
                    modal.selected_asset_id = Some(asset_id);
                }
            }
            DashboardMessage::ConfirmAssetPickerSelection => {
                if let Some(modal) = &dashboard.asset_picker_modal {
                    if let Some(sel_id) = &modal.selected_asset_id {
                        let asset_id_val = sel_id.clone();
                        match modal.target {
                            AssetPickerTarget::Block(b_idx) => {
                                if let Some(scene) = &mut dashboard.current_scene {
                                    match scene.blocks.get_mut(b_idx) {
                                        Some(ActionBlock::Background { asset_id, .. }) => {
                                            *asset_id = asset_id_val;
                                        }
                                        Some(ActionBlock::Character { sprite_asset_id, .. }) => {
                                            *sprite_asset_id = Some(asset_id_val);
                                        }
                                        Some(ActionBlock::PlayBgm { asset_id, .. }) => {
                                            *asset_id = asset_id_val;
                                        }
                                        Some(ActionBlock::PlaySfx { asset_id, .. }) => {
                                            *asset_id = asset_id_val;
                                        }
                                        Some(ActionBlock::Dialogue { voice_asset_id, .. }) => {
                                            *voice_asset_id = Some(asset_id_val);
                                        }
                                        _ => {}
                                    }
                                }
                                Self::auto_save_current_scene(dashboard, project_dir);
                            }
                            AssetPickerTarget::TitleBackground => {
                                dashboard.system_screens_config.title_background_asset_id = Some(asset_id_val);
                            }
                            AssetPickerTarget::TitleBgm => {
                                dashboard.system_screens_config.title_bgm_asset_id = Some(asset_id_val);
                            }
                            AssetPickerTarget::UiAsset(slot) => {
                                let mut font_load_bytes: Option<Vec<u8>> = None;
                                if matches!(slot, UiAssetSlot::Font) {
                                    if let Some(asset) = project.assets.get(&asset_id_val) {
                                        let full_path = project_dir.join(&asset.relative_path);
                                        if let Ok(bytes) = std::fs::read(&full_path) {
                                            if let Some(family) = blanc_core::ui_config::read_ttf_family_name(&bytes) {
                                                dashboard.system_screens_config.ui_config.theme.font_family = family;
                                            }
                                            font_load_bytes = Some(bytes);
                                        }
                                    }
                                }
                                let slider_img_height = |asset_id: &str| -> Option<f32> {
                                    project.assets.get(asset_id).and_then(|asset| {
                                        let full_path = project_dir.join(&asset.relative_path);
                                        image::image_dimensions(&full_path)
                                            .ok()
                                            .map(|(_w, h)| h as f32)
                                    })
                                };
                                let track_auto_h = if matches!(slot, UiAssetSlot::SliderTrack) {
                                    slider_img_height(&asset_id_val)
                                } else {
                                    None
                                };
                                let handle_auto_h = if matches!(slot, UiAssetSlot::SliderHandle) {
                                    slider_img_height(&asset_id_val)
                                } else {
                                    None
                                };
                                let ui = &mut dashboard.system_screens_config.ui_config;
                                match slot {
                                    UiAssetSlot::Font => ui.theme.font_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ButtonNormal => ui.theme.button_normal_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ButtonFocus => ui.theme.button_focus_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ButtonPressed => ui.theme.button_pressed_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ButtonDisabled => ui.theme.button_disabled_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SmallButtonNormal => ui.theme.small_button_normal_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SmallButtonFocus => ui.theme.small_button_focus_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SmallButtonPressed => ui.theme.small_button_pressed_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SmallButtonDisabled => ui.theme.small_button_disabled_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SliderTrack => {
                                        ui.theme.slider_track_asset_id = Some(asset_id_val);
                                        ui.theme.slider_track_height = track_auto_h.unwrap_or(28.0).clamp(2.0, 128.0);
                                    }
                                    UiAssetSlot::SliderFill => ui.theme.slider_fill_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SliderHandle => {
                                        ui.theme.slider_handle_asset_id = Some(asset_id_val);
                                        ui.theme.slider_handle_size = handle_auto_h.unwrap_or(34.0).clamp(8.0, 128.0);
                                    }
                                    UiAssetSlot::MainMenuLogo => {
                                        dashboard.system_screens_config.logo_asset_id = Some(asset_id_val);
                                    }
                                    UiAssetSlot::MainMenuPanel => ui.main_menu.panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::DialogueBox => ui.gameplay.dialogue_box_asset_id = Some(asset_id_val),
                                    UiAssetSlot::Nameplate => ui.gameplay.nameplate_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ChoiceButton => ui.gameplay.choice_button_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ChoiceButtonHover => ui.gameplay.choice_button_hover_asset_id = Some(asset_id_val),
                                    UiAssetSlot::ChoiceButtonDisabled => ui.gameplay.choice_button_disabled_asset_id = Some(asset_id_val),
                                    UiAssetSlot::AdvancePrompt => ui.gameplay.advance_prompt_asset_id = Some(asset_id_val),
                                    UiAssetSlot::AdvancePromptFrame2 => ui.gameplay.advance_prompt_frame2_asset_id = Some(asset_id_val),
                                    UiAssetSlot::AdvancePromptFrame3 => ui.gameplay.advance_prompt_frame3_asset_id = Some(asset_id_val),
                                    UiAssetSlot::PausePanel => ui.pause_settings.pause_panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SettingsPanel => ui.pause_settings.settings_panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SaveLoadPanel => ui.save_load.panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SaveLoadSlotCard => ui.save_load.slot_card_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SaveLoadSlotCardHover => ui.save_load.slot_card_hover_asset_id = Some(asset_id_val),
                                    UiAssetSlot::SaveLoadSlotCardDisabled => ui.save_load.slot_card_disabled_asset_id = Some(asset_id_val),
                                    UiAssetSlot::HistoryPanel => ui.history.panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::HistoryEntryCard => ui.history.entry_card_asset_id = Some(asset_id_val),
                                    UiAssetSlot::EndingPanel => ui.ending.panel_asset_id = Some(asset_id_val),
                                    UiAssetSlot::EndingBackground => ui.ending.background_asset_id = Some(asset_id_val),
                                }
                                if let Some(bytes) = font_load_bytes {
                                    dashboard.asset_picker_modal = None;
                                    return iced::font::load(bytes).discard();
                                }
                            }
                        }
                    }
                }
                dashboard.asset_picker_modal = None;
                if dashboard.active_tab == WorkflowTab::SceneEditor {
                    return iced::widget::scrollable::scroll_to(
                        iced::widget::scrollable::Id::new("scene_editor_blocks_scroll"),
                        dashboard.scene_blocks_scroll_offset,
                    );
                }
            }
            DashboardMessage::ImportAssetFromModal(category) => {
                let mut dialog = rfd::FileDialog::new();
                match category {
                    AssetCategory::Background => {
                        dialog = dialog.add_filter("Images (*.png, *.jpg, *.webp)", &["png", "jpg", "jpeg", "webp"]);
                    }
                    AssetCategory::Character | AssetCategory::CharacterExpression => {
                        dialog = dialog.add_filter("Character Sprites (*.png, *.webp)", &["png", "webp"]);
                    }
                    AssetCategory::Bgm => {
                        dialog = dialog.add_filter("BGM Audio (*.ogg, *.mp3, *.wav)", &["ogg", "mp3", "wav"]);
                    }
                    AssetCategory::Sfx => {
                        dialog = dialog.add_filter("SFX Audio (*.wav, *.ogg, *.mp3)", &["wav", "ogg", "mp3"]);
                    }
                    AssetCategory::Voice => {
                        dialog = dialog.add_filter("Voice Audio (*.ogg, *.wav, *.mp3)", &["ogg", "wav", "mp3"]);
                    }
                    AssetCategory::Font => {
                        dialog = dialog.add_filter("Fonts (*.ttf, *.otf)", &["ttf", "otf"]);
                    }
                    AssetCategory::Ui | AssetCategory::Other => {
                        dialog = dialog.add_filter("Supported Assets", &["png", "jpg", "jpeg", "webp", "ogg", "wav", "mp3", "ttf", "otf"]);
                    }
                }

                if let Some(picked_path) = dialog.pick_file() {
                    match project.import_asset(project_dir, &picked_path, category) {
                        Ok(asset) => {
                            if let Some(modal) = &mut dashboard.asset_picker_modal {
                                modal.selected_asset_id = Some(asset.id.clone());
                            }
                            dashboard.scene_status_message = Some(format!(
                                "Imported '{}' to {}.",
                                asset.name,
                                asset.category.display_name()
                            ));
                            return Task::perform(
                                tokio::time::sleep(std::time::Duration::from_secs(3)),
                                |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                            );
                        }
                        Err(err) => {
                            dashboard.scene_status_message = Some(format!("Failed to import asset: {}", err));
                        }
                    }
                }
            }
            DashboardMessage::DismissSceneStatusMessage => {
                dashboard.scene_status_message = None;
            }
            DashboardMessage::DismissSettingsStatusMessage => {
                dashboard.settings_status_message = None;
            }
            DashboardMessage::UpdateTitleText(text) => {
                dashboard.system_screens_config.title_text = text;
            }
            DashboardMessage::UpdateStartingScene(scene) => {
                dashboard.system_screens_config.starting_scene_id = if scene.is_empty() { None } else { Some(scene) };
            }
            DashboardMessage::UpdateAspectScalingMode(mode) => {
                dashboard.system_screens_config.aspect_scaling_mode = mode;
            }
            DashboardMessage::UpdateFullscreenDefault(fs) => {
                dashboard.system_screens_config.fullscreen_default = fs;
            }
            DashboardMessage::UpdateDefaultResolution(w, h) => {
                dashboard.system_screens_config.default_resolution_width = w;
                dashboard.system_screens_config.default_resolution_height = h;
            }
            DashboardMessage::UpdateSceneTransitionMode(mode) => {
                dashboard.system_screens_config.scene_transition_mode = mode;
            }
            DashboardMessage::UpdateMasterVolume(v) => {
                dashboard.system_screens_config.master_volume = v;
            }
            DashboardMessage::UpdateBgmVolume(v) => {
                dashboard.system_screens_config.bgm_volume = v;
            }
            DashboardMessage::UpdateSfxVolume(v) => {
                dashboard.system_screens_config.sfx_volume = v;
            }
            DashboardMessage::UpdateVoiceVolume(v) => {
                dashboard.system_screens_config.voice_volume = v;
            }
            DashboardMessage::UpdateTextSpeed(s) => {
                dashboard.system_screens_config.text_speed = s;
            }
            DashboardMessage::UpdateAutoForwardDelay(d) => {
                dashboard.system_screens_config.auto_forward_delay = d;
            }
            DashboardMessage::SaveSystemSettings => {
                let config_dir = project_dir.join("Settings");
                let _ = std::fs::create_dir_all(&config_dir);
                let config_path = config_dir.join("system_screens.bconfig");
                match dashboard.system_screens_config.save_to_file(&config_path) {
                    Ok(_) => {
                        dashboard.settings_status_message = Some("System screen settings saved successfully.".to_string());
                        return Task::perform(
                            tokio::time::sleep(std::time::Duration::from_secs(3)),
                            |_| AppMessage::Dashboard(DashboardMessage::DismissSettingsStatusMessage),
                        );
                    }
                    Err(err) => {
                        dashboard.settings_status_message = Some(format!("Failed to save settings: {}", err));
                    }
                }
            }
            DashboardMessage::ImportUiPackFolder => {
                return Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select UI Pack PNG Folder")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    |path| AppMessage::Dashboard(DashboardMessage::UiPackFolderSelected(path)),
                );
            }
            DashboardMessage::UiPackFolderSelected(Some(folder)) => {
                let mut matched_count = 0usize;
                if let Ok(entries) = std::fs::read_dir(&folder) {
                    let mut pngs: Vec<PathBuf> = entries
                        .flatten()
                        .map(|e| e.path())
                        .filter(|p| {
                            p.is_file()
                                && p.extension()
                                    .and_then(|s| s.to_str())
                                    .map(|ext| ext.eq_ignore_ascii_case("png"))
                                    .unwrap_or(false)
                        })
                        .collect();
                    pngs.sort();

                    let ui = &mut dashboard.system_screens_config.ui_config;
                    let mut import_and_get_id = |path: &PathBuf| -> Option<String> {
                        if let Ok(asset) = project.import_asset(project_dir, path, AssetCategory::Ui) {
                            matched_count += 1;
                            Some(asset.id.clone())
                        } else {
                            None
                        }
                    };

                    for path in &pngs {
                        let stem = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        if stem.contains("small_button_idle") || stem.contains("small_button_normal") {
                            ui.theme.small_button_normal_asset_id = import_and_get_id(path);
                        } else if stem.contains("small_button_hover") && !stem.contains("selected") {
                            ui.theme.small_button_focus_asset_id = import_and_get_id(path);
                        } else if stem.contains("small_button_pressed") {
                            ui.theme.small_button_pressed_asset_id = import_and_get_id(path);
                        } else if stem.contains("small_button_disabled") {
                            ui.theme.small_button_disabled_asset_id = import_and_get_id(path);
                        } else if stem.contains("button_idle") || stem.contains("button_normal") {
                            ui.theme.button_normal_asset_id = import_and_get_id(path);
                        } else if (stem.contains("button_hover") || stem.contains("button_focus")) && !stem.contains("selected") {
                            ui.theme.button_focus_asset_id = import_and_get_id(path);
                        } else if stem.contains("button_pressed") {
                            ui.theme.button_pressed_asset_id = import_and_get_id(path);
                        } else if stem.contains("button_disabled") {
                            ui.theme.button_disabled_asset_id = import_and_get_id(path);
                        } else if stem.contains("dialogue_wide") || (stem.contains("dialogue_box") && ui.gameplay.dialogue_box_asset_id.is_none()) {
                            ui.gameplay.dialogue_box_asset_id = import_and_get_id(path);
                        } else if stem.contains("nameplate") {
                            ui.gameplay.nameplate_asset_id = import_and_get_id(path);
                        } else if stem.contains("choice_idle") || stem.contains("choice_normal") {
                            ui.gameplay.choice_button_asset_id = import_and_get_id(path);
                        } else if stem.contains("choice_hover") && !stem.contains("selected") {
                            ui.gameplay.choice_button_hover_asset_id = import_and_get_id(path);
                        } else if stem.contains("choice_disabled") {
                            ui.gameplay.choice_button_disabled_asset_id = import_and_get_id(path);
                        } else if stem.contains("continue_0") {
                            ui.gameplay.advance_prompt_asset_id = import_and_get_id(path);
                        } else if stem.contains("continue_1") {
                            ui.gameplay.advance_prompt_frame2_asset_id = import_and_get_id(path);
                        } else if stem.contains("continue_2") {
                            ui.gameplay.advance_prompt_frame3_asset_id = import_and_get_id(path);
                        } else if stem.contains("menu_panel") {
                            if let Some(id) = import_and_get_id(path) {
                                ui.main_menu.panel_asset_id = Some(id.clone());
                                ui.save_load.panel_asset_id = Some(id);
                            }
                        } else if stem.contains("modal") {
                            if let Some(id) = import_and_get_id(path) {
                                ui.pause_settings.pause_panel_asset_id = Some(id.clone());
                                ui.pause_settings.settings_panel_asset_id = Some(id);
                            }
                        } else if stem.contains("save_slot_idle") {
                            ui.save_load.slot_card_asset_id = import_and_get_id(path);
                        } else if stem.contains("save_slot_hover") && !stem.contains("selected") {
                            ui.save_load.slot_card_hover_asset_id = import_and_get_id(path);
                        } else if stem.contains("save_slot_disabled") {
                            ui.save_load.slot_card_disabled_asset_id = import_and_get_id(path);
                        }
                    }

                    // Auto-adjust text contrast if folder name hints a light theme (e.g. "cream")
                    let folder_name = folder.file_name().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    if folder_name.contains("cream") || folder_name.contains("light") {
                        ui.theme.text_hex = "#3b2f2f".to_string();
                        ui.theme.button_text_hex = "#3b2f2f".to_string();
                        ui.theme.button_focus_text_hex = "#1e1b18".to_string();
                        ui.gameplay.dialogue_text_hex = "#3b2f2f".to_string();
                        ui.save_load.slot_text_hex = "#3b2f2f".to_string();
                    }

                    let manifest_filename = format!("{}.{}", project.name.replace(' ', "_"), Project::MANIFEST_EXTENSION);
                    let _ = project.save_manifest(&project_dir.join(manifest_filename));
                }
                dashboard.settings_status_message = Some(format!("Imported & mapped {} UI pack PNGs.", matched_count));
                return Task::perform(
                    tokio::time::sleep(std::time::Duration::from_secs(4)),
                    |_| AppMessage::Dashboard(DashboardMessage::DismissSettingsStatusMessage),
                );
            }
            DashboardMessage::UiPackFolderSelected(None) => {}
            DashboardMessage::SetExportExeName(name) => {
                dashboard.export_exe_name = name;
            }
            DashboardMessage::SetExportOutputDir(dir) => {
                dashboard.export_output_dir = dir;
            }
            DashboardMessage::BrowseExportOutputDir => {
                return Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select Export Output Folder")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    |path| AppMessage::Dashboard(DashboardMessage::ExportOutputDirSelected(path)),
                );
            }
            DashboardMessage::ExportOutputDirSelected(Some(folder)) => {
                dashboard.export_output_dir = folder.to_string_lossy().to_string();
            }
            DashboardMessage::ExportOutputDirSelected(None) => {}
            DashboardMessage::SetExportPacked(packed) => {
                dashboard.export_packed = packed;
            }
            DashboardMessage::SetExportSaveMode(mode) => {
                dashboard.export_save_mode = mode;
            }
            DashboardMessage::SetExportBuildKind(kind) => {
                dashboard.export_build_kind = kind;
                if kind == blanc_core::installer::ExportBuildKind::Installer {
                    dashboard.export_save_mode = blanc_core::pack::ExportSaveMode::AppData;
                }
            }
            DashboardMessage::SetInstallerVersion(v) => {
                dashboard.installer_version = v;
            }
            DashboardMessage::SetInstallerPublisher(v) => {
                dashboard.installer_publisher = v;
            }
            DashboardMessage::SetInstallerWelcome(v) => {
                dashboard.installer_welcome_text = v;
            }
            DashboardMessage::SelectInstallerPolicyLang(code) => {
                let prev_code = dashboard.installer_policy_lang.clone();
                let prev_text = dashboard.installer_policy_editor.text();
                dashboard.installer_policy_texts.insert(prev_code, prev_text);
                dashboard.installer_policy_lang = code.clone();
                let next_text = dashboard
                    .installer_policy_texts
                    .get(&code)
                    .cloned()
                    .unwrap_or_default();
                dashboard.installer_policy_editor = iced::widget::text_editor::Content::with_text(&next_text);
            }
            DashboardMessage::InstallerPolicyEditorAction(action) => {
                dashboard.installer_policy_editor.perform(action);
                let code = dashboard.installer_policy_lang.clone();
                let txt = dashboard.installer_policy_editor.text();
                dashboard.installer_policy_texts.insert(code, txt);
            }
            DashboardMessage::ResetInstallerPolicyDefault => {
                let defaults = blanc_core::installer::default_privacy_and_terms_map();
                for (k, v) in defaults {
                    dashboard.installer_policy_texts.insert(k, v);
                }
                let cur_txt = dashboard
                    .installer_policy_texts
                    .get(&dashboard.installer_policy_lang)
                    .cloned()
                    .unwrap_or_default();
                dashboard.installer_policy_editor = iced::widget::text_editor::Content::with_text(&cur_txt);
            }
            DashboardMessage::ToggleAddInstallerPolicyLang => {
                dashboard.installer_add_lang_open = !dashboard.installer_add_lang_open;
                if !dashboard.installer_add_lang_open {
                    dashboard.installer_new_lang_code.clear();
                    dashboard.installer_new_lang_name.clear();
                }
            }
            DashboardMessage::SetNewPolicyLangCode(v) => {
                dashboard.installer_new_lang_code = v;
            }
            DashboardMessage::SetNewPolicyLangName(v) => {
                dashboard.installer_new_lang_name = v;
            }
            DashboardMessage::ConfirmAddInstallerPolicyLang => {
                let code = dashboard
                    .installer_new_lang_code
                    .trim()
                    .to_ascii_lowercase();
                let display_name = dashboard.installer_new_lang_name.trim().to_string();
                let is_builtin = blanc_core::installer::InstallerLang::ALL
                    .iter()
                    .any(|l| l.code() == code);
                if !code.is_empty()
                    && !display_name.is_empty()
                    && !is_builtin
                    && !dashboard
                        .installer_custom_policy_langs
                        .iter()
                        .any(|c| c.code == code)
                {
                    let prev_code = dashboard.installer_policy_lang.clone();
                    let prev_text = dashboard.installer_policy_editor.text();
                    dashboard.installer_policy_texts.insert(prev_code, prev_text);

                    dashboard.installer_custom_policy_langs.push(
                        blanc_core::installer::CustomPolicyLanguage {
                            code: code.clone(),
                            display_name,
                        },
                    );
                    dashboard.installer_policy_lang = code;
                    dashboard.installer_policy_editor =
                        iced::widget::text_editor::Content::with_text("");
                    dashboard.installer_add_lang_open = false;
                    dashboard.installer_new_lang_code.clear();
                    dashboard.installer_new_lang_name.clear();
                }
            }
            DashboardMessage::RemoveCustomPolicyLang(code) => {
                dashboard
                    .installer_custom_policy_langs
                    .retain(|c| c.code != code);
                dashboard.installer_policy_texts.remove(&code);
                if dashboard.installer_policy_lang == code {
                    let en_code = blanc_core::installer::InstallerLang::En.code().to_string();
                    dashboard.installer_policy_lang = en_code.clone();
                    let en_text = dashboard
                        .installer_policy_texts
                        .get(&en_code)
                        .cloned()
                        .unwrap_or_default();
                    dashboard.installer_policy_editor =
                        iced::widget::text_editor::Content::with_text(&en_text);
                }
            }
            DashboardMessage::SetInstallerDesktopShortcut(v) => {
                dashboard.installer_desktop_shortcut = v;
            }
            DashboardMessage::SetInstallerStartMenuShortcut(v) => {
                dashboard.installer_start_menu_shortcut = v;
            }
            DashboardMessage::SetInstallerIncludeLogo(v) => {
                dashboard.installer_include_logo = v;
            }
            DashboardMessage::SetInstallerIncludeBanner(v) => {
                dashboard.installer_include_banner = v;
            }
            DashboardMessage::PickInstallerCustomBanner => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg", "webp", "bmp"])
                    .set_title("Choose Wizard Sidebar Image")
                    .pick_file()
                {
                    dashboard.installer_custom_banner_path = Some(path);
                    dashboard.installer_include_banner = true;
                }
            }
            DashboardMessage::ClearInstallerCustomBanner => {
                dashboard.installer_custom_banner_path = None;
            }
            DashboardMessage::OpenExportFolder => {
                let out_dir = if dashboard.export_output_dir.trim().is_empty() {
                    project_dir.join("Exports")
                } else {
                    PathBuf::from(dashboard.export_output_dir.trim())
                };
                let _ = std::fs::create_dir_all(&out_dir);
                let _ = Command::new("explorer").arg(&out_dir).spawn();
            }
            DashboardMessage::RunExportGame => {
                if dashboard.export_in_progress {
                    return Task::none();
                }
                Self::auto_save_current_scene(dashboard, project_dir);
                dashboard.export_in_progress = true;
                dashboard.export_progress = 0.05;
                dashboard.export_stage_label = "Preparing export...".to_string();
                dashboard.export_status_message = None;

                let mut license_translations = dashboard.installer_policy_texts.clone();
                license_translations.insert(
                    dashboard.installer_policy_lang.clone(),
                    dashboard.installer_policy_editor.text(),
                );
                let snapshot = ExportJobSnapshot {
                    export_exe_name: dashboard.export_exe_name.clone(),
                    export_output_dir: dashboard.export_output_dir.clone(),
                    export_packed: dashboard.export_packed,
                    export_save_mode: dashboard.export_save_mode,
                    export_build_kind: dashboard.export_build_kind,
                    logo_asset_id: dashboard.system_screens_config.logo_asset_id.clone(),
                    title_bg_asset_id: dashboard
                        .system_screens_config
                        .title_background_asset_id
                        .clone(),
                    accent_hex: dashboard
                        .system_screens_config
                        .ui_config
                        .theme
                        .accent_hex
                        .clone(),
                    installer_include_logo: dashboard.installer_include_logo,
                    installer_include_banner: dashboard.installer_include_banner,
                    installer_custom_banner_path: dashboard.installer_custom_banner_path.clone(),
                    installer_publisher: dashboard.installer_publisher.clone(),
                    installer_version: dashboard.installer_version.clone(),
                    installer_welcome_text: dashboard.installer_welcome_text.clone(),
                    installer_desktop_shortcut: dashboard.installer_desktop_shortcut,
                    installer_start_menu_shortcut: dashboard.installer_start_menu_shortcut,
                    license_translations,
                    custom_policy_languages: dashboard.installer_custom_policy_langs.clone(),
                    project: project.clone(),
                    project_dir: project_dir.clone(),
                };

                return Task::run(
                    iced::stream::channel(16, move |mut sender| async move {
                        let mut tx = sender.clone();
                        let handle = tokio::task::spawn_blocking(move || {
                            Self::export_game_package_with_progress(snapshot, |p, label| {
                                let _ = tx.try_send(DashboardMessage::ExportProgress(
                                    p,
                                    label.to_string(),
                                ));
                            })
                        });
                        let res = match handle.await {
                            Ok(r) => r,
                            Err(e) => Err(format!("Export task failed: {}", e)),
                        };
                        let _ = sender.try_send(DashboardMessage::ExportDone(res));
                    }),
                    AppMessage::Dashboard,
                );
            }
            DashboardMessage::ExportProgress(p, label) => {
                dashboard.export_progress = p;
                dashboard.export_stage_label = label;
            }
            DashboardMessage::ExportDone(res) => {
                dashboard.export_in_progress = false;
                dashboard.export_progress = if res.is_ok() { 1.0 } else { 0.0 };
                dashboard.export_stage_label.clear();
                dashboard.export_status_message = Some(match res {
                    Ok(summary) => summary,
                    Err(err) => format!("Export failed: {}", err),
                });
            }
            DashboardMessage::SetUseCustomLogo(use_custom) => {
                project.use_custom_logo = use_custom;
                let manifest_filename = format!("{}.{}", project.name.replace(' ', "_"), Project::MANIFEST_EXTENSION);
                let manifest_path = project_dir.join(manifest_filename);
                let _ = project.save_manifest(&manifest_path);
            }
            DashboardMessage::ChooseProjectLogo => {
                return Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
                            .pick_file()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    |path| AppMessage::Dashboard(DashboardMessage::ProjectLogoSelected(path)),
                );
            }
            DashboardMessage::ProjectLogoSelected(Some(path)) => {
                let dest_dir = project_dir.join("Assets").join("UI");
                let _ = std::fs::create_dir_all(&dest_dir);
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("png");
                let dest_filename = format!("game_logo.{}", ext);
                let dest_path = dest_dir.join(&dest_filename);
                if std::fs::copy(&path, &dest_path).is_ok() {
                    let rel_path = format!("Assets/UI/{}", dest_filename);
                    project.custom_logo_path = Some(rel_path);
                    project.use_custom_logo = true;
                    let manifest_filename = format!("{}.{}", project.name.replace(' ', "_"), Project::MANIFEST_EXTENSION);
                    let manifest_path = project_dir.join(manifest_filename);
                    let _ = project.save_manifest(&manifest_path);
                    dashboard.settings_status_message = Some("Game logo updated and saved.".to_string());
                }
            }
            DashboardMessage::ProjectLogoSelected(None) => {}
            DashboardMessage::FlowAction(action) => {
                match action {
                    FlowCanvasAction::SelectNode(node_id) => {
                        dashboard.flow_selected_node_id = Some(node_id);
                    }
                    FlowCanvasAction::ClearSelection => {
                        dashboard.flow_selected_node_id = None;
                    }
                    FlowCanvasAction::OpenSceneInEditor(scene_name) => {
                        if let Some(idx) = dashboard.scenes_list.iter().position(|s| s == &scene_name) {
                            Self::auto_save_current_scene(dashboard, project_dir);
                            dashboard.selected_scene_index = idx;
                            let scene_file = project_dir
                                .join("Scenes")
                                .join(&dashboard.story_chapter_name)
                                .join(format!("{}.bscene", scene_name));
                            if let Ok(loaded) = Scene::load_from_file(&scene_file) {
                                dashboard.current_scene = Some(loaded);
                            }
                            dashboard.active_tab = WorkflowTab::SceneEditor;
                            dashboard.scene_status_message = Some(format!("Opened scene '{}'.", scene_name));
                            return Task::perform(
                                tokio::time::sleep(std::time::Duration::from_secs(3)),
                                |_| AppMessage::Dashboard(DashboardMessage::DismissSceneStatusMessage),
                            );
                        }
                    }
                    FlowCanvasAction::ConnectPins {
                        from_node,
                        from_pin,
                        to_node,
                        to_pin,
                    } => {
                        dashboard
                            .flow_graph
                            .connect_pins(&from_node, &from_pin, &to_node, &to_pin);

                        // 1. If connecting from Main Menu: update starting_scene_id in settings
                        if from_node == "system_main_menu" && from_pin == "new_game" {
                            if let Some(sc_name) = to_node.strip_prefix("scene_") {
                                dashboard.system_screens_config.starting_scene_id =
                                    Some(sc_name.to_string());
                                let config_dir = project_dir.join("Settings");
                                let _ = std::fs::create_dir_all(&config_dir);
                                let _ = dashboard
                                    .system_screens_config
                                    .save_to_file(&config_dir.join("system_screens.bconfig"));
                            }
                        }

                        // 2. If connecting from a Scene's Choice option: update target_scene_id in the scene
                        if let Some(from_sc) = from_node.strip_prefix("scene_") {
                            if let Some(choice_idx_str) = from_pin.strip_prefix("choice_") {
                                if let Ok(choice_idx) = choice_idx_str.parse::<usize>() {
                                    let target_sc = to_node
                                        .strip_prefix("scene_")
                                        .unwrap_or(&to_node);
                                    let sc_file = project_dir
                                        .join("Scenes")
                                        .join(&dashboard.story_chapter_name)
                                        .join(format!("{}.bscene", from_sc));
                                    if let Ok(mut sc) = Scene::load_from_file(&sc_file) {
                                        for block in sc.blocks.iter_mut().rev() {
                                            if let ActionBlock::Choice { options, .. } = block {
                                                if let Some(opt) = options.get_mut(choice_idx) {
                                                    opt.target_scene_id = target_sc.to_string();
                                                }
                                                break;
                                            }
                                        }
                                        let _ = sc.save_to_file(&sc_file);
                                        if let Some(cur) = &mut dashboard.current_scene {
                                            if cur.name == from_sc {
                                                *cur = sc.clone();
                                            }
                                        }
                                        if let Some(cached) = dashboard
                                            .all_scenes
                                            .iter_mut()
                                            .find(|s| s.name == from_sc)
                                        {
                                            *cached = sc;
                                        }
                                    }
                                }
                            }
                        }

                        // Auto-save flow graph
                        let story_dir = project_dir.join("Story");
                        let _ = std::fs::create_dir_all(&story_dir);
                        let _ = dashboard
                            .flow_graph
                            .save_to_file(&story_dir.join("story_flow.bgraph"));
                    }
                    FlowCanvasAction::DisconnectPin { node_id, pin_id } => {
                        dashboard.flow_graph.disconnect_pin(&node_id, &pin_id);
                        let story_dir = project_dir.join("Story");
                        let _ = std::fs::create_dir_all(&story_dir);
                        let _ = dashboard
                            .flow_graph
                            .save_to_file(&story_dir.join("story_flow.bgraph"));
                    }
                    FlowCanvasAction::SetNodePosition { node_id, x, y } => {
                        if let Some(node) = dashboard.flow_graph.find_node_mut(&node_id) {
                            node.x = x;
                            node.y = y;
                        }
                    }
                    FlowCanvasAction::PanCanvas { dx, dy } => {
                        dashboard.flow_graph.pan_x += dx;
                        dashboard.flow_graph.pan_y += dy;
                    }
                    FlowCanvasAction::ZoomDelta(delta) => {
                        dashboard.flow_graph.zoom =
                            (dashboard.flow_graph.zoom + delta).clamp(0.4, 2.5);
                    }
                    FlowCanvasAction::RemoveNode(node_id) => {
                        dashboard.flow_graph.remove_node(&node_id);
                        let story_dir = project_dir.join("Story");
                        let _ = std::fs::create_dir_all(&story_dir);
                        let _ = dashboard
                            .flow_graph
                            .save_to_file(&story_dir.join("story_flow.bgraph"));
                    }
                    FlowCanvasAction::SetMode(mode) => {
                        dashboard.flow_mode = mode;
                    }
                    FlowCanvasAction::FocusAll => {
                        if !dashboard.flow_graph.nodes.is_empty() {
                            let min_x = dashboard.flow_graph.nodes.iter().map(|n| n.x).fold(f32::INFINITY, f32::min);
                            let max_x = dashboard.flow_graph.nodes.iter().map(|n| n.x + n.width).fold(f32::NEG_INFINITY, f32::max);
                            let min_y = dashboard.flow_graph.nodes.iter().map(|n| n.y).fold(f32::INFINITY, f32::min);
                            let max_y = dashboard.flow_graph.nodes.iter().map(|n| n.y + n.height).fold(f32::NEG_INFINITY, f32::max);
                            let center_x = (min_x + max_x) * 0.5;
                            let center_y = (min_y + max_y) * 0.5;
                            dashboard.flow_graph.pan_x = 420.0 - center_x;
                            dashboard.flow_graph.pan_y = 260.0 - center_y;
                            dashboard.flow_graph.zoom = 1.0;
                        }
                    }
                    FlowCanvasAction::AutoLayout => {
                        let mut cur_y = 120.0;
                        if let Some(mm) = dashboard.flow_graph.find_node_mut("system_main_menu") {
                            mm.x = 60.0;
                            mm.y = 180.0;
                        }
                        for node in dashboard.flow_graph.nodes.iter_mut() {
                            if matches!(node.node_type, FlowNodeType::Scene { .. }) {
                                node.x = 360.0;
                                node.y = cur_y;
                                cur_y += 160.0;
                            }
                        }
                        for node in dashboard.flow_graph.nodes.iter_mut() {
                            if matches!(node.node_type, FlowNodeType::Ending { .. } | FlowNodeType::PauseMenu | FlowNodeType::Settings | FlowNodeType::SaveLoad) {
                                node.x = 720.0;
                                node.y = 180.0;
                            }
                        }
                        dashboard.flow_graph.pan_x = 0.0;
                        dashboard.flow_graph.pan_y = 0.0;
                        dashboard.flow_graph.zoom = 1.0;
                        let story_dir = project_dir.join("Story");
                        let _ = std::fs::create_dir_all(&story_dir);
                        let _ = dashboard
                            .flow_graph
                            .save_to_file(&story_dir.join("story_flow.bgraph"));
                    }
                    FlowCanvasAction::ToggleSnapGrid => {
                        dashboard.flow_snap_grid = !dashboard.flow_snap_grid;
                    }
                    FlowCanvasAction::ResetView => {
                        dashboard.flow_graph.pan_x = 0.0;
                        dashboard.flow_graph.pan_y = 0.0;
                        dashboard.flow_graph.zoom = 1.0;
                    }
                }
            }
            DashboardMessage::FlowAddScene(scene_name) => {
                let count = dashboard.flow_graph.nodes.len();
                let new_x = 380.0 + (count as f32 % 3.0) * 260.0;
                let new_y = 140.0 + (count as f32 / 3.0).floor() * 160.0;
                if dashboard.flow_graph.add_scene_node(&scene_name, new_x, new_y) {
                    let story_dir = project_dir.join("Story");
                    let _ = std::fs::create_dir_all(&story_dir);
                    let _ = dashboard
                        .flow_graph
                        .save_to_file(&story_dir.join("story_flow.bgraph"));
                    dashboard.flow_status_message =
                        Some(format!("Added scene '{}' to Flow canvas.", scene_name));
                    return Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(3)),
                        |_| AppMessage::Dashboard(DashboardMessage::DismissFlowStatusMessage),
                    );
                }
            }
            DashboardMessage::FlowAddSystemNode(sys_type) => {
                let count = dashboard.flow_graph.nodes.len();
                let new_x = 420.0 + (count as f32 % 3.0) * 260.0;
                let new_y = 160.0 + (count as f32 / 3.0).floor() * 160.0;
                if dashboard.flow_graph.add_system_node(&sys_type, new_x, new_y) {
                    let story_dir = project_dir.join("Story");
                    let _ = std::fs::create_dir_all(&story_dir);
                    let _ = dashboard
                        .flow_graph
                        .save_to_file(&story_dir.join("story_flow.bgraph"));
                    dashboard.flow_status_message =
                        Some(format!("Added system node '{}' to Flow canvas.", sys_type));
                    return Task::perform(
                        tokio::time::sleep(std::time::Duration::from_secs(3)),
                        |_| AppMessage::Dashboard(DashboardMessage::DismissFlowStatusMessage),
                    );
                }
            }
            DashboardMessage::FlowSetMode(mode) => {
                dashboard.flow_mode = mode;
            }
            DashboardMessage::FlowToggleSnapGrid => {
                dashboard.flow_snap_grid = !dashboard.flow_snap_grid;
            }
            DashboardMessage::FlowFocusAll => {
                if !dashboard.flow_graph.nodes.is_empty() {
                    let min_x = dashboard.flow_graph.nodes.iter().map(|n| n.x).fold(f32::INFINITY, f32::min);
                    let max_x = dashboard.flow_graph.nodes.iter().map(|n| n.x + n.width).fold(f32::NEG_INFINITY, f32::max);
                    let min_y = dashboard.flow_graph.nodes.iter().map(|n| n.y).fold(f32::INFINITY, f32::min);
                    let max_y = dashboard.flow_graph.nodes.iter().map(|n| n.y + n.height).fold(f32::NEG_INFINITY, f32::max);
                    let center_x = (min_x + max_x) * 0.5;
                    let center_y = (min_y + max_y) * 0.5;
                    dashboard.flow_graph.pan_x = 420.0 - center_x;
                    dashboard.flow_graph.pan_y = 260.0 - center_y;
                    dashboard.flow_graph.zoom = 1.0;
                }
            }
            DashboardMessage::FlowRemoveNode(node_id) => {
                dashboard.flow_graph.remove_node(&node_id);
                let story_dir = project_dir.join("Story");
                let _ = std::fs::create_dir_all(&story_dir);
                let _ = dashboard
                    .flow_graph
                    .save_to_file(&story_dir.join("story_flow.bgraph"));
            }
            DashboardMessage::FlowAutoLayout => {
                let mut cur_y = 120.0;
                if let Some(mm) = dashboard.flow_graph.find_node_mut("system_main_menu") {
                    mm.x = 60.0;
                    mm.y = 180.0;
                }
                for node in dashboard.flow_graph.nodes.iter_mut() {
                    if matches!(node.node_type, FlowNodeType::Scene { .. }) {
                        node.x = 360.0;
                        node.y = cur_y;
                        cur_y += 160.0;
                    }
                }
                for node in dashboard.flow_graph.nodes.iter_mut() {
                    if matches!(node.node_type, FlowNodeType::Ending { .. }) {
                        node.x = 720.0;
                        node.y = 180.0;
                    }
                }
                dashboard.flow_graph.pan_x = 0.0;
                dashboard.flow_graph.pan_y = 0.0;
                dashboard.flow_graph.zoom = 1.0;
                let story_dir = project_dir.join("Story");
                let _ = std::fs::create_dir_all(&story_dir);
                let _ = dashboard
                    .flow_graph
                    .save_to_file(&story_dir.join("story_flow.bgraph"));
                dashboard.flow_status_message = Some("Auto-aligned nodes on canvas.".to_string());
                return Task::perform(
                    tokio::time::sleep(std::time::Duration::from_secs(3)),
                    |_| AppMessage::Dashboard(DashboardMessage::DismissFlowStatusMessage),
                );
            }
            DashboardMessage::FlowResetView => {
                dashboard.flow_graph.pan_x = 0.0;
                dashboard.flow_graph.pan_y = 0.0;
                dashboard.flow_graph.zoom = 1.0;
            }
            DashboardMessage::FlowZoomIn => {
                dashboard.flow_graph.zoom = (dashboard.flow_graph.zoom + 0.15).min(2.5);
            }
            DashboardMessage::FlowZoomOut => {
                dashboard.flow_graph.zoom = (dashboard.flow_graph.zoom - 0.15).max(0.4);
            }
            DashboardMessage::FlowSearchChanged(query) => {
                dashboard.flow_search_query = query;
            }
            DashboardMessage::DismissFlowStatusMessage => {
                dashboard.flow_status_message = None;
            }
            DashboardMessage::SaveStoryFlow => {
                let story_dir = project_dir.join("Story");
                let _ = std::fs::create_dir_all(&story_dir);
                let graph_path = story_dir.join("story_flow.bgraph");
                match dashboard.flow_graph.save_to_file(&graph_path) {
                    Ok(_) => {
                        dashboard.flow_status_message =
                            Some("Saved story flow graph successfully.".to_string());
                        return Task::perform(
                            tokio::time::sleep(std::time::Duration::from_secs(3)),
                            |_| AppMessage::Dashboard(DashboardMessage::DismissFlowStatusMessage),
                        );
                    }
                    Err(err) => {
                        dashboard.flow_status_message =
                            Some(format!("Failed to save graph: {}", err));
                    }
                }
            }
            DashboardMessage::UiDesigner(ui_msg) => {
                if let crate::screens::ui_designer::UiDesignerMessage::ClearAsset(UiAssetSlot::MainMenuLogo) = ui_msg {
                    dashboard.system_screens_config.logo_asset_id = None;
                }
                handle_ui_designer_message(
                    ui_msg,
                    &mut dashboard.system_screens_config.ui_config,
                    &mut dashboard.ui_designer_sub_tab,
                    &mut dashboard.ui_designer_sections,
                    &mut dashboard.ui_preview_mode,
                );
            }
            DashboardMessage::PlayPreview => {
                if crate::screens::welcome::HubConfig::load().auto_save_on_play_preview {
                    Self::auto_save_current_scene(dashboard, project_dir);
                }
                Self::launch_runtime_preview(dashboard, project_dir, None);
            }
            DashboardMessage::PlayFromCurrentScene => {
                Self::auto_save_current_scene(dashboard, project_dir);
                let scene_name = dashboard
                    .scenes_list
                    .get(dashboard.selected_scene_index)
                    .cloned();
                Self::launch_runtime_preview(dashboard, project_dir, scene_name.as_deref());
            }
        }

        Task::none()
    }

    #[cfg(feature = "bundle-binaries")]
    fn ensure_bundled_binary(filename: &str, bytes: &[u8]) -> Option<PathBuf> {
        let base = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        let dir = base
            .join("BlancEngine")
            .join("bin")
            .join(env!("CARGO_PKG_VERSION"));
        std::fs::create_dir_all(&dir).ok()?;
        let target = dir.join(filename);
        let needs_write = match std::fs::metadata(&target) {
            Ok(meta) => meta.len() != bytes.len() as u64,
            Err(_) => true,
        };
        if needs_write {
            std::fs::write(&target, bytes).ok()?;
        }
        Some(target)
    }

    fn ensure_bundled_runtime_extracted() -> Option<PathBuf> {
        #[cfg(feature = "bundle-binaries")]
        {
            let name = if cfg!(windows) { "blanc_runtime.exe" } else { "blanc_runtime" };
            let bytes = include_bytes!(env!("BLANC_BUNDLED_RUNTIME"));
            return Self::ensure_bundled_binary(name, bytes);
        }
        #[cfg(not(feature = "bundle-binaries"))]
        {
            None
        }
    }

    fn ensure_bundled_installer_extracted() -> Option<PathBuf> {
        #[cfg(feature = "bundle-binaries")]
        {
            let name = if cfg!(windows) { "blanc_installer.exe" } else { "blanc_installer" };
            let bytes = include_bytes!(env!("BLANC_BUNDLED_INSTALLER"));
            return Self::ensure_bundled_binary(name, bytes);
        }
        #[cfg(not(feature = "bundle-binaries"))]
        {
            None
        }
    }

    fn launch_runtime_preview(dashboard: &DashboardState, project_dir: &PathBuf, start_scene: Option<&str>) {
        // Ensure latest system screen settings are persisted before launching runtime
        let config_dir = project_dir.join("Settings");
        let _ = std::fs::create_dir_all(&config_dir);
        let config_path = config_dir.join("system_screens.bconfig");
        let _ = dashboard.system_screens_config.save_to_file(&config_path);

        let target_dir = project_dir.to_string_lossy().to_string();
        let mut runtime_args: Vec<String> = vec![target_dir.clone()];
        if let Some(scene_name) = start_scene {
            runtime_args.push("--scene".to_string());
            runtime_args.push(scene_name.to_string());
        }

        let sibling_runtime = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|dir| {
                let name = if cfg!(windows) { "blanc_runtime.exe" } else { "blanc_runtime" };
                dir.join(name)
            }))
            .filter(|p| p.exists())
            .or_else(Self::ensure_bundled_runtime_extracted);

        let launched = if let Some(runtime_path) = sibling_runtime {
            Command::new(runtime_path).args(&runtime_args).spawn()
        } else {
            Command::new("blanc_runtime").args(&runtime_args).spawn()
        };

        let _ = launched.or_else(|_| {
            let mut cargo_args = vec!["run", "-p", "blanc_runtime", "--", &target_dir];
            if let Some(scene_name) = start_scene {
                cargo_args.push("--scene");
                cargo_args.push(scene_name);
            }
            Command::new("cargo").args(&cargo_args).spawn()
        });
    }

    fn export_game_package_with_progress(
        snap: ExportJobSnapshot,
        mut on_progress: impl FnMut(f32, &str),
    ) -> Result<String, String> {
        let project = &snap.project;
        let project_dir = &snap.project_dir;

        let raw_exe = if snap.export_exe_name.trim().is_empty() {
            project.name.as_str()
        } else {
            snap.export_exe_name.trim().trim_end_matches(".exe")
        };
        let exe_name: String = raw_exe
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect();
        let exe_name = if exe_name.trim_matches('_').is_empty() {
            "BlancGame".to_string()
        } else {
            exe_name
        };

        let out_dir = if snap.export_output_dir.trim().is_empty() {
            project_dir.join("Exports")
        } else {
            PathBuf::from(snap.export_output_dir.trim())
        };
        std::fs::create_dir_all(&out_dir)
            .map_err(|e| format!("Failed to create output folder: {}", e))?;

        let runtime_name = if cfg!(windows) { "blanc_runtime.exe" } else { "blanc_runtime" };
        let runtime_src = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|dir| dir.join(runtime_name)))
            .filter(|p| p.exists())
            .or_else(Self::ensure_bundled_runtime_extracted)
            .ok_or_else(|| {
                format!(
                    "{} not found next to editor executable. Please build blanc_runtime first.",
                    runtime_name
                )
            })?;

        let exe_ext = if cfg!(windows) { ".exe" } else { "" };
        let game_exe_filename = format!("{}{}", exe_name, exe_ext);

        on_progress(0.12, "Collecting project files & assets...");
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();
        Self::collect_project_files(project_dir, project_dir, &mut files)?;

        let effective_save_mode = if snap.export_build_kind == blanc_core::installer::ExportBuildKind::Installer {
            blanc_core::pack::ExportSaveMode::AppData
        } else {
            snap.export_save_mode
        };

        let meta = blanc_core::pack::ExportMetadata {
            save_mode: effective_save_mode,
            game_title: project.name.clone(),
        };
        let meta_bytes = serde_json::to_vec_pretty(&meta)
            .map_err(|e| format!("Failed to serialize export metadata: {}", e))?;

        let project_logo_png = snap
            .logo_asset_id
            .as_deref()
            .and_then(|id| project.assets.get(id))
            .map(|a| project_dir.join(&a.relative_path))
            .or_else(|| project.custom_logo_path.as_ref().map(|p| project_dir.join(p)))
            .and_then(|p| std::fs::read(p).ok());

        if snap.export_build_kind == blanc_core::installer::ExportBuildKind::Installer {
            let installer_name = if cfg!(windows) { "blanc_installer.exe" } else { "blanc_installer" };
            let installer_src = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|dir| dir.join(installer_name)))
                .filter(|p| p.exists())
                .or_else(Self::ensure_bundled_installer_extracted)
                .ok_or_else(|| {
                    format!(
                        "{} not found next to editor executable. Please build blanc_installer first.",
                        installer_name
                    )
                })?;

            on_progress(0.32, "Embedding application icon...");
            let tmp_stub = std::env::temp_dir().join(format!("blanc_tmp_inst_{}{}", uuid::Uuid::new_v4(), exe_ext));
            std::fs::copy(&installer_src, &tmp_stub)
                .map_err(|e| format!("Failed to stage installer stub: {}", e))?;
            let stub_embed_res = blanc_core::win_icon::embed_png_icon_into_exe(&tmp_stub, project_logo_png.as_deref());
            let stub_bytes = std::fs::read(&tmp_stub)
                .map_err(|e| format!("Failed to read installer stub: {}", e));
            let _ = std::fs::remove_file(&tmp_stub);
            stub_embed_res?;
            let stub_bytes = stub_bytes?;

            let tmp_runtime = std::env::temp_dir().join(format!("blanc_tmp_rt_{}{}", uuid::Uuid::new_v4(), exe_ext));
            std::fs::copy(&runtime_src, &tmp_runtime)
                .map_err(|e| format!("Failed to stage runtime binary: {}", e))?;
            let rt_embed_res = blanc_core::win_icon::embed_png_icon_into_exe(&tmp_runtime, project_logo_png.as_deref());
            let runtime_bytes = std::fs::read(&tmp_runtime)
                .map_err(|e| format!("Failed to read runtime binary: {}", e));
            let _ = std::fs::remove_file(&tmp_runtime);
            rt_embed_res?;
            let runtime_bytes = runtime_bytes?;

            let mut installer_files: Vec<(String, Vec<u8>)> = vec![
                (game_exe_filename.clone(), runtime_bytes),
            ];

            if snap.export_packed {
                on_progress(0.55, "Packing game data (.bpack)...");
                files.push((blanc_core::pack::EXPORT_META_ENTRY.to_string(), meta_bytes));
                let temp_pack = std::env::temp_dir().join(format!("blanc_tmp_pack_{}.bpack", uuid::Uuid::new_v4()));
                blanc_core::pack::write_bpack(&temp_pack, &files)?;
                let pack_bytes = std::fs::read(&temp_pack)
                    .map_err(|e| format!("Failed to read temporary bpack: {}", e))?;
                let _ = std::fs::remove_file(&temp_pack);
                installer_files.push(("game.bpack".to_string(), pack_bytes));
            } else {
                on_progress(0.55, "Staging GameData files...");
                for (rel, bytes) in files {
                    installer_files.push((format!("GameData/{}", rel), bytes));
                }
                installer_files.push((
                    format!("GameData/{}", blanc_core::pack::EXPORT_META_ENTRY),
                    meta_bytes,
                ));
            }

            let logo_png = if snap.installer_include_logo {
                project_logo_png.clone()
            } else {
                None
            };

            let banner_png = if snap.installer_include_banner {
                snap.installer_custom_banner_path
                    .as_ref()
                    .and_then(|p| std::fs::read(p).ok())
                    .or_else(|| {
                        snap.title_bg_asset_id
                            .as_deref()
                            .and_then(|id| project.assets.get(id))
                            .map(|a| project_dir.join(&a.relative_path))
                            .and_then(|p| std::fs::read(p).ok())
                    })
            } else {
                None
            };

            let publisher = if snap.installer_publisher.trim().is_empty() {
                project.author.clone()
            } else {
                snap.installer_publisher.trim().to_string()
            };
            let version = if snap.installer_version.trim().is_empty() {
                "1.0.0".to_string()
            } else {
                snap.installer_version.trim().to_string()
            };

            let license_text = snap
                .license_translations
                .get(blanc_core::installer::InstallerLang::En.code())
                .cloned()
                .unwrap_or_default();

            let inst_cfg = blanc_core::installer::InstallerConfig {
                game_title: project.name.clone(),
                version,
                publisher,
                welcome_text: snap.installer_welcome_text.clone(),
                license_text,
                license_translations: snap.license_translations,
                custom_policy_languages: snap.custom_policy_languages,
                default_install_subdir: exe_name.clone(),
                create_desktop_shortcut_default: snap.installer_desktop_shortcut,
                create_start_menu_shortcut_default: snap.installer_start_menu_shortcut,
                accent_hex: snap.accent_hex,
                logo_png,
                banner_png,
            };

            on_progress(0.82, "Building Setup.exe installer...");
            let setup_filename = format!("{}_Setup{}", exe_name, exe_ext);
            let setup_dest = out_dir.join(&setup_filename);
            blanc_core::installer::write_installer_exe(
                &stub_bytes,
                &setup_dest,
                &inst_cfg,
                &game_exe_filename,
                &installer_files,
            )?;

            on_progress(1.0, "Export complete!");
            return Ok(format!(
                "Export succeeded! Created standalone installer {}.",
                setup_filename
            ));
        }

        on_progress(0.35, "Copying runtime & embedding icon...");
        let exe_dest = out_dir.join(&game_exe_filename);
        std::fs::copy(&runtime_src, &exe_dest)
            .map_err(|e| format!("Failed to copy runtime binary: {}", e))?;
        blanc_core::win_icon::embed_png_icon_into_exe(&exe_dest, project_logo_png.as_deref())?;

        if snap.export_packed {
            on_progress(0.70, "Packing game.bpack...");
            files.push((blanc_core::pack::EXPORT_META_ENTRY.to_string(), meta_bytes));
            let pack_path = out_dir.join("game.bpack");
            blanc_core::pack::write_bpack(&pack_path, &files)?;
            on_progress(1.0, "Export complete!");
            Ok(format!(
                "Export succeeded! Created {}{} + encrypted game.bpack ({} files).",
                exe_name,
                exe_ext,
                files.len()
            ))
        } else {
            on_progress(0.70, "Writing GameData folder...");
            let data_dir = out_dir.join("GameData");
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("Failed to create GameData folder: {}", e))?;
            for (rel, bytes) in &files {
                let target = data_dir.join(rel);
                if let Some(parent) = target.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                std::fs::write(&target, bytes)
                    .map_err(|e| format!("Failed to write {}: {}", rel, e))?;
            }
            std::fs::write(data_dir.join(blanc_core::pack::EXPORT_META_ENTRY), &meta_bytes)
                .map_err(|e| format!("Failed to write export metadata: {}", e))?;
            on_progress(1.0, "Export complete!");
            Ok(format!(
                "Export succeeded! Created {}{} + GameData/ ({} files).",
                exe_name,
                exe_ext,
                files.len()
            ))
        }
    }

    fn collect_project_files(
        root: &std::path::Path,
        dir: &std::path::Path,
        out: &mut Vec<(String, Vec<u8>)>,
    ) -> Result<(), String> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read {}: {}", dir.display(), e))?;
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if matches!(name.as_str(), "Saves" | "Exports" | ".git" | "target") {
                    continue;
                }
                Self::collect_project_files(root, &path, out)?;
            } else if path.is_file() {
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| format!("Path prefix error: {}", e))?;
                let rel_norm = rel.to_string_lossy().replace('\\', "/");
                let bytes = std::fs::read(&path)
                    .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
                out.push((rel_norm, bytes));
            }
        }
        Ok(())
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        match &self.screen {
            CurrentScreen::Welcome(w_state) => w_state.view().map(AppMessage::Welcome),
            CurrentScreen::Workspace {
                project,
                project_dir,
                dashboard,
            } => dashboard.view(project, project_dir).map(AppMessage::Dashboard),
        }
    }
}
