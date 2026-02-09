use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// [nb:recipe]
/// Walks the directory tree to find all Rust source files, respecting `.gitignore` via the `walkdir` crate.
/// Returns absolute paths to `.rs` files.
pub fn scan_workspace(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root) {
        let entry: walkdir::DirEntry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    if let Ok(abs) = entry.path().canonicalize() {
                        files.push(abs);
                    }
                }
            }
        }
    }
    Ok(files)
}
