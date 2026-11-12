use std::fs;
use blanc_core::project::Project;
use blanc_core::story::Story;

#[test]
fn test_project_creation_and_manifest() {
    let temp_dir = std::env::temp_dir().join("blanc_test_project");
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }

    let project = Project::create_project_structure(&temp_dir, "My Story", "Author Name")
        .expect("Failed to create project structure");

    assert_eq!(project.name, "My Story");
    assert_eq!(project.author, "Author Name");
    assert!(project.starting_scene_id.is_some());

    // Verify files on disk
    let manifest_file = temp_dir.join("My_Story.blanc");
    assert!(manifest_file.exists(), "Manifest file must exist");

    let scene_file = temp_dir.join("Scenes/Chapter_01/01_opening.bscene");
    assert!(scene_file.exists(), "Opening scene file must exist");

    let config_file = temp_dir.join("Settings/system_screens.bconfig");
    assert!(config_file.exists(), "System screens config file must exist");

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_story_script_parsing() {
    let script = r#"
        Matahari pagi bersinar hangat.
        Alice (Happy): "Halo semuanya!"
        Bob: "Selamat pagi, Alice."
    "#;

    let elements = Story::parse_script(script);
    assert_eq!(elements.len(), 3);

    // First line should be Narration
    match &elements[0] {
        blanc_core::story::StoryElement::Narration(n) => {
            assert_eq!(n.text, "Matahari pagi bersinar hangat.");
        }
        _ => panic!("Expected narration"),
    }

    // Second line should be Dialogue with expression
    match &elements[1] {
        blanc_core::story::StoryElement::Dialogue(d) => {
            assert_eq!(d.speaker, "Alice");
            assert_eq!(d.expression.as_deref(), Some("Happy"));
            assert_eq!(d.text, "Halo semuanya!");
        }
        _ => panic!("Expected dialogue"),
    }
}

#[test]
fn test_advanced_script_parsing() {
    let script = r#"
        [Scene: Classroom]
        [BGM: peaceful_afternoon]
        Alice: "Ready?"
        [Choice: What to do?
          - "Study" -> Scene_Library
          - "Play" -> Scene_Courtyard
        ]
    "#;

    let elements = Story::parse_script(script);
    assert_eq!(elements.len(), 4);

    match &elements[0] {
        blanc_core::story::StoryElement::Scene(s) => assert_eq!(s.scene_name, "Classroom"),
        _ => panic!("Expected scene marker"),
    }

    match &elements[1] {
        blanc_core::story::StoryElement::Audio(a) => {
            assert!(a.is_bgm);
            assert_eq!(a.track_name, "peaceful_afternoon");
        }
        _ => panic!("Expected audio marker"),
    }

    match &elements[3] {
        blanc_core::story::StoryElement::Choice(c) => {
            assert_eq!(c.options.len(), 2);
            assert_eq!(c.options[0].text, "Study");
            assert_eq!(c.options[0].target_scene, "Scene_Library");
            assert_eq!(c.options[1].text, "Play");
            assert_eq!(c.options[1].target_scene, "Scene_Courtyard");
        }
        _ => panic!("Expected choice branch"),
    }
}
