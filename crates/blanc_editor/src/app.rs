use std::path::PathBuf;
use std::process::Command;
use iced::{Element, Task};

use blanc_core::asset::AssetCategory;
use blanc_core::project::Project;
use blanc_core::scene::{ActionBlock, CharacterPosition, ChoiceOption as SceneChoiceOption, Scene};
use blanc_core::story::{Chapter, Story, StoryElement};
use crate::screens::{
    AssetPickerModalState, AssetPickerTarget, DashboardMessage, DashboardState, WelcomeMessage, WelcomeState,
    WorkflowTab, DEFAULT_SCRIPT_TEMPLATE,
};

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
                            project_dir: target_dir.clone(),
                            dashboard: DashboardState::init_for_project(&target_dir),
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
                                project_dir: project_dir.clone(),
                                dashboard: DashboardState::init_for_project(&project_dir),
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
                            project_dir: project_dir.clone(),
                            dashboard: DashboardState::init_for_project(&project_dir),
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
                Self::auto_save_current_scene(dashboard, project_dir);
                self.screen = CurrentScreen::Welcome(WelcomeState::new());
            }
            DashboardMessage::SelectWorkflowTab(tab) => {
                Self::auto_save_current_scene(dashboard, project_dir);
                dashboard.active_tab = tab;
                dashboard.status_message = None;
                dashboard.story_status_message = None;
                if tab == WorkflowTab::SceneEditor {
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
                                Some(ActionBlock::Character { character_id, .. }) => Some(character_id.clone()),
                                Some(ActionBlock::PlayBgm { asset_id, .. }) => Some(asset_id.clone()),
                                Some(ActionBlock::PlaySfx { asset_id, .. }) => Some(asset_id.clone()),
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
                };
                dashboard.asset_picker_modal = Some(AssetPickerModalState {
                    target,
                    category,
                    search_query: String::new(),
                    selected_asset_id: initial_selected,
                });
            }
            DashboardMessage::CloseAssetPickerModal => {
                dashboard.asset_picker_modal = None;
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
                                        Some(ActionBlock::Character { character_id, .. }) => {
                                            *character_id = asset_id_val;
                                        }
                                        Some(ActionBlock::PlayBgm { asset_id, .. }) => {
                                            *asset_id = asset_id_val;
                                        }
                                        Some(ActionBlock::PlaySfx { asset_id, .. }) => {
                                            *asset_id = asset_id_val;
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
                        }
                    }
                }
                dashboard.asset_picker_modal = None;
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
            DashboardMessage::UpdateMasterVolume(v) => {
                dashboard.system_screens_config.master_volume = v;
            }
            DashboardMessage::UpdateBgmVolume(v) => {
                dashboard.system_screens_config.bgm_volume = v;
            }
            DashboardMessage::UpdateSfxVolume(v) => {
                dashboard.system_screens_config.sfx_volume = v;
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
