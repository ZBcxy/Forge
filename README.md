# Forge OS

**AI-native operating system. Rust from the kernel up. Voice-first. Community-driven GUI.**

Forge OS 是一个基于 Linux 内核、Rust 语言构建的 AI 原生操作系统。通过语音指令控制，内置多模态 LLM 作为大脑，AI Agent 作为操作手。GUI 完全可生态化，社区可自由设计主题、组件和扩展。

## 架构

```
┌──────────────────────────────────────────────────────────┐
│                   Forge OS Architecture                    │
├───────────────┬──────────────┬──────────────┬────────────┤
│  Voice Input  │    Display   │   Camera     │  Ecosystem │
│  唤醒词 → STT │  主题 → 渲染  │  视觉输入     │  社区市场   │
├───────────────┴──────────────┴──────────────┴────────────┤
│              Forge Agent (OODA Loop)                      │
│          Observe → Orient → Decide → Act                 │
├──────────────────────────────────────────────────────────┤
│              Forge LLM (多模态推理引擎)                     │
│         llama.cpp / GGUF / Text + Vision                  │
├──────────────────────────────────────────────────────────┤
│       Forge Services (svc manager → init → kernel)        │
├──────────────────────────────────────────────────────────┤
│  Linux Kernel  |  GPU (Vulkan/CUDA)  |  Audio  |  USB    │
└──────────────────────────────────────────────────────────┘
```

## 组件

| Crate | 功能 | 二进制 |
|-------|------|--------|
| `forge-init` | PID 1 — 挂载文件系统、启动服务管理器 | `/usr/bin/forge-init` |
| `forge-svc` | 服务管理器 — 守护进程监控与自动重启 | `/usr/bin/forge-svc` |
| `forge-hal` | 硬件抽象层 — 音频/摄像头/GPU 检测 | `/usr/bin/forge-hal` |
| `forge-llm` | 多模态推理引擎 — HTTP API | `/usr/bin/forge-llm` |
| `forge-voice` | 语音流水线 — 唤醒词 + STT + TTS | `/usr/bin/forge-voice` |
| `forge-agent` | AI Agent — OODA 循环执行器 | `/usr/bin/forge-agent` |
| `forge-gui` | 图形界面 — 主题系统 + 组件 + 社区 | `/usr/bin/forge-gui` |

## 构建

```bash
# 1. 编译所有 Rust 组件
cargo build --release

# 2. 构建 ISO (需要 mkosi + xorriso)
cd iso-builder
chmod +x build-iso.sh
sudo ./build-iso.sh
```

## 社区生态

Forge GUI 完全可定制：

```
/opt/forge/themes/
├── builtin/          # 内置主题
│   └── default/
│       ├── theme.json
│       └── assets/
└── community/        # 社区主题市场
    └── <user-theme>/
        ├── theme.json
        ├── assets/
        └── README.md

/opt/forge/widgets/   # 桌面组件 (时钟、天气、系统监控...)
~/.forge/themes/      # 用户个人主题
```

## 许可证

Apache-2.0

## 路线图

- [x] 架构设计
- [x] 项目骨架
- [x] Init 系统 + 服务管理
- [x] HAL + LLM 引擎
- [x] 语音流水线
- [x] Agent OODA 循环
- [x] GUI 主题引擎 + 社区生态
- [x] ISO 构建脚本
- [ ] 在 VM 上编译验证
- [ ] 生成首个可启动 ISO
- [ ] 集成真实 LLM 模型
- [ ] 集成 Whisper STT
- [ ] GPU 加速推理
- [ ] 社区主题市场上线
- [ ] v1.0 发布
