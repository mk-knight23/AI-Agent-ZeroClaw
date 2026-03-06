# CI Security Gate — ZeroClaw

Automated security checks that run on every PR and block merge if critical issues are found.

## Overview

The CI Security Gate integrates ZeroClaw's `security-scanner` into your CI pipeline as a merge gate. It runs on every PR, generates a machine-readable report, and fails the build if CRITICAL or HIGH issues are found.

## GitHub Actions Integration

```yaml
# .github/workflows/security-gate.yml
name: ZeroClaw Security Gate

on:
  pull_request:
    branches: [main, develop]

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install security tools
        run: cargo install cargo-audit cargo-deny

      - name: Run ZeroClaw Security Gate
        run: cargo run -- security-scanner --full --fail-on high --output-json security_audit.json

      - name: Upload security report
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: security-report
          path: |
            SECURITY_REPORT.md
            security_audit.json

      - name: Comment on PR
        if: failure()
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('SECURITY_REPORT.md', 'utf8');
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: '## Security Gate Failed\n\n' + report.slice(0, 65000)
            });
```

## Gate Thresholds

| Severity | Default action |
|----------|---------------|
| CRITICAL | Always blocks merge |
| HIGH | Blocks merge (configurable) |
| MEDIUM | Warning only |
| LOW | Advisory only |

Configure in `zeroclaw-security.toml`:
```toml
[gate]
fail_on = "high"     # "critical" | "high" | "medium"
allow_unfixed = []   # Advisory IDs to allow even if over threshold
```

## What the Gate Checks (same as security-scanner)
1. `cargo audit` — known CVEs
2. `cargo deny` — license and supply chain
3. Unsafe block analysis
4. `unwrap()` in non-test code
5. Semgrep Rust rules

## Merge Queue Integration

Compatible with GitHub's merge queue: the gate runs against the merge commit, not just the PR branch, catching conflicts that introduce vulnerabilities.
