# Forge OS

**AI-native operating system. Rust from the kernel up. Voice-first. Community-driven GUI.**

Forge OS 是一个基于 Linux 内核、Rust 语言构建的 AI 原生操作系统。通过语音指令控制，内置多模态 LLM 作为大脑，AI Agent 作为操作手。GUI 完全可生态化，社区可自由设计主题、组件和扩展。

![Forge Logo](assets/logo.svg)

---

## 版本

| 版本 | 日期 | ISO | 里程碑 |
|------|------|-----|--------|
| **v0.2.0** | 2026-06-24 | 82 MB | 8 binaries, 20 tools, UEFI, community SDK |
| v0.1.0 | 2026-06-24 | 77 MB | 7 binaries, bootable ISO

---

## 架构

```
  Voice I/O ──→ STT (whisper.cpp) ──→ Agent (OODA) ──→ LLM (llama.cpp)
                                       │
              ┌────────────────────────┤
              ↓                        ↓
         TTS (piper)              Tools (20 tools)
              │                        │
         Speaker                   File/Proc/Net/System
```

---

## 组件 (8 crates)

| Crate | 大小 | 功能 |
|-------|------|------|
| `forge-init` | 670K | PID 1，挂载文件系统，信号处理 |
| `forge-svc` | 989K | 服务管理器，守护进程监控 + 指数退避重启 |
| `forge-hal` | 993K | 硬件抽象层，音频/摄像头/GPU 检测，IPC |
| `forge-llm` | 4.2M | 多模态 LLM 引擎，llama.cpp CLI 后端，HTTP API |
| `forge-voice` | 1.3M | 语音流水线，whisper.cpp STT + piper TTS |
| `forge-agent` | 5.3M | AI 操作手，OODA 循环 + 20 工具注册表 |
| `forge-gui` | 1.0M | 图形界面，framebuffer 渲染，主题引擎 |
| `forge-theme` | ~500K | 社区主题 CLI，创建/校验/打包/安装 |

---

## Agent 工具 (20 tools)

| 类别 | 工具 |
|------|------|
| **文件系统** | fs_list, fs_read, fs_write, fs_delete, fs_mkdir, fs_search, fs_copy, fs_move |
| **进程** | proc_launch, proc_kill, proc_list |
| **系统** | sys_info, sys_shutdown, sys_reboot, sys_volume |
| **网络** | net_http, net_download |
| **Web** | web_search, web_browse |
| **显示** | display_screenshot |

---

## 品牌

![Icon](assets/icon.svg)

| 资源 | 说明 |
|------|------|
| 主色 | `#FF6B35` Forge Orange → `#F7931E` Amber → `#FFD200` Gold |
| 暗色底 | `#0a0a0f` Deep Void |
| 字体 | Inter (UI) + JetBrains Mono (Code) |

---

## 构建

```bash
# 编译
cargo build --release

# 构建 ISO (BIOS + UEFI)
bash iso-builder/build-iso.sh
```

## 首次启动

```bash
# 下载 AI 模型（ISO 不含模型，启动后按需下载）
forge-model-dl

# 管理社区主题
forge-theme list          # 已安装主题
forge-theme create mine   # 创建新主题
forge-theme install url   # 安装社区主题
forge-theme set "Neon Nights"
```

---

## 社区主题市场

无需后端服务器，零成本分发：

1. 阅读 [community/THEME_SPEC.md](community/THEME_SPEC.md)
2. `forge-theme create my-theme` 脚手架
3. 编辑 `theme.json`，`forge-theme pack`
4. 丢 GitHub Release 或直接 `.forge-theme` 文件分享
5. 别人 `forge-theme install <文件/URL>` 即可

详见 [community/README.md](community/README.md)

---

## 路线图

- [x] 架构设计 + Rust 项目骨架
- [x] PID 1 init + 服务管理器
- [x] HAL (音频/摄像头/GPU)
- [x] LLM 引擎 (llama.cpp + GGUF)
- [x] 语音流水线 (whisper.cpp + piper)
- [x] Agent OODA + 20 工具
- [x] GUI 渲染 (framebuffer compositor)
- [x] 品牌 Logo + 视觉规范
- [x] 社区主题 CLI + SDK + 规范
- [x] UEFI + BIOS 双启动 ISO
- [x] 模型下载器 (forge-model-dl)
- [x] 推送到 GitHub
- [ ] 集成真实 GGUF 模型到 ISO
- [ ] GPU 加速推理 (Vulkan/CUDA)
- [ ] 字体渲染引擎 (fontdue)
- [ ] 窗口管理器
- [ ] 文件管理器 GUI
- [ ] 系统设置面板
- [ ] ISO 瘦身 < 50MB
- [ ] v1.0 正式发布

---

## 许可证

Apache-2.0