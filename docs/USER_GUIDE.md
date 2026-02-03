# GroovePush ユーザーガイド

## インストール

```bash
cargo install --path .
```

## 基本的な使い方

### 1. プロジェクトの初期化

プロジェクトディレクトリに移動して実行：

```bash
cd /path/to/your/project
gp init
```

これにより以下が作成されます：
- `.gp/` - GroovePush管理フォルダ
- `.gp-ignore` - 除外設定ファイル

### 2. 状態の確認

```bash
gp status
```

ローカルファイル数、合計サイズ、変更ファイル数を表示します。

### 3. S3へのプッシュ

```bash
# 基本的なプッシュ
gp push

# メッセージ付き
gp push -m "ミックス完了"

# ドライラン（実際にはアップロードしない）
gp push --dry-run
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

### 4. S3からプロジェクトをクローン

```bash
gp clone my-project
```

カレントディレクトリに`my-project/`フォルダが作成され、最新のスナップショットが復元されます。

## コマンド一覧

すべてのコマンドはカレントディレクトリで実行します。

| コマンド | 説明 |
|---------|------|
| `gp init` | プロジェクト初期化 |
| `gp push` | S3にプッシュ |
| `gp status` | 状態確認 |
| `gp log` | 履歴表示 |
| `gp checkout <id>` | 指定スナップショットに復元 |
| `gp clone <project>` | S3からプロジェクトをクローン |
