# Tagging Guide: `codemap` and LLM Interaction

This guide defines the standard tags and conventions for marking Rust codebases to generate useful, low-noise code maps. These maps help both humans and LLMs navigate the system effectively.

## Core Tags

We use a minimal set of tags to capture the system's "shape" without overwhelming detail.

### 1. `entry`
**Purpose**: Marks the starting points of major processes or subsystems.
**When to use**:
*   `main` functions or CLI command handlers.
*   Entry points of significant subsystems (e.g., `Scanner::scan`, `Parser::parse`).
*   Public API boundaries (library `lib.rs` exports).
**Example**:
```rust
/// [nb:entry]
/// CLI entry point. Orchestrates argument parsing and dispatches commands.
fn main() { ... }
```

### Tag Format

```rust
/// [nb:tag_name] optional_key=value
/// Description of what this item is and why it matters.
pub fn my_function() { ... }
```

> **Note**: The default prefix is `nb`. It can be configured via `--prefix <custom>` in the CLI.

### 2. `concept` (or `info`)
**Purpose**: Marks types or modules that represent core domain concepts or architectural components. Focus on the *meaningful* abstractions, not implementation details.
**When to use**:
*   Data models that are central to the system (e.g., `MapEntry`, `CrateMap`).
*   Classes that encapsulate significant logic (e.g., `RustParser`).
*   Do NOT mark helper structs, private builders, or trivial enums.
**Example**:
```rust
/// [nb:concept]
/// Represents a single item in the code map. Contains location, summary, and metadata.
pub struct MapEntry { ... }
```

### 3. `recipe`
**Purpose**: Marks utilities or patterns that users (or LLMs) should proactively reuse. "How do I do X?"
**When to use**:
*   Helper functions that solve tricky problems (e.g., `resolve_crate_path`).
*   Macros that reduce boilerplate.
*   Standard ways to initialize or configure the system.
**Example**:
```rust
/// [nb:recipe] tags=utils
/// Resolves the absolute path of a crate root given a file path, handling workspace boundaries.
fn resolve_crate_root(file: &Path) -> PathBuf { ... }
```

## Best Practices for Summaries

*   **Be Conclusive**: Start with a verb or noun phrase describing *what it is*.
*   **Contextualize**: explain *why* it exists or its role in the larger system.
*   **Keep it Brief**: Summaries are truncated. Put the most important info first.
*   **Link**: Use metadata to link to related specs or recipes if needed (`rfc=...`, `recipe=...`).

## Usage Workflow for LLMs

When analyzing a codebase:
1.  Check for `docs/map.generated.md` first.
2.  Use the map to find relevant `entry` points for the task.
3.  Consult `concept` entries to understand the data flow.
4.  Check `recipe` entries before implementing new helpers to avoid reinvention.
