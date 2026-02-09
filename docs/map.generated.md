# Code Map

## concept

* `marked-code-map::model::MapEntry` — Represents a single item in the code map. Contains the tag, anchor (name), file location, summaries, and extension metadata.
* `marked-code-map::parser::RustParser` — The core parsing engine using Tree-sitter. Responsible for reading Rust files, extracting doc comments with tags, and finding the associated "anchor" items (functions, structs, etc.).

## entry

* `marked-code-map::generator::generate` — Generates the Markdown and JSON outputs from the collected map entries. Groups entries by tag and sorts them deterministically.

## entrypoint

* `marked-code-map::main` — The main entry point for the CLI. Orchestrates the `gen` command.

## recipe

* `marked-code-map::resolve_crate_map` — recursively discovers all `Cargo.toml` files in a directory tree and resolves their crate names. Useful for handling complex workspaces, excluded members, and nested repositories.
* `marked-code-map::scanner::scan_workspace` — Walks the directory tree to find all Rust source files, respecting `.gitignore` via the `walkdir` crate. Returns absolute paths to `.rs` files.

## tag_a

* `crate_a::item_a` — Item in crate_a

## tag_b

* `crate_b::item_b` — Item in crate_b

## tag_c

* `nested_c::item_c` — Item in nested_c

## tag_d

* `crate_d::item_d` — Item in crate_d

## tag_isolated

* `isolated_crate::item_isolated` — Item in isolated_crate

