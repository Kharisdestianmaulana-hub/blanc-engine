use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=BLANC_BUNDLE_RUNTIME_PATH");
    println!("cargo:rerun-if-env-changed=BLANC_BUNDLE_INSTALLER_PATH");

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
