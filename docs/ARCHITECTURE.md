# アーキテクチャ

## ディレクトリ構造

```
src/
├── main.rs          # エントリポイント、コマンドディスパッチ
├── cli.rs           # CLIの定義（clap）
├── error.rs         # エラー型定義
├── scanner.rs       # ファイルスキャン、差分検出
├── utils.rs         # 共通ユーティリティ（サイズフォーマット）
├── commands/        # コマンド実装（1コマンド1ファイル）
│   ├── mod.rs
│   ├── commit.rs    # gp commit
│   ├── log.rs       # gp log
│   ├── checkout.rs  # gp checkout
│   ├── init.rs      # gp init
│   └── status.rs    # gp status
└── storage/
    ├── mod.rs       # storageモジュール
    ├── local.rs     # ローカルストレージ（.gp/への読み書き）
    └── history.rs   # スナップショット履歴・状態管理
```

## モジュール説明

### cli.rs
clapを使用したCLI定義。サブコマンド（commit, log, checkout, init, status）を定義。

### commands/
各コマンドを`run()`関数として実装。`main.rs`はディスパッチのみ担当。

### scanner.rs
- `Scanner`: ディレクトリをスキャンし、ファイル一覧を取得
- `ScannedFile`: ファイル情報（パス、サイズ、SHA256ハッシュ）
- `diff_files()`: 前回コミットとの差分を検出

### storage/local.rs
- `LocalStorage`: `.gp/` ディレクトリへの読み書きを担当
- `save_blobs()`: 変更ファイルを `.gp/blobs/{hash}` にコピー
- `get_current_state()` / `save_state()`: 現在のファイルハッシュマップを管理
- `get_history()` / `save_history()`: スナップショット履歴を管理

### storage/history.rs
- `Snapshot`: スナップショット（ミリ秒精度ID、filesマップ、メタデータ）
- `History`: プロジェクトの履歴（スナップショット一覧、head管理）
- Content-Addressable Storageで重複ファイルを排除

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
    diff_files()  ←── .gp/current_state.json
        │
        ▼
  [変更ファイル一覧]
        │
        ▼
LocalStorage.save_blobs()
        │
        ▼
  [.gp/blobs/]  +  current_state.json  +  history.json
```

## ローカルデータ構造

```
{project_dir}/
└── .gp/
    ├── blobs/{sha256hash}   # Content-Addressable Storage
    ├── current_state.json   # ファイルハッシュマップ（現在の状態）
    └── history.json         # スナップショット履歴
```

## 技術スタック

| 用途 | クレート |
|------|---------|
| ファイルスキャン | ignore |
| CLI | clap |
| 進捗バー | indicatif |
| ハッシュ | sha2 |
| シリアライズ | serde, serde_json |
| エラー処理 | anyhow, thiserror |
| 時間 | chrono |
