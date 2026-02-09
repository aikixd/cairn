use crate::model::{MapEntry, Metadata};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser};

/// [nb:concept]
/// The core parsing engine using Tree-sitter.
/// Responsible for reading Rust files, extracting doc comments with tags, and finding the associated "anchor" items (functions, structs, etc.).
pub struct RustParser {
    parser: Parser,
    prefix: String,
}

impl RustParser {
    pub fn new(prefix: &str) -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .context("Error loading Rust grammar")?;
        Ok(Self {
            parser,
            prefix: prefix.to_string(),
        })
    }

    pub fn parse_file(
        &mut self,
        path: &Path,
        crate_name: &str,
        relative_path: &Path,
    ) -> Result<Vec<MapEntry>> {
        let content = fs::read_to_string(path)?;
        let tree = self
            .parser
            .parse(&content, None)
            .context("Failed to parse file")?;
        let root_node = tree.root_node();

        // Calculate base module path from file path
        // e.g. src/foo/bar.rs -> foo::bar
        // src/lib.rs -> "" (root)
        let mut module_path_parts = Vec::new();
        if let Some(parent) = relative_path.parent() {
            for part in parent.components() {
                if let Some(s) = part.as_os_str().to_str() {
                    if s != "src" {
                        module_path_parts.push(s.to_string());
                    }
                }
            }
        }

        let file_stem = relative_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if file_stem != "lib" && file_stem != "mod" && file_stem != "main" {
            module_path_parts.push(file_stem.to_string());
        }

        let base_module_path = if module_path_parts.is_empty() {
            crate_name.to_string()
        } else {
            format!("{}::{}", crate_name, module_path_parts.join("::"))
        };

        let mut entries = Vec::new();
        self.walk_tree(
            root_node,
            &content,
            &base_module_path,
            path.to_string_lossy().as_ref(),
            &mut entries,
        );

        Ok(entries)
    }

    fn walk_tree(
        &self,
        node: Node,
        source: &str,
        current_path: &str,
        file_path: &str,
        entries: &mut Vec<MapEntry>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            // Check for doc comments
            if child.kind() == "line_comment" || child.kind() == "block_comment" {
                // Simplified: Tree-sitter usually treats comments as separate nodes.
                // We need to check if this comment has our tag.
                let text = &source[child.byte_range()];
                if let Some((tag, meta_str, desc)) = self.parse_comment(text) {
                    let mut full_desc = desc;

                    // Aggregate subsequent comments
                    // We need a separate cursor to look ahead without disrupting the main walk?
                    // Or since we are inside a loop over children, we can't consume the main iterator.
                    // But we can peek valid siblings using `next_sibling` logic on the child node?

                    let mut current_comment = child;
                    while let Some(next) = current_comment.next_sibling() {
                        if next.kind() == "line_comment" || next.kind() == "block_comment" {
                            let next_text = &source[next.byte_range()];
                            // Check if it's a continuation (starts with ///)
                            // Usually line_comment includes the slashes.
                            full_desc.push('\n');
                            full_desc.push_str(next_text.trim_start_matches('/').trim());
                            current_comment = next;
                            // Note: this will result in the main loop visiting these nodes again and finding no tag.
                            // That is harmless but inefficient.
                        } else {
                            break;
                        }
                    }

                    // Look ahead for the item using the *last* comment node
                    let anchor = self.find_anchor_item(current_comment, current_path, source);

                    let metadata = self.parse_metadata(meta_str);

                    entries.push(MapEntry {
                        tag,
                        anchor,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        summary: self.clean_summary(&full_desc),
                        metadata,
                    });
                }
            }

            // Recurse for nested modules/items
            // Logic to update `current_path` based on `mod foo { ... }` or `impl Bar { ... }`
            // This requires more complex state handling during walk (passing down modified path).
            // For now, doing a flat walk, will refine path tracking.
            let new_path = self.update_path(child, source, current_path);
            self.walk_tree(child, source, &new_path, file_path, entries);
        }
    }

    fn parse_comment(&self, text: &str) -> Option<(String, String, String)> {
        // Looking for /// [prefix:<tag>] ...
        let content = text.trim_start_matches('/').trim();
        let prefix_marker = format!("[{}:", self.prefix);

        if content.starts_with(&prefix_marker) {
            if let Some(end_bracket) = content.find(']') {
                let tag_start = prefix_marker.len();
                let tag = &content[tag_start..end_bracket];
                let rest = &content[end_bracket + 1..];
                // Split rest into metadata (key=value) and description
                // For now simple heuristic
                return Some((tag.to_string(), rest.to_string(), rest.to_string())); // TODO: refine
            }
        }
        None
    }

    fn find_anchor_item(&self, comment_node: Node, current_path: &str, source: &str) -> String {
        // Look at next named sibling
        if let Some(next) = comment_node.next_named_sibling() {
            // Extract name from `next` node based on its type (fn_item, struct_item, etc.)
            if let Some(name_node) = next.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                return format!("{}::{}", current_path, name);
            }
        }
        format!("{}:{}", current_path, comment_node.start_position().row + 1)
    }

    fn update_path(&self, node: Node, source: &str, current_path: &str) -> String {
        if node.kind() == "mod_item" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                return format!("{}::{}", current_path, name);
            }
        }
        current_path.to_string()
    }

    fn parse_metadata(&self, _meta_str: String) -> Metadata {
        Metadata::default() // TODO
    }

    fn clean_summary(&self, text: &str) -> String {
        text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
