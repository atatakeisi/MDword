# MDword

<p align="center">
  <b>純Rust + WebAssembly 製の高速・安全な次世代AI連携・日本語Webワープロ</b>
</p>

<p align="center">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-2b47b5?style=flat-square&logo=rust&logoColor=white">
  <img alt="WebAssembly" src="https://img.shields.io/badge/target-WebAssembly-654ff0?style=flat-square&logo=webassembly&logoColor=white">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-2b47b5?style=flat-square">
  <img alt="100% Client-Side Private" src="https://img.shields.io/badge/privacy-100%25%20client--side-success?style=flat-square">
</p>

<p align="center">
  <a href="https://atatakeisi.github.io/MDword/"><b>🚀 ブラウザで今すぐ使う（GitHub Pages）</b></a>
</p>

---

## 📖 MDword とは

**MDword（エムディー・ワード）** は、オープンソースの純Rust製ワードプロセッサ（[storytold/wordcraft](https://github.com/storytold/wordcraft)）をベースに、**日本のビジネス文書作成・AI連携・情報セキュリティ（マクロ無害化）** に特化して開発されたWebワープロです。

リッチなWord形式（`.docx`）の直感的な編集体験をそのままに、ワンクリックでクリーンなMarkdownやプレーンテキストに変換。ChatGPT、Claude、GeminiなどのLLMやメール本文、Notionへのスムーズな情報連携を実現します。

インストール不要で、PCやタブレットのブラウザから瞬時に起動します。

---

## ⚡ MDword の 4大特徴

### 1. 🎯 AI & Markdown ファーストの執筆ワークフロー
- **ホームリボンの最前列に大型「AI / 変換」ボタン群を配置**:
  - **`⚡ Markdown 参照・出力`**: 送信前・共有前に相手やAIへの見え方を確認できるプレビュー画面を開きます。
  - **`📋 ワンクリック MDコピー`**: 執筆した文書全体を一瞬でMarkdownに変換し、クリップボードにコピー。そのままAIチャットやメールに貼り付けられます。
  - **`📄 テキスト 出力`**: 余計な装飾を削ぎ落としたプレーンテキスト（`.txt`）のダウンロード。
- **ダブル（デュアル）プレビュー画面**:
  - **`[👁 整形表示（ブラウザ・Notion風）]`**: 見出し・箇条書き・テーブル罫線が美しく整ったリッチな表示。
  - **`[📄 生テキスト（メール・メモ帳風）]`**: メール本文やテキストエディタに貼り付けた際の実際の文字列並び（等幅）を確認。
- **Markdown執筆に最適化されたUI配置**:
  Markdownでは不要なフォント・文字色装飾ツールをリボン右側に退避し、左側に「AI/変換」「スタイル（見出し1・2）」「段落（箇条書き・リスト）」を集中配置。

### 2. 🛡️ 安心のCDR（コンテンツ無害化・再構築）セキュリティ
- **マクロ付き文書の検知 & 警告**:
  外部や取引先から受け取ったマクロ付き文書（`.docm`、`.dotm`、またはVBAマクロを含む `.docx`）を開く際に、即座に検知して警告ダイアログを表示。
- **自動無害化（CDR: Content Disarm & Reconstruction）**:
  文書内の悪意あるVBAマクロコードや埋め込みOLEバイナリ（スクリプト、実行可能ファイル等）を完全に除去。本文テキスト・見出し・表・標準画像のみをクリーンに抽出して安全に開きます。
- **危険形式への保存遮断**:
  保存ダイアログから `.docm` などのマクロ形式を完全排除し、常に安全な `.docx` として正規化して保存します。

### 3. 🌸 美しい日本語タイポグラフィ & 9つの日本語Webフォント
- **日本語UI完全ローカライズ**:
  メニュー、リボン、ダイアログ、ツールチップを自然な日本語にローカライズ。
- **高品質フォントの内蔵 & オンデマンド配信**:
  - 視認性に優れた「BIZ UDPゴシック（Regular）」をWebAssemblyに内蔵。
  - 「Zen丸ゴシック」「M PLUS 1p」「さわらびゴシック/明朝」「Noto Sans/Serif JP」など **9種類の日本語オープンフォント（SIL OFL）** をサポート。IndexedDBキャッシュにより、2回目以降はオフラインでも高速に描画されます。
- **親しみやすい日本語サンプルテンプレート**:
  標準フォントに柔らかな丸ゴシックを採用した、すぐに使える活用ガイド・テンプレートを同梱。

### 4. 🔒 完全ローカル完結（プライバシー保護）
- **サーバー不要の100%クライアントサイド実行**:
  すべての処理はブラウザ内の WebAssembly（WASM）エンジンで実行されます。
- **情報漏洩リスクゼロ**:
  編集中の文書、読み込んだDOCXファイル、変換されたMarkdownデータが外部サーバーへ送信されることは一切ありません。企業の機密文書、企画書、個人情報を含むドキュメントも安心して扱えます。

---

## 🚀 使い方

### ブラウザで開く（インストール不要）
以下のURLにアクセスするだけで、すぐに使い始められます。  
👉 **[https://atatakeisi.github.io/MDword/](https://atatakeisi.github.io/MDword/)**

- **ファイルを開く**: `.docx` または `.docm` ファイルをブラウザ画面にドラッグ＆ドロップ（または「ファイル」＞「開く」）。
- **Markdownに変換**: ホームリボンの「⚡ Markdown 参照・出力」または「📋 ワンクリック MDコピー」をクリック。
- **安全に保存**: 「ファイル」＞「保存」または `Cmd/Ctrl + S` で、マクロが除去されたクリーンな `.docx` として保存されます。

---

## 🛠️ ローカルでのビルド・開発手順

### 前提環境
- [Rust](https://rustup.rs/) (1.95以上)
- `wasm32-unknown-unknown` ターゲット: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) (0.22.0)

### ビルド & ローカル実行
```bash
# リポジトリのクローン
git clone https://github.com/atatakeisi/MDword.git
cd MDword

# 依存アセット（翻訳・フォント）の取得（初回のみ）
git clone https://github.com/storytold/craft-i18n.git /tmp/craft-i18n
git -C /tmp/craft-i18n checkout $(cat craft-i18n.rev)
git clone https://github.com/storytold/craft-fonts.git /tmp/craft-fonts
git -C /tmp/craft-fonts checkout b076417b5880bd4154bd22f00695b1d28c6bea83

# Web版のビルド
cd apps/wordcraft-web
CRAFT_I18N_DIR=/tmp/craft-i18n CRAFT_FONTS_DIR=/tmp/craft-fonts trunk build --dist ../../dist/web-ja

# ローカルサーバー起動
cd ../..
python3 -m http.server 8771 --directory dist/web-ja
# ブラウザで http://127.0.0.1:8771/?sample&lang=ja を開く
```

---

## 📄 ライセンス & クレジット

- **ベースプロジェクト**:  
  本ソフトウェアは [storytold/wordcraft](https://github.com/storytold/wordcraft)（Copyright © 2026 ArtCraft Team and the WordCraft contributors）をフォークし、[atatakeisi](https://github.com/atatakeisi) によって日本語化、マクロCDR無害化、およびAI/Markdown連携機能を追加したものです。
- **ソースコードライセンス**:  
  本家のライセンスを継承し、**[MIT License](LICENSE-MIT)** または **[Apache License, Version 2.0](LICENSE-APACHE)** のデュアルライセンスのもとで公開されています。
- **同梱フォント**:  
  本プロジェクトで使用される日本語フォントおよび欧文フォントは、それぞれの作者により **SIL Open Font License 1.1** に基づいて配布されています（詳細は `NOTICE` および各フォントライセンスファイルを参照してください）。
- **商標について**:  
  「WordCraft」および「ArtCraft」の名称・ロゴ・商標は ArtCraft Team の著作物・商標です。本プロジェクト（MDword）は独自に改変・公開されている派生版であり、ArtCraft Team による公式なサポートや保証を受けるものではありません。
