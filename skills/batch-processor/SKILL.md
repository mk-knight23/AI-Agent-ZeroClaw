---
name: batch-processor
description: "Parallel batch processing of large datasets using Rayon. Processes CSV, JSON, Parquet, or binary files with transformation logic described in natural language. ZeroClaw generates the Rust transformation closure, compiles it, benchmarks with Criterion, and runs it on the full dataset. Use for CPU-intensive data transformation where performance matters. Generates BATCH_REPORT.md."
---

# batch-processor

Describe your transformation. ZeroClaw writes Rust, compiles it, and saturates all CPU cores with Rayon.

## Usage
```
@ZeroClaw batch-processor --input records.csv --describe "parse timestamp as UTC datetime, normalize email to lowercase, filter rows where revenue > 0, output parquet"
@ZeroClaw batch-processor --input large.parquet --output clean.parquet
@ZeroClaw batch-processor --input raw/*.json --describe "flatten nested items array, deduplicate by id"
```

## What It Does

1. **Profile**: Reads schema (columns, types, null counts) from input using polars
2. **Plan**: Reasons through transformation steps, asks for confirmation on ambiguous choices
3. **Generate**: Writes a Rust transformation closure with Rayon parallel iterator
4. **Compile**: `cargo build --release` in isolated temp workspace
5. **Benchmark**: Measures throughput (rows/second) via Criterion
6. **Run**: Full dataset with progress bar (indicatif)
7. **Report**: `BATCH_REPORT.md`

## Performance Targets
- CSV: >500K rows/sec (Rayon + csv crate)
- Parquet: >2M rows/sec (polars columnar read + Rayon)

## Files Created
```
batch_<name>.rs                 # Standalone transformation — reusable and committable
BATCH_REPORT.md                 # Before/after schema, row counts, throughput stats
```

## Example Generated Code
```rust
// batch_records.rs (generated)
use rayon::prelude::*;
use chrono::DateTime;

fn transform(record: CsvRecord) -> Option<OutputRecord> {
    let ts = DateTime::parse_from_rfc3339(&record.timestamp).ok()?;
    let email = record.email.to_lowercase();
    let revenue: f64 = record.revenue.parse().ok()?;
    if revenue <= 0.0 { return None; }
    Some(OutputRecord { timestamp: ts.to_utc(), email, revenue })
}

fn main() {
    let records: Vec<_> = read_csv("records.csv").par_bridge().filter_map(transform).collect();
    write_parquet("output.parquet", records);
}
```

## Philosophy
Generated batch processors are standalone Rust programs — no ZeroClaw dependency at runtime. Schedule them with cron, run them in CI, or commit them to the data engineering repo.
