# cairn

`cairn` is a Rust CLI that generates a low-noise code map from tagged Rust doc comments.

It scans a workspace, finds tag directives (default prefix: `nb`), and writes a curated map for humans and LLM agents.

## Install

Build and place the binary on your `PATH`:

```bash
cargo build --release
cp target/release/cairn ~/.local/bin/
```

Or install directly from a cloned repo:

```bash
cargo install --path .
```

## Quick Start

1. Mark code with a tag:

```rust
/// [nb:entry]
/// Main entry for generation.
fn main() {}
```

2. Run generation:

```bash
cairn gen
```

3. Open the generated map at:

`docs/map.generated.md`


## Tags and Help

Supported tag details are documented in `docs/tagging-guide.md`.

For full command and option details, run:

```bash
cairn --help
cairn gen --help
```
