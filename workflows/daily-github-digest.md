# Daily GitHub Digest — ZeroClaw

ZeroClaw fetches GitHub activity for your Rust projects and ecosystem repos every morning.

## Trigger
- **Schedule**: Daily at 8:00 AM
- **Manual**: `@ZeroClaw run daily-github-digest`

## Prerequisites
- `GITHUB_TOKEN` env var
- Repos in `config.toml`

## What ZeroClaw Reports

### Your Rust Repos
- New issues and PRs
- CI failures (particularly cargo test, clippy, audit)
- New releases from dependencies in your `Cargo.toml`

### Rust Ecosystem
- New RustSec advisories affecting your deps
- crates.io new versions for deps you pin
- Rust release announcements (stable, beta)

## Example Output

```
📋 GitHub Digest — 2026-03-05

your-org/zeroclaw
  • PR #34 merged: "Add Criterion benchmarks"
  • Issue #67: "WASM target panics on empty input"
  • ⚠ CI fail: test_security_scanner (cargo audit found advisory)

RustSec:
  • RUSTSEC-2026-0012: openssl 0.9.x — update to 0.10.x
    Affects: your-org/zeroclaw via openssl (dev dep)

Rust:
  • Rust 1.82.0 stable released
```

## Configuration

```toml
# config.toml
[github]
watched_repos = ["your-org/zeroclaw", "rust-lang/rust"]
digest_time = "08:00"
rustsec_alerts = true
```
