---
name: security-scanner
description: "Comprehensive Rust security audit: cargo-audit for known CVEs, cargo-deny for supply chain policy, detection of unsafe blocks and unwrap() overuse in non-test code, and semgrep Rust rules for logic vulnerabilities. Generates SECURITY_REPORT.md with CRITICAL/HIGH/MEDIUM severity ratings. Run before any release or dependency update. Can be integrated as a CI gate."
---

# security-scanner

Rust security audit: CVEs, supply chain, unsafe code, logic vulnerabilities — one command.

## Usage
```
@ZeroClaw security-scanner --full
@ZeroClaw security-scanner --deps-only         # cargo audit + cargo deny only
@ZeroClaw security-scanner --unsafe-audit      # unsafe block analysis only
@ZeroClaw security-scanner --fail-on high      # Exit non-zero if HIGH+ issues found (CI mode)
```

## What It Checks

### 1. Known CVEs — `cargo audit`
- Checks all direct + transitive deps against the RustSec Advisory Database
- Flags every CVE with: advisory ID, severity, affected versions, recommended fix version

### 2. Supply Chain Policy — `cargo-deny`
- License compliance: blocks GPL, AGPL (configurable allowed list)
- Duplicate versions: flags multiple versions of the same crate
- Banned crates: configurable blocklist (e.g., old `openssl` versions)

### 3. Unsafe Code Audit
- Finds all `unsafe { }` blocks across the codebase
- Flags blocks missing `// SAFETY:` justification comments
- Counts `transmute`, `from_raw_parts`, pointer dereferences
- Reports file and line number for each

### 4. Panic Risk in Non-Test Code
- `unwrap()` calls in `src/` (not in `tests/` or `benches/`)
- `expect()` without explanation
- `panic!()` in library code
- Integer arithmetic on user-controlled values without overflow checks

### 5. Semgrep Rust Rules
- Shell injection patterns in `Command::new` argument construction
- Hardcoded credentials (regex pattern matching)
- Path traversal in file open/read operations

## Files Created
```
SECURITY_REPORT.md      # Full report with severity ratings and remediation steps
security_audit.json     # Machine-readable JSON (for CI artifact upload)
```

## SECURITY_REPORT.md Format
```
## Summary
CRITICAL: 0 | HIGH: 1 | MEDIUM: 3 | LOW: 8

## HIGH Issues
### Dependency: openssl 0.9.24
Advisory: RUSTSEC-2023-0044 — Use after free in X.509 cert parsing
Severity: HIGH
Fix: Upgrade to openssl >= 0.10.55

## MEDIUM Issues
### src/tools/vault_search.rs:47 — unwrap() in non-test code
...
```

## CI Integration
```yaml
# .github/workflows/security.yml
- name: ZeroClaw Security Scan
  run: |
    cargo install cargo-audit cargo-deny
    zeroclaw security-scanner --full --fail-on high
```

## Philosophy
CRITICAL issues block release. HIGH issues block merge. MEDIUM issues go in the backlog. LOW issues are tracked. The threshold is a team decision — but the scan runs on every PR.
