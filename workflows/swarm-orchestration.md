# Swarm Orchestration — ZeroClaw

ZeroClaw as swarm orchestrator: parallel Rust sub-processes for CPU-intensive work that benefits from true multicore parallelism.

## Architecture

Unlike async swarms, ZeroClaw workers are OS processes (not goroutines or coroutines). This enables true multicore utilization for CPU-bound tasks like batch processing or compilation.

```
ZeroClaw (Orchestrator, main process)
  ├── Worker process 1: cargo build --target wasm32-wasi (CPU-bound)
  ├── Worker process 2: criterion benchmarks
  ├── Worker process 3: cargo audit + cargo deny
  └── Aggregate → final report
```

Workers communicate via Unix domain sockets or temp files. No network overhead.

## Use Cases

### Parallel Cross-Compilation
```
@ZeroClaw swarm cross-compile --targets "wasm32-wasi,x86_64-unknown-linux-musl,aarch64-unknown-linux-musl"
```
3 `cargo build` processes run in parallel on 3 CPU cores.

### Parallel Benchmark Suite
```
@ZeroClaw swarm benchmark --functions "batch_processor,data_validator,vault_search"
```
3 Criterion workers running concurrently.

### Full Audit
```
@ZeroClaw swarm audit
```
Parallel: cargo audit, cargo deny, clippy, security-scanner, unused deps analysis.

## Process Pool

```toml
# config.toml
[swarm]
max_workers = 4          # Limit to physical CPU cores
worker_timeout_secs = 600
ipc_method = "unix_socket"  # or "tmpfile"
```

## Rayon Integration

For data-parallel tasks, ZeroClaw uses Rayon within a single process instead of spawning separate processes:

```rust
use rayon::prelude::*;

let results: Vec<_> = tasks.par_iter()
    .map(|task| process(task))
    .collect();
```

Use process swarm for I/O-bound tasks (compilation, network). Use Rayon for CPU-bound data tasks.
