use std::collections::{HashSet, VecDeque};
use std::path::Path;

use crate::asset::AssetRegistry;
use crate::config::SystemScreensConfig;
use crate::flow::{FlowNodeType, StoryFlowGraph};
use crate::scene::{ActionBlock, Scene, SceneEndAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationCategory {
    Asset,
    StoryFlow,
    Scene,
    Config,
}

impl ValidationCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Asset => "Asset",
            Self::StoryFlow => "Story Flow",
            Self::Scene => "Scene",
            Self::Config => "Config",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub category: ValidationCategory,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .count()
    }

    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }
}

fn check_asset_ref(
    issues: &mut Vec<ValidationIssue>,
    project_dir: &Path,
    assets: &AssetRegistry,
    asset_id: &str,
    context: &str,
) {
    if asset_id.trim().is_empty() {
        return;
    }
    match assets.get(asset_id) {
        None => {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::Asset,
                title: format!("Missing asset reference ({})", context),
                detail: format!("Asset ID '{}' is not registered in the Asset Registry.", asset_id),
            });
        }
        Some(asset) => {
            let full_path = project_dir.join(&asset.relative_path);
            if !full_path.exists() {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    category: ValidationCategory::Asset,
                    title: format!("Missing asset file ({})", context),
                    detail: format!(
                        "Asset '{}' is missing on disk at '{}'. Relink or remove it.",
                        asset.name, asset.relative_path
                    ),
                });
            }
        }
    }
}

