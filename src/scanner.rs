use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct TargetDir {
    pub path: PathBuf,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub enum ScanResult {
    Success(Vec<TargetDir>),
    PathNotFound(String),
}

pub fn scan_target_directories(root: &str, exclude_dirs: &[String]) -> ScanResult {
    let root_path = Path::new(root);

    // ディレクトリの存在チェック
    if !root_path.exists() {
        return ScanResult::PathNotFound(root.to_string());
    }

    let mut targets = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !exclude_dirs.iter().any(|ex| name == ex.as_str())
        })
        .flatten()
    {
        if entry.file_type().is_dir() && entry.file_name() == "target" {
            // Cargo.tomlが親ディレクトリにあるか確認
            if let Some(parent) = entry.path().parent() {
                if parent.join("Cargo.toml").exists() {
                    let size = calculate_dir_size(entry.path());
                    targets.push(TargetDir {
                        path: entry.path().to_path_buf(),
                        size_bytes: size,
                    });
                }
            }
        }
    }

    ScanResult::Success(targets)
}

fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}
