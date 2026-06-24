# Changelog / 更新日志

All notable changes to Forge OS will be documented in this file.

## [1.1.0] — 2024-06-24

### Added / 新增
- **forge-guard**: Human-centric security layer with 5 subsystems
  - `auth`: Passphrase-based authentication with attempt limits
  - `confirm`: Risk-classified operation confirmation (3 levels: low/medium/critical)
  - `monitor`: Behavior anomaly detection with hourly rate tracking
  - `lock`: System lock after repeated failed attempts
  - `audit`: Full audit trail logging (who + time + action + result)
- `guard.json` runtime config deployed to `/etc/forge/`
- Audit log directory `/var/log/forge/` in rootfs

### Changed / 变更
- README updated: 9 crates (was 8), human-centric security section
- Cargo.toml: version bumped to 1.1.0, repository URL fixed
- ISO builder script updated to include forge-guard binary

### Security / 安全
- Sensitive operations (shutdown, reboot, delete, kill) require explicit confirmation
- Behavior monitoring flags unusual command patterns
- Lockout mechanism after repeated authentication failures

---

## [1.0.0] — 2024-06-23

### Initial Release / 首发

- 8 core crates: forge-init, forge-svc, forge-hal, forge-llm, forge-voice, forge-agent, forge-gui, forge-theme
- Linux kernel + Rust userspace, UEFI + BIOS dual-boot ISO
- Voice pipeline: wake word → STT (whisper.cpp) → Agent → TTS (piper)
- AI Agent with OODA loop + 20 tools (file, process, system, network, web, display)
- Multimodal LLM engine via llama.cpp CLI backend with GPU acceleration
- Framebuffer-based GUI with theme engine and community SDK
- Community theme marketplace — offline, serverless, `.forge-theme` packaging
- Brand assets: logo, icon, color palette (orange-gold gradient)