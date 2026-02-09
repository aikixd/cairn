use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// [nb:concept]
/// Represents a single item in the code map.
/// Contains the tag, anchor (name), file location, summaries, and extension metadata.
pub struct MapEntry {
    pub tag: String,
    pub anchor: String,
    pub file: String,
    pub line: usize,
    pub summary: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub tags: Vec<String>,
    pub rfc: Option<String>,
    pub recipe: Option<String>,
    pub owner: Option<String>,
}
