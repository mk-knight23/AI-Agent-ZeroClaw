# Friday Review — ZeroClaw

Weekly summary: Rust code quality, performance trends, security posture, and WASM deliverables.

## Trigger
- **Schedule**: Every Friday at 5:00 PM
- **Manual**: `@ZeroClaw run friday-review`

## What ZeroClaw Reviews

### Code Quality
- `cargo clippy` warnings across the week (trend: improving/worsening?)
- `cargo test` pass rate
- Total `unsafe` blocks added this week vs removed

### Performance
- Criterion benchmark deltas (if perf-benchmarker ran this week)
- Binary size change week-over-week
- WASM bundle size (main + all skill WASMs)

### Security
- New `cargo audit` findings since Monday
- `unwrap()` count delta (trend toward better error handling?)
- New `unsafe` blocks (count + justification coverage)

### Deliverables
- Skills deployed to edge targets this week
- WASM bundles published
- Cargo workspaces with version bumps

## Example Output

```
📅 Friday Review — Week of Mar 3–7

Code quality:
  clippy: 42 warnings (was 51 — improving ✓)
  tests: 247 passing, 0 failing
  unsafe blocks: 12 (+1 this week, justification: ✓)

Performance:
  Main binary: 4.1MB → 3.9MB (-4.9%)
  batch-processor benchmark: 2.87ms mean (-31.8% vs last week ✓)

Security:
  cargo audit: 0 findings
  unwrap() count: 34 → 29 (improving ✓)

Deliverables:
  • data-validator deployed to Cloudflare Workers (178KB WASM)
  • zeroclaw v0.4.0 released
```
