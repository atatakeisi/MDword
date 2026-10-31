# MDword

<p align="center">
  <img src="docs/brand/mdword-icon.png" width="140" height="140" alt="MDword Logo">
</p>

<p align="center">
  <b>純Rust + WebAssembly 製の高速・安全な次世代AI連携・情報セキュリティWebワープロ</b>
</p>

<p align="center">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-2b47b5?style=flat-square&logo=rust&logoColor=white">
  <img alt="WebAssembly" src="https://img.shields.io/badge/target-WebAssembly-654ff0?style=flat-square&logo=webassembly&logoColor=white">
  <img alt="Security: Macro CDR Sanitized" src="https://img.shields.io/badge/security-CDR%20Sanitized-critical?style=flat-square&logo=shield">
  <img alt="Privacy: 100% Client-Side" src="https://img.shields.io/badge/privacy-100%25%20client--side-success?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-2b47b5?style=flat-square">
</p>

<p align="center">
  <a href="https://atatakeisi.github.io/MDword/"><b>🚀 ブラウザで今すぐ使う（GitHub Pages）</b></a>
</p>

---

## 📖 MDword とは

**MDword（エムディー・ワード）** は、オープンソースの純Rust製ワードプロセッサ（[storytold/wordcraft](https://github.com/storytold/wordcraft)）をベースに、**「AI連携」「ビジネス文書の構造化」「情報セキュリティ（マクロ無害化）」** に特化して設計されたWebワープロです。

Word（`.docx`）ライクな直感的な執筆体験を保ちながら、作成した文書を一瞬でクリーンな **Markdown（構造化テキスト）** へ変換。ChatGPT、Claude、Gemini などの生成AIプロンプトや、Slack、メール、Notion へのスムーズな情報引き渡しを実現します。

インストール不要、サーバー通信不要（100%クライアントサイド実行）で、PCやタブレットのブラウザから瞬時に起動します。

---

## 💡 MDword の思想とセキュリティへの問いかけ
### ―― なぜテンプレートにあえて「かわいい丸文字（Hachi Maru Pop）」を採用したのか？

> **「こんな丸文字、お客様や取引先にそのまま送れるかよ！」**  
> **――そう感じて立ち止まった瞬間こそが、セキュリティの第一歩です。**

#### 1. 無意識な「保存・添付・送信」にブレーキをかける
日々の業務で、メールに添付されたOfficeファイルを無警戒にダブルクリックで開いたり、確認もそこそこに転送していませんか？  
MDword のテンプレートを開いた瞬間に目に飛び込んでくる「ビジネスらしからぬ可愛い丸文字（八丸ポップ体）」は、日常のルーチンワークに潜む無意識に対して、**「本当にこのまま送っていいのか？」「危険なファイルを開いていないか？」と人間に立ち止まらせるための意図的な仕掛け**です。

#### 2. 受け取る相手の手間を想像する（重苦しい装飾 vs 相手への配慮）
従来のワープロソフトで、文字幅の微調整やスペース連打の中央揃え、複雑な罫線で「重々しく飾り立てた書類」を作っていませんか？  
受け取った側は、その書式が崩れたり、中身のテキストをコピーして再利用するのに多大な手間を強いられます。  
MDword は、無駄な装飾から脱却し、**「受け取った相手が最も編集・活用しやすいクリーンなデータ（Markdown・テキスト）で手渡す」** という本質的なビジネスの配慮を促します。

#### 3. 「人間だからこそ気付ける」最後の砦
巧妙化する標的型メール攻撃やマクロウイルス、現代のAIを標的とした **プロンプトインジェクション** など、自動スキャンやウイルス対策ソフトをすり抜ける脅威は日々進化しています。  
機械やシステム任せの過信を捨て、**「人間が違和感に気づき、自らの目で確認する」** 意識こそが、現代の情報セキュリティにおける最大の防壁です。

---

## 🛡️ 強力なセキュリティ（CDR：コンテンツ無害化と再構築）

> [!WARNING]
> **Officeファイル（.docx / .docm）はサイバー攻撃の最大の温床です**  
> 取引先や外部から届くWord文書には、悪意あるVBAマクロコードや、不正な実行ファイルを潜ませたOLEオブジェクトが埋め込まれているリスクが常に存在します。

MDword は、金融機関や自治体でも採用される **CDR（Content Disarm and Reconstruction：コンテンツ無害化・再構築）** の思想をブラウザ上で完全実装しています。

| 防御機能 | 従来のリスク | MDword による保護 |
|:---|:---|:---|
| **マクロ検知 & 警告** | `.docm` やマクロ入り `.docx` を無警戒に開いて感染 | ファイル読み込み時に即座に検知し、危険性を警告表示 |
| **自動無害化 (CDR)** | 悪意あるVBAスクリプトや埋め込みOLEバイナリの実行 | マクロコード・危険なOLEバイナリを**完全に物理除去**。本文・見出し・表・安全な画像のみをクリーンに再構築 |
| **保存時の脅威遮断** | 誤ってマクロ有効形式で再配布・感染拡大 | 保存ダイアログから `.docm` 形式を完全排除。常に安全な `.docx` として正規化して保存 |
| **完全クライアント完結** | 文書データが外部サーバーに送信されて漏洩 | 処理はブラウザのWebAssembly内でのみ完結。外部サーバー通信は一切行いません（通信遮断環境でも動作可能） |

---

## ⚡ 特徴と執筆ワークフロー

### 1. 🤖 AI連携・Markdownファーストの高速書き出し
ホームリボンの最前列（左端）に、AIや外部ツールへの連携に特化した大型カラフルボタンを配置しています。

* **スカイブルー `[ ⚡ Markdown ]` (1クリック変換・コピー)**  
  執筆中のドキュメント全体を一瞬でMarkdownに変換し、クリップボードへ格納。そのまま ChatGPT、Claude、Gemini などのAIチャット欄や、Slack・Notionへ貼り付けられます。
* **エメラルドグリーン `[ 👁 プレビュー ]` (デュアル比較モーダル)**  
  送信・共有前に相手やAIへの見え方を確認できる2面プレビュー画面を展開：
  - **`[ プレビュー（Notion/ブラウザ風）]`**: 見出し・箇条書き・表が整ったレンダリング表示。
  - **`[ 生テキスト（メール/メモ帳風）]`**: 実際にコピーされる文字列並び（等幅）を直接確認。
* **アンバーオレンジ `[ 📄 テキスト ]` (プレーンテキスト出力)**  
  装飾や制御文字を完全に削ぎ落とした純粋なプレーンテキストとしてコピー・ダウンロード。

### 2. ✍️ 本家Word同等のインライン日本語入力（リアルタイム押し出し）
- **文字のリアルタイム押し出し**:  
  文字入力中（IME変換中）も、後ろの文字がリアルタイムに右へ押し出され、文字同士が重ならずに快適にタイピングできます。
- **入力中からスタイルフォント＆下線表示**:  
  未確定の入力文字も、文書に指定されたフォント（Hachi Maru Pop 等）かつ下線付きで自然に文章に差し込まれ、確定後も違和感なくスムーズに執筆を続けられます。

### 3. 🌸 多彩な日本語Webフォント & 自然な日本語ローカライズ
- **直感的な日本語UI**: メニュー、リボン、ダイアログ、ツールチップを完全日本語化。
- **安心のオープンソースフォント（SIL OFL）**:
  - デフォルトの可愛い手書き風フォント **「Hachi Maru Pop（八丸ポップ）」**
  - 高い可読性を誇る **「BIZ UDPゴシック / BIZ UD明朝」**
  - **「Zen丸ゴシック」「しっぽり明朝」「Klee One（教科書体）」「M PLUS 1c」** など、多彩な表現に対応。IndexedDBキャッシュにより2回目以降はオフラインでも高速表示。

---

## 🚀 使い方

### ブラウザで開く（インストール不要）
以下のURLにアクセスするだけで、すぐに使い始められます。  
👉 **[https://atatakeisi.github.io/MDword/](https://atatakeisi.github.io/MDword/)**

1. **ファイルを開く**: `.docx` や `.docm` をブラウザにドラッグ＆ドロップ（または「ファイル」＞「開く」）。マクロは自動で無害化されます。
2. **執筆・編集**: 見出しスタイルや箇条書き、表を使って文書を作成。
3. **AIチャットやメールへ連携**: リボンの `[Markdown]` ボタンを押してクリップボードにコピーし、AIチャットやメールに貼り付け。
4. **安全に保存**: `Cmd/Ctrl + S` で、クリーンな `.docx` として保存されます。

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
  本ソフトウェアは [storytold/wordcraft](https://github.com/storytold/wordcraft)（Copyright © 2026 ArtCraft Team and the WordCraft contributors）をフォークし、[atatakeisi](https://github.com/atatakeisi) によって日本語化、マクロCDR無害化、AI/Markdown連携機能、およびインラインIME入力対応を追加したものです。
- **ソースコードライセンス**:  
  本家のライセンスを継承し、**[MIT License](LICENSE-MIT)** または **[Apache License, Version 2.0](LICENSE-APACHE)** のデュアルライセンスのもとで公開されています。
- **同梱フォント**:  
  本プロジェクトで使用される日本語フォントおよび欧文フォントは、それぞれの作者により **SIL Open Font License 1.1** に基づいて配布されています（詳細は `NOTICE` および各フォントライセンスファイルを参照してください）。
- **商標について**:  
  「WordCraft」および「ArtCraft」の名称・ロゴ・商標は ArtCraft Team の著作物・商標です。本プロジェクト（MDword）は独自に改変・公開されている派生版であり、ArtCraft Team による公式なサポートや保証を受けるものではありません。
