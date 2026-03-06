# ZeroClaw Advanced Configuration

This configuration is optimized for high-performance, mission-critical agentic workflows with a focus on safety and reliability.

```toml
# Reliability & Fallbacks
[reliability]
provider_retries = 3
provider_backoff_ms = 1000
fallback_providers = [
    "zai",
    "kimi-code",
    "anthropic",
    "gemini"
]

# Security Sandbox
[security.sandbox]
backend = "firejail"
strict_mode = true

# Memory Management
[memory]
backend = "sqlite"
auto_hydrate = true
hygiene_enabled = true
archive_after_days = 7

# Cost Guard
[cost]
enabled = true
daily_limit_usd = 5.0
warn_at_percent = 90

# Estop Safety
[security.estop]
enabled = true
require_otp_to_resume = true
```

## Key Benefits
- **Zero Downtime**: Automated provider fallback ensures your agent stays responsive.
- **Hardened Security**: Firejail isolation prevents unauthorized system access.
- **Transparent Costs**: Real-time tracking prevents unexpected API bills.
