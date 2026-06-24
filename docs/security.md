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
---

## Human-Centric Security / 人因安全

"The biggest threat isn't the code — it's the person sitting at the keyboard."

最大威胁不是代码，是坐在键盘前的人。

### Social Engineering Protection / 防社工

| Attack / 攻击 | Defense / 防护 |
|---------------|----------------|
| Phishing links / 钓鱼链接 | URL reputation check before opening |
| Impersonation calls / 冒充电话 | Voiceprint verification for sensitive ops |
| "Urgent" scams / 紧急诈骗 | Rate-limit + cool-down on critical operations |
| Fake support / 假冒客服 | Built-in support channel verification |

### Anti-Misoperation / 防误操作

| Operation / 操作 | Confirmation / 确认要求 |
|-----------------|------------------------|
| Delete files / 删除文件 | Single confirm + are-you-sure prompt |
| System shutdown / 关机 | Double confirm with challenge question |
| Process kill / 杀进程 | Confirm PID + process name |
| Network download / 下载 | URL preview + file size warning |
| Reboot / 重启 | Triple confirm + 10s grace period |

### Mandatory Security / 强制安全

| Feature / 功能 | Description / 说明 |
|----------------|-------------------|
| PBKDF2 passphrases / PBKDF2 暗号 | 100,000 iterations, salted, SHA-256 |
| Auto-lockout / 自动锁定 | 5 failed attempts → 5 minute lockout |
| Idle auto-lock / 空闲锁屏 | Screen locks after 5 min idle, clipboard cleared |
| Voiceprint auth / 声纹认证 | Speaker verification for high-risk operations |
| Behavioral anomaly detection / 行为异常检测 | Flags command floods, after-hours activity, rapid destructive ops |
| USB device guard / USB 设备管控 | Unauthorized USB devices blocked by default |
| Application signing / 应用签名 | Only signed binaries can execute |

### Password/Passphrase Policy / 暗号策略

- Minimum 12 characters, mixed case, numbers, symbols
- No dictionary words (checked against common password list)
- Must differ from last 5 passphrases
- Expires every 90 days (configurable)
- Never displayed on screen, never logged

### Audit Trail / 审计追踪

Every security-relevant event is logged in human-readable format:

```
[TIMESTAMP] [USER] [ACTION] [TARGET] [RESULT]
2026-06-24 22:00:00 polaris fs_delete /home/polaris/secret.txt DENIED (risk:high)
2026-06-24 22:00:05 polaris authenticate   success (voiceprint match)
2026-06-24 22:00:10 polaris fs_delete /home/polaris/secret.txt APPROVED (double-confirmed)
```

View with: `forge-guard audit --human`