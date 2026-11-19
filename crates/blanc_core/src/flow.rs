use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of node in the story flowchart
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FlowNodeType {
    MainMenu,
    Scene { scene_name: String },
    PauseMenu,
    Settings,
    SaveLoad,
    Ending { ending_name: String },
}

/// A pin on a flowchart node for connecting narrative paths
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowPin {
    pub id: String,
    pub label: String,
    pub is_output: bool,
}

/// A node positioned on the Story Flow 2D canvas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub node_type: FlowNodeType,
    pub title: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl FlowNode {
    pub fn main_menu(x: f32, y: f32) -> Self {
        Self {
            id: "system_main_menu".to_string(),
            node_type: FlowNodeType::MainMenu,
            title: "Main Menu (Title)".to_string(),
            x,
            y,
            width: 220.0,
            height: 110.0,
        }
    }

    pub fn scene(scene_name: &str, x: f32, y: f32) -> Self {
        Self {
            id: format!("scene_{}", scene_name),
            node_type: FlowNodeType::Scene {
                scene_name: scene_name.to_string(),
            },
            title: scene_name.to_string(),
            x,
            y,
            width: 240.0,
            height: 130.0,
        }
    }

    pub fn pause_menu(x: f32, y: f32) -> Self {
        Self {
            id: "system_pause_menu".to_string(),
            node_type: FlowNodeType::PauseMenu,
            title: "In-Game Menu (Pause)".to_string(),
            x,
            y,
            width: 250.0,
            height: 140.0,
        }
    }

    pub fn settings(x: f32, y: f32) -> Self {
        Self {
            id: "system_settings".to_string(),
            node_type: FlowNodeType::Settings,
            title: "In-Game Settings".to_string(),
            x,
            y,
            width: 240.0,
            height: 110.0,
        }
    }

    pub fn save_load(x: f32, y: f32) -> Self {
        Self {
            id: "system_save_load".to_string(),
            node_type: FlowNodeType::SaveLoad,
            title: "Save & Load Screen".to_string(),
            x,
            y,
            width: 240.0,
            height: 110.0,
        }
    }

    pub fn ending(id: &str, title: &str, x: f32, y: f32) -> Self {
        Self {
            id: id.to_string(),
            node_type: FlowNodeType::Ending {
                ending_name: title.to_string(),
            },
            title: title.to_string(),
            x,
            y,
            width: 200.0,
            height: 100.0,
        }
    }

    /// Extract scene name if this is a scene node
    pub fn scene_name(&self) -> Option<&str> {
        match &self.node_type {
            FlowNodeType::Scene { scene_name } => Some(scene_name.as_str()),
            _ => None,
        }
    }
}

/// Directed connection (wire) between two pins on the flowchart
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowConnection {
    pub id: String,
    pub from_node_id: String,
    pub from_pin_id: String,
    pub to_node_id: String,
    pub to_pin_id: String,
}

impl FlowConnection {
    pub fn new(
        from_node_id: impl Into<String>,
        from_pin_id: impl Into<String>,
        to_node_id: impl Into<String>,
        to_pin_id: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_node_id: from_node_id.into(),
            from_pin_id: from_pin_id.into(),
            to_node_id: to_node_id.into(),
            to_pin_id: to_pin_id.into(),
        }
    }
}

/// The entire visual narrative graph stored in `Story/story_flow.bgraph`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryFlowGraph {
    pub nodes: Vec<FlowNode>,
    pub connections: Vec<FlowConnection>,
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom: f32,
}

impl Default for StoryFlowGraph {
    fn default() -> Self {
        Self {
            nodes: vec![
                FlowNode::main_menu(60.0, 160.0),
                FlowNode::ending("ending_return_to_title", "Return to Title", 720.0, 160.0),
            ],
            connections: Vec::new(),
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
        }
    }
}

impl StoryFlowGraph {
    pub const EXTENSION: &'static str = "bgraph";

