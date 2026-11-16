use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=BLANC_BUNDLE_RUNTIME_PATH");
    println!("cargo:rerun-if-env-changed=BLANC_BUNDLE_INSTALLER_PATH");

    let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("assets")
        .join("icons")
        .join("blanc_engine.ico");
    println!("cargo:rerun-if-changed={}", icon_path.display());

    #[cfg(target_os = "windows")]
    if icon_path.exists() {
        let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
        if target_env == "gnu" {
            let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
            let rc_path = out_dir.join("blanc_icon.rc");
            let obj_path = out_dir.join("blanc_icon.o");
            let icon_escaped = icon_path.to_string_lossy().replace('\\', "/");
            let rc_content = format!("1 ICON \"{}\"\n", icon_escaped);
            std::fs::write(&rc_path, rc_content).expect("Failed to write .rc file");

            let status = std::process::Command::new("windres")
                .args([
                    rc_path.to_string_lossy().as_ref(),
                    "-O",
                    "coff",
                    "-o",
                    obj_path.to_string_lossy().as_ref(),
                ])
                .status();

            match status {
                Ok(s) if s.success() && obj_path.exists() => {
                    println!("cargo:rustc-link-arg={}", obj_path.display());
                }
                Ok(s) => {
                    println!("cargo:warning=windres exited with status {}", s);
                }
                Err(e) => {
                    println!("cargo:warning=Failed to run windres: {}", e);
                }
            }
        } else {
            let mut res = winres::WindowsResource::new();
            res.set_icon(&icon_path.to_string_lossy());
            if let Err(e) = res.compile() {
                println!("cargo:warning=Failed to embed Windows icon: {}", e);
            }
        }
    }

    if std::env::var_os("CARGO_FEATURE_BUNDLE_BINARIES").is_some() {
        let runtime_path = std::env::var("BLANC_BUNDLE_RUNTIME_PATH")
            .map(PathBuf::from)
            .expect("BLANC_BUNDLE_RUNTIME_PATH must be set when feature 'bundle-binaries' is enabled");
        let installer_path = std::env::var("BLANC_BUNDLE_INSTALLER_PATH")
            .map(PathBuf::from)
            .expect("BLANC_BUNDLE_INSTALLER_PATH must be set when feature 'bundle-binaries' is enabled");

        assert!(
            runtime_path.exists(),
            "Bundled runtime binary not found at {}",
            runtime_path.display()
        );
        assert!(
            installer_path.exists(),
            "Bundled installer binary not found at {}",
            installer_path.display()
        );

        println!("cargo:rustc-env=BLANC_BUNDLED_RUNTIME={}", runtime_path.display());
        println!("cargo:rustc-env=BLANC_BUNDLED_INSTALLER={}", installer_path.display());
    }
}
