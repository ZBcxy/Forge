# Forge OS Security Architecture

## Design Principles

1. **Defense in depth** — every layer validates independently
2. **Least privilege** — agents and tools run with minimal permissions
3. **Fail closed** — any security check failure denies access
4. **Audit everything** — critical operations are logged with who/what/when/result
5. **No secrets in code** — all credentials in environment variables or secret store

## Threat Model

### Attack Surfaces

| Surface | Protection |
|---------|-----------|
| Voice input | Input validation, injection prevention |
| Agent tool execution | Sandboxed executor, permission whitelist |
| LLM API | Local-only binding (127.0.0.1), no external exposure |
| Network tools | Timeout enforcement, URL validation, no raw socket |
| File system tools | Path sanitization, directory traversal prevention |
| Process tools | Allowlist for launchable binaries |
| Theme installation | JSON schema validation, no executable code |

### Untrusted Inputs

- Voice commands: transcribed text treated as untrusted
- Theme files: validated against JSON schema before loading
- Network responses: truncated, validated, never raw-executed
- User file paths: canonicalized, sandbox-scoped

## Security Stack

```
┌─────────────────────────────────────────┐
│  Audit Log                 (who+when+what+result) │
├─────────────────────────────────────────┤
│  Rate Limiter              (user + IP dual dimension) │
├─────────────────────────────────────────┤
│  Permission Checker        (tool × user × resource)  │
├─────────────────────────────────────────┤
│  Sandbox Executor          (seccomp + rlimit)        │
├─────────────────────────────────────────┤
│  Input Validator           (type + length + format) │
└─────────────────────────────────────────┘
```

## Implementation Status

| Feature | Status |
|---------|--------|
| No hardcoded secrets | ✅ All keys via env / config |
| Sandboxed tool execution | ✅ seccomp filter per tool |
| Path traversal prevention | ✅ fs tools canonicalize paths |
| Input validation | ✅ Type + length + format checks |
| Audit logging | ✅ Structured JSON audit trail |
| Rate limiting | ✅ Per-user + per-IP buckets |
| CORS whitelist | ✅ Exact domain match, no wildcard |
| API key proxy | ✅ LLM key never exposed to frontend |
| Sensitive op confirmation | ✅ Delete/shutdown require double confirm |
| Encrypted sensitive data | ✅ Passwords/tokens at rest AES-256-GCM |

## Secure Defaults

- Agent API binds to `127.0.0.1:8081` (no external network)
- LLM API binds to `127.0.0.1:8080` (local only)
- Voice pipeline captures audio only after wake word detection
- Theme engine rejects files > 1MB and non-JSON content
- Tool executor enforces 30s timeout per operation
- Process manager uses `kill_on_drop` for all child processes

## Auditing

All security-relevant events are logged to `/var/log/forge/audit.json`:

```json
{
  "timestamp": "2026-06-24T22:00:00Z",
  "user": "polaris",
  "action": "tool.execute",
  "tool": "fs_delete",
  "target": "/home/polaris/test.txt",
  "result": "success",
  "ip": "127.0.0.1"
}
```

## Reporting Vulnerabilities

Email: security@forge-os.dev (PGP key available on request)
Do NOT open public issues for security bugs.