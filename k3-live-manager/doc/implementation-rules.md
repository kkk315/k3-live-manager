# 実装ルール

## コマンド/サービス

- コマンドは引数検証とサービス呼び出しに限定
- サービスは Repository 経由で単件取得 (`get_credential_by_id`) を使う
- OAuth: `state` を発行しコールバックで一致検証
- API実行時は `ensure_valid_access_token(credential_id, skew)` を先行実行し、期限切れや猶予不足なら自動リフレッシュする

## DB

- マイグレーションでスキーマ管理
- `oauth_tokens.credentials_id` はユニーク（1 Credentials 1 Token）
- SQL は `?` プレースホルダ、`sqlx::query_as` を使用
- `expires_at` はUTCで管理し、リフレッシュ時に必ず更新する（フォーマットは `YYYY-MM-DD HH:MM:SS`）

## ログ/エラー

- 機微情報のマスク（アクセストークン非出力）
- `anyhow::Context` で原因を連結

## UI

- ブラウザ起動は @tauri-apps/plugin-opener を使用
- コールバックページは日本語表示、5秒で自動クローズ
