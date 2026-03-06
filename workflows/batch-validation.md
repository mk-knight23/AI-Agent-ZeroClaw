# Batch Validation — ZeroClaw

Automated data quality validation pipeline: validate large datasets against defined schemas, generate reports, and optionally gate processing.

## Trigger
- **Manual**: `@ZeroClaw batch-validation --input data/ --schema schema.json`
- **Pre-processing gate**: Run automatically before batch-processor modifies data
- **Scheduled**: Daily validation of incoming data drops

## Pipeline

```
Input files (CSV/JSON/Parquet)
       ↓
  Schema detection (if no schema provided)
       ↓
  Parallel validation (Rayon — one worker per file)
       ↓
  Aggregate violations
       ↓
  VALIDATION_REPORT.md
       ↓
  Gate: pass if error rate < threshold, else alert
```

## Validation Rules

Defined in `schema.json` or described in natural language:

```json
{
  "fields": {
    "user_id": { "type": "uuid", "required": true },
    "email": { "type": "email", "required": true },
    "age": { "type": "integer", "min": 0, "max": 150 },
    "signup_date": { "type": "datetime", "format": "ISO8601" },
    "revenue": { "type": "float", "min": 0 }
  },
  "error_threshold": 0.01
}
```

## Multi-File Batch

```
@ZeroClaw batch-validation --input exports/*.csv --schema schema.json --parallel 8
```

Validates 8 files simultaneously using Rayon. Summary report aggregates violations across all files.

## VALIDATION_REPORT.md

```
## Batch Validation Summary — 2026-03-05

Files: 24 processed | 24 valid (0 failed gate)
Total records: 1,200,000
Valid: 1,198,430 (99.87%)
Violations: 1,570

Top violations:
  email (890): invalid format
  age (412): out of range (0-150)
  revenue (268): negative values

Files with highest error rate:
  exports/2026-03-02.csv: 0.8% (within threshold)
```

## Gate Mode

With `--gate`:
- Exit code 0 if error rate < threshold
- Exit code 1 if error rate ≥ threshold (blocks downstream processing)
- Integrates with CI pipelines and workflow gates
