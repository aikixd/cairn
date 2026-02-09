use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

mod generator;
mod model;
mod parser;
mod scanner;

use parser::RustParser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen {
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long, default_value = "docs/map.generated.md")]
        out_md: PathBuf,
        #[arg(long)]
        out_json: Option<PathBuf>,
    },
}

/// [map:entrypoint]
/// The main entry point for the CLI.
/// Orchestrates the `gen` command.
fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen {
            root,
            out_md,
            out_json,
        } => {
            // 1. Resolve workspace crate map
            let crate_map = resolve_crate_map(root)?;
            println!("Found {} crates in workspace", crate_map.len());

            // 2. Scan files
            let files = scanner::scan_workspace(root)?;
            println!("Found {} Rust files", files.len());

            // 3. Parse files
            let mut parser = RustParser::new()?;
            let mut all_entries = Vec::new();

            for file_path in files {
                // Determine crate name for this file
                let crate_name = find_crate_for_file(&file_path, &crate_map)
                    .unwrap_or_else(|| "unknown".to_string());

                // Determine relative path from crate root
                let crate_root = crate_map
                    .iter()
                    .find(|(_, name)| *name == &crate_name)
                    .map(|(path, _)| path)
                    .unwrap_or(root);

                let relative_path = file_path.strip_prefix(crate_root).unwrap_or(&file_path);

                let entries = parser
                    .parse_file(&file_path, &crate_name, relative_path)
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to parse {:?}: {}", file_path, e);
                        Vec::new()
                    });
                all_entries.extend(entries);
            }

            println!("Found {} tagged items", all_entries.len());

            // 4. Generate Output
            generator::generate(&all_entries, out_md, out_json.as_ref())?;
        }
    }
    Ok(())
}

/// [map:recipe]
/// recursively discovers all `Cargo.toml` files in a directory tree and resolves their crate names.
/// Useful for handling complex workspaces, excluded members, and nested repositories.
fn resolve_crate_map(root: &PathBuf) -> Result<HashMap<PathBuf, String>> {
    let mut map = HashMap::new();

    // Find all Cargo.toml files
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            let manifest_path = entry.path();

            // Run metadata on this manifest
            if let Ok(packages) = get_all_packages(manifest_path) {
                for (pkg_root, name) in packages {
                    map.insert(pkg_root, name);
                }
            }
        }
    }

    Ok(map)
}

fn get_all_packages(manifest_path: &std::path::Path) -> Result<Vec<(PathBuf, String)>> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version=1")
        .arg("--manifest-path")
        .arg(manifest_path)
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new()); // Ignore failures
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let mut packages_vec = Vec::new();

    if let Some(packages) = metadata["packages"].as_array() {
        for pkg in packages {
            let name = pkg["name"].as_str().unwrap_or("unknown").to_string();
            let manifest_path_str = pkg["manifest_path"].as_str().unwrap_or("");
            if !manifest_path_str.is_empty() {
                let root_path = PathBuf::from(manifest_path_str)
                    .parent()
                    .unwrap()
                    .to_path_buf();
                packages_vec.push((root_path, name));
            }
        }
    }
    Ok(packages_vec)
}

fn find_crate_for_file(file: &PathBuf, map: &HashMap<PathBuf, String>) -> Option<String> {
    let mut best_match: Option<(PathBuf, String)> = None;
    for (root, name) in map {
        if file.starts_with(root) {
            if let Some((best_root, _)) = &best_match {
                if root.components().count() > best_root.components().count() {
                    best_match = Some((root.clone(), name.clone()));
                }
            } else {
                best_match = Some((root.clone(), name.clone()));
            }
        }
    }
    best_match.map(|(_, name)| name)
}
