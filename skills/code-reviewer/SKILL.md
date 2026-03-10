---
name: code-reviewer
description: "Rust static analysis + AI review using clippy (all lints), rustfmt, and cargo-semver-checks. AI pass focuses on: ownership violations, lifetime issues, unnecessary clones, panic risk in library code, and WASM-compatibility issues. Outputs CODE_REVIEW.md with CRITICAL/HIGH/MEDIUM/LOW severity. Adapted from Nanobot's Python code-reviewer for Rust's ownership model and zero-cost abstraction philosophy."
---

# code-reviewer

Rust static analysis + AI review. Ownership, lifetimes, panics, WASM compatibility.

## Usage
```
@ZeroClaw code-reviewer --path src/
@ZeroClaw code-reviewer --diff HEAD~1..HEAD
@ZeroClaw code-reviewer --file src/batch/processor.rs
@ZeroClaw code-reviewer --strict       # Fail on MEDIUM+ issues
```

## What It Runs

| Tool | Purpose |
|------|---------|
| `cargo clippy -- -D warnings` | All lints, pedantic group |
| `rustfmt --check` | Formatting consistency |
| `cargo-semver-checks` | API breaking changes (for library crates) |
| `cargo-udeps` | Unused dependencies |
| AI review | Lifetime correctness, clone() overuse, panic risk |

## Files Created
```
CODE_REVIEW.md                  # Full report with severity-rated findings
```

## Rust/ZeroClaw-Specific Checks
- **Unnecessary clones**: `.clone()` where a reference would suffice
- **Panic risk**: `unwrap()`, `expect()`, `panic!()` in library code
- **WASM compatibility**: `std::fs`, `std::thread`, `std::time::SystemTime` — all unavailable in WASM32
- **Lifetime issues**: lifetime annotations that could be simplified or are incorrect
- **Unsafe justification**: `unsafe` blocks missing `// SAFETY:` documentation
- **Error propagation**: `?` operator vs. manual `match` inconsistency

## Sample Report
```
## Summary
CRITICAL: 0 | HIGH: 2 | MEDIUM: 6 | LOW: 11

## HIGH Issues
### src/tools/vault.rs:83 — unwrap() in library code
**Issue**: `config.get("key").unwrap()` will panic if key is missing.
**Fix**: Return `Result<T, ConfigError>` and propagate with `?`.

### src/wasm/bridge.rs:12 — std::fs usage in WASM target
**Issue**: `std::fs::read_to_string()` panics in WASM32 runtime.
**Fix**: Gate with `#[cfg(not(target_arch = "wasm32"))]`.
```

## Exit Codes (for CI)
```bash
zeroclaw code-reviewer --path src/ --strict
# Exit 0: no MEDIUM+ issues
# Exit 1: issues found above threshold
```

## Philosophy
In Rust, "it compiles" is necessary but not sufficient. Clippy catches what the compiler allows but shouldn't. The AI pass catches what clippy allows but shouldn't — especially in library code that other crates depend on.
