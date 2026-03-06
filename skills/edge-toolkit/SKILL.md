---
name: edge-toolkit
description: "Packages ZeroClaw skills as WASM modules and deploys them to edge runtimes: Cloudflare Workers, Deno Deploy, Fastly Compute, or local Wasmtime. Handles wasm-pack build, wasm-opt size optimization, wasm-bindgen JS/TS bindings, and deployment manifests. Use when you want ZeroClaw logic running at the network edge with sub-millisecond cold starts."
---

# edge-toolkit

Compile ZeroClaw to WASM and deploy to any edge runtime in one command.

## Usage
```
@ZeroClaw edge-toolkit --skill data-validator --deploy cloudflare
@ZeroClaw edge-toolkit --skill batch-processor --target wasmtime
@ZeroClaw edge-toolkit --list-targets
```

## Supported Targets

| Target | Binary | Cold start | Deploy command |
|--------|--------|------------|---------------|
| Cloudflare Workers | <1MB | <5ms | `wrangler deploy` |
| Deno Deploy | <2MB | <10ms | `deployctl deploy` |
| Fastly Compute | <1MB | <1ms | `fastly compute publish` |
| Wasmtime (local) | Native | <1ms | `wasmtime skill.wasm` |

## Build Pipeline

1. `cargo build --target wasm32-wasi --release` — compile to WASM
2. `wasm-opt -O3 --strip-debug skill.wasm -o skill.opt.wasm` — optimize (40% size reduction avg)
3. `wasm-bindgen skill.opt.wasm --out-dir dist/ --target web` — generate JS/TS bindings
4. Deploy via target CLI

## Files Created
```
dist/
  skill.wasm              # Optimized WASM binary
  skill.d.ts              # TypeScript bindings (auto-generated)
  skill_bg.js             # JS glue (auto-generated)
  wrangler.toml           # (Cloudflare) Generated deployment config
EDGE_DEPLOY_REPORT.md     # Deployed URLs, binary size, estimated cold start
```

## Environment Variables
```
CLOUDFLARE_API_TOKEN=     # For Cloudflare deployment
DENO_AUTH_TOKEN=          # For Deno Deploy
```

## Example wrangler.toml (generated)
```toml
name = "zeroclaw-data-validator"
main = "dist/skill.js"
compatibility_date = "2024-01-01"

[build]
command = "cargo build --target wasm32-wasi --release && wasm-opt -O3 ..."

[[rules]]
type = "ESModule"
globs = ["**/*.js"]
```

## Philosophy
Edge deployment eliminates the latency of a round-trip to a datacenter region. For validation logic, a Cloudflare Worker running ZeroClaw's compiled rules is faster than hitting any API — and runs identically in 300+ PoPs.
