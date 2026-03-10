# HR Campaign — ZeroClaw

Automated job outreach pipeline optimized for Rust/systems engineering roles. Adapted from OpenClaw's job-hunter + Nanobot's hr-campaign for the ZeroClaw persona.

## Trigger
- **Scheduled**: Monday 8:00 AM
- **Manual**: `@ZeroClaw hr-campaign --run --role "Systems Engineer"`

## Target Profile
ZeroClaw's campaign targets companies that value:
- Rust, WebAssembly, systems programming
- Edge computing, embedded systems
- Zero-overhead runtime performance
- Safety-critical software

## Pipeline

### Phase 1: Company Discovery
```
Search criteria:
- GitHub: repos with >1K stars using Rust
- Job boards: "Rust", "WebAssembly", "embedded", "systems"
- Target companies: Cloudflare, Oxide, Fastly, Fermyon, AWS Lambda, Fly.io
```

### Phase 2: Contact Discovery
```bash
# Same Hunter.io + Apollo pipeline as OpenClaw's job-hunter
@ZeroClaw deploy-everywhere  # Shows ZeroClaw's WASM portfolio
```

### Phase 3: Email Personalization (Claude Sonnet)
Each email is personalized with:
- Reference to the company's Rust/WASM usage from their GitHub
- Link to relevant ZeroClaw benchmark results
- Specific skill match: security-scanner → Cloudflare, batch-processor → data companies

Sample email hook:
> "I saw Cloudflare's blog post on running Rust at the edge — ZeroClaw achieves 8ms cold start in WASM32, and I'd love to help push that further."

### Phase 4: Tracking
- Email opens tracked via Supabase pixel
- Reply detection via IMAP
- Weekly digest: response rates by company category

## Configuration
```toml
[campaign]
role = "Systems Software Engineer"
runtime_specialty = "Rust + WASM"
portfolio = "https://github.com/mk-knight23/AI-Agent-ZeroClaw"
benchmark_results = "benchmarks/RESULTS.md"
target_companies = ["cloudflare", "oxide", "fastly", "fermyon", "fly.io"]
```

## Output
```
workspace/
├── zeroclaw_hr_contacts.md     # Discovered contacts
├── personalized_emails.md      # Generated email drafts
├── campaign_tracker.md         # Sent / opened / replied
└── interview_pipeline.md       # Active interview tracking
```
