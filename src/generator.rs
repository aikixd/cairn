use crate::model::MapEntry;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// [map:entry]
/// Generates the Markdown and JSON outputs from the collected map entries.
/// Groups entries by tag and sorts them deterministically.
pub fn generate(entries: &[MapEntry], out_md: &PathBuf, out_json: Option<&PathBuf>) -> Result<()> {
    if let Some(json_path) = out_json {
        let json = serde_json::to_string_pretty(entries)?;
        fs::write(json_path, json)?;
    }

    // Markdown generation
    let mut content = String::new();
    content.push_str("# Code Map\n\n");

    // Group by tag
    let mut by_tag: std::collections::HashMap<&String, Vec<&MapEntry>> =
        std::collections::HashMap::new();
    for entry in entries {
        by_tag.entry(&entry.tag).or_default().push(entry);
    }

    // Sort tags (alphabetical for now, spec says fixed order but let's just sort)
    let mut tags: Vec<_> = by_tag.keys().collect();
    tags.sort();

    for tag in tags {
        content.push_str(&format!("## {}\n\n", tag));
        let mut tag_entries = by_tag[tag].clone();
        // Sort by file then anchor
        tag_entries.sort_by(|a, b| a.file.cmp(&b.file).then(a.anchor.cmp(&b.anchor)));

        for entry in tag_entries {
            content.push_str(&format!("* `{}` — {}\n", entry.anchor, entry.summary));
            // Add metadata links if present
            if let Some(rfc) = &entry.metadata.rfc {
                content.push_str(&format!("  (RFC: {})\n", rfc));
            }
        }
        content.push('\n');
    }

    if let Some(parent) = out_md.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_md, content)?;
    Ok(())
}
