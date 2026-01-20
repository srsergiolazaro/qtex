use std::path::PathBuf;
use std::fs;
use semver::Version;
use crate::ui;

pub fn get_qtex_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".qtex")
}

pub fn get_versions_dir() -> PathBuf {
    get_qtex_dir().join("versions")
}

pub fn setup_installation(version: &str) -> std::io::Result<PathBuf> {
    let qtex_dir = get_qtex_dir();
    
    // Check for legacy non-rust installation (usually has a 'runtime' folder or JS files)
    let legacy_paths = [
        qtex_dir.join("runtime"),
        qtex_dir.join("package.json"),
        qtex_dir.join("index.js"),
    ];

    for path in &legacy_paths {
        if path.exists() {
            ui::warn("Detected legacy non-Rust installation. Cleaning up for a fresh start...");
            let _ = fs::remove_dir_all(&qtex_dir);
            break;
        }
    }

    let versions_dir = get_versions_dir();
    
    if !versions_dir.exists() {
        fs::create_dir_all(&versions_dir)?;
    }

    let current_version_dir = versions_dir.join(version);
    if !current_version_dir.exists() {
        fs::create_dir_all(&current_version_dir)?;
    }

    Ok(current_version_dir)
}

pub fn cleanup_old_versions(current_version: &str) -> std::io::Result<()> {
    let versions_dir = get_versions_dir();
    if !versions_dir.exists() {
        return Ok(());
    }

    let current_v = Version::parse(current_version).ok();

    for entry in fs::read_dir(versions_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let version_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if let Ok(v) = Version::parse(version_str) {
                if let Some(ref curr) = current_v {
                    if &v < curr {
                        ui::info(&format!("Cleaning up old version: {}", version_str));
                        fs::remove_dir_all(path)?;
                    }
                } else if version_str != current_version {
                    // Fallback for non-semver or if current fails
                    ui::info(&format!("Cleaning up unknown version folder: {}", version_str));
                    fs::remove_dir_all(path)?;
                }
            }
        }
    }
    Ok(())
}

pub async fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    ui::info("Checking for updates...");
    // Future implementation: Fetch latest version from GitHub/API
    // Download and install to versioned folder
    // For now, it's a stub to keep feature parity with updater.js
    Ok(())
}
