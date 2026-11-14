use blanc_core::pack::{write_bpack, BpackArchive};

#[test]
fn test_bpack_roundtrip_and_encryption() {
    let dir = std::env::temp_dir().join(format!("blanc_pack_test_{}", uuid::Uuid::new_v4()));
    let pack_path = dir.join("game.bpack");

    let secret_script = b"{\"name\":\"Scene_1\",\"blocks\":[{\"Dialogue\":{\"text\":\"Secret story line\"}}]}".to_vec();
    let binary_png = vec![0x89, 0x50, 0x4E, 0x47, 1, 2, 3, 4, 5, 6, 7, 8];

    let inputs = vec![
        ("Scenes/Scene_1.json".to_string(), secret_script.clone()),
        ("Assets/Images/bg.png".to_string(), binary_png.clone()),
    ];
    write_bpack(&pack_path, &inputs).expect("write_bpack should succeed");

    // Verify raw archive file does not expose plaintext script
    let raw_archive = std::fs::read(&pack_path).expect("read archive");
    assert!(raw_archive.starts_with(b"BLNCPAK1"));
    assert!(
        !raw_archive.windows(17).any(|w| w == b"Secret story line"),
        "Encrypted pack should not contain plaintext dialogue"
    );

    let archive = BpackArchive::open(&pack_path).expect("open archive");
    assert_eq!(archive.list_files().len(), 2);
    assert_eq!(archive.read_file("Scenes\\Scene_1.json").as_deref(), Some(secret_script.as_slice()));
    assert_eq!(archive.read_file("Assets/Images/bg.png").as_deref(), Some(binary_png.as_slice()));
    assert!(archive.read_file("Missing.json").is_none());

    let _ = std::fs::remove_dir_all(&dir);
}
