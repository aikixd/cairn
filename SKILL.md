---
description: Use the `cairn` tool to generate and understand the repository's high-level architecture.
---

# `cairn` Skill

This skill allows you to leverage the project's curated code map to quickly understand the codebase structure, key concepts, and entry points.

## How to use

1.  **Check for existence**: Look for `docs/map.generated.md`. If it exists, read it first to get an overview.
    *   Command: `ls docs/map.generated.md` -> `view_file docs/map.generated.md`
2.  **Generate if missing**: If the map is missing or feels stale, run the generator.
    *   Command: `cargo run -- gen` (or `cairn gen` if installed)
    *   Note: Default tag prefix is `nb`. Use `--prefix <custom>` if needed.
3.  **Interpret the Map**:
    *   `entry`: Start here to trace execution flow.
    *   `core`: Read these to understand the domain model.
    *   `invariant`: Critical constraints and rules that must not be violated.
    *   `recipe`: usage patterns for internal utilities.

## When to Tag Code

When writing *new* code, you should tag it if it meets the criteria:

*   **Entry Points**: `/// [nb:entry]` for main functions, significant public APIs, or process starters.
*   **Core Concepts**: `/// [nb:core]` for structs/enums that define the domain.
*   **Invariants**: `/// [nb:invariant]` for constraints and rules that must not be violated.
*   **Recipes**: `/// [nb:recipe]` for utilities you want others (including future-you or AI) to find easily.

### Tag Format

```rust
/// [nb:tag_name] optional_key=value
/// Description of what this item is and why it matters.
pub fn my_function() { ... }
```
