---
name: deploy-everywhere
description: "Build and deploy ZeroClaw Rust binaries to all supported targets: WASM32-WASI (Cloudflare Workers, Deno, Wasmtime), Linux musl (Docker, Lambda, edge), native Linux (x86_64, ARM64, RISC-V), and WebAssembly browsers. Generates platform-specific configs, CI/CD pipelines, and deployment scripts. Uses cargo-cross for cross-compilation and cargo-component for WASM components. Adapted from OpenClaw's deploy-everywhere for Rust's cross-compilation ecosystem."
---

# deploy-everywhere

Compile Rust to every target. Ship WASM to the edge. Deploy native to servers.

## Usage
```
@ZeroClaw deploy-everywhere --target wasm32-wasi --platform cloudflare
@ZeroClaw deploy-everywhere --target all                               # Build all targets
@ZeroClaw deploy-everywhere --target linux-musl-x64 --platform lambda
@ZeroClaw deploy-everywhere --build-only                               # Build, don't deploy
```

## Supported Targets

| Target | Triple | Platform | Use Case |
|--------|--------|----------|---------|
| `wasm32-wasi` | `wasm32-wasip1` | Cloudflare Workers, Deno | Edge functions |
| `wasm-browser` | `wasm32-unknown-unknown` | Browser, Node.js | Client-side WASM |
| `linux-musl-x64` | `x86_64-unknown-linux-musl` | Docker, Lambda | Serverless |
| `linux-musl-arm64` | `aarch64-unknown-linux-musl` | AWS Graviton | ARM servers |
| `linux-riscv64` | `riscv64gc-unknown-linux-gnu` | LicheeRV Nano | Edge hardware |
| `linux-native` | host triple | Local | Development |

## Build Pipeline

```bash
# WASM edge (Cloudflare Workers)
cargo build --target wasm32-wasip1 --release
wasm-opt -Oz dist/zeroclaw.wasm -o dist/zeroclaw-opt.wasm

# Linux musl (truly static binary, no libc deps)
cross build --target x86_64-unknown-linux-musl --release

# Size report
ls -lh target/*/release/zeroclaw
# wasm32:  1.2MB → 0.8MB (after wasm-opt)
# musl:    3.8MB (statically linked, no runtime deps)
```

## Platform-Specific Configs

### Cloudflare Workers
```toml
# wrangler.toml (generated)
name = "zeroclaw-edge"
main = "dist/zeroclaw-opt.wasm"
compatibility_date = "2025-01-01"
```

### AWS Lambda
```yaml
# serverless.yml (generated)
functions:
  zeroclaw:
    handler: bootstrap
    runtime: provided.al2023
    package:
      artifact: target/x86_64-unknown-linux-musl/release/zeroclaw
```

### Docker (scratch image)
```dockerfile
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/zeroclaw /zeroclaw
ENTRYPOINT ["/zeroclaw"]
# Final image: ~4MB total
```

## Generated Files
```
dist/
├── zeroclaw.wasm               # Cloudflare / Deno
├── zeroclaw-opt.wasm           # Optimized WASM (wasm-opt -Oz)
├── zeroclaw-linux-x64          # Linux amd64 musl
├── zeroclaw-linux-arm64        # Linux arm64 musl
├── zeroclaw-linux-riscv64      # RISC-V edge
platform/
├── wrangler.toml               # Cloudflare Workers
├── serverless.yml              # AWS Lambda
├── Dockerfile                  # Scratch image
├── fly.toml                    # Fly.io
.github/workflows/
└── release.yml                 # Matrix build CI
DEPLOY_MANIFEST.md              # All targets + deployment instructions
```

## Philosophy
The same Rust source that compiles to 8MB on a server compiles to 1MB WASM for the edge and 4KB for a microcontroller. Write once, deploy everywhere — ZeroClaw's core promise.
