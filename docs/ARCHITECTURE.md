# アーキテクチャ

## ディレクトリ構造

```
src/
├── main.rs          # エントリポイント、コマンドディスパッチ
├── cli.rs           # CLIの定義（clap）
├── error.rs         # エラー型定義
├── scanner.rs       # ファイルスキャン、Smart Diff
├── utils.rs         # 共通ユーティリティ（サイズフォーマット、バリデーション）
├── commands/        # コマンド実装（1コマンド1ファイル）
│   ├── mod.rs
│   ├── push.rs      # gp push
│   ├── log.rs       # gp log
│   ├── checkout.rs  # gp checkout
│   ├── init.rs      # gp init
│   ├── status.rs    # gp status
│   └── clone.rs     # gp clone
└── storage/
    ├── mod.rs       # storageモジュール
    ├── s3.rs        # S3クライアント、並列アップロード
    └── history.rs   # スナップショット履歴・状態管理
```

## モジュール説明

### cli.rs
clapを使用したCLI定義。サブコマンド（push, log, checkout, init, status, clone）を定義。

### commands/
各コマンドを`run()`関数として実装。`main.rs`はディスパッチのみ担当。

### scanner.rs
- `Scanner`: ディレクトリをスキャンし、ファイル一覧を取得
- `ScannedFile`: ファイル情報（パス、サイズ、SHA256ハッシュ）
- `diff_files()`: ローカルとリモートの差分を検出

### storage/s3.rs
- `S3Storage`: S3クライアントラッパー
- `upload_blobs()`: Semaphore制限付きの並列アップロード（デフォルト10件同時）
- `get_remote_state()` / `get_history()`: NoSuchKey以外のエラーを適切に伝搬
- バケット名は`GROOVEPUSH_BUCKET`環境変数で上書き可能

### storage/history.rs
- `Snapshot`: スナップショット（ミリ秒精度ID、files マップ、メタデータ）
- `History`: プロジェクトの履歴（スナップショット一覧、head管理）
- Content-Addressable Storage で重複ファイルを排除

### utils.rs
- `format_size()`: バイト数を人間が読みやすい形式に変換
- `validate_project_name()`: パス走査攻撃を防ぐ入力バリデーション

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
S3Storage.upload_blobs()  ← Semaphore(10)で並列制限
        │
        ▼
  [S3 blobs/]  +  current_state.json  +  history.json
```

## S3バケット構造

```
s3://groovepush-bucket/            (GROOVEPUSH_BUCKET環境変数で変更可)
└── {project_name}/
    ├── .gp/
    │   ├── blobs/{sha256hash}     # Content-Addressable Storage
    │   ├── current_state.json     # ファイルハッシュマップ（現在の状態）
    │   └── history.json           # スナップショット履歴
    ├── Project.als                # プロジェクトファイル
    └── Samples/                   # サンプルフォルダ
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
| エラー処理 | anyhow, thiserror |
| 時間 | chrono |
