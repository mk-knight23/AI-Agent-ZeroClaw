# Monday Briefing — ZeroClaw

Start the week with a Rust project health check and priorities.

## Trigger
- **Schedule**: Every Monday at 8:00 AM
- **Manual**: `@ZeroClaw run monday-briefing`

## What ZeroClaw Covers

### Cargo Health
- Any new `cargo update` available?
- Security advisories from the weekend (RustSec)
- New Rust stable release (if relevant)

### Open Work
- PRs and issues assigned to you
- Failing CI on any branch
- WASM deployments that need updating

### This Week's Focus
- Issues labeled `this-week` in your repos
- Performance regressions flagged last week that need investigation
- Benchmarks scheduled to run

## Example Output

```
☕ Monday Briefing — March 10

Cargo health:
  cargo update available: 4 packages (non-breaking)
  RustSec: RUSTSEC-2026-0015 — affects tokio 1.35.1, update to 1.35.2

Open work:
  🔴 CI failing: main — test_data_validator panics on large input
  🟡 PR #89: "Add DHAT memory profiling" — awaiting review
  🟢 Issue #94: "Implement perf-benchmarker for batch-processor"

This week:
  Run full Criterion suite (compare against last week's baseline)
  Deploy edge-toolkit update to Cloudflare Workers (v0.4.0)
```
