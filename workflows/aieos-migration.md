# AIEos Migration — ZeroClaw

Migration workflow for moving from another AI agent framework (AIEos, LangChain Rust, custom implementation) to ZeroClaw.

## Overview

AIEos Migration automates the incremental migration of an existing agent codebase to ZeroClaw's architecture. It analyzes your existing code, generates ZeroClaw equivalents for each component, and validates that behavior is preserved.

## Trigger
- **Manual**: `@ZeroClaw aieos-migration --source-dir /path/to/old-agent`

## Migration Steps

### Step 1: Inventory
ZeroClaw scans the source directory:
- Count: skills, tools, channels, memory backends
- Identify: architecture patterns (callback-based? async? sync?)
- Map: dependencies that have ZeroClaw equivalents

### Step 2: Compatibility Report
```
MIGRATION_INVENTORY.md:

Source: AIEos v2.1 agent
  Skills: 8 (4 have direct equivalents, 4 need custom migration)
  Tools: 12 (10 map to MCP tools, 2 need glue code)
  Channels: Slack ✓, Discord (needs custom channel), Telegram ✓
  Memory: Redis (no built-in — use add-supabase or custom)
  Estimated: medium complexity migration
```

### Step 3: Generate Equivalents
For each skill/tool with a known equivalent, ZeroClaw generates the scaffold:
```
@ZeroClaw channel analyze: SlackChannel (AIEos) → SlackChannel (ZeroClaw)
@ZeroClaw channel generate: SlackChannel
```

### Step 4: Custom Migrations
For non-standard components, ZeroClaw reads the implementation and generates a ZeroClaw-native equivalent, preserving the logic and behavior.

### Step 5: Validation
Runs both agents side-by-side on a test message set and diffs the outputs.

## Files Created
```
migration/
  MIGRATION_INVENTORY.md    # What was found and mapped
  MIGRATION_PLAN.md         # Ordered steps with complexity estimates
  generated/                # Auto-generated ZeroClaw equivalents
  validation/               # Test messages + expected outputs
```