/// Validates a Blanc Engine project before Play Preview or Export.
pub fn validate_project(
    project_dir: &Path,
    assets: &AssetRegistry,
    flow_graph: &StoryFlowGraph,
    scenes: &[Scene],
    config: &SystemScreensConfig,
) -> ValidationReport {
    let mut issues = Vec::new();
    let scene_names: HashSet<&str> = scenes.iter().map(|s| s.name.as_str()).collect();

    if scenes.is_empty() {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Warning,
            category: ValidationCategory::Scene,
            title: "No scenes found".to_string(),
            detail: "Create at least one scene so players have playable content.".to_string(),
        });
    }

    // 1. Validate SystemScreensConfig
    if let Some(start_scene) = &config.starting_scene_id {
        if !start_scene.trim().is_empty() && !scene_names.is_empty() && !scene_names.contains(start_scene.as_str()) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::Config,
                title: "Invalid starting scene".to_string(),
                detail: format!(
                    "Settings reference starting scene '{}', which does not exist.",
                    start_scene
                ),
            });
        }
    }
    if let Some(id) = &config.title_background_asset_id {
        check_asset_ref(&mut issues, project_dir, assets, id, "Title Screen Background");
    }
    if let Some(id) = &config.title_bgm_asset_id {
        check_asset_ref(&mut issues, project_dir, assets, id, "Title Screen BGM");
    }
    if let Some(id) = &config.logo_asset_id {
        check_asset_ref(&mut issues, project_dir, assets, id, "Title Screen Logo");
    }

    // 2. Validate Story Flow Graph
    let node_ids: HashSet<&str> = flow_graph.nodes.iter().map(|n| n.id.as_str()).collect();
    let start_nodes: Vec<&str> = flow_graph
        .nodes
        .iter()
        .filter(|n| matches!(n.node_type, FlowNodeType::MainMenu))
        .map(|n| n.id.as_str())
        .collect();

    if start_nodes.is_empty() {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            category: ValidationCategory::StoryFlow,
            title: "Missing Main Menu (Start) node".to_string(),
            detail: "Story Flow must have a Main Menu node as the entry point.".to_string(),
        });
    }

    for conn in &flow_graph.connections {
        if !node_ids.contains(conn.from_node_id.as_str()) || !node_ids.contains(conn.to_node_id.as_str()) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::StoryFlow,
                title: "Broken Story Flow connection".to_string(),
                detail: format!(
                    "Connection '{}' points to a deleted node ({} -> {}).",
                    conn.id, conn.from_node_id, conn.to_node_id
                ),
            });
        }
    }

    for node in &flow_graph.nodes {
        if let FlowNodeType::Scene { scene_name } = &node.node_type {
            if !scene_names.is_empty() && !scene_names.contains(scene_name.as_str()) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    category: ValidationCategory::StoryFlow,
                    title: format!("Missing scene in Story Flow ('{}')", node.title),
                    detail: format!(
                        "Flow node '{}' references scene '{}', which was not found.",
                        node.title, scene_name
                    ),
                });
            }
        }
    }

    // Reachability check (Warning)
    if !start_nodes.is_empty() {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        for s in start_nodes {
            if visited.insert(s) {
                queue.push_back(s);
            }
        }
        while let Some(curr) = queue.pop_front() {
            for conn in &flow_graph.connections {
                if conn.from_node_id == curr && node_ids.contains(conn.to_node_id.as_str()) {
                    if visited.insert(conn.to_node_id.as_str()) {
                        queue.push_back(conn.to_node_id.as_str());
                    }
                }
            }
        }
        for node in &flow_graph.nodes {
            if !visited.contains(node.id.as_str()) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: ValidationCategory::StoryFlow,
                    title: format!("Unreachable node '{}'", node.title),
                    detail: "This node is not connected from Main Menu.".to_string(),
                });
            }
        }
    }

    // 3. Validate Scenes & Referenced Assets
    for scene in scenes {
        if scene.blocks.is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: ValidationCategory::Scene,
                title: format!("Empty scene '{}'", scene.name),
                detail: "This scene has no action blocks yet.".to_string(),
            });
        }

        for (idx, block) in scene.blocks.iter().enumerate() {
            let ctx = format!("Scene '{}' #{}", scene.name, idx + 1);
            match block {
                ActionBlock::Background { asset_id, .. } => {
                    check_asset_ref(&mut issues, project_dir, assets, asset_id, &ctx);
                }
                ActionBlock::Character { sprite_asset_id, .. } => {
                    if let Some(id) = sprite_asset_id {
                        check_asset_ref(&mut issues, project_dir, assets, id, &ctx);
                    }
                }
                ActionBlock::Dialogue { text, voice_asset_id, .. } => {
                    if let Some(id) = voice_asset_id {
                        check_asset_ref(&mut issues, project_dir, assets, id, &ctx);
                    }
                    if text.trim().is_empty() {
                        issues.push(ValidationIssue {
                            severity: ValidationSeverity::Warning,
                            category: ValidationCategory::Scene,
                            title: format!("Empty dialogue ({})", ctx),
                            detail: "Dialogue block has empty text.".to_string(),
                        });
                    }
                }
                ActionBlock::PlayBgm { asset_id, .. } | ActionBlock::PlaySfx { asset_id, .. } => {
                    check_asset_ref(&mut issues, project_dir, assets, asset_id, &ctx);
                }
                ActionBlock::Choice { options, .. } => {
                    for opt in options {
                        if !opt.target_scene_id.trim().is_empty()
                            && !scene_names.contains(opt.target_scene_id.as_str())
                        {
                            issues.push(ValidationIssue {
                                severity: ValidationSeverity::Error,
                                category: ValidationCategory::Scene,
                                title: format!("Broken choice target ({})", ctx),
                                detail: format!(
                                    "Choice '{}' jumps to missing scene '{}'.",
                                    opt.text, opt.target_scene_id
                                ),
                            });
                        }
                    }
                }
                ActionBlock::SceneEnd { action } => {
                    if let SceneEndAction::JumpToScene(scene_name) = action {
                        if !scene_name.trim().is_empty() && !scene_names.contains(scene_name.as_str()) {
                            issues.push(ValidationIssue {
                                severity: ValidationSeverity::Error,
                                category: ValidationCategory::Scene,
                                title: format!("Broken SceneEnd target ({})", ctx),
                                detail: format!(
                                    "SceneEnd jumps to missing scene '{}'.",
                                    scene_name
                                ),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ValidationReport { issues }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset::{Asset, AssetCategory};
    use crate::flow::{FlowConnection, FlowNode};
    use std::fs;

    fn make_temp_dir(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("blanc_validator_{}_{}", tag, nanos));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn validate_clean_project_passes() {
        let dir = make_temp_dir("clean");
        let assets_dir = dir.join("Assets/Backgrounds");
        fs::create_dir_all(&assets_dir).unwrap();
        fs::write(assets_dir.join("bg.png"), b"png").unwrap();

        let mut assets = AssetRegistry::new();
        assets.insert(Asset::new(
            "bg1",
            "bg.png",
            AssetCategory::Background,
            "Assets/Backgrounds/bg.png",
            3,
        ));

        let mut scene = Scene::new("scene_1", "ch1");
        scene.blocks.push(ActionBlock::Background {
            asset_id: "bg1".to_string(),
            transition: None,
        });
        scene.blocks.push(ActionBlock::Dialogue {
            speaker: "Alice".to_string(),
            text: "Hello!".to_string(),
            voice_asset_id: None,
            nameplate_position: None,
        });

        let flow = StoryFlowGraph::starter_graph(Some("scene_1"));
        let config = SystemScreensConfig::default();

        let report = validate_project(&dir, &assets, &flow, &[scene], &config);
        assert!(!report.has_errors(), "unexpected errors: {:?}", report.issues);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn validate_detects_missing_asset_and_broken_link() {
        let dir = make_temp_dir("broken");
        let mut assets = AssetRegistry::new();
        assets.insert(Asset::new(
            "bg_missing",
            "gone.png",
            AssetCategory::Background,
            "Assets/Backgrounds/gone.png",
            10,
        ));

        let mut scene = Scene::new("scene_1", "ch1");
        scene.blocks.push(ActionBlock::Background {
            asset_id: "bg_missing".to_string(),
            transition: None,
        });

        let mut flow = StoryFlowGraph::default();
        flow.nodes = vec![FlowNode::scene("scene_1", 100.0, 100.0)]; // no MainMenu!
        flow.connections.push(FlowConnection::new("scene_scene_1", "out", "deleted_node", "in"));

        let config = SystemScreensConfig::default();
        let report = validate_project(&dir, &assets, &flow, &[scene], &config);

        assert!(report.has_errors());
        assert!(report.error_count() >= 3);
        let _ = fs::remove_dir_all(&dir);
    }
}
