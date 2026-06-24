# Forge OS · 锻造操作系统

**AI-native operating system. Voice-first. Community-driven. Built in Rust.**

**AI 原生操作系统。语音优先。社区驱动。Rust 全栈构建。**

---

[English](#english) | [中文](#中文)

---

## English

Forge OS is an AI-native operating system built on the Linux kernel, written entirely in Rust. It features built-in multimodal LLM as its brain, an AI Agent as its operator, and voice command as its primary interface. The GUI is fully customizable — the community can design themes, widgets, and extensions freely, without needing a backend server.

### Architecture

```
  Voice I/O ──→ STT (whisper.cpp) ──→ Agent (OODA) ──→ LLM (llama.cpp)
                                       │
              ┌────────────────────────┤
              ↓                        ↓
         TTS (piper)              Tools (20 tools)
              │                        │
         Speaker              File/Proc/Net/System/Web
```

### Components (8 crates)

| Crate | Size | Purpose |
|-------|------|---------|
| `forge-init` | 670K | PID 1 — mounts filesystems, spawns service manager |
| `forge-svc` | 989K | Service manager — daemon supervision with exponential backoff |
| `forge-hal` | 993K | Hardware abstraction — audio, camera, GPU detection, IPC |
| `forge-llm` | 4.2M | Multimodal LLM engine — llama.cpp CLI backend, HTTP API |
| `forge-voice` | 1.3M | Voice pipeline — wake word, STT (whisper.cpp), TTS (piper) |
| `forge-agent` | 5.3M | AI operator — OODA loop + 20-tool registry |
| `forge-gui` | 1.0M | Desktop GUI — framebuffer compositor, theme engine |
| `forge-theme` | ~500K | Theme CLI — scaffold, validate, package, install |

### Agent Tools (20 tools)

| Category | Tools |
|----------|-------|
| **File System** | list, read, write, delete, mkdir, search, copy, move |
| **Process** | launch, kill, list |
| **System** | info, shutdown, reboot, volume |
| **Network** | http, download |
| **Web** | search, browse |
| **Display** | screenshot |

### Build

```bash
cargo build --release
bash iso-builder/build-iso.sh     # produces bootable ISO (BIOS + UEFI)
```

### Community Themes

No server needed. Create, share, install offline:

1. Read [community/THEME_SPEC.md](community/THEME_SPEC.md)
2. `forge-theme create my-theme`
3. Edit `theme.json`, run `forge-theme pack`
4. Share the `.forge-theme` file via GitHub Releases, USB, email, etc.
5. Others install with `forge-theme install <file/url>`

### Security

- All Agent tools run in a sandboxed executor with permission checks
- API keys stored in environment variables only
- No secrets in source code or logs
- CORS whitelist, rate limiting, audit logging
- See [docs/security.md](docs/security.md)

### License

Apache-2.0

---

## 中文

Forge OS 是一个基于 Linux 内核、Rust 语言全栈构建的 AI 原生操作系统。内置多模态 LLM 作为大脑，AI Agent 作为操作手，语音指令作为主要交互方式。GUI 完全可生态化——社区可自由设计主题、组件和扩展，无需后端服务器。

### 架构

```
  语音输入 ──→ STT (whisper.cpp) ──→ Agent (OODA) ──→ LLM (llama.cpp)
                                        │
               ┌────────────────────────┤
               ↓                        ↓
          TTS (piper)              Tools (20 个工具)
               │                        │
           扬声器              文件/进程/网络/系统/Web
```

### 组件 (8 crates)

| Crate | 大小 | 功能 |
|-------|------|------|
| `forge-init` | 670K | PID 1，挂载文件系统，信号处理 |
| `forge-svc` | 989K | 服务管理器，守护进程监控 + 指数退避重启 |
| `forge-hal` | 993K | 硬件抽象层，音频/摄像头/GPU 检测，IPC |
| `forge-llm` | 4.2M | 多模态 LLM 引擎，llama.cpp CLI 后端，HTTP API |
| `forge-voice` | 1.3M | 语音流水线，唤醒词 + STT (whisper.cpp) + TTS (piper) |
| `forge-agent` | 5.3M | AI 操作手，OODA 循环 + 20 工具注册表 |
| `forge-gui` | 1.0M | 图形界面，framebuffer 渲染，主题引擎 |
| `forge-theme` | ~500K | 社区主题 CLI，脚手架/校验/打包/安装 |

### Agent 工具 (20 个)

| 类别 | 工具 |
|------|------|
| **文件系统** | 列出、读取、写入、删除、创建目录、搜索、复制、移动 |
| **进程** | 启动、杀死、列表 |
| **系统** | 信息、关机、重启、音量 |
| **网络** | HTTP 请求、下载 |
| **Web** | 搜索、浏览 |
| **显示** | 截图 |

### 构建

```bash
cargo build --release
bash iso-builder/build-iso.sh     # 生成可启动 ISO (BIOS + UEFI)
```

### 社区主题市场

无需服务器。离线创建、分享、安装：

1. 阅读 [community/THEME_SPEC.md](community/THEME_SPEC.md)
2. `forge-theme create my-theme`
3. 编辑 `theme.json`，运行 `forge-theme pack`
4. 通过 GitHub Release、U 盘、邮件等分享 `.forge-theme` 文件
5. 别人用 `forge-theme install <文件/URL>` 安装

### 安全设计

- 所有 Agent 工具在沙箱执行器中运行，权限逐项校验
- API 密钥仅存环境变量，不在代码和日志中留痕
- CORS 白名单、速率限制、审计日志
- 敏感操作二次确认
- 详见 [docs/security.md](docs/security.md)

### 许可证

Apache-2.0