    /// Create default starter flow graph with initial starter scene connected
    pub fn starter_graph(starter_scene: Option<&str>) -> Self {
        let mut graph = Self {
            nodes: vec![
                FlowNode::main_menu(60.0, 160.0),
            ],
            connections: Vec::new(),
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
        };

        let ending_node = FlowNode::ending("ending_return_to_title", "Return to Title", 760.0, 160.0);

        if let Some(name) = starter_scene {
            let scene_node = FlowNode::scene(name, 380.0, 160.0);
            let conn1 = FlowConnection::new(
                "system_main_menu",
                "new_game",
                &scene_node.id,
                "in",
            );
            let conn2 = FlowConnection::new(
                &scene_node.id,
                "next",
                &ending_node.id,
                "in",
            );
            graph.nodes.push(scene_node);
            graph.connections.push(conn1);
            graph.connections.push(conn2);
        }

        graph.nodes.push(ending_node);
        graph
    }

    /// Add a scene node if it doesn't already exist on the canvas
    pub fn add_scene_node(&mut self, scene_name: &str, x: f32, y: f32) -> bool {
        let node_id = format!("scene_{}", scene_name);
        if self.nodes.iter().any(|n| n.id == node_id) {
            return false;
        }
        self.nodes.push(FlowNode::scene(scene_name, x, y));
        true
    }

    /// Add a system node (PauseMenu, Settings, SaveLoad, Ending) to the canvas
    pub fn add_system_node(&mut self, system_type: &str, x: f32, y: f32) -> bool {
        let node = match system_type {
            "pause_menu" => {
                if self.nodes.iter().any(|n| n.id == "system_pause_menu") {
                    return false;
                }
                FlowNode::pause_menu(x, y)
            }
            "settings" => {
                if self.nodes.iter().any(|n| n.id == "system_settings") {
                    return false;
                }
                FlowNode::settings(x, y)
            }
            "save_load" => {
                if self.nodes.iter().any(|n| n.id == "system_save_load") {
                    return false;
                }
                FlowNode::save_load(x, y)
            }
            "return_to_title" => {
                if self.nodes.iter().any(|n| n.id == "ending_return_to_title") {
                    return false;
                }
                FlowNode::ending("ending_return_to_title", "Return to Title", x, y)
            }
            "credits" => {
                let id = format!("ending_credits_{}", Uuid::new_v4().simple());
                FlowNode::ending(&id, "Credits Roll", x, y)
            }
            "game_over" => {
                let id = format!("ending_game_over_{}", Uuid::new_v4().simple());
                FlowNode::ending(&id, "Game Over", x, y)
            }
            _ => return false,
        };
        self.nodes.push(node);
        true
    }

    /// Remove a node and any connections linked to it
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        // Prevent deleting main menu
        if node_id == "system_main_menu" {
            return false;
        }
        let initial_len = self.nodes.len();
        self.nodes.retain(|n| n.id != node_id);
        self.connections.retain(|c| c.from_node_id != node_id && c.to_node_id != node_id);
        self.nodes.len() < initial_len
    }

    /// Connect two pins. Replaces any existing connection starting from `from_node_id` + `from_pin_id`
    pub fn connect_pins(
        &mut self,
        from_node_id: &str,
        from_pin_id: &str,
        to_node_id: &str,
        to_pin_id: &str,
    ) {
        // Remove existing connection from the same output pin (single source output)
        self.connections.retain(|c| !(c.from_node_id == from_node_id && c.from_pin_id == from_pin_id));

        self.connections.push(FlowConnection::new(
            from_node_id,
            from_pin_id,
            to_node_id,
            to_pin_id,
        ));
    }

    /// Disconnect an output pin
    pub fn disconnect_pin(&mut self, from_node_id: &str, from_pin_id: &str) {
        self.connections.retain(|c| !(c.from_node_id == from_node_id && c.from_pin_id == from_pin_id));
    }

    /// Find node by id
    pub fn find_node(&self, node_id: &str) -> Option<&FlowNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Find mutable node by id
    pub fn find_node_mut(&mut self, node_id: &str) -> Option<&mut FlowNode> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    /// Save graph to JSON file
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        crate::project::atomic_write(path, json)?;
        Ok(())
    }

    /// Load graph from JSON file
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let graph: Self = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(graph)
    }
}
