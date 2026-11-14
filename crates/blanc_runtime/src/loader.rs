use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use blanc_core::asset::AssetCategory;
use blanc_core::config::SystemScreensConfig;
use blanc_core::flow::StoryFlowGraph;
use blanc_core::localization::{LocaleManifest, LocaleStrings, LOCALES_DIR, LOCALE_MANIFEST_FILE};
use blanc_core::pack::{BpackArchive, ExportMetadata, ExportSaveMode, EXPORT_META_ENTRY};
use blanc_core::project::Project;
use blanc_core::scene::Scene;

/// Loaded runtime bundle for a Blanc Engine project
#[derive(Debug, Clone)]
pub struct LoadedProject {
    pub project_dir: PathBuf,
    pub save_dir: PathBuf,
    pub pack: Option<Arc<BpackArchive>>,
    pub project: Project,
    pub config: SystemScreensConfig,
    pub flow_graph: Option<StoryFlowGraph>,
    /// Scenes indexed by both scene.name and scene.id
    pub scenes: HashMap<String, Scene>,
    pub scene_order: Vec<String>,
    pub locale_manifest: LocaleManifest,
    pub locale_tables: HashMap<String, LocaleStrings>,
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

        // Load all .bscene files recursively from Scenes/ (e.g. Scenes/Chapter_01/*.bscene)
        let mut scenes = HashMap::new();
        let mut scene_order = Vec::new();
        let scenes_dir = project_dir.join("Scenes");
        if scenes_dir.exists() {
            let mut paths = Vec::new();
            collect_bscene_files(&scenes_dir, &mut paths);
            paths.sort();

            for path in paths {
                if let Ok(scene) = Scene::load_from_file(&path) {
                    scene_order.push(scene.name.clone());
                    scenes.insert(scene.id.clone(), scene.clone());
                    scenes.insert(scene.name.clone(), scene);
                }
            }
        }

