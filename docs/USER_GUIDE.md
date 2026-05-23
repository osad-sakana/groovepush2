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
- `.gp/` - GroovePush管理フォルダ（スナップショットデータを保存）
- `.gp-ignore` - 除外設定ファイル

### 2. 状態の確認

```bash
gp status
```

ローカルファイル数、合計サイズ、変更ファイル数を表示します。

### 3. スナップショットの保存

```bash
# 基本的なコミット
gp commit

# メッセージ付き
gp commit -m "ミックス完了"

# ドライラン（実際には保存しない）
gp commit --dry-run
```

### 4. 履歴の確認

```bash
gp log
```

### 5. 過去の状態に復元

```bash
gp checkout 20260523T050456
```

スナップショットIDの先頭数文字だけでも指定できます。

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

## コマンド一覧

| コマンド | 説明 |
|---------|------|
| `gp init` | プロジェクト初期化 |
| `gp commit` | スナップショットを保存 |
| `gp status` | 状態確認 |
| `gp log` | 履歴表示 |
| `gp checkout <id>` | 指定スナップショットに復元 |
