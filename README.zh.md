# Prism - AI翻译软件

<div align="center">

**[中文](./README.zh.md) | [English](./README.md) | [Español](./README.es.md) | [日本語](./README.ja.md) | [한국어](./README.ko.md) | [Tiếng Việt](./README.vi.md)**

🚀 一款强大的跨平台AI翻译应用，采用先进的语言模型和OCR技术。

[下载](#下载) • [功能特性](#功能特性) • [快速开始](#快速开始) • [文档](#文档)

</div>

---
<img src="icon.png" alt="图标" width="350" height="350">

## 功能特性

- **🌍 跨平台支持** - 完美支持 Windows、macOS 和 Linux，功能完全一致
- **🤖 先进的AI翻译** - 完全自由的ai配置接口
- **📸 内置OCR识别** - 完全自由的ai配置接口
- **⚡ 极速翻译** - 实时翻译，延迟最小化
- **🎯 易用的界面** - 基于Vue 3的直观用户界面，交互流畅
- **🔗 全局快捷键** - 自定义快捷键快速访问（开发中）
- **💾 本地翻译历史** - 基于SQLite的翻译记录存储
- **🎨 现代化架构** - 基于Tauri + Rust构建，性能优异且安全可靠

---

## 技术栈

### 前端技术

- **Vue 3** (3.5.13) - 现代化的渐进式JavaScript框架
- **Vite** (6.0.3) - 下一代构建工具
- **Tauri UI组件** - 原生级桌面应用体验

### 后端技术

- **Rust** (2021 edition) - 高性能系统编程语言
- **Tauri** (2.9.3) - 轻量级桌面应用框架
- **Tokio** (1.48.0) - 异步运行时，支持并发操作

### AI与处理

- **推荐翻译模型** - Qwen3-Next-80B-A3B-Instruct
- **OCR模型** - Qwen3-VL-30B-A3B-Instruct
- **API提供商** - 硅基流动 (SiliconFlow)
- **完全支持openai协议的所有AI自定义**

### 存储与工具库

- **SQLite** (rusqlite 0.37.0) - 本地数据库存储
- **Reqwest** (0.12.24) - HTTP客户端
- **图像处理** (0.25.9) - 截图和图像处理
- **全局快捷键** (2.3.1) - 键盘快捷键插件

---

## 快速开始

### 环境要求

- Rust 1.91.0 或更高版本
- Node.js 18+ 和 pnpm
- Git

### 安装步骤

**1. 克隆仓库** 
``` bash
git clone https://github.com/qyzhg/prism.git cd prism
```

**2. 安装依赖**
#### 安装前端依赖
```bash
pnpm install
```

#### Rust依赖由Cargo管理
**3. 获取API密钥**
- 使用自己的兼容openai的baseurl和API密钥，即可开始使用。
- 使用邀请链接注册硅基流动账户：[https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq)注册后会获得赠金额度（有效期很长）

**4. 开发模式运行**
```bash
pnpm tauri dev
```
**5. 生产环境构建**
```bash
pnpm tauri build
```

## 自动更新 & 发布

借助内置的 `tauri-plugin-updater` 插件和 `.github/workflows/release.yml`，项目可以通过 GitHub Releases 分发更新，并在客户端内自动检测/安装。

### 1. 生成签名密钥

Tauri Updater 需要对安装包进行签名：

```bash
pnpm tauri signer generate
```

命令会输出 **Private Key**、**Public Key** 以及可选的密码。请妥善保存：

- `TAURI_PRIVATE_KEY`：复制整段私钥（含 BEGIN/END）到 GitHub 仓库的 Secrets。
- `TAURI_PUBLIC_KEY`：复制公钥，分别配置在本地环境变量（构建或调试时导出）以及 GitHub Secrets 中。
- `TAURI_KEY_PASSWORD`：如果生成密钥时设置了密码，也需要作为 Secret 写入。

本地开发可在 shell 中导出公钥，例如：

```bash
export TAURI_PUBLIC_KEY="your-public-key"
```

### 2. 配置 GitHub Actions

在仓库的 **Settings → Secrets and variables → Actions** 中新增：

| 名称 | 说明 |
| --- | --- |
| `TAURI_PRIVATE_KEY` | 第一步生成的私钥内容 |
| `TAURI_PUBLIC_KEY` | 与 `tauri.conf.json` 一致的公钥 |
| `TAURI_KEY_PASSWORD` | 如果设置了密码则必填，未设置可留空 |

`release` 工作流会在 macOS / Windows / Linux 三个平台构建应用、上传产物，并生成 `latest.json` 供客户端更新使用。

### 3. 触发发布

1. 更新 `package.json` 以及 `src-tauri/tauri.conf.json` 中的版本号。
2. 提交并 push。
3. 在 GitHub 上创建 **Release**（或手动触发 `Release` workflow）。

工作流执行完成后，Release 页面会包含各平台安装包以及 `latest.json`。

### 4. 客户端更新体验

- 应用启动后会自动检查一次更新，并通过系统对话框提示是否下载。
- “设置 → 软件更新” 中可以查看当前版本、手动检查以及执行更新，过程会展示下载进度。
- 所有更新包都来自 GitHub Releases，并通过签名验证，确保来源可靠。

---

## 下载

| 平台 | 下载链接 |
|------|---------|
| 🪟 Windows | [最新版本](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [最新版本](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | 敬请期待 |

### macOS 安装提示

我们只能使用免费（Ad-hoc）的签名方式，苹果不会信任没有付费证书的应用，所以第一次运行时仍然会出现“Prism.app 已损坏/无法打开”的 Gatekeeper 警告，这是所有开源免费应用的常态。解决方法：

1. 将 `Prism.app` 拖到 `/Applications`。
2. 打开 **终端** 执行：
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. 回到 Finder，右键（或按住 Control 点击）`Prism.app` 选择“打开”，确认一次即可。之后就能像普通应用一样直接打开。

---

## 文档

### 配置说明

在设置面板中配置您的翻译偏好：

- 默认语言对选择
- API密钥管理
- 快捷键自定义（开发中）

### 快捷键

当前开发中 - 敬请期待

### AI模型

- **翻译模型** - `Qwen/Qwen3-Next-80B-A3B-Instruct` 企业级多语言翻译
- **OCR模型** - `Qwen/Qwen3-VL-30B-A3B-Instruct` 高级视觉语言理解

---

## 开发路线图

- [x] 核心翻译功能
- [x] 截图OCR集成
- [x] 自定义快捷键配置
- [ ] 翻译记忆和术语表管理
- [ ] 批量文件翻译
- [ ] 插件生态系统
- [ ] 移动端配套应用
- [x] 代理
- [ ] 自带魔法的国际AI支持(有成本,可能不会免费)

---

## 常见问题 (FAQ)

**Q: 我可以免费使用吗？**

A: 可以！使用我们的邀请链接注册硅基流动账户，即可获得免费赠金额度，足以长期使用。

**Q: 支持哪些语言？**

A: 腾讯混元-MT-7B 模型支持多种主流语言的互译，包括中文、英文、日文、韩文等。或者使用自己喜欢的模型进行翻译。

**Q: 我的数据会被保存吗？**

A: 翻译历史会保存在本地SQLite数据库中，不会上传到服务器。您的隐私得到完全保护。

**Q: 可以离线使用吗？**

A: 使用在线模型不可以，翻译和OCR功能需要连接到模型API。如果使用本地模型，可以离线使用。

**Q: 快捷键功能何时上线？**

A: 快捷键功能目前正在开发中，即将推出。

---

## 贡献指南

欢迎提交问题和拉取请求！我们很高兴能得到您的贡献。

---

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 致谢

- 基于 [Tauri](https://tauri.app/) 构建
- 开发中使用的翻译功能由 [硅基流动](https://siliconflow.cn/) 提供
- UI框架采用 [Vue 3](https://vuejs.org/)

---

## 获取帮助

- 🐛 问题反馈：[GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ 由 Prism 团队@pity开发

**[⬆ 返回顶部](#prism---ai翻译软件)**

</div>
