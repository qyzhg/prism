# Prism - AI Translation Software

<div align="center">

**[English](./README.md) | [中文](./README.zh.md)**

🚀 A powerful cross-platform AI translation application powered by advanced language models and OCR technology.

[Download](#download) • [Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation)

</div>

---
<img src="icon.png" alt="图标" width="350" height="350">

## Features

- **🌍 Cross-Platform Support** - Full support for Windows, macOS, and Linux with consistent functionality
- **🤖 Advanced AI Translation** - Powered by Tencent Hunyuan-MT-7B model for accurate context-aware translation
- **📸 Built-in OCR Recognition** - Extract and translate text directly from screenshots using Qwen3-VL-8B-Instruct
- **⚡ Lightning-Fast Translation** - Real-time translation with minimal latency
- **🎯 User-Friendly Interface** - Intuitive Vue 3-based UI with smooth interactions
- **🔗 Global Hotkeys** - Customizable shortcuts for quick access (in development)
- **💾 Local Translation History** - SQLite-based translation record storage
- **🎨 Modern Architecture** - Built with Tauri + Rust for superior performance and security

---

## Tech Stack

### Frontend Technologies

- **Vue 3** (3.5.13) - Modern progressive JavaScript framework
- **Vite** (6.0.3) - Next-generation build tool
- **Tauri UI Components** - Native desktop application experience

### Backend Technologies

- **Rust** (2021 edition) - High-performance systems programming language
- **Tauri** (2.9.3) - Lightweight desktop application framework
- **Tokio** (1.48.0) - Async runtime for concurrent operations

### AI & Processing

- **Translation Model** - Tencent Hunyuan-MT-7B
- **OCR Model** - Qwen3-VL-8B-Instruct
- **API Provider** - SiliconFlow
- **Full support for all OpenAI-compatible AI custom models**

### Storage & Libraries

- **SQLite** (rusqlite 0.37.0) - Local database storage
- **Reqwest** (0.12.24) - HTTP client
- **Image Processing** (0.25.9) - Screenshot and image processing
- **Global Hotkeys** (2.3.1) - Keyboard shortcut plugin

---

## Quick Start

### Prerequisites

- Rust 1.91.0 or higher
- Node.js 18+ and pnpm
- Git

### Installation

**1. Clone the repository**
```bash
git clone https://github.com/qyzhg/prism.git
cd prism
```

**2. Install dependencies**
#### Install frontend dependencies
```bash
pnpm install
```

#### Rust dependencies are managed by Cargo

**3. Get API Key**
- Use your own OpenAI-compatible base URL and API key to start using
- Register for a SiliconFlow account with our invite link: [https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq) to get free credits (long validity period)

**4. Run in development mode**
```bash
pnpm tauri dev
```

**5. Build for production**
```bash
pnpm tauri build
```

---

## Download

| Platform | Download Link |
|----------|---------------|
| 🪟 Windows | [Latest Release](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [Latest Release](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | Coming Soon |

### macOS installation notes

Prism is ad-hoc signed (Developer ID certificates cost ~$99/year), so macOS Gatekeeper will still warn the first time you try to run the download.  
To get past “Prism.app is damaged / can’t be opened” without a paid certificate:

1. Move `Prism.app` into `/Applications`.
2. Open **Terminal** and run:
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. Right-click the app, choose **Open**, and confirm once. Future launches can be done normally.

---

## Documentation

### Configuration

Configure your translation preferences in the settings panel:

- Default language pair selection
- API key management
- Hotkey customization (in development)

### Hotkeys

Currently in development - Coming soon

### AI Models

- **Translation Model** - `tencent/Hunyuan-MT-7B` Enterprise-grade multilingual translation
- **OCR Model** - `Qwen/Qwen3-VL-8B-Instruct` Advanced vision-language understanding

---

## Roadmap

- [x] Core translation functionality
- [x] Screenshot OCR integration
- [ ] Custom hotkey configuration
- [ ] Translation memory and terminology management
- [ ] Batch file translation
- [ ] Plugin ecosystem
- [ ] Mobile companion app

---

## Frequently Asked Questions (FAQ)

**Q: Can I use it for free?**

A: Yes! Register for a SiliconFlow account using our invite link to get free credits that are sufficient for long-term use.

**Q: What languages are supported?**

A: The Tencent Hunyuan-MT-7B model supports mutual translation between multiple mainstream languages, including Chinese, English, Japanese, Korean, and more. You can also use your preferred models for translation.

**Q: Is my data saved?**

A: Translation history is saved locally in a SQLite database and never uploaded to servers. Your privacy is fully protected.

**Q: Can I use it offline?**

A: Online models cannot be used offline. Translation and OCR features require connection to the model API. If using local models, offline usage is possible.

**Q: When will the hotkey feature be available?**

A: The hotkey feature is currently under development and will be released soon.

---

## Contributing

Issues and pull requests are welcome! We'd love to have your contribution.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Translation services during development provided by [SiliconFlow](https://siliconflow.cn/)
- UI framework powered by [Vue 3](https://vuejs.org/)

---

## Get Help

- 🐛 Bug Reports: [GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ Developed by Prism Team@pity

**[⬆ Back to Top](#prism---ai-translation-software)**

</div>
