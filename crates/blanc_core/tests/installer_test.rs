use blanc_core::installer::{
    default_privacy_and_terms_en, extract_embedded_files, read_embedded_payload,
    uninstall_exe_filename, write_installer_exe, InstallerConfig, InstallerLang,
    UNINSTALL_META_FILENAME,
};

#[test]
fn test_installer_payload_roundtrip() {
    let temp_dir = std::env::temp_dir().join(format!("blanc_inst_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let stub = b"FAKE_STUB_EXE_BYTES_123456";
    let setup_path = temp_dir.join("MyGame_Setup.exe");

    let cfg = InstallerConfig {
        game_title: "My VN".to_string(),
        version: "1.2.0".to_string(),
        publisher: "Studio Test".to_string(),
        welcome_text: "Hello installer".to_string(),
        license_text: "MIT License".to_string(),
        default_install_subdir: "MyVN".to_string(),
        ..Default::default()
    };

    let files = vec![
        ("MyVN.exe".to_string(), b"GAME_BIN_DATA".to_vec()),
        ("game.bpack".to_string(), b"BPACK_ENCRYPTED_BYTES".to_vec()),
    ];

    write_installer_exe(stub, &setup_path, &cfg, "MyVN.exe", &files).unwrap();

    let (manifest, data_start) = read_embedded_payload(&setup_path).unwrap();
    assert_eq!(data_start, stub.len() as u64);
    assert_eq!(manifest.config.game_title, "My VN");
    assert_eq!(manifest.config.version, "1.2.0");
    assert_eq!(manifest.files.len(), 2);

    let dest_dir = temp_dir.join("Installed");
    let mut progress_calls = 0usize;
    extract_embedded_files(&setup_path, &dest_dir, |_cur, _total, _name| {
        progress_calls += 1;
    })
    .unwrap();

    assert_eq!(progress_calls, 2);
    assert_eq!(std::fs::read(dest_dir.join("MyVN.exe")).unwrap(), b"GAME_BIN_DATA");
    assert_eq!(std::fs::read(dest_dir.join("game.bpack")).unwrap(), b"BPACK_ENCRYPTED_BYTES");
    assert_eq!(std::fs::read(dest_dir.join(uninstall_exe_filename("My VN"))).unwrap(), stub);
    assert!(dest_dir.join(UNINSTALL_META_FILENAME).exists());

    let default_png = blanc_core::win_icon::default_blanc_logo_png();
    let ico = blanc_core::win_icon::png_to_ico_bytes(&default_png).unwrap();
    assert_eq!(&ico[0..4], &[0, 0, 1, 0]);

    let mut policy_cfg = InstallerConfig {
        game_title: "Echoes".to_string(),
        publisher: "Blanc".to_string(),
        license_text: default_privacy_and_terms_en("{GAME_TITLE}", "{PUBLISHER}"),
        ..Default::default()
    };
    policy_cfg
        .license_translations
        .insert("id".to_string(), "Kebijakan {GAME_TITLE} oleh {PUBLISHER}".to_string());
    assert_eq!(
        policy_cfg.resolve_license_text(InstallerLang::Id).unwrap(),
        "Kebijakan Echoes oleh Blanc"
    );
    // Missing language falls back to EN with placeholders replaced
    let ja_fallback = policy_cfg.resolve_license_text(InstallerLang::Ja).unwrap();
    assert!(ja_fallback.contains("Echoes"));
    assert!(ja_fallback.contains("Blanc"));

    let defaults_map = blanc_core::installer::default_privacy_and_terms_map();
    for lang in InstallerLang::ALL {
        assert!(
            defaults_map.get(lang.code()).is_some_and(|t| !t.trim().is_empty()),
            "missing default policy for {}",
            lang.code()
        );
    }

    // Verify path traversal in installer payload is rejected
    let bad_setup = temp_dir.join("Bad_Setup.exe");
    let bad_files = vec![("../escaped.exe".to_string(), b"EVIL".to_vec())];
    write_installer_exe(stub, &bad_setup, &cfg, "MyVN.exe", &bad_files).unwrap();
    assert!(read_embedded_payload(&bad_setup).is_err());

    let _ = std::fs::remove_dir_all(temp_dir);
}
