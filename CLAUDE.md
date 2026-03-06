# AI-Agent-ZeroClaw

Rust agent for edge, WASM, and serverless. Zero overhead, maximum safety.

## Quick Context
- Language: Rust (stable), Cargo workspaces
- Deploy targets: WASM32, Linux musl, AWS Lambda, Cloudflare Workers
- Philosophy: zero-cost abstractions, memory safety by default
- Binary size: <5MB stripped | no GC pauses

## Key Skills
- `batch-processor` — Parallel processing of large datasets with Rayon
- `data-validator` — Schema validation with Serde + custom rule engine
- `edge-toolkit` — WASM compilation + Cloudflare/Deno Deploy packaging
- `perf-benchmarker` — Criterion.rs benchmarks + flamegraph generation
- `security-scanner` — cargo-audit + unsafe code detection + supply chain check
- Plus: add-telegram, add-gmail, add-slack, add-obsidian, add-supabase

## Architecture
- benchmarks/ — criterion.rs benchmark- **Identities**:
  - `identity/privacy-guardian.json`: High-security persona.
  - `identity/security-auditor.json`: CI/CD gatekeeper.
  - `identity/data-analyst.json`: Pattern recognition & reporting.
- **Skills**:
  - `skills/security-scanner`: Rust-based auditing.
- **Config**:
  - `docs/advanced-config.md`: sub-10ms reliability & estop safety.
- use-cases/ — real deployment examples

## Build
```bash
cargo build --release                          # native binary
cargo build --target wasm32-wasi --release     # WASM edge
```
