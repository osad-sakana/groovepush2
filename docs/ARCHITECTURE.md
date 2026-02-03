# アーキテクチャ

## ディレクトリ構造

```
src/
├── main.rs          # エントリポイント、コマンドルーティング
├── cli.rs           # CLIの定義（clap）
├── error.rs         # エラー型定義
├── scanner.rs       # ファイルスキャン、Smart Diff
└── storage/
    ├── mod.rs       # storageモジュール
    └── s3.rs        # S3アップロード処理
```

## モジュール説明

### cli.rs
clapを使用したCLI定義。サブコマンド（push, log, checkout, init, status）を定義。

### scanner.rs
- `Scanner`: ディレクトリをスキャンし、ファイル一覧を取得
- `ScannedFile`: ファイル情報（パス、サイズ、SHA256ハッシュ）
- `diff_files()`: ローカルとリモートの差分を検出

### storage/s3.rs
- `S3Storage`: S3クライアントラッパー
- 並列アップロード（tokio::spawn）
- 状態管理（current_state.json）

## データフロー

```
[ローカルプロジェクト]
        │
        ▼
    Scanner.scan()
        │
        ▼
  [ScannedFile一覧]
        │
        ▼
    diff_files()  ←── S3 current_state.json
        │
        ▼
  [変更ファイル一覧]
        │
        ▼
S3Storage.upload_files()
        │
        ▼
    [S3バケット]
```

## S3バケット構造

```
s3://groovepush-bucket/
└── {project_name}/
    ├── .gp/
    │   └── current_state.json   # ファイルハッシュマップ
    ├── Project.als              # プロジェクトファイル
    └── Samples/                 # サンプルフォルダ
```

## 技術スタック

| 用途 | クレート |
|------|---------|
| 非同期 | tokio |
| AWS | aws-sdk-s3, aws-config |
| ファイルスキャン | ignore |
| CLI | clap |
| 進捗バー | indicatif |
| ハッシュ | sha2 |
| シリアライズ | serde, serde_json |
