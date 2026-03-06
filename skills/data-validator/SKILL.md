---
name: data-validator
description: "Schema validation engine for JSON, CSV, or TOML data. Describe validation rules in natural language; ZeroClaw generates a Rust struct with #[derive(Deserialize, Validate)] and custom validators, runs it against your full dataset, and reports every violation with location. Can also compile the validator to WASM for browser-side or edge validation. Generates VALIDATION_REPORT.md."
---

# data-validator

Natural language schema → compiled Serde validator → complete violation report.

## Usage
```
@ZeroClaw data-validator --input users.json --schema "id is UUID, email is valid email format, age is integer between 0 and 150, name is non-empty string max 200 chars"
@ZeroClaw data-validator --input config.toml --strict
@ZeroClaw data-validator --input records.csv --output-wasm    # Also compile to WASM
```

## What It Does

1. **Parse schema**: Understands natural language constraints (or accepts JSON Schema)
2. **Generate**: Creates `src/validator.rs` with `#[derive(Deserialize, Validate)]` and custom validators using the `validator` crate
3. **Compile**: `cargo build --release`
4. **Run**: Validates every record, collects ALL violations (no fail-fast)
5. **Report**: `VALIDATION_REPORT.md` — violations by field, sample bad values, line numbers

## Files Created
```
src/validator.rs                # Generated Serde + Validate struct
VALIDATION_REPORT.md            # Full violation report
validator.wasm                  # (if --output-wasm) WASM32 binary
```

## Example Generated Validator
```rust
// src/validator.rs (generated)
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct UserRecord {
    #[validate(length(min = 1))]
    pub id: String,  // UUID validated at parse

    #[validate(email)]
    pub email: String,

    #[validate(range(min = 0, max = 150))]
    pub age: i32,

    #[validate(length(min = 1, max = 200))]
    pub name: String,
}
```

## VALIDATION_REPORT.md Format
```
## Validation Summary
Total records: 50,000
Valid: 49,847 (99.69%)
Invalid: 153 (0.31%)

## Violations by Field
email: 89 violations
  - Line 145: "notanemail" — not a valid email address
  - Line 891: "" — empty string
  ...

age: 41 violations
  - Line 23: 200 — exceeds maximum (150)
```

## WASM Mode
Compiles to `wasm32-wasi` — deploy to Cloudflare Workers for client-side validation with identical business rules as your server.

## Philosophy
Collect all violations in one pass. Fail-fast validation hides the true extent of data quality problems.
