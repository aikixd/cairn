# Tagging Guide

Quick reference for tagging Rust code with `cairn`. For complete workflow guidance, run `cairn llm-skill`.

## Tag Types

- **`entry`** - Starting points (main functions, CLI handlers, API boundaries)
- **`core`** - Domain concepts and architectural components
- **`invariant`** - Constraints and rules that must not be violated
- **`recipe`** - Reusable utilities and patterns

## Tag Format

```rust
/// [nb:tag_name] optional_key=value
/// Description of what this is and why it matters.
pub fn my_function() { ... }
```

## Metadata Keys (Optional)

- `tags=foo,bar` - Additional categorization
- `rfc=RFC-0007` or `rfc=docs/rfcs/RFC-0007.md` - Link to RFC
- `recipe=docs/recipes/foo.md` - Link to recipe document
- `owner=team` - Ownership attribution

## Examples

```rust
/// [nb:entry]
/// CLI entry point. Orchestrates command parsing and dispatch.
fn main() -> Result<()> { ... }

/// [nb:core]
/// Represents a tagged code item with location, summary, and metadata.
pub struct MapEntry { ... }

/// [nb:invariant] rfc=RFC-0007
/// Never hand-roll integer wrappers; always use `define_named!` macro.
pub macro define_named { ... }

/// [nb:recipe] tags=workspace
/// Recursively discovers all Cargo.toml files and resolves crate names.
fn resolve_crate_map(root: &PathBuf) -> Result<HashMap<PathBuf, String>> { ... }
```

## Full Workflow Guide

For comprehensive guidance on using `cairn` as an AI agent, including:
- Discovery and installation
- Workflow integration
- Verification checklists
- Integration with RFCs and documentation

Run: `cairn llm-skill`
