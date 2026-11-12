use std::path::PathBuf;
use std::process::Command;
use iced::{Element, Task};

use blanc_core::asset::AssetCategory;
use blanc_core::project::Project;
use crate::screens::{DashboardMessage, DashboardState, WelcomeMessage, WelcomeState};

#[derive(Debug, Clone)]
pub enum AppMessage {
    Welcome(WelcomeMessage),
    Dashboard(DashboardMessage),
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

                        self.screen = CurrentScreen::Workspace {
                            project,
                            project_dir: target_dir,
                            dashboard: DashboardState::default(),
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
                            self.screen = CurrentScreen::Workspace {
                                project,
                                project_dir,
                                dashboard: DashboardState::default(),
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
                        self.screen = CurrentScreen::Workspace {
                            project,
                            project_dir,
                            dashboard: DashboardState::default(),
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
                }
            }
        }

        Task::none()
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
                self.screen = CurrentScreen::Welcome(WelcomeState::new());
            }
            DashboardMessage::SelectWorkflowTab(tab) => {
                dashboard.active_tab = tab;
                dashboard.status_message = None;
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
                            dashboard.status_message = Some(format!(
                                "Imported '{}' to {}.",
                                asset.name,
                                asset.category.display_name()
                            ));
                        }
                        Err(err) => {
                            dashboard.status_message = Some(format!("Failed to import asset: {}", err));
                        }
                    }
                }
            }
            DashboardMessage::DeleteAssetClicked(asset_id) => {
                match project.delete_asset(project_dir, &asset_id, true) {
                    Ok(_) => {
                        dashboard.status_message = Some("Asset deleted successfully.".to_string());
                    }
                    Err(err) => {
                        dashboard.status_message = Some(format!("Failed to delete asset: {}", err));
                    }
                }
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
            DashboardMessage::PlayPreview => {
                // Launch blanc_runtime as child process
                let target_dir = project_dir.to_string_lossy().to_string();
                println!("Launching blanc_runtime for: {}", target_dir);

                // Try to find blanc_runtime executable
                let _ = Command::new("blanc_runtime")
                    .arg(&target_dir)
                    .spawn()
                    .or_else(|_| {
                        // Fallback using cargo run in development
                        Command::new("cargo")
                            .args(["run", "-p", "blanc_runtime", "--", &target_dir])
                            .spawn()
                    });
            }
        }

        Task::none()
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
