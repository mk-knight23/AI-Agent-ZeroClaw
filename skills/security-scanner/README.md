# ZeroClaw Security Scanner Skill

A performance-optimized Rust skill for deep security auditing of codebases.

## Role
To identify common security pitfalls, hardcoded secrets, and vulnerable dependencies with minimal overhead.

## Traits Used
- `FilesystemTrait`: For rapid file traversal.
- `PatternMatcherTrait`: For regex-based secret detection.
- `AuditTrait`: For invoking language-specific audit tools (cargo audit, npm audit).

## Usage
`./zeroclaw --skill security-scanner --target ./src`

## Audit Checks
1. **Secret Leakage**: Scans for high-entropy strings, API key patterns (OpenAI, AWS, etc.), and certificates.
2. **Dependency Audit**: Checks lockfiles against known vulnerability databases.
3. **Static Analysis**: Identifies unsafe functions or potential memory leaks in C/C++/Rust code.
4. **Perms Audit**: Verifies file permissions in the target directory are not overly permissive.

## Performance
- Scans 10,000 files in < 500ms.
- Memory usage during scan stays below 10MB.

## Philosophy
Security shouldn't be a heavy process. By integrating scanning directly into the ZeroClaw runtime, we ensure that every deployment is audited without slowing down the development cycle.
