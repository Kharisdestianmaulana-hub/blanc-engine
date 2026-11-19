use std::fs;
use std::path::{Component, Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::asset::AssetRegistry;
use crate::config::SystemScreensConfig;
use crate::scene::Scene;

/// Safely joins a relative path to `base_dir`, rejecting any path traversal (`..`),
/// root directories, or drive prefixes. Normalizes `\` to `/` before parsing components.
pub fn safe_join_relative(base_dir: &Path, rel_path: &str) -> Option<PathBuf> {
    let trimmed = rel_path.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = trimmed.replace('\\', "/");
    let mut out = base_dir.to_path_buf();
    let mut pushed_any = false;

    for comp in Path::new(&normalized).components() {
        match comp {
            Component::CurDir => {}
            Component::Normal(seg) => {
                let s = seg.to_string_lossy();
                if s.is_empty() || s.contains(':') || s.contains('\0') {
                    return None;
                }
                out.push(seg);
                pushed_any = true;
            }
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return None;
            }
        }
    }

    if pushed_any {
        Some(out)
    } else {
        None
    }
}

/// Writes `bytes` to `path` atomically via a sibling temporary file + rename,
/// falling back to direct `fs::write` if rename is blocked on Windows.
pub fn atomic_write(path: &Path, bytes: impl AsRef<[u8]>) -> std::io::Result<()> {
    let data = bytes.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let tmp_name = format!(".{}.blanc_tmp_{}", file_name, std::process::id());
    let tmp_path = path.with_file_name(tmp_name);

    if fs::write(&tmp_path, data).is_ok() {
        if fs::rename(&tmp_path, path).is_ok() {
            return Ok(());
        }
        let _ = fs::remove_file(&tmp_path);
    }
    fs::write(path, data)
}

/// Validates that the parent folder of `manifest_path` is a dedicated Blanc Engine
/// project directory and NOT a broad user/system folder (like `Documents`, `Desktop`, or a drive root).
pub fn validate_project_folder_for_deletion(manifest_path: &Path) -> Result<PathBuf, String> {
    let folder = manifest_path
        .parent()
        .ok_or_else(|| "Project path has no parent directory.".to_string())?;

    if folder.parent().is_none() {
        return Err("Refusing to delete a drive root directory.".to_string());
    }

    let normal_count = folder
        .components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .count();
    if normal_count < 2 {
        return Err(format!(
            "Refusing to delete top-level directory '{}'.",
            folder.display()
        ));
    }

    let folder_name = folder
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .trim();
    let forbidden_names = [
        "documents",
        "desktop",
        "downloads",
        "pictures",
        "music",
        "videos",
        "onedrive",
        "users",
        "home",
        "program files",
        "program files (x86)",
        "windows",
        "appdata",
        "local",
        "roaming",
    ];
    if forbidden_names
        .iter()
        .any(|f| folder_name.eq_ignore_ascii_case(f))
    {
        return Err(format!(
            "Refusing to delete system/user folder '{}'. Move the project into its own subfolder first.",
            folder.display()
        ));
    }

    for env_key in ["USERPROFILE", "HOME", "APPDATA", "LOCALAPPDATA"] {
        if let Some(val) = std::env::var_os(env_key) {
            if folder == Path::new(&val) {
                return Err(format!(
                    "Refusing to delete user profile directory '{}'.",
                    folder.display()
                ));
            }
        }
    }

    let has_project_markers = manifest_path.exists()
        || folder.join("Scenes").is_dir()
        || folder.join("Assets").is_dir()
        || folder.join("Settings").is_dir()
        || folder.join("Story").is_dir();

    if !has_project_markers {
        return Err(format!(
            "Directory '{}' does not contain recognizable Blanc Engine project files.",
            folder.display()
        ));
    }

    Ok(folder.to_path_buf())
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Project manifest not found: {0}")]
    NotFound(PathBuf),
    #[error("Invalid project file: {0}")]
    Invalid(String),
    #[error("Unsupported asset format: {0}")]
    UnsupportedFormat(String),
}

/// Project manifest model stored as `<ProjectName>.blanc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assets: AssetRegistry,
    pub starting_scene_id: Option<String>,
    #[serde(default)]
    pub custom_logo_path: Option<String>,
    #[serde(default)]
    pub use_custom_logo: bool,
}

impl Project {
    pub const MANIFEST_EXTENSION: &'static str = "blanc";

