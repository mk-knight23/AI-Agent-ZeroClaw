---
name: add-gmail
description: "Adds Gmail monitoring to ZeroClaw via async IMAP over TLS using async-imap + tokio. Creates src/channels/gmail.rs with a GmailChannel struct, polls INBOX for unread messages matching a trigger pattern, and sends replies via lettre (SMTP). Zero CGo, zero external runtime. Requires a Gmail App Password (not your Google account password)."
---

# add-gmail

Gmail monitoring for ZeroClaw: async-imap + TLS, lettre SMTP, purely safe Rust.

## Usage
```
/add-gmail
```

## Files Created
```
src/channels/gmail.rs           # Async GmailChannel with IMAP + SMTP
```

## Files Modified
```
src/main.rs                     # Spawn GmailChannel in tokio runtime
src/config.rs                   # Add gmail_addr, gmail_app_password fields
Cargo.toml                      # Add async-imap, native-tls, lettre
```

## Environment Variables
```
GMAIL_ADDR=you@gmail.com
GMAIL_APP_PASSWORD=your_app_password   # Google Account → Security → App Passwords
GMAIL_POLL_SECS=300                    # Default: 5 minutes
GMAIL_FILTER_LABEL=zeroclaw            # Apply Gmail label to trigger processing
```

## Getting a Gmail App Password

1. Google Account → Security → 2-Step Verification must be on
2. App Passwords → Mail, Other Device → Generate
3. Copy 16-character password

## Code Sample
```rust
// src/channels/gmail.rs (generated)
use async_imap::Client;
use native_tls::TlsConnector;
use tokio::net::TcpStream;

pub struct GmailChannel {
    addr: String,
    password: String,
}

impl GmailChannel {
    pub async fn poll(&self) -> anyhow::Result<Vec<String>> {
        let tls = TlsConnector::builder().build()?;
        let tcp = TcpStream::connect("imap.gmail.com:993").await?;
        let tls_stream = tokio_native_tls::TlsConnector::from(tls).connect("imap.gmail.com", tcp).await?;
        let client = Client::new(tls_stream);
        let mut session = client.login(&self.addr, &self.password).await.map_err(|e| e.0)?;
        session.select("INBOX").await?;
        // ... fetch UNSEEN messages
        session.logout().await?;
        Ok(messages)
    }
}
```
