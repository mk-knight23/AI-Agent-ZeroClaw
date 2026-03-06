---
name: add-telegram
description: "Adds a Telegram bot channel to ZeroClaw using the teloxide library. Creates src/channels/telegram.rs with an async handler struct, registers it with the tokio runtime in main.rs, and wires trigger-word dispatch. Compiles to a stripped static binary under 5MB. Requires a bot token from @BotFather."
---

# add-telegram

Telegram integration for ZeroClaw: async Rust, teloxide, zero-cost dispatch, static binary.

## Usage
```
/add-telegram
```

## Files Created
```
src/channels/telegram.rs        # TelegramChannel using teloxide async handler
```

## Files Modified
```
src/main.rs                     # Spawn TelegramChannel in tokio runtime
src/config.rs                   # Add telegram_bot_token to Config struct
Cargo.toml                      # Add teloxide with macros and auto-send features
```

## Environment Variables
```
TELEGRAM_BOT_TOKEN=your_bot_token_from_botfather
ZEROCLAW_TRIGGER=@ZeroClaw      # Default trigger word
```

## Step-by-Step Walkthrough

1. Message `@BotFather` on Telegram → `/newbot` → copy token
2. Run `/add-telegram`
3. Set `TELEGRAM_BOT_TOKEN` in `.env`
4. Build: `cargo build --release`
5. Binary: `target/release/zeroclaw` (~3MB stripped)
6. Deploy and run: no runtime, no Docker, just the binary

## Code Sample
```rust
// src/channels/telegram.rs (generated)
use teloxide::{prelude::*, types::Message};

pub struct TelegramChannel {
    bot: Bot,
    trigger: String,
}

impl TelegramChannel {
    pub fn new(token: impl Into<String>, trigger: impl Into<String>) -> Self {
        Self { bot: Bot::new(token), trigger: trigger.into() }
    }

    pub async fn start<F, Fut>(self, dispatch: F)
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = String> + Send,
    {
        let trigger = self.trigger.clone();
        let handler = Update::filter_message()
            .filter(move |msg: Message| {
                msg.text().map(|t| t.contains(&trigger)).unwrap_or(false)
            })
            .endpoint(move |bot: Bot, msg: Message| {
                let reply_fut = dispatch(msg.text().unwrap_or("").to_string());
                async move {
                    let reply = reply_fut.await;
                    bot.send_message(msg.chat.id, reply).await?;
                    respond(())
                }
            });

        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
```

## Philosophy
teloxide is fully async and integrates with tokio's single-threaded runtime. No blocking, no threads — just futures scheduled by the existing task executor.
