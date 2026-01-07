use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct CleanResult {
    pub path: String,
    pub size_freed: u64,
    pub success: bool,
    pub error: Option<String>,
}

pub fn clean_target(target_path: &Path, size_bytes: u64, dry_run: bool) -> CleanResult {
    let parent = match target_path.parent() {
        Some(p) => p,
        None => {
            return CleanResult {
                path: target_path.display().to_string(),
                size_freed: 0,
                success: false,
                error: Some("Cannot find parent directory".to_string()),
            };
        }
    };

    let cargo_toml = parent.join("Cargo.toml");

    if !cargo_toml.exists() {
        return CleanResult {
            path: target_path.display().to_string(),
            size_freed: 0,
            success: false,
            error: Some("Cargo.toml not found".to_string()),
        };
    }

    if dry_run {
        return CleanResult {
            path: target_path.display().to_string(),
            size_freed: size_bytes,
            success: true,
            error: None,
        };
    }

    // cargo cleanを実行
    let output = Command::new("cargo")
        .arg("clean")
        .arg("--manifest-path")
        .arg(&cargo_toml)
        .output();

    match output {
        Ok(out) if out.status.success() => CleanResult {
            path: target_path.display().to_string(),
            size_freed: size_bytes,
            success: true,
            error: None,
        },
        Ok(out) => CleanResult {
            path: target_path.display().to_string(),
            size_freed: 0,
            success: false,
            error: Some(String::from_utf8_lossy(&out.stderr).to_string()),
        },
        Err(e) => CleanResult {
            path: target_path.display().to_string(),
            size_freed: 0,
            success: false,
            error: Some(e.to_string()),
        },
    }
}
