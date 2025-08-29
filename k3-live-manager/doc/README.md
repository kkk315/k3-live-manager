# k3-live-manager ドキュメント

このフォルダは、k3-live-manager のアーキテクチャ、設計ルール、実装ルール、運用ガイドをまとめたドキュメントの集約場所です。

- architecture.md: 全体構成と責務境界
- design-rules.md: 設計ルール（層の責務、依存方向、命名など）
- implementation-rules.md: 実装ルール（API・エラーハンドリング・ログ・DB・OAuthなど）
- oauth-flow.md: OAuth 認証フローの詳細（時系列・I/Oコントラクト・セキュリティ）
- database.md: スキーマとマイグレーション方針
- operations.md: 開発・ビルド・リリース時の運用ガイド
  
補足: トークンの有効期限確認・リフレッシュ手順は `oauth-flow.md` と `specs/src-tauri_src_services_oauth_service.md` を参照。
