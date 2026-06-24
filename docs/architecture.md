# Forge OS 架构设计文档

## 设计目标

| 目标 | 描述 |
|------|------|
| AI 原生 | 不是"操作系统上跑个 AI 应用"，而是 AI 就是操作系统的一部分 |
| 语音优先 | 主要交互方式是人类语音指令 |
| 内置 LLM | 无需网络连接，本地运行多模态大模型 |
| 社区驱动 GUI | 用户可自行设计主题、桌面组件、扩展 |
| Rust 全栈 | 从内核级到用户态，全 Rust 编写 |

## 分层架构

### Layer 0: Linux Kernel
- 最小化内核，仅包含必要驱动
- GPU (Intel/AMD/NVIDIA Vulkan/CUDA), Audio (ALSA), USB, Network
- 作为 Rust 之上的可靠底层基石

### Layer 1: Forge Init + Service Manager
- `forge-init` 作为 PID 1
- 挂载虚拟文件系统，创建设备节点
- 启动 `forge-svc` 进行服务管理
- 子进程回收、信号处理、关机流程

### Layer 2: Forge HAL
- 音频设备检测与捕获
- 摄像头枚举与帧抓取
- GPU 能力检测 (Vulkan/CUDA 后端选择)
- Unix Socket IPC 供其他服务查询硬件能力

### Layer 3: Forge LLM
- 基于 llama.cpp 的 GGUF 模型推理
- 支持文本 + 视觉多模态
- HTTP API (兼容部分 OpenAI 格式)
- 自动选择 GPU/CPU 后端

### Layer 4: Forge Voice + Agent
- 语音流水线: 唤醒词 → STT → 意图分类 → Agent → TTS
- Agent OODA 循环: 观察 → 定向 → 决策 → 执行
- 工具注册表: 文件系统、进程、网络、系统、显示、Web

### Layer 5: Forge GUI
- 可换肤的主题引擎 (JSON + 资源文件)
- 桌面组件系统 (时钟、天气、系统监控、音乐...)
- 社区生态: 主题市场、组件分享、扩展框架
- Compositor: 基于 wgpu/softbuffer 渲染

## 启动流程

```
BIOS/UEFI
  → GRUB2
    → Linux Kernel
      → forge-init (PID 1)
        → 挂载 /proc /sys /dev /run /tmp
        → 创建 /dev/* 设备节点
        → fork + exec forge-svc (PID 2)
          → 启动 forge-hal
          → 启动 forge-llm
          → 启动 forge-voice
          → 启动 forge-agent
          → 启动 forge-gui
        → 主循环: 回收子进程 + 信号处理
```

## 数据流

```
麦克风 → [Wake Word] ──检测到──→ [STT] ──文本──→ [Agent OODA]
                                                        │
                                              ┌─────────┘
                                              ↓
                                        [LLM 推理]
                                              │
                                              ↓
                                        动作决策 + 回复
                                         │          │
                                    ┌────┘          └────┐
                                    ↓                      ↓
                               [TTS → 扬声器]        [工具执行]
                                                      │
                                          ┌───────────┤
                                          ↓           ↓
                                       [文件]     [进程/网络/系统]
```
