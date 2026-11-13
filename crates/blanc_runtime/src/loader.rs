use std::collections::HashMap;
use std::path::{Path, PathBuf};

use blanc_core::asset::AssetCategory;
use blanc_core::config::SystemScreensConfig;
use blanc_core::flow::StoryFlowGraph;
use blanc_core::project::Project;
use blanc_core::scene::Scene;

/// Loaded runtime bundle for a Blanc Engine project
#[derive(Debug, Clone)]
pub struct LoadedProject {
    pub project_dir: PathBuf,
    pub project: Project,
    pub config: SystemScreensConfig,
    pub flow_graph: Option<StoryFlowGraph>,
    /// Scenes indexed by both scene.name and scene.id
    pub scenes: HashMap<String, Scene>,
    pub scene_order: Vec<String>,
}

impl LoadedProject {
    /// Load a project from its root directory
    pub fn load_from_dir(project_dir: &Path) -> Result<Self, String> {
        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(format!("Project directory does not exist: {}", project_dir.display()));
        }

        // Find the .blanc manifest file in project_dir
        let manifest_path = find_manifest_file(project_dir)
            .ok_or_else(|| format!("No .blanc project manifest found in {}", project_dir.display()))?;

        let project = Project::load_from_manifest(&manifest_path)
            .map_err(|e| format!("Failed to load project manifest: {}", e))?;

        // Load SystemScreensConfig
        let config_path = project_dir.join("Settings").join("system_screens.bconfig");
        let mut config = if config_path.exists() {
            SystemScreensConfig::load_from_file(&config_path).unwrap_or_default()
        } else {
            let mut cfg = SystemScreensConfig::default();
            cfg.title_text = project.name.clone();
            cfg
        };
        if config.title_text.trim().is_empty() {
            config.title_text = project.name.clone();
        }
        if config.starting_scene_id.is_none() {
            config.starting_scene_id = project.starting_scene_id.clone();
        }

        // Load StoryFlowGraph if present
        let flow_path = project_dir.join("Story").join("story_flow.bgraph");
        let flow_graph = if flow_path.exists() {
            StoryFlowGraph::load_from_file(&flow_path).ok()
        } else {
            None
        };

        // Load all .bscene files from Scenes/
        let mut scenes = HashMap::new();
        let mut scene_order = Vec::new();
        let scenes_dir = project_dir.join("Scenes");
        if scenes_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
                let mut paths: Vec<PathBuf> = entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().and_then(|s| s.to_str()) == Some(Scene::EXTENSION))
                    .collect();
                paths.sort();

                for path in paths {
                    if let Ok(scene) = Scene::load_from_file(&path) {
                        scene_order.push(scene.name.clone());
                        scenes.insert(scene.id.clone(), scene.clone());
                        scenes.insert(scene.name.clone(), scene);
                    }
                }
            }
        }

        Ok(Self {
            project_dir: project_dir.to_path_buf(),
            project,
            config,
            flow_graph,
            scenes,
            scene_order,
        })
    }

    /// Resolve a scene by name or ID
    pub fn get_scene(&self, name_or_id: &str) -> Option<&Scene> {
        self.scenes.get(name_or_id)
    }

    /// Determine the starting scene name for "New Game"
    pub fn resolve_starting_scene(&self) -> Option<String> {
        // 1. Check StoryFlowGraph connection from system_main_menu -> new_game
        if let Some(graph) = &self.flow_graph {
            if let Some(conn) = graph
                .connections
                .iter()
                .find(|c| c.from_node_id == "system_main_menu" && c.from_pin_id == "new_game")
            {
                if let Some(node) = graph.find_node(&conn.to_node_id) {
                    if let Some(scene_name) = node.scene_name() {
                        if self.scenes.contains_key(scene_name) {
                            return Some(scene_name.to_string());
                        }
                    }
                }
            }
        }

        // 2. Check config.starting_scene_id
        if let Some(start) = &self.config.starting_scene_id {
            if let Some(scene) = self.scenes.get(start) {
                return Some(scene.name.clone());
            }
        }

        // 3. Fallback to first scene alphabetically
        self.scene_order.first().cloned()
    }

    /// Resolve an asset file path by asset_id, name, or partial match within a category
    pub fn resolve_asset_path(&self, asset_ref: &str, category_hint: Option<AssetCategory>) -> Option<PathBuf> {
        if asset_ref.trim().is_empty() {
            return None;
        }

        // 1. Exact ID lookup in AssetRegistry
        if let Some(asset) = self.project.assets.get(asset_ref) {
            let full = self.project_dir.join(&asset.relative_path);
            if full.exists() {
                return Some(full);
            }
        }

        // 2. Match by asset name (case-insensitive, with or without extension)
        let needle = asset_ref.trim().to_lowercase();
        for asset in self.project.assets.assets.values() {
            if let Some(cat) = category_hint {
                if asset.category != cat
                    && !(cat == AssetCategory::Character && asset.category == AssetCategory::CharacterExpression)
                {
                    continue;
                }
            }
            let name_lower = asset.name.to_lowercase();
            let stem_lower = Path::new(&asset.name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            if name_lower == needle || stem_lower == needle {
                let full = self.project_dir.join(&asset.relative_path);
                if full.exists() {
                    return Some(full);
                }
            }
        }

        // 3. Direct relative path check in project_dir
        let direct = self.project_dir.join(asset_ref);
        if direct.exists() && direct.is_file() {
            return Some(direct);
        }

        None
    }

    /// Resolve a character sprite path using character_id and expression
    pub fn resolve_character_path(&self, character_id: &str, expression: &str) -> Option<PathBuf> {
        let char_lower = character_id.trim().to_lowercase();
        let expr_lower = expression.trim().to_lowercase();

        // Try combined "<character>_<expression>" or "<character> (<expression>)"
        if !expr_lower.is_empty() && expr_lower != "normal" && expr_lower != "default" {
            let combined_underscore = format!("{}_{}", char_lower, expr_lower);
            if let Some(p) = self.resolve_asset_path(&combined_underscore, Some(AssetCategory::Character)) {
                return Some(p);
            }
        }

        // Try exact character_id match
        if let Some(p) = self.resolve_asset_path(character_id, Some(AssetCategory::Character)) {
            return Some(p);
        }

        // Fuzzy fallback: any character asset whose name starts with character_id
        for asset in self.project.assets.assets.values() {
            if asset.category == AssetCategory::Character
                || asset.category == AssetCategory::CharacterExpression
            {
                let name_lower = asset.name.to_lowercase();
                if name_lower.starts_with(&char_lower) {
                    let full = self.project_dir.join(&asset.relative_path);
                    if full.exists() {
                        return Some(full);
                    }
                }
            }
        }

        None
    }

    /// Resolve custom project logo path if enabled and present
    pub fn resolve_custom_logo(&self) -> Option<PathBuf> {
        if !self.project.use_custom_logo {
            return None;
        }
        let rel = self.project.custom_logo_path.as_ref()?;
        let abs = if Path::new(rel).is_absolute() {
            PathBuf::from(rel)
        } else {
            self.project_dir.join(rel)
        };
        if abs.exists() {
            Some(abs)
        } else {
            None
        }
    }
}

fn find_manifest_file(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some(Project::MANIFEST_EXTENSION)
        {
            return Some(path);
        }
    }
    None
}
