---
name: add-slack
description: "Adds Slack integration to ZeroClaw using reqwest for the Slack API and tokio-tungstenite for the Socket Mode WebSocket connection. Creates src/channels/slack.rs with a SlackChannel struct, handles app_mention events, routes to the skill dispatcher, and posts threaded replies. Compiles to a static binary. Requires Slack bot token and app-level token with Socket Mode enabled."
---

# add-slack

Slack Socket Mode for ZeroClaw: reqwest + tokio-tungstenite, async Rust, static binary.

## Usage
```
/add-slack
```

## Files Created
```
src/channels/slack.rs           # SlackChannel using reqwest + tokio-tungstenite
```

## Files Modified
```
src/main.rs                     # Spawn SlackChannel task in tokio runtime
src/config.rs                   # Add slack_bot_token, slack_app_token
Cargo.toml                      # Add reqwest (json, tls), tokio-tungstenite, serde_json
```

## Environment Variables
```
SLACK_BOT_TOKEN=xoxb-your-bot-token
SLACK_APP_TOKEN=xapp-your-app-level-token
```

## Why Not the Official Slack SDK?
No official async Rust Slack SDK exists. Using `reqwest` + `tokio-tungstenite` directly gives us:
- Full async/await support
- No blocking threads
- Minimal binary size increase (~200KB vs multi-MB for a third-party SDK)

## Code Sample
```rust
// src/channels/slack.rs (generated)
use reqwest::Client;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;

pub struct SlackChannel {
    bot_token: String,
    app_token: String,
    http: Client,
}

impl SlackChannel {
    pub async fn get_websocket_url(&self) -> anyhow::Result<String> {
        let res = self.http
            .post("https://slack.com/api/apps.connections.open")
            .bearer_auth(&self.app_token)
            .send().await?
            .json::<Value>().await?;
        Ok(res["url"].as_str().unwrap_or("").to_string())
    }

    pub async fn post_reply(&self, channel: &str, text: &str, thread_ts: &str) -> anyhow::Result<()> {
        self.http
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(&self.bot_token)
            .json(&json!({ "channel": channel, "text": text, "thread_ts": thread_ts }))
            .send().await?;
        Ok(())
    }
}
```
