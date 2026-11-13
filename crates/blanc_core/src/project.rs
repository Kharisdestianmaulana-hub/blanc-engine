use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::asset::AssetRegistry;
use crate::config::SystemScreensConfig;
use crate::scene::Scene;

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
        fs::write(manifest_path, content)?;
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
                let file_path = project_dir.join(&removed.relative_path);
                if file_path.exists() {
                    let _ = fs::remove_file(file_path);
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
