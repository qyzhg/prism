# Prism - AI翻訳ソフトウェア

<div align="center">

**[English](./README.md) | [中文](./README.zh.md) | [Español](./README.es.md) | [日本語](./README.ja.md) | [한국어](./README.ko.md) | [Tiếng Việt](./README.vi.md)**

🚀 先進的な言語モデルとOCRを搭載したクロスプラットフォームAI翻訳アプリ。

[ダウンロード](#ダウンロード) • [機能](#機能) • [クイックスタート](#クイックスタート) • [ドキュメント](#ドキュメント)

</div>

---
<img src="icon.png" alt="アイコン" width="350" height="350">

## 機能

- **🌍 クロスプラットフォーム対応** - Windows / macOS / Linux で同等の機能を提供
- **🤖 高度なAI翻訳** - Tencent Hunyuan-MT-7B を採用し、文脈を考慮した正確な翻訳
- **📸 内蔵OCR** - Qwen3-VL-8B-Instruct でスクリーンショットから直接テキスト抽出・翻訳
- **⚡ 高速翻訳** - リアルタイムかつ低遅延
- **🎯 使いやすいUI** - Vue 3 ベースの直感的でスムーズなUI
- **🔗 グローバルホットキー** - カスタマイズ可能なショートカット（開発中）
- **💾 ローカル履歴** - SQLite に翻訳履歴を保存
- **🎨 モダンなアーキテクチャ** - Tauri + Rust による高性能かつ安全な構成

---

## 技術スタック

### フロントエンド

- **Vue 3** (3.5.13) - 近代的なプログレッシブフレームワーク
- **Vite** (6.0.3) - 次世代ビルドツール
- **Tauri UI コンポーネント** - ネイティブに近いデスクトップ体験

### バックエンド

- **Rust** (2021 edition) - 高性能システム言語
- **Tauri** (2.9.3) - 軽量デスクトップフレームワーク
- **Tokio** (1.48.0) - 非同期ランタイム

### AI と処理

- **翻訳モデル** - Tencent Hunyuan-MT-7B
- **OCRモデル** - Qwen3-VL-8B-Instruct
- **APIプロバイダ** - SiliconFlow
- **OpenAI互換のカスタムモデルを幅広くサポート**

### ストレージとライブラリ

- **SQLite** (rusqlite 0.37.0) - ローカルデータベース
- **Reqwest** (0.12.24) - HTTPクライアント
- **画像処理** (0.25.9) - スクリーンショットと画像処理
- **グローバルホットキー** (2.3.1) - キーボードショートカットプラグイン

---

## クイックスタート

### 必要環境

- Rust 1.91.0 以上
- Node.js 18+ と pnpm
- Git

### セットアップ

**1. リポジトリをクローン**
```bash
git clone https://github.com/qyzhg/prism.git
cd prism
```

**2. 依存関係をインストール**
#### フロントエンド依存
```bash
pnpm install
```

#### Rust 依存は Cargo が管理

**3. API キーを取得**
- OpenAI 互換の Base URL と API Key を用意すれば利用開始できます。
- 招待リンクから SiliconFlow に登録すると無料クレジットが得られます: [https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq)

**4. 開発モードで起動**
```bash
pnpm tauri dev
```

**5. 本番ビルド**
```bash
pnpm tauri build
```

---

## ダウンロード

| プラットフォーム | リンク |
|----------------|-------|
| 🪟 Windows | [最新版](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [最新版](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | 近日対応 |

### macOS インストールメモ

Prism は ad-hoc 署名のため、初回起動時に Gatekeeper の警告が表示されます。

1. `Prism.app` を `/Applications` に移動。
2. **Terminal** で以下を実行:
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. Finder で右クリック（または Control+クリック）し **開く** を選択して一度承認すれば、次回以降は通常起動できます。

---

## ドキュメント

### 設定

設定パネルで翻訳の好みを管理できます:

- デフォルトの言語ペア選択
- API キー管理
- ホットキーのカスタマイズ（開発中）

### ホットキー

開発中 - 近日公開

### AI モデル

- **翻訳モデル** - `tencent/Hunyuan-MT-7B` エンタープライズ向け多言語翻訳
- **OCRモデル** - `Qwen/Qwen3-VL-8B-Instruct` 高度なビジョン・ランゲージ

---

## ロードマップ

- [x] 基本の翻訳機能
- [x] スクリーンショットOCR統合
- [x] カスタムホットキー設定
- [ ] 翻訳メモリと用語管理
- [ ] ファイルのバッチ翻訳
- [ ] プラグインエコシステム
- [ ] モバイル連携アプリ

---

## FAQ

**Q: 無料で使えますか？**  
はい。招待リンク経由で SiliconFlow に登録すると、長期間使える無料クレジットがもらえます。

**Q: どの言語に対応していますか？**  
Tencent Hunyuan-MT-7B は主要言語（中国語・英語・日本語・韓国語など）の相互翻訳に対応しています。好みのモデルを使うことも可能です。

**Q: データは保存されますか？**  
翻訳履歴はローカルの SQLite に保存され、サーバーへは送信されません。プライバシーは保護されます。

**Q: オフラインで使えますか？**  
オンラインモデルは接続が必須です。ローカルモデルを利用する場合はオフラインも可能です。

**Q: ホットキーはいつ使えますか？**  
現在開発中で、近日リリース予定です。

---

## コントリビュート

Issue と Pull Request を歓迎します。ぜひ貢献してください！

---

## ライセンス

MIT ライセンスです。詳細は [LICENSE](LICENSE) を参照してください。

---

## 謝辞

- [Tauri](https://tauri.app/) によって構築
- 開発中の翻訳サービスは [SiliconFlow](https://siliconflow.cn/) 提供
- UI フレームワークは [Vue 3](https://vuejs.org/)

---

## サポート

- 🐛 バグ報告: [GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ Prism チーム@pity による開発

**[⬆ トップに戻る](#prism---ai翻訳ソフトウェア)**

</div>
