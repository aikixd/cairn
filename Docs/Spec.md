## Spec: Code Map Utility

### Goal

Generate a **curated, low-noise code map** from inline Rust doc-comment tags, producing Markdown files that help humans and LLMs quickly route to the right entrypoints, rules, and recipes—without dumping full symbol lists.

### Inputs

* A repository root path.
* Source files to scan:
  * Default: `**/*.rs`
  * Exclude by default: `target/`, `.git/`, `vendor/`, `node_modules/`, `dist/`
  * Configurable allow/deny globs.

### Tagging Convention (source-of-truth)

A tagged note is a Rust doc-comment block beginning with a directive line:

```rust
/// [map:<tag>] key=value key=value ...
/// <free-form description lines>
/// (block ends at first blank doc line `///` or non-doc line)
```

* `<tag>` is a single identifier (e.g. `entrypoint`, `invariant`, `recipe`, `decision`, `pitfall`).
* Metadata is optional; recognized keys (optional):

  * `tags=ids,base,ffi` (comma-separated)
  * `rfc=RFC-0007` or `rfc=docs/rfcs/active/RFC-0007-...md`
  * `recipe=docs/recipes/...md`
  * `owner=...` (optional)
* The description is the subsequent `///` lines until termination.

### Extraction Rules

For each tagged block, emit exactly **one map entry**:

* **Anchor**: nearest “item” name above/at the doc block (best-effort):

  * macro invocation name / `macro_rules!` name / `struct` / `enum` / `trait` / `fn` / `mod`
  * If no item can be confidently detected, fall back to `path:line`.
* **Summary**: collapse description lines into a single paragraph (preserve short bullets if present).
* **Links**: include RFC/recipe references from metadata if provided.

Noise control:

* Only tagged blocks are included (no symbol enumeration).
* Enforce maximum summary length per entry (e.g. 240 chars) with hard truncation + `…` (configurable).

### Outputs

Generate these Markdown files (overwrite deterministically):

1. `docs/map.generated.md`

   * Group entries by `<tag>` (in a fixed tag order).
   * Within each tag: sort by file path then anchor.
   * Format example:

     * `base::qol::define_named!` — Defines integer newtype wrappers to avoid primitive obsession. (RFC-0007, recipe: docs/recipes/ids.md)

2. Optional: `docs/map.generated.json`

   * Machine-readable list of entries:

     * `{ tag, anchor, file, line, summary, meta }`

### CLI

`codemap [command] [options]`

Commands:

* `codemap gen` (default): generate outputs
* `codemap check`: exit non-zero if generated output differs from checked-in file(s)
* `codemap lint` (optional): validate tagged blocks (unknown keys, missing tag, overlong summaries, etc.)

Options:

* `--root <path>`
* `--out-md <path>` (default `docs/map.generated.md`)
* `--out-json <path>` (optional)
* `--include <glob>` / `--exclude <glob>`
* `--max-summary-len <n>`

### Determinism & CI

* Output must be stable across runs (fixed ordering, normalized whitespace).
* Recommended workflow:

  * Commit `docs/map.generated.md`
  * CI runs `codemap check`

### Non-goals (v1)

* Full parsing via Rust AST (regex/line-based “good enough” is fine initially).
* Building call graphs, dependency graphs, or listing every symbol.
* Extracting from non-Rust files (can be added later).
