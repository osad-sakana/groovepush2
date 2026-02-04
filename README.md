# 🎵 GroovePush

**音楽制作者のための高速S3バックアップツール**

ローカルSSDの速度を活かして制作し、任意のタイミングでS3にスナップショット保存。
過去のテイクへのロールバックやプロジェクトごとの独立管理を実現します。

---

## ✨ 特徴

- **⚡ 高速並列アップロード** - tokioによる並列処理で大量のサンプルファイルも高速転送
- **🔍 Smart Diff** - 変更されたファイルのみを検出してアップロード
- **🎛️ DAW対応** - Ableton Live, Logic Pro, FL Studio等の一時ファイルを自動除外
- **📦 シンプルCLI** - `gp push` だけでバックアップ完了

---

## 🚀 クイックスタート

```bash
# インストール
cargo install --path .

# プロジェクトディレクトリに移動して初期化
cd ~/Music/MyProject
gp init

# S3にプッシュ
gp push -m "ミックス完了"
```

---

## 📖 コマンド

| コマンド | 説明 |
|---------|------|
| `gp init` | プロジェクト初期化 |
| `gp push` | S3にプッシュ |
| `gp push --dry-run` | ドライラン |
| `gp status` | 状態確認 |
| `gp log` | スナップショット履歴 |
| `gp checkout <id>` | 過去の状態に復元 |
| `gp clone <project>` | S3からクローン |

---

## 📚 ドキュメント

| | |
|---|---|
| [📘 ユーザーガイド](docs/USER_GUIDE.md) | インストール、使い方、設定 |
| [🏗️ アーキテクチャ](docs/ARCHITECTURE.md) | モジュール構成、データフロー |
| [🗺️ ロードマップ](docs/ROADMAP.md) | 開発計画、今後の機能 |

---

## 🛠️ 技術スタック

```
Rust + tokio + aws-sdk-s3 + ignore + clap + indicatif
```

---

## 📄 ライセンス

MIT
