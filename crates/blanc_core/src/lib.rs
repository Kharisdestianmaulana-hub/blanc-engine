//! Blanc Engine Core Library
//!
//! Provides the fundamental data structures, persistence, validation,
//! and design tokens for Blanc Engine projects.

pub mod asset;
pub mod config;
pub mod project;
pub mod scene;
pub mod story;
pub mod theme;

pub use asset::{Asset, AssetCategory, AssetRegistry};
pub use config::SystemScreensConfig;
pub use project::Project;
pub use scene::{ActionBlock, Scene, SceneNavigation};
pub use story::Story;
