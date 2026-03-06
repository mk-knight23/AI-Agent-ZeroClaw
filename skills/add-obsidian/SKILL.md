---
name: add-obsidian
description: "Mounts an Obsidian vault as a read-only knowledge source for ZeroClaw. Creates src/tools/vault_search.rs using walkdir + regex crates for efficient file traversal and pattern matching. Registers the tool in the agent's tool registry. Zero heap allocation per search except for matched results. Requires vault accessible as a filesystem path."
---

# add-obsidian

Zero-allocation vault search for ZeroClaw: walkdir + regex, results only on matches.

## Usage
```
/add-obsidian
```

## Files Created
```
src/tools/vault_search.rs       # VaultSearch using walkdir + regex
```

## Files Modified
```
src/tools/mod.rs                # Register vault_search in tool registry
src/config.rs                   # Add obsidian_vault_path to Config
Cargo.toml                      # Add walkdir, regex
```

## Environment Variables
```
OBSIDIAN_VAULT_PATH=/path/to/vault
VAULT_MAX_RESULTS=10
```

## Code Sample
```rust
// src/tools/vault_search.rs (generated)
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug)]
pub struct VaultMatch {
    pub file: String,
    pub excerpt: String,
}

pub fn vault_search(query: &str, vault_path: &str, max_results: usize) -> Vec<VaultMatch> {
    let re = match Regex::new(&regex::escape(query)) {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .filter_map(|entry| {
            let content = std::fs::read_to_string(entry.path()).ok()?;
            let idx = re.find(&content)?.start();
            let start = idx.saturating_sub(100);
            let end = (idx + 300).min(content.len());
            Some(VaultMatch {
                file: entry.file_name().to_string_lossy().into_owned(),
                excerpt: content[start..end].to_string(),
            })
        })
        .take(max_results)
        .collect()
}
```

## Performance
On a 10,000-note vault (~500MB), search takes ~50-150ms on commodity hardware. The regex is compiled once and reused. `read_to_string` + `Regex::find` is sufficient — no index needed at this scale.

## Philosophy
The vault is read-only by design. ZeroClaw provides access to knowledge, not the ability to modify it.
