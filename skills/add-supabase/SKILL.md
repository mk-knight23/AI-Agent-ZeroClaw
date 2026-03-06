---
name: add-supabase
description: "Adds Supabase as ZeroClaw's persistent backend using the Supabase REST API via reqwest. Creates typed Rust structs for each table (serde Deserialize/Serialize), db/supabase.rs with async CRUD helpers, and memory_read/memory_write tools. Zero CGo, no heavy ORM. Requires a Supabase project URL and anon key."
---

# add-supabase

Supabase persistence for ZeroClaw: typed REST via reqwest, serde structs, async tokio.

## Usage
```
/add-supabase
```

## Files Created
```
src/db/supabase.rs              # Typed async Supabase REST client
src/db/models.rs                # Serde structs: AgentMemory, SkillInvocation
src/db/migrations/001_init.sql  # Schema (run manually in Supabase SQL editor)
src/tools/memory.rs             # memory_read / memory_write tools
```

## Files Modified
```
src/tools/mod.rs                # Register memory_read, memory_write
src/config.rs                   # Add supabase_url, supabase_anon_key
Cargo.toml                      # Add reqwest (json, tls), serde, serde_json
```

## Environment Variables
```
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your_anon_key
```

## Tables
```sql
-- Run in Supabase SQL Editor
CREATE TABLE agent_memory (
  key TEXT PRIMARY KEY,
  value JSONB NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE skill_invocations (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  skill_name TEXT NOT NULL,
  duration_ms INTEGER,
  ts TIMESTAMPTZ DEFAULT now()
);
```

## Code Sample
```rust
// src/db/supabase.rs (generated)
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct SupabaseClient {
    url: String,
    anon_key: String,
    http: Client,
}

impl SupabaseClient {
    pub async fn memory_get(&self, key: &str) -> anyhow::Result<Option<Value>> {
        let url = format!("{}/rest/v1/agent_memory?key=eq.{}&select=value", self.url, key);
        let rows: Vec<serde_json::Map<String, Value>> = self.http
            .get(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.anon_key))
            .send().await?
            .json().await?;
        Ok(rows.first().and_then(|r| r.get("value").cloned()))
    }

    pub async fn memory_set(&self, key: &str, value: &Value) -> anyhow::Result<()> {
        let url = format!("{}/rest/v1/agent_memory", self.url);
        self.http
            .post(&url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", self.anon_key))
            .header("Prefer", "resolution=merge-duplicates")
            .json(&serde_json::json!({ "key": key, "value": value }))
            .send().await?;
        Ok(())
    }
}
```
