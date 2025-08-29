# アーキテクチャ概要

- フロントエンド: React + TypeScript (Vite)。UI とユーザー操作を担当。
- バックエンド: Tauri (Rust)。アプリプロセス、DB、OAuth コールバック HTTP サーバを担当。
- データベース: SQLite + sqlx。ローカル永続化。
- 認証: OAuth2（Google/YouTube）。外部ブラウザ + ローカル HTTP サーバでコールバックを受領。

## 層構造と責務

- pages/components (FE): UI 構築。Tauri commands を呼び出すのみ。
- commands.rs (BE): Tauri コマンドのエントリポイント。入出力バリデーション、サービス呼び出し、タスク起動。
- services/* (BE): ユースケース/ドメインロジック。CSRF state 検証、OAuth フロー統括、検証、トランザクション制御。
- db/repositories.rs (BE): データアクセス。SQL 文の保持、入出力モデル変換。副作用は DB のみ。
- db/models.rs (BE): DB モデル/ペイロード定義。
- db/setup.rs (BE): コネクションプール初期化、AppState の DI。
- oauth_server.rs (BE): OAuth コールバック専用 HTTP サーバ（単一接続）。

依存方向: UI -> commands -> services -> repositories -> DB

## データフロー（認証）

1) UI から start_oauth_flow を呼ぶ
2) commands.rs が auth URL を生成、ローカルサーバを起動し、URL を返す
3) 外部ブラウザで認証
4) oauth_server が code/state を受領して oneshot で commands 側に通知
5) commands.rs が state 検証後、OAuthService でトークン交換→保存
6) 保存済みトークンは repository 経由で取得

## 重要な非機能

- セキュリティ: CSRF(state) 検証、アクセストークン非出力、DB 非公開
- 可用性: 固定ポート(1421) 利用の前提（設計意図）。
- テスト: repository テストはインメモリ SQLite、サービスのユニットテストでビジネスロジック検証。
- 運用: API実行前にアクセストークンの有効性確認→必要に応じてリフレッシュ（サービス層で共通化）
