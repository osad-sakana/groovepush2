# GroovePush

**音楽制作者のためのローカルバージョン管理ツール**

制作中の任意のタイミングでスナップショットを保存し、過去のテイクにいつでも戻れます。
AWSアカウント不要、インターネット不要、ローカルで完結します。

---

## 特徴

- **差分保存** - 変更されたファイルのみを検出して保存（CASによる重複排除）
- **DAW対応** - Ableton Live, Logic Pro, FL Studio等の一時ファイルを自動除外
- **シンプルCLI** - `gp commit` だけでスナップショット保存

---

## クイックスタート

```bash
# インストール
cargo install --path .

# プロジェクトディレクトリに移動して初期化
cd ~/Music/MyProject
gp init

# スナップショットを保存
gp commit -m "ミックス完了"
```

---

## コマンド

| コマンド | 説明 |
|---------|------|
| `gp init` | プロジェクト初期化 |
| `gp commit` | スナップショットを保存 |
| `gp commit -m "メモ"` | メッセージ付きで保存 |
| `gp commit --dry-run` | ドライラン（保存しない） |
| `gp status` | 変更状態を確認 |
| `gp log` | スナップショット履歴を表示 |
| `gp checkout <id>` | 過去の状態に復元 |

---

## データの保存場所

すべてのデータはプロジェクト内の `.gp/` ディレクトリに保存されます。

```
.gp/
├── blobs/{sha256hash}   # ファイルの実データ（差分のみ）
├── current_state.json   # 現在のファイルハッシュマップ
└── history.json         # スナップショット履歴
```

---

## 技術スタック

```
Rust + clap + ignore + indicatif + sha2
```

---

## ライセンス

MIT
