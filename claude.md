# GroovePush

音楽制作者向けのCLIベースのS3バックアップツール。

## 概要

ローカルSSDで制作を行い、任意のタイミングで「スナップショット」をS3に保存。
過去の特定のテイクへの復元（ロールバック）やプロジェクトごとの独立管理を実現する。

## 技術スタック

- **言語**: Rust
- **非同期**: tokio
- **AWS**: aws-sdk-s3
- **ファイルスキャン**: ignore (ripgrepエンジン)
- **CLI**: clap
- **進捗バー**: indicatif

## ドキュメント

| ファイル | 内容 |
|---------|------|
| [docs/USER_GUIDE.md](docs/USER_GUIDE.md) | 使い方、コマンド一覧、.gp-ignore設定 |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | モジュール構成、データフロー、S3構造 |
| [docs/ROADMAP.md](docs/ROADMAP.md) | フェーズ別ロードマップ |

## 開発ルール

- 作業1つごとに日本語で簡潔にgit commitする
- `gp` コマンドとしてビルド (`cargo install --path .`)
- 機能ごとに`/docs`やclaude.mdを更新する

## S3バケット構造

バケット名は`GROOVEPUSH_BUCKET`環境変数で変更可能（デフォルト: `groovepush-bucket`）。

```
s3://groovepush-bucket/
└── {project_name}/
    ├── .gp/
    │   ├── blobs/{sha256hash}     # Content-Addressable Storage
    │   ├── current_state.json     # ファイルハッシュマップ
    │   └── history.json           # スナップショット履歴
    └── (プロジェクトファイル)
```
