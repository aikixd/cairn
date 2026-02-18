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

#[derive(Clone, Copy, PartialEq, Eq)]
enum DocCommentKind {
    Outer,
    Inner,
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
                if let Some((tag, meta_str, desc, comment_kind)) = self.parse_comment(text) {
                    let mut full_desc = desc;

                    // Aggregate subsequent comments
                    // We need a separate cursor to look ahead without disrupting the main walk?
                    // Or since we are inside a loop over children, we can't consume the main iterator.
                    // But we can peek valid siblings using `next_sibling` logic on the child node?

                    let mut current_comment = child;
                    while let Some(next) = current_comment.next_sibling() {
                        if next.kind() == "line_comment" || next.kind() == "block_comment" {
                            let next_text = &source[next.byte_range()];
                            if let Some((_, next_content)) = self.extract_doc_comment_content(next_text) {
                                if next_content.is_empty() {
                                    break;
                                }
                                let prefix_marker = format!("[{}:", self.prefix);
                                if next_content.starts_with(&prefix_marker) {
                                    break;
                                }
                                full_desc.push('\n');
                                full_desc.push_str(&next_content);
                                current_comment = next;
                                // Note: this will result in the main loop visiting these nodes again and finding no tag.
                                // That is harmless but inefficient.
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let anchor = if comment_kind == DocCommentKind::Inner {
                        current_path.to_string()
                    } else {
                        // Look ahead for the item using the *last* comment node
                        self.find_anchor_item(current_comment, current_path, source)
                    };

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

    fn parse_comment(&self, text: &str) -> Option<(String, String, String, DocCommentKind)> {
        // Looking for /// [prefix:<tag>] ... or //! [prefix:<tag>] ...
        let (comment_kind, content) = self.extract_doc_comment_content(text)?;
        let prefix_marker = format!("[{}:", self.prefix);

        if content.starts_with(&prefix_marker) {
            if let Some(end_bracket) = content.find(']') {
                let tag_start = prefix_marker.len();
                let tag = &content[tag_start..end_bracket];
                let rest = &content[end_bracket + 1..];
                // Split rest into metadata (key=value) and description
                // For now simple heuristic
                return Some((
                    tag.to_string(),
                    rest.to_string(),
                    rest.to_string(),
                    comment_kind,
                )); // TODO: refine
            }
        }
        None
    }

    fn extract_doc_comment_content(&self, text: &str) -> Option<(DocCommentKind, String)> {
        if let Some(rest) = text.strip_prefix("///") {
            return Some((DocCommentKind::Outer, rest.trim().to_string()));
        }
        if let Some(rest) = text.strip_prefix("//!") {
            return Some((DocCommentKind::Inner, rest.trim().to_string()));
        }
        None
    }

    fn find_anchor_item(&self, comment_node: Node, current_path: &str, source: &str) -> String {
        // Look at next named sibling, skipping attributes and non-tag doc comments
        // that may follow a blank doc-comment separator.
        let mut next = comment_node.next_named_sibling();

        while let Some(sibling) = next {
            if sibling.kind() == "attribute_item" {
                next = sibling.next_named_sibling();
                continue;
            }
            if sibling.kind() == "line_comment" || sibling.kind() == "block_comment" {
                let text = &source[sibling.byte_range()];
                if self.extract_doc_comment_content(text).is_some() {
                    next = sibling.next_named_sibling();
                    continue;
                }
            }

            // Extract name from the item
            if let Some(name_node) = sibling.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                return format!("{}::{}", current_path, name);
            }
            break;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_enum_with_derive() {
        let code = r#"
            /// [nb:core]
            /// Classifies the semantic role of a code chunk.
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum ChunkKind {
                Function,
                Struct,
            }
        "#;

        let parser = RustParser::new("nb").unwrap();
        let mut entries = Vec::new();

        let mut ts_parser = Parser::new();
        ts_parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");
        let tree = ts_parser.parse(code, None).expect("Failed to parse");

        parser.walk_tree(
            tree.root_node(),
            code,
            "test_crate",
            "test_file.rs",
            &mut entries,
        );

        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.tag, "core");
        // This is what fails currently: matches "derive" or something instead of "ChunkKind"
        assert!(
            entry.anchor.contains("ChunkKind"),
            "Expected anchor to contain 'ChunkKind', found: '{}'",
            entry.anchor
        );
    }

    #[test]
    fn test_inner_doc_comment_anchors_to_containing_module() {
        let code = r#"
            mod graph {
                //! [nb:core]
                //! Graph construction subsystem.
                pub struct Node;
            }
        "#;

        let parser = RustParser::new("nb").unwrap();
        let mut entries = Vec::new();

        let mut ts_parser = Parser::new();
        ts_parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");
        let tree = ts_parser.parse(code, None).expect("Failed to parse");

        parser.walk_tree(
            tree.root_node(),
            code,
            "test_crate",
            "test_file.rs",
            &mut entries,
        );

        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.tag, "core");
        assert_eq!(entry.anchor, "test_crate::graph");
        assert!(entry.summary.contains("Graph construction subsystem."));
    }

    #[test]
    fn test_blank_doc_separator_before_item_still_resolves_anchor() {
        let code = r#"
            /// [nb:core]
            /// Symbolic stack state.
            ///
            /// Used during abstract interpretation.
            #[derive(Debug, Clone)]
            pub struct AbstractStack {
                data: Vec<u8>,
            }
        "#;

        let parser = RustParser::new("nb").unwrap();
        let mut entries = Vec::new();

        let mut ts_parser = Parser::new();
        ts_parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");
        let tree = ts_parser.parse(code, None).expect("Failed to parse");

        parser.walk_tree(
            tree.root_node(),
            code,
            "test_crate",
            "test_file.rs",
            &mut entries,
        );

        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.tag, "core");
        assert!(
            entry.anchor.contains("AbstractStack"),
            "Expected anchor to contain 'AbstractStack', found: '{}'",
            entry.anchor
        );
        assert!(entry.summary.contains("Symbolic stack state."));
        assert!(!entry.summary.contains("Used during abstract interpretation."));
    }
}
