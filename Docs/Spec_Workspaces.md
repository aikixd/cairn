# Spec: Workspace & Crate Resolution

This document specifies the behavior of `codemap` regarding Rust workspaces, crate discovery, and name resolution.

## Core Principle

The tool MUST discover and correctly name every Rust crate within the target directory tree, regardless of whether it is:
*   Member of the root workspace.
*   Excluded from the root workspace (`[workspace.exclude]`).
*   Part of a nested, independent workspace.
*   A nested member crate (physically located inside another crate's directory).

## Resolution Algorithm

To achieve comprehensive coverage, the tool implements **Recursive Metadata Discovery**:

1.  **Discovery**: The tool walks the file system starting from `--root` to identify all `Cargo.toml` files, ignoring standard ignore patterns (`.git`, `target`, etc.).
2.  **Query**: For *every* discovered `Cargo.toml`, the tool executes `cargo metadata --no-deps --manifest-path <path>`.
3.  **Accumulation**: All packages reported by these metadata queries are aggregated into a global `CrateMap`.
    *   **Key**: Crate Root Path (absolute path to the directory containing `Cargo.toml`).
    *   **Value**: Package Name (as defined in `[package] name`).
4.  **Conflict Resolution**: If multiple manifests report the same crate root (e.g. root workspace and member manifest), the entries are merged (they are identical).

## File-to-Crate Mapping

For any given source file `F`:

1.  Find the **Longest Prefix Match** in `CrateMap` such that `CrateRoot` is a prefix of `F`.
2.  If a match is found, the file belongs to that crate.
    *   *Relative Path*: `F` stripped of `CrateRoot`.
    *   *Module Path*: derived from Relative Path (e.g. `src/foo/bar.rs` -> `foo::bar`).
3.  If no match is found, the file is assigned to the `unknown` crate.

## Edge Case Behavior

| Layout Scenario | Spec Behavior | Crate Name |
| :--- | :--- | :--- |
| **Standard Member** | Metadata from Root finds it. | `[package.name]` |
| **Nested Member** (`crates/outer/inner`) | Metadata from `crates/outer/inner/Cargo.toml` finds it. | `[package.name]` |
| **Excluded Crate** | Metadata from Root ignores it. Metadata from `excluded/Cargo.toml` finds it. | `[package.name]` |
| **Nested Workspace** | Metadata from Root ignores it. Metadata from `nested/Cargo.toml` finds the workspace members. | `[package.name]` |

## Test Verification

The `tests/fixture_complex` directory contains a canonical layout verifying these rules:

*   `crate_a`: Standard member.
*   `crate_b`: Standard member.
*   `nested_c`: Nested physically within `crate_b`, but distinct package.
*   `isolated_crate`: Excluded from root workspace.
*   `crate_d`: Inside a nested, independent workspace `nested_ws`.

The tool MUST extract tags from all 5 crates and correctly attribute them to their respective package names.
