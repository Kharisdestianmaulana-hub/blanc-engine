use blanc_core::flow::StoryFlowGraph;

#[test]
fn test_story_flow_graph_creation_and_connections() {
    let mut graph = StoryFlowGraph::starter_graph(Some("01_opening"));

    // Verify starter nodes
    assert_eq!(graph.nodes.len(), 3);
    assert!(graph.nodes.iter().any(|n| n.id == "system_main_menu"));
    assert!(graph.nodes.iter().any(|n| n.id == "scene_01_opening"));
    assert!(graph.nodes.iter().any(|n| n.id == "ending_return_to_title"));

    // Verify connections
    assert_eq!(graph.connections.len(), 2);

    // Add a new scene node
    let added = graph.add_scene_node("scene_courtyard", 500.0, 300.0);
    assert!(added);
    assert_eq!(graph.nodes.len(), 4);

    // Adding existing scene should return false
    let added_dup = graph.add_scene_node("scene_courtyard", 500.0, 300.0);
    assert!(!added_dup);

    // Connect from 01_opening to scene_courtyard
    graph.connect_pins(
        "scene_01_opening",
        "choice_0",
        "scene_scene_courtyard",
        "in",
    );
    assert_eq!(graph.connections.len(), 3);

    // Removing courtyard node should clean up connections
    let removed = graph.remove_node("scene_scene_courtyard");
    assert!(removed);
    assert_eq!(graph.nodes.len(), 3);
    assert_eq!(graph.connections.len(), 2);
}

#[test]
fn test_story_flow_graph_serialization() {
    let temp_dir = std::env::temp_dir().join(format!("blanc_flow_test_{}", uuid::Uuid::new_v4()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let graph_file = temp_dir.join("story_flow.bgraph");

    let mut graph = StoryFlowGraph::starter_graph(Some("intro"));
    graph.add_scene_node("chapter_one", 300.0, 200.0);

    // Save
    let save_res = graph.save_to_file(&graph_file);
    assert!(save_res.is_ok());

    // Load
    let load_res = StoryFlowGraph::load_from_file(&graph_file);
    assert!(load_res.is_ok());
    let loaded = load_res.unwrap();

    assert_eq!(loaded.nodes.len(), graph.nodes.len());
    assert_eq!(loaded.connections.len(), graph.connections.len());

    let _ = std::fs::remove_dir_all(temp_dir);
}
