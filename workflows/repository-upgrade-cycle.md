# Repository Upgrade Cycle — ZeroClaw

Automated weekly pass across all ZeroClaw Rust workspaces: dependency updates, security audit, benchmark validation, and WASM size optimization.

## Schedule
- **Trigger**: Sunday 12:00 AM (first in the ecosystem cycle)
- **Duration**: ~20 minutes

## Pipeline

### Phase 1: Cargo Update
```bash
cargo update                     # Update Cargo.lock to latest compatible
cargo outdated --root-deps-only  # Report what can be bumped in Cargo.toml
cargo audit                      # Immediate CVE check post-update
```

### Phase 2: Security Gate
```bash
@ZeroClaw security-scanner --full
# Blocks if any CRITICAL or HIGH issues
```

### Phase 3: Full Test + Benchmark Suite
```bash
cargo test --all-features --workspace
cargo bench --workspace          # Criterion.rs baseline
```

Compare benchmarks vs. last week's baseline. Flag regressions > 5%.

### Phase 4: WASM Build Validation
```bash
cargo build --target wasm32-wasip1 --release
wasm-opt -Oz dist/zeroclaw.wasm -o dist/zeroclaw-opt.wasm

# Size check (Cloudflare Workers limit: 1MB)
wasm-size-report dist/zeroclaw-opt.wasm
```

### Phase 5: Code Quality
```bash
@ZeroClaw code-reviewer --path src/ --workspace
cargo clippy --all-targets -- -D warnings
```

### Phase 6: Release Candidate
If all steps pass, create a versioned release tag:
```bash
cargo semver-checks            # No API breaking changes?
git tag v$(date +%Y.%-m.%-d)
@ZeroClaw deploy-everywhere --target all  # Build all release artifacts
```

### Phase 7: Digest
```
🦀 ZEROCLAW UPGRADE REPORT — March 10, 2026

Crates updated: 5
CVEs found: 0
Tests: 247/247 passing
Benchmarks: no regressions (best: batch_processor 12% faster 🚀)
WASM size: 0.82MB (< 1MB Cloudflare limit ✅)
Release: v2026.3.10 tagged and built for all targets
```
