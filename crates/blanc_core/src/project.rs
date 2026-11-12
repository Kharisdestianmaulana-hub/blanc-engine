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

        // Create default initial scene
        let initial_scene = Scene::new("Opening", "Chapter_01");
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
}
