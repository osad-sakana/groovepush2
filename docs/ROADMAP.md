# ロードマップ

## フェーズ 1: 高速シンクエンジン (MVP) ✅

- [x] Parallel Push - 並列アップロード
- [x] Smart Diff - 変更ファイルのみ検出
- [x] Auto Ignore - .gp-ignore対応

## フェーズ 2: Gitライクな履歴管理 ✅

- [x] `gp push -m "メッセージ"` - コミットメッセージ保存
- [x] `gp log` - スナップショット履歴表示
- [x] `gp checkout [snapshot_id]` - 過去の状態に復元
- [x] history.json によるコミット履歴管理
- [x] Content-Addressable Storage (CAS) による重複排除

## フェーズ 3: 音楽制作特化機能

- [ ] Deduplication - 重複サンプルの排除
- [ ] WAVプレビュー生成 - 軽量MP3の自動生成
- [ ] プロジェクト間のサンプル共有
