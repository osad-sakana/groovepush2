# GroovePush ユーザーガイド

## インストール

```bash
cargo install --path .
```

## 基本的な使い方

### 1. プロジェクトの初期化

```bash
gp init /path/to/your/project
```

これにより以下が作成されます：
- `.gp/` - GroovePush管理フォルダ
- `.gp-ignore` - 除外設定ファイル

### 2. 状態の確認

```bash
gp status /path/to/your/project
```

ローカルファイル数、合計サイズ、変更ファイル数を表示します。

### 3. S3へのプッシュ

```bash
# 基本的なプッシュ
gp push /path/to/your/project

# メッセージ付き
gp push /path/to/your/project -m "ミックス完了"

# ドライラン（実際にはアップロードしない）
gp push /path/to/your/project --dry-run
```

## .gp-ignore 設定

プロジェクトルートに `.gp-ignore` ファイルを作成し、除外パターンを指定できます。

```
# DAWの一時ファイル
*.tmp
Backup/
*.asd

# 大きなサンプルフォルダを除外
Samples/Archive/
```

## AWS認証

以下の順序で認証情報を探します：

1. 環境変数 (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
2. `~/.aws/credentials` ファイル

## コマンド一覧

| コマンド | 説明 |
|---------|------|
| `gp init <path>` | プロジェクト初期化 |
| `gp push <path>` | S3にプッシュ |
| `gp status <path>` | 状態確認 |
| `gp log` | 履歴表示（フェーズ2） |
| `gp checkout` | 復元（フェーズ2） |
