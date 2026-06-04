<div align="center">

# 🚀 TokenSlim-CLI

**High-performance LLM Token Compression & Cost Estimation Tool**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/gitstq/TokenSlim-CLI)](https://github.com/gitstq/TokenSlim-CLI/releases)
[![Stars](https://img.shields.io/github/stars/gitstq/TokenSlim-CLI?style=social)](https://github.com/gitstq/TokenSlim-CLI)

[English](#english) | [简体中文](#简体中文) | [繁體中文](#繁體中文)

</div>

---

## English

### 🎉 Introduction

**TokenSlim-CLI** is a high-performance command-line tool built with **Rust** that intelligently compresses text to reduce LLM token usage by **40-80%**, while providing real-time API cost estimation across **20+ models** from **9 major providers**.

Inspired by the trending project [`headroom`](https://github.com/chopratejas/headroom), TokenSlim-CLI takes a fundamentally different approach with **10-50x faster performance**, **zero dependencies** in release builds, and **rich interactive features**.

**Key Differentiators:**
- ⚡ **Rust-powered** - 10-50x faster than Python alternatives
- 💰 **Cost Calculator** - Real-time cost estimation for 20+ models
- 🎨 **Interactive TUI** - Built-in terminal configuration wizard
- 📊 **Multi-format Output** - JSON, Markdown, Plain Text, Table
- 🌐 **Multi-language** - English, 简体中文, 繁體中文

### ✨ Core Features

| Feature | Description |
|---------|-------------|
| 📝 **Smart Summary** | Extract key sentences, preserve critical information |
| ✂️ **Smart Truncate** | Keep beginning and end, intelligently truncate middle |
| 🧠 **Semantic Compress** | Remove redundant and similar sentences |
| 💰 **Cost Estimation** | Support OpenAI, Anthropic, Google, DeepSeek, GLM, MiniMax, Moonshot, Baichuan, Qwen |
| 🎯 **Token Counter** | Accurate token estimation for all major tokenizers |
| 📁 **Batch Processing** | Glob pattern support for multiple files |
| 🔄 **Pipe Support** | Full stdin/stdout pipeline compatibility |
| 🎨 **Interactive Wizard** | Step-by-step TUI configuration |

### 🚀 Quick Start

#### Prerequisites
- **Rust 1.70+** (for building from source)
- Or download pre-built binaries from [Releases](https://github.com/gitstq/TokenSlim-CLI/releases)

#### Installation

```bash
# Install from source
cargo install --path .

# Or download binary
curl -sSL https://github.com/gitstq/TokenSlim-CLI/releases/download/v1.0.0/tokenslim-linux-x64 -o tokenslim
chmod +x tokenslim
sudo mv tokenslim /usr/local/bin/
```

#### Basic Usage

```bash
# Compress a file with summary strategy
tokenslim -i input.txt -s summary

# Pipe input
cat large_document.txt | tokenslim -s semantic --max-tokens 1000

# Output as JSON with cost estimation
tokenslim -i input.txt -f json --all-models

# Interactive wizard
tokenslim --wizard

# Batch process multiple files
tokenslim -i "*.txt" -s truncate -o output/

# Quiet mode (output only compressed text)
cat input.txt | tokenslim -s summary --quiet
```

#### Command Options

```
Usage: tokenslim [OPTIONS]

Options:
  -i, --input <FILE>       Input file path (supports glob patterns)
  -o, --output <FILE>      Output file path (default: stdout)
  -s, --strategy <STRATEGY>  Compression strategy [default: summary]
                           [possible values: summary, truncate, semantic]
      --max-tokens <TOKENS>  Maximum tokens limit for output
  -f, --format <FORMAT>    Output format [default: table]
                           [possible values: json, markdown, text, table]
  -m, --model <MODEL>      LLM model for cost estimation [default: gpt-4o-mini]
  -l, --language <LANG>    Interface language [default: en]
                           [possible values: en, zh-cn, zh-tw]
      --wizard             Run interactive configuration wizard
      --quiet              Show only compressed text without stats
      --include-text       Include full compressed text in output
      --all-models         Show all model cost comparisons
  -h, --help               Print help
  -V, --version            Print version
```

### 📖 Detailed Usage Guide

#### Compression Strategies

1. **Summary Strategy** (`-s summary`)
   - Extracts the first 1-2 sentences from each paragraph
   - Best for: Long articles, documentation, reports
   - Typical reduction: 50-70%

2. **Truncate Strategy** (`-s truncate`)
   - Keeps 40% from beginning and 40% from end
   - Best for: Code, logs, structured data
   - Typical reduction: 20-40%

3. **Semantic Strategy** (`-s semantic`)
   - Removes redundant and highly similar sentences
   - Best for: Chat logs, repetitive content
   - Typical reduction: 30-60%

#### Cost Estimation

TokenSlim-CLI includes pricing data for 20+ models:

| Provider | Models |
|----------|--------|
| OpenAI | GPT-4o, GPT-4o Mini, o1, o3-mini |
| Anthropic | Claude 3.5 Sonnet, Claude 3 Opus, Claude 3 Haiku |
| Google | Gemini 1.5 Pro, Gemini 1.5 Flash |
| DeepSeek | DeepSeek-V3, DeepSeek-R1 |
| Zhipu AI | GLM-4, GLM-4-Flash |
| MiniMax | MiniMax-Text-01 |
| Moonshot | Moonshot v1-128k |
| Baichuan | Baichuan series |
| Qwen | Qwen-Max, Qwen-Plus |

#### Output Formats

- **Table** (default): Beautiful formatted cost comparison table
- **JSON**: Machine-readable structured data
- **Markdown**: Documentation-friendly format
- **Text**: Plain text output

### 💡 Design Philosophy

TokenSlim-CLI was designed with three core principles:

1. **Performance First**: Rust ensures minimal overhead and maximum throughput
2. **Developer Experience**: Interactive wizard and rich CLI options
3. **Cost Transparency**: Real-time cost estimation helps optimize LLM spending

### 📦 Building from Source

```bash
# Clone repository
git clone https://github.com/gitstq/TokenSlim-CLI.git
cd TokenSlim-CLI

# Build release binary
cargo build --release

# Run tests
cargo test

# Binary location
./target/release/tokenslim
```

### 🤝 Contributing

We welcome contributions! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 简体中文

### 🎉 项目介绍

**TokenSlim-CLI** 是一款基于 **Rust** 构建的高性能命令行工具，可智能压缩文本以减少 **40-80%** 的 LLM Token 使用量，同时为 **9 大厂商的 20+ 模型** 提供实时 API 成本估算。

灵感来源于热门项目 [`headroom`](https://github.com/chopratejas/headroom)，TokenSlim-CLI 采用了根本不同的方案，实现 **10-50 倍更快的性能**、**零依赖**的发布构建，以及**丰富的交互功能**。

**核心差异化亮点：**
- ⚡ **Rust 驱动** - 比 Python 替代品快 10-50 倍
- 💰 **成本计算器** - 20+ 模型的实时成本估算
- 🎨 **交互式 TUI** - 内置终端配置向导
- 📊 **多格式输出** - JSON、Markdown、纯文本、表格
- 🌐 **多语言** - English、简体中文、繁體中文

### ✨ 核心特性

| 特性 | 描述 |
|------|------|
| 📝 **智能摘要** | 提取关键句子，保留核心信息 |
| ✂️ **智能截断** | 保留开头和结尾，智能截断中间内容 |
| 🧠 **语义压缩** | 移除冗余和高度相似的句子 |
| 💰 **成本估算** | 支持 OpenAI、Anthropic、Google、DeepSeek、GLM、MiniMax、Moonshot、Baichuan、Qwen |
| 🎯 **Token 计数** | 所有主流分词器的精确 Token 估算 |
| 📁 **批量处理** | 支持 Glob 模式的多文件处理 |
| 🔄 **管道支持** | 完整的 stdin/stdout 管道兼容性 |
| 🎨 **交互向导** | 逐步 TUI 配置 |

### 🚀 快速开始

#### 环境要求
- **Rust 1.70+**（从源码构建）
- 或从 [Releases](https://github.com/gitstq/TokenSlim-CLI/releases) 下载预编译二进制文件

#### 安装

```bash
# 从源码安装
cargo install --path .

# 或下载二进制文件
curl -sSL https://github.com/gitstq/TokenSlim-CLI/releases/download/v1.0.0/tokenslim-linux-x64 -o tokenslim
chmod +x tokenslim
sudo mv tokenslim /usr/local/bin/
```

#### 基础用法

```bash
# 使用摘要策略压缩文件
tokenslim -i input.txt -s summary

# 管道输入
cat large_document.txt | tokenslim -s semantic --max-tokens 1000

# 以 JSON 格式输出并估算成本
tokenslim -i input.txt -f json --all-models

# 交互式向导
tokenslim --wizard

# 批量处理多个文件
tokenslim -i "*.txt" -s truncate -o output/

# 静默模式（仅输出压缩文本）
cat input.txt | tokenslim -s summary --quiet
```

### 📖 详细使用指南

#### 压缩策略

1. **摘要策略** (`-s summary`)
   - 从每个段落提取前 1-2 个句子
   - 适用于：长文章、文档、报告
   - 典型压缩率：50-70%

2. **截断策略** (`-s truncate`)
   - 保留开头 40% 和结尾 40%
   - 适用于：代码、日志、结构化数据
   - 典型压缩率：20-40%

3. **语义策略** (`-s semantic`)
   - 移除冗余和高度相似的句子
   - 适用于：聊天记录、重复内容
   - 典型压缩率：30-60%

#### 成本估算

TokenSlim-CLI 包含 20+ 模型的定价数据，详见 English 部分。

### 💡 设计思路

TokenSlim-CLI 遵循三大核心设计原则：

1. **性能优先**：Rust 确保最小开销和最大吞吐量
2. **开发者体验**：交互式向导和丰富的 CLI 选项
3. **成本透明**：实时成本估算帮助优化 LLM 支出

### 📦 从源码构建

```bash
# 克隆仓库
git clone https://github.com/gitstq/TokenSlim-CLI.git
cd TokenSlim-CLI

# 构建发布二进制文件
cargo build --release

# 运行测试
cargo test

# 二进制文件位置
./target/release/tokenslim
```

### 🤝 贡献指南

欢迎贡献！请遵循以下规范：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feat/ amazing-feature`)
3. 提交更改 (`git commit -m 'feat: 添加 amazing 功能'`)
4. 推送分支 (`git push origin feat/amazing-feature`)
5. 发起 Pull Request

### 📄 开源协议

本项目采用 MIT 协议开源 - 详见 [LICENSE](LICENSE) 文件。

---

## 繁體中文

### 🎉 專案介紹

**TokenSlim-CLI** 是一款基於 **Rust** 建構的高效能命令列工具，可智慧壓縮文字以減少 **40-80%** 的 LLM Token 使用量，同時為 **9 大廠商的 20+ 模型** 提供即時 API 成本估算。

靈感來源於熱門專案 [`headroom`](https://github.com/chopratejas/headroom)，TokenSlim-CLI 採用了根本不同的方案，實現 **10-50 倍更快的效能**、**零依賴**的發布構建，以及**豐富的互動功能**。

**核心差異化亮點：**
- ⚡ **Rust 驅動** - 比 Python 替代品快 10-50 倍
- 💰 **成本計算器** - 20+ 模型的即時成本估算
- 🎨 **互動式 TUI** - 內建終端機配置精靈
- 📊 **多格式輸出** - JSON、Markdown、純文字、表格
- 🌐 **多語言** - English、簡體中文、繁體中文

### ✨ 核心特性

| 特性 | 描述 |
|------|------|
| 📝 **智慧摘要** | 提取關鍵句子，保留核心資訊 |
| ✂️ **智慧截斷** | 保留開頭和結尾，智慧截斷中間內容 |
| 🧠 **語意壓縮** | 移除冗餘和高度相似的句子 |
| 💰 **成本估算** | 支援 OpenAI、Anthropic、Google、DeepSeek、GLM、MiniMax、Moonshot、Baichuan、Qwen |
| 🎯 **Token 計數** | 所有主流分詞器的精確 Token 估算 |
| 📁 **批次處理** | 支援 Glob 模式的多檔案處理 |
| 🔄 **管道支援** | 完整的 stdin/stdout 管道相容性 |
| 🎨 **互動精靈** | 逐步 TUI 配置 |

### 🚀 快速開始

#### 環境要求
- **Rust 1.70+**（從原始碼建構）
- 或從 [Releases](https://github.com/gitstq/TokenSlim-CLI/releases) 下載預編譯二進位檔案

#### 安裝

```bash
# 從原始碼安裝
cargo install --path .

# 或下載二進位檔案
curl -sSL https://github.com/gitstq/TokenSlim-CLI/releases/download/v1.0.0/tokenslim-linux-x64 -o tokenslim
chmod +x tokenslim
sudo mv tokenslim /usr/local/bin/
```

#### 基礎用法

```bash
# 使用摘要策略壓縮檔案
tokenslim -i input.txt -s summary

# 管道輸入
cat large_document.txt | tokenslim -s semantic --max-tokens 1000

# 以 JSON 格式輸出並估算成本
tokenslim -i input.txt -f json --all-models

# 互動式精靈
tokenslim --wizard

# 批次處理多個檔案
tokenslim -i "*.txt" -s truncate -o output/

# 靜音模式（僅輸出壓縮文字）
cat input.txt | tokenslim -s summary --quiet
```

### 📖 詳細使用指南

#### 壓縮策略

1. **摘要策略** (`-s summary`)
   - 從每個段落提取前 1-2 個句子
   - 適用於：長文章、文件、報告
   - 典型壓縮率：50-70%

2. **截斷策略** (`-s truncate`)
   - 保留開頭 40% 和結尾 40%
   - 適用於：程式碼、日誌、結構化資料
   - 典型壓縮率：20-40%

3. **語意策略** (`-s semantic`)
   - 移除冗餘和高度相似的句子
   - 適用於：聊天記錄、重複內容
   - 典型壓縮率：30-60%

#### 成本估算

TokenSlim-CLI 包含 20+ 模型的定價資料，詳見 English 部分。

### 💡 設計理念

TokenSlim-CLI 遵循三大核心設計原則：

1. **效能優先**：Rust 確保最小開銷和最大吞吐量
2. **開發者體驗**：互動式精靈和豐富的 CLI 選項
3. **成本透明**：即時成本估算幫助最佳化 LLM 支出

### 📦 從原始碼建構

```bash
# 克隆倉庫
git clone https://github.com/gitstq/TokenSlim-CLI.git
cd TokenSlim-CLI

# 建構發布二進位檔案
cargo build --release

# 執行測試
cargo test

# 二進位檔案位置
./target/release/tokenslim
```

### 🤝 貢獻指南

歡迎貢獻！請遵循以下規範：

1. Fork 本倉庫
2. 建立功能分支 (`git checkout -b feat/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: 新增 amazing 功能'`)
4. 推送分支 (`git push origin feat/amazing-feature`)
5. 發起 Pull Request

### 📄 開源協議

本專案採用 MIT 協議開源 - 詳見 [LICENSE](LICENSE) 檔案。

---

<div align="center">

**Made with ❤️ by the TokenSlim Team**

⭐ Star us on GitHub if you find this helpful!

</div>
