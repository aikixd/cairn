---
description: Use the `codemap` tool to generate and understand the repository's high-level architecture.
---

# `marked-code-map` Skill

This skill allows you to leverage the project's curated code map to quickly understand the codebase structure, key concepts, and entry points.

## How to use

1.  **Check for existence**: Look for `docs/map.generated.md`. If it exists, read it first to get an overview.
    *   Command: `ls docs/map.generated.md` -> `view_file docs/map.generated.md`
2.  **Generate if missing**: If the map is missing or feels stale, run the generator.
    *   Command: `cargo run -- gen` (or `codemap gen` if installed)
3.  **Interpret the Map**:
    *   `entry`: Start here to trace execution flow.
    *   `concept`: Read these to understand the domain model.
    *   `recipe`: usage patterns for internal utilities.

## When to Tag Code

When writing *new* code, you should tag it if it meets the criteria:

*   **Entry Points**: `/// [map:entry]` for main functions, significant public APIs, or process starters.
*   **Core Concepts**: `/// [map:concept]` for structs/enums that define the domain.
*   **Recipes**: `/// [map:recipe]` for utilities you want others (including future-you or AI) to find easily.

### Tag Format

```rust
/// [map:tag_name] optional_key=value
/// Description of what this item is and why it matters.
pub fn my_function() { ... }
```
