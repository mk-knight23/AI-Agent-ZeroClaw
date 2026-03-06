# Edge Deployment — ZeroClaw

Full deployment workflow for shipping ZeroClaw skills as WASM to edge runtimes with zero downtime.

## Overview

Edge deployment takes a ZeroClaw skill, compiles it to WASM, optimizes it, deploys to one or more edge platforms, and verifies the deployment is live.

## Trigger
- **Manual**: `@ZeroClaw edge-deployment --skill data-validator --target cloudflare`
- **On release**: Auto-deploys all edge-eligible skills when a version tag is pushed

## Deployment Pipeline

```
Source skill (Rust)
       ↓
  cargo build --target wasm32-wasi --release
       ↓
  wasm-opt -O3 (size optimization)
       ↓
  wasm-bindgen (JS/TS bindings)
       ↓
  Deploy to target (wrangler / deployctl / fastly)
       ↓
  Health check: invoke deployed worker
       ↓
  EDGE_DEPLOY_REPORT.md
```

## Multi-Target Deployment

```
@ZeroClaw edge-deployment --all-skills --targets cloudflare,deno
```

Deploys all skills marked `edge = true` in `Cargo.toml` metadata to both targets in parallel.

## Rollback

```
@ZeroClaw edge-deployment --rollback --skill data-validator --target cloudflare
```
Activates the previous deployment version via platform API.

## EDGE_DEPLOY_REPORT.md

```
## Edge Deployment Report — 2026-03-05

Target: Cloudflare Workers
  data-validator:  178KB → deployed → https://data-validator.your-subdomain.workers.dev ✓
  edge-toolkit:    220KB → deployed → https://edge-toolkit.your-subdomain.workers.dev ✓

Target: Deno Deploy
  data-validator:  192KB → deployed → https://data-validator.deno.dev ✓

Health checks:
  data-validator (CF): 2ms P50, 8ms P95 — PASS
  data-validator (Deno): 4ms P50, 12ms P95 — PASS
```

## Blue-Green Deployments

ZeroClaw supports blue-green deployment on Cloudflare via named routes:

```
Production:  data-validator.workers.dev → blue (current live)
Staging:     data-validator-staging.workers.dev → green (new deploy)

Once verified: swap routes (atomic, zero downtime)
```

## Environment Strategy

```toml
# config.toml
[edge]
production_targets = ["cloudflare", "fastly"]
staging_targets = ["deno"]
auto_promote_after_minutes = 30  # Promote staging to prod after 30min of healthy metrics
```
