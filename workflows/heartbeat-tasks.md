# Heartbeat Tasks — ZeroClaw

Continuous background tasks for the ZeroClaw Rust ecosystem: security monitoring, benchmark regression detection, and WASM edge health checks. Adapted from PicoClaw's HEARTBEAT.md model.

## Active Heartbeats

### Heartbeat 1: Security Advisory Monitor (every 4 hours)
```
INTERVAL: 4h
CHANNEL: telegram/@security-alerts

- govulncheck against all ZeroClaw workspaces
- Check RustSec advisory database for new Rust CVEs
- Alert IMMEDIATELY on any CRITICAL advisory
```

### Heartbeat 2: Benchmark Regression Guard (every push to main)
```
TRIGGER: git push to main
CHANNEL: github PR comment + telegram

- Run criterion.rs benchmarks
- Compare vs. baseline (last 10 runs average)
- Alert if any benchmark regresses > 10%
- Block release if regression > 20%
```

### Heartbeat 3: WASM Edge Health (every 15 minutes)
```
INTERVAL: 15m
CHANNEL: telegram/@ops

- Ping Cloudflare Worker endpoint
- Verify response time < 50ms (edge SLA)
- Check Worker CPU time < 10ms (Cloudflare limit)
- Alert if WASM binary rejected (size > 1MB Cloudflare limit)
```

### Heartbeat 4: Supply Chain Watch (daily)
```
INTERVAL: 24h
CHANNEL: telegram/@security-alerts

- cargo-deny license + ban check on all workspaces
- Snyk advisory database delta (new vulns since last check)
- Flag any new crate version that breaks deny.toml policy
```

## Configuration
```toml
# heartbeat.toml
[[heartbeat]]
id = "security-advisory"
interval = "4h"
channel = "telegram"

[[heartbeat]]
id = "benchmark-guard"
trigger = "git-push"
threshold_warn = 0.10
threshold_fail = 0.20

[[heartbeat]]
id = "wasm-health"
interval = "15m"
endpoint = "https://zeroclaw.workers.dev/health"
sla_ms = 50
```

## Implementation
ZeroClaw's heartbeat daemon is a Tokio async task running alongside the main agent. Zero CPU when idle (event-driven), sub-millisecond wake time.