    pub fn new(name: impl Into<String>, author: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            author: author.into(),
            description: String::new(),
            created_at: now,
            updated_at: now,
            assets: AssetRegistry::new(),
            starting_scene_id: None,
            custom_logo_path: None,
            use_custom_logo: false,
        }
    }

    /// Creates a complete directory structure and initial files for a new project
    pub fn create_project_structure(project_dir: &Path, project_name: &str, author: &str) -> Result<Self, ProjectError> {
        // Create standard folder structure
        let folders = [
            "Assets/Backgrounds",
            "Assets/Characters",
            "Assets/BGM",
            "Assets/SFX",
            "Assets/Fonts",
            "Assets/UI",
            "Assets/Other",
            "Story/Chapters",
            "Scenes/Chapter_01",
            "Settings",
            "Builds",
            "Backups",
        ];

        for folder in &folders {
            fs::create_dir_all(project_dir.join(folder))?;
        }

        // Initialize project manifest
        let mut project = Self::new(project_name, author);

        // Create default initial scene with starter action blocks
        let mut initial_scene = Scene::new("01_opening", "Chapter_01");
        initial_scene.add_block(crate::scene::ActionBlock::Background {
            asset_id: "classroom_afternoon".to_string(),
            transition: Some(crate::scene::TransitionType::Fade),
        });
        initial_scene.add_block(crate::scene::ActionBlock::Character {
            character_id: "Alice".to_string(),
            sprite_asset_id: None,
            expression: "Happy".to_string(),
            position: crate::scene::CharacterPosition::Center,
            offset_x: 0.0,
            offset_y: 0.0,
            transition: Some(crate::scene::TransitionType::Fade),
        });
        initial_scene.add_block(crate::scene::ActionBlock::Dialogue {
            speaker: "Alice".to_string(),
            text: "Hello! Welcome to Blanc Engine's Visual Scene Editor.".to_string(),
            voice_asset_id: None,
            nameplate_position: None,
        });
        initial_scene.add_block(crate::scene::ActionBlock::Choice {
            prompt: "What would you like to explore next?".to_string(),
            position: crate::scene::ChoiceLayoutPosition::Center,
            options: vec![
                crate::scene::ChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Explore Classroom".to_string(),
                    target_scene_id: "Scene_Classroom".to_string(),
                },
                crate::scene::ChoiceOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: "Head Outside to Courtyard".to_string(),
                    target_scene_id: "Scene_Courtyard".to_string(),
                },
            ],
        });
        project.starting_scene_id = Some(initial_scene.id.clone());

        let initial_scene_path = project_dir.join("Scenes/Chapter_01/01_opening.bscene");
        let initial_scene_json = serde_json::to_string_pretty(&initial_scene)?;
        fs::write(initial_scene_path, initial_scene_json)?;

        // Create default system_screens.bconfig
        let system_screens = SystemScreensConfig {
            title_text: project_name.to_string(),
            starting_scene_id: project.starting_scene_id.clone(),
            ..Default::default()
        };
        let config_path = project_dir.join("Settings/system_screens.bconfig");
        let config_json = serde_json::to_string_pretty(&system_screens)?;
        fs::write(config_path, config_json)?;

        // Save <ProjectName>.blanc
        let manifest_filename = format!("{}.{}", project_name.replace(' ', "_"), Self::MANIFEST_EXTENSION);
        let manifest_path = project_dir.join(manifest_filename);
        let manifest_json = serde_json::to_string_pretty(&project)?;
        fs::write(manifest_path, manifest_json)?;

        Ok(project)
    }

    /// Loads project manifest from a .blanc file
    pub fn load_from_manifest(manifest_path: &Path) -> Result<Self, ProjectError> {
        if !manifest_path.exists() {
            return Err(ProjectError::NotFound(manifest_path.to_path_buf()));
        }
        let content = fs::read_to_string(manifest_path)?;
        let project: Project = serde_json::from_str(&content)?;
        Ok(project)
    }

    /// Saves project manifest back to file
    pub fn save_manifest(&mut self, manifest_path: &Path) -> Result<(), ProjectError> {
        self.updated_at = Utc::now();
        let content = serde_json::to_string_pretty(&self)?;
        atomic_write(manifest_path, content)?;
        Ok(())
    }

    pub fn manifest_path_in_dir(project_dir: &Path, project_name: &str) -> PathBuf {
        let filename = format!("{}.{}", project_name.replace(' ', "_"), Self::MANIFEST_EXTENSION);
        project_dir.join(filename)
    }

    /// Imports an external file into the project's asset directory and registers it
    pub fn import_asset(
        &mut self,
        project_dir: &Path,
        source_path: &Path,
        category: crate::asset::AssetCategory,
    ) -> Result<crate::asset::Asset, ProjectError> {
        if !source_path.exists() {
            return Err(ProjectError::NotFound(source_path.to_path_buf()));
        }

        let extension = source_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        if !AssetRegistry::is_supported_file_extension(source_path) {
            return Err(ProjectError::UnsupportedFormat(format!(
                "File format '.{}' is not supported in Blanc Engine.",
                extension
            )));
        }

        let file_stem = source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("asset");
        let safe_name = file_stem.replace(' ', "_");
        let filename = format!("{}.{}", safe_name, extension);

        let target_folder = project_dir.join(category.subfolder());
        fs::create_dir_all(&target_folder)?;

        let mut target_path = target_folder.join(&filename);
        let mut final_filename = filename.clone();
        let mut counter = 1;
        while target_path.exists() && target_path != source_path {
            final_filename = format!("{}_{}.{}", safe_name, counter, extension);
            target_path = target_folder.join(&final_filename);
            counter += 1;
        }

        if source_path != target_path {
            fs::copy(source_path, &target_path)?;
        }

        let file_size = fs::metadata(&target_path)?.len();
        let relative_path = format!("{}/{}", category.subfolder(), final_filename);
        let asset_id = format!("{}_{}", category.subfolder().replace('/', "_").to_lowercase(), final_filename.replace('.', "_"));

        let asset = crate::asset::Asset::new(
            asset_id,
            safe_name,
            category,
            relative_path,
            file_size,
        );

        self.assets.insert(asset.clone());
        let manifest_path = Self::manifest_path_in_dir(project_dir, &self.name);
        self.save_manifest(&manifest_path)?;

        Ok(asset)
    }

    /// Removes an asset from registry and optionally deletes file from disk
    pub fn delete_asset(
        &mut self,
        project_dir: &Path,
        asset_id: &str,
        delete_file: bool,
    ) -> Result<Option<crate::asset::Asset>, ProjectError> {
        if let Some(removed) = self.assets.remove(asset_id) {
            if delete_file {
                if let Some(file_path) = safe_join_relative(project_dir, &removed.relative_path) {
                    if file_path.exists() && file_path.is_file() {
                        let _ = fs::remove_file(file_path);
                    }
                }
            }
            let manifest_path = Self::manifest_path_in_dir(project_dir, &self.name);
            self.save_manifest(&manifest_path)?;
            Ok(Some(removed))
        } else {
            Ok(None)
        }
    }
}
