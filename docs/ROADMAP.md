# ロードマップ

## フェーズ 1: ローカルMVP ✅

- [x] ファイルスキャン・SHA256ハッシュ
- [x] Smart Diff - 変更ファイルのみ検出
- [x] Auto Ignore - .gp-ignore対応
- [x] Content-Addressable Storage (CAS) による重複排除

## フェーズ 2: バージョン管理 ✅

- [x] `gp commit -m "メッセージ"` - スナップショット保存
- [x] `gp log` - スナップショット履歴表示
- [x] `gp checkout [snapshot_id]` - 過去の状態に復元
- [x] history.json によるコミット履歴管理

## フェーズ 3: リモートバックアップ

- [ ] `gp remote add <url>` - バックアップ先の設定
- [ ] `gp push` - リモートへの同期
- [ ] `gp clone <url>` - リモートからの復元
- [ ] バックアップ先の選択（S3, Backblaze B2, ローカルNAS等）

## フェーズ 4: 音楽制作特化機能

- [ ] WAVプレビュー生成 - 軽量MP3の自動生成
- [ ] プロジェクト間のサンプル共有
- [ ] DAW別のスマートな除外設定
