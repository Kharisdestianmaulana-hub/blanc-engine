//! Blanc Engine Core Library
//!
//! Provides the fundamental data structures, persistence, validation,
//! and design tokens for Blanc Engine projects.

pub mod asset;
pub mod config;
pub mod flow;
pub mod installer;
pub mod localization;
pub mod pack;
pub mod win_icon;
pub mod project;
pub mod scene;
pub mod story;
pub mod theme;
pub mod ui_config;
pub mod validator;

pub use asset::{Asset, AssetCategory, AssetRegistry};
pub use config::SystemScreensConfig;
pub use flow::{FlowConnection, FlowNode, FlowNodeType, FlowPin, StoryFlowGraph};
pub use project::Project;
pub use scene::{ActionBlock, Scene, SceneNavigation};
pub use story::Story;
pub use ui_config::RuntimeUiConfig;
