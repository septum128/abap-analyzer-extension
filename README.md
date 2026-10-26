# ABAP Analyzer（Zed 拡張機能）

[Zed](https://zed.dev) で ABAP を編集するための拡張機能です。言語サーバー [abap-language-server](https://github.com/septum128/abap-language-server) を起動し、次の機能を使えるようにします。

- 診断
- 補完
- 定義へのジャンプ
- ホバー
- シンタックスハイライト
- アウトライン

## 必要なもの

`abap-language-server` が `PATH` から見つかる場所にインストールされている必要があります。

```sh
git clone https://github.com/septum128/abap-language-server.git
cd abap-language-server
cargo install --path .
```

## インストール

この拡張機能は、Zed の拡張機能一覧には登録していません。開発用の拡張機能としてインストールします。

1. このリポジトリを clone します。
2. Zed のコマンドパレットで `zed: install dev extension` を実行し、clone したフォルダを選びます。ビルドには Rust と `wasm32-wasip2` ターゲットが必要です。

## 設定

シンタックスハイライトとアウトラインは、言語サーバーから取ります。

- シンタックスハイライト: セマンティックトークン
- アウトライン: ドキュメントシンボル

どちらも Zed では既定で無効なので、settings.json で有効にしてください。

```json
{
  "languages": {
    "ABAP": {
      "semantic_tokens": "full",
      "document_symbols": "on"
    }
  }
}
```

言語サーバーを入れ直したときは、`editor: restart language server` を実行します。

## 構成

| ファイル | 内容 |
|---|---|
| `extension.toml` | 拡張機能の情報と、言語サーバー `abap-language-server` の登録 |
| `languages/abap/config.toml` | 言語 ABAP（拡張子 `.abap`、行コメント `"` と `*`） |
| `src/lib.rs` | `PATH` から `abap-language-server` を探して起動する |

Tree-sitter の文法は含めていません。そのため、括弧の対応は使えません。

## Windows + WSL で使う場合

- `abap-language-server` は WSL 側にインストールし、Zed の WSL リモート接続でプロジェクトを開きます。
- `\\wsl.localhost\...` のフォルダを Windows の Zed で直接ビルドすると、インクリメンタルコンパイルのロックファイルを作れずに失敗します。
  - `Cargo.toml` の `[profile.dev] incremental = false` は、その対策です。
  - それでも失敗する場合は、Windows 側のフォルダ（例: `C:\Dev\abap-analyzer-extension`）にコピーしてからインストールしてください。

## ライセンス

[MIT](LICENSE)