        let export_meta: Option<ExportMetadata> = std::fs::read_to_string(project_dir.join(EXPORT_META_ENTRY))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok());
        let save_dir = resolve_save_dir(project_dir, &project.name, export_meta.as_ref());
        let locale_manifest = LocaleManifest::load_from_dir(project_dir);
        let mut locale_tables = HashMap::new();
        for lang in locale_manifest.all_languages() {
            let strings = LocaleStrings::load_from_dir(project_dir, &lang.code);
            locale_tables.insert(lang.code.to_ascii_lowercase(), strings);
        }

        Ok(Self {
            project_dir: project_dir.to_path_buf(),
            save_dir,
            pack: None,
            project,
            config,
            flow_graph,
            scenes,
            scene_order,
            locale_manifest,
            locale_tables,
        })
    }

    /// Load a project from an encrypted `game.bpack` archive next to the runtime executable
    pub fn load_from_bpack(exe_dir: &Path, bpack_path: &Path) -> Result<Self, String> {
        let archive = Arc::new(BpackArchive::open(bpack_path)?);
        let files = archive.list_files();

        let manifest_rel = files
            .iter()
            .find(|p| p.ends_with(".blanc") && !p.contains('/'))
            .copied()
            .ok_or_else(|| "No .blanc manifest found inside game.bpack".to_string())?;

        let manifest_bytes = archive
            .read_file(manifest_rel)
            .ok_or_else(|| "Failed to read .blanc manifest from game.bpack".to_string())?;
        let project: Project = serde_json::from_slice(&manifest_bytes)
            .map_err(|e| format!("Failed to parse project manifest from pack: {}", e))?;

        let mut config = archive
            .read_file("Settings/system_screens.bconfig")
            .and_then(|b| serde_json::from_slice::<SystemScreensConfig>(&b).ok())
            .unwrap_or_else(|| {
                let mut cfg = SystemScreensConfig::default();
                cfg.title_text = project.name.clone();
                cfg
            });
        if config.title_text.trim().is_empty() {
            config.title_text = project.name.clone();
        }
        if config.starting_scene_id.is_none() {
            config.starting_scene_id = project.starting_scene_id.clone();
        }

        let flow_graph = archive
            .read_file("Story/story_flow.bgraph")
            .and_then(|b| serde_json::from_slice::<StoryFlowGraph>(&b).ok());

        let mut scene_paths: Vec<String> = files
            .iter()
            .filter(|p| p.starts_with("Scenes/") && p.ends_with(".bscene"))
            .map(|p| p.to_string())
            .collect();
        scene_paths.sort();

        let mut scenes = HashMap::new();
        let mut scene_order = Vec::new();
        for sp in scene_paths {
            if let Some(bytes) = archive.read_file(&sp) {
                if let Ok(scene) = serde_json::from_slice::<Scene>(&bytes) {
                    scene_order.push(scene.name.clone());
                    scenes.insert(scene.id.clone(), scene.clone());
                    scenes.insert(scene.name.clone(), scene);
                }
            }
        }

        let export_meta: Option<ExportMetadata> = archive
            .read_file(EXPORT_META_ENTRY)
            .and_then(|b| serde_json::from_slice(&b).ok());
        let save_dir = resolve_save_dir(exe_dir, &project.name, export_meta.as_ref());
        let manifest_rel = format!("{}/{}", LOCALES_DIR, LOCALE_MANIFEST_FILE);
        let locale_manifest = archive
            .read_file(&manifest_rel)
            .and_then(|b| serde_json::from_slice::<LocaleManifest>(&b).ok())
            .unwrap_or_default();
        let mut locale_tables = HashMap::new();
        for lang in locale_manifest.all_languages() {
            let rel = LocaleStrings::file_rel_path(&lang.code);
            let strings = archive
                .read_file(&rel)
                .and_then(|b| serde_json::from_slice::<LocaleStrings>(&b).ok())
                .unwrap_or_default();
            locale_tables.insert(lang.code.to_ascii_lowercase(), strings);
        }

        Ok(Self {
            project_dir: exe_dir.to_path_buf(),
            save_dir,
            pack: Some(archive),
            project,
            config,
            flow_graph,
            scenes,
            scene_order,
            locale_manifest,
            locale_tables,
        })
    }

    pub fn asset_exists(&self, rel_path: &str) -> bool {
        if let Some(pack) = &self.pack {
            pack.read_file(rel_path).is_some()
        } else {
            let full = self.project_dir.join(rel_path);
            full.exists() && full.is_file()
        }
    }

    pub fn read_path_bytes(&self, path: &Path) -> Option<Vec<u8>> {
        if let Some(pack) = &self.pack {
            let rel = path
                .strip_prefix(&self.project_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            pack.read_file(&rel)
        } else {
            std::fs::read(path).ok()
        }
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
            if self.asset_exists(&asset.relative_path) {
                return Some(self.project_dir.join(&asset.relative_path));
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
            if (name_lower == needle || stem_lower == needle) && self.asset_exists(&asset.relative_path) {
                return Some(self.project_dir.join(&asset.relative_path));
            }
        }

        // 3. Direct relative path check in project_dir / pack
        if self.asset_exists(asset_ref) {
            return Some(self.project_dir.join(asset_ref));
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
                if name_lower.starts_with(&char_lower) && self.asset_exists(&asset.relative_path) {
                    return Some(self.project_dir.join(&asset.relative_path));
                }
            }
        }

        None
    }

    /// Resolve custom project logo path if enabled and present
    pub fn resolve_custom_logo(&self) -> Option<PathBuf> {
        if let Some(logo_id) = self.config.logo_asset_id.as_deref() {
            if let Some(path) = self.resolve_asset_path(logo_id, Some(AssetCategory::Ui)) {
                return Some(path);
            }
        }
        if !self.project.use_custom_logo {
            return None;
        }
        let rel = self.project.custom_logo_path.as_ref()?;
        if self.asset_exists(rel) {
            return Some(self.project_dir.join(rel));
        }
        let abs = PathBuf::from(rel);
        if abs.is_absolute() && abs.exists() {
            Some(abs)
        } else {
            None
        }
    }
}

fn resolve_save_dir(base_dir: &Path, project_name: &str, meta: Option<&ExportMetadata>) -> PathBuf {
    if let Some(m) = meta {
        if m.save_mode == ExportSaveMode::AppData {
            let safe_title: String = m
                .game_title
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                .collect();
            let folder = if safe_title.trim_matches('_').is_empty() {
                let fallback: String = project_name
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                    .collect();
                if fallback.is_empty() { "BlancGame".to_string() } else { fallback }
            } else {
                safe_title
            };
            if let Ok(appdata) = std::env::var("APPDATA") {
                return PathBuf::from(appdata).join("BlancGames").join(folder).join("Saves");
            }
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(".local").join("share").join("BlancGames").join(folder).join("Saves");
            }
        }
    }
    base_dir.join("Saves")
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

fn collect_bscene_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_bscene_files(&path, out);
        } else if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some(Scene::EXTENSION)
        {
            out.push(path);
        }
    }
}
