---
name: perf-benchmarker
description: "Criterion.rs micro-benchmarks on any Rust function with flamegraph generation and memory profiling via DHAT. Compares performance before/after code changes by stashing and re-benchmarking. Identifies hot paths and allocation patterns. Use before and after any performance-critical refactor to prove improvement. Generates PERF_REPORT.md."
---

# perf-benchmarker

Criterion benchmarks + flamegraphs + memory profiling. Prove your optimization with data.

## Usage
```
@ZeroClaw perf-benchmarker --function process_batch
@ZeroClaw perf-benchmarker --compare HEAD~1..HEAD
@ZeroClaw perf-benchmarker --flamegraph --function parse_records
@ZeroClaw perf-benchmarker --memory --function vault_search
```

## Modes

### Benchmark Mode
1. Identifies target function(s) from description
2. Generates `benches/<function>_bench.rs` with Criterion setup
3. Runs: `cargo bench -- --warm-up-time 3 --measurement-time 10`
4. Captures: mean, std dev, min, max, throughput (if applicable)

### Comparison Mode
1. Stash current changes → benchmark baseline (on HEAD~1)
2. Pop stash → benchmark new code (on HEAD)
3. Report delta (%) per measurement — flags regressions >5% in red

### Flamegraph Mode
1. Builds with `cargo flamegraph --bin zeroclaw`
2. Saves `perf/flamegraph.svg`
3. Opens in browser (or reports path)

### Memory Profiling via DHAT
1. Adds `dhat` feature flag temporarily
2. Runs with `cargo +nightly test -- --features dhat-heap`
3. Reports: total allocations, peak heap, top allocation hot spots

## Files Created
```
benches/<function>_bench.rs     # Criterion benchmark harness
perf/flamegraph.svg             # (if --flamegraph)
PERF_REPORT.md                  # Summary table + regression flags
```

## PERF_REPORT.md Example
```
## Benchmark Results: process_batch

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Mean   | 4.21ms | 2.87ms | -31.8% ✓ |
| p95    | 5.10ms | 3.12ms | -38.8% ✓ |
| Allocs | 12,400 | 2,100 | -83.1% ✓ |

No regressions detected.
```

## Cargo.toml additions (generated)
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "process_batch_bench"
harness = false
```

## Philosophy
Performance claims without benchmarks are just opinions. Always measure before and after. Criterion's statistical analysis catches noise — a 2% speedup from measurement variance is not a speedup.
