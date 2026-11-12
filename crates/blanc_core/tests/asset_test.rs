use std::fs;
use blanc_core::asset::AssetCategory;
use blanc_core::project::Project;

#[test]
fn test_asset_import_and_delete() {
    let temp_dir = std::env::temp_dir().join(format!("blanc_test_asset_{}", uuid::Uuid::new_v4()));
    let project = Project::create_project_structure(&temp_dir, "AssetTest", "Author").unwrap();
    assert_eq!(project.assets.assets.len(), 0);

    // Create a dummy image file to import
    let dummy_source = temp_dir.join("sample_bg.png");
    fs::write(&dummy_source, b"fake png content").unwrap();

    let mut loaded_project = Project::load_from_manifest(
        &Project::manifest_path_in_dir(&temp_dir, "AssetTest"),
    ).unwrap();

    // Import the asset into Backgrounds category
    let imported = loaded_project
        .import_asset(&temp_dir, &dummy_source, AssetCategory::Background)
        .unwrap();

    assert_eq!(imported.name, "sample_bg");
    assert_eq!(imported.category, AssetCategory::Background);
    assert_eq!(imported.relative_path, "Assets/Backgrounds/sample_bg.png");
    assert!(temp_dir.join("Assets/Backgrounds/sample_bg.png").exists());

    // Verify manifest persisted the asset
    let reloaded = Project::load_from_manifest(
        &Project::manifest_path_in_dir(&temp_dir, "AssetTest"),
    ).unwrap();
    assert_eq!(reloaded.assets.assets.len(), 1);
    assert!(reloaded.assets.get(&imported.id).is_some());

    // Delete asset
    let removed = loaded_project
        .delete_asset(&temp_dir, &imported.id, true)
        .unwrap();
    assert!(removed.is_some());
    assert!(!temp_dir.join("Assets/Backgrounds/sample_bg.png").exists());

    // Cleanup
    let _ = fs::remove_dir_all(temp_dir);
}
