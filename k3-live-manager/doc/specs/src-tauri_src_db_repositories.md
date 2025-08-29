# 仕様書: DB Repositories

対象実装: `src-tauri/src/db/repositories.rs`

## 概要

- 目的: 資格情報とトークンのDB入出力を抽象化（Trait）し、`SqliteRepository`でSQLite実装を提供。

## I/F 定義

- `trait CredentialRepository`
  - `get_all_credentials() -> Vec<ServiceCredential>`
  - `add_credential(payload: AddCredentialPayload) -> ServiceCredential`
  - `get_credential_by_id(id: i64) -> Option<ServiceCredential>`

- `trait TokenRepository`
  - `upsert_token(payload: AddTokenPayload) -> OauthToken`
  - `get_token_by_credential_id(credential_id: i64) -> Option<OauthToken>`

## 実装（SqliteRepository）

- `get_all_credentials`: `SELECT * FROM service_credentials`
- `add_credential`: `INSERT ... RETURNING *`
- `get_credential_by_id`: `SELECT * WHERE id = ?`
- `upsert_token`: `INSERT ... ON CONFLICT(credentials_id) DO UPDATE ... RETURNING *`
- `get_token_by_credential_id`: `SELECT * WHERE credentials_id = ?`

備考:

- `expires_at` は文字列（TIMESTAMP）で保存。リフレッシュ時に新しい値へ更新される

## 設計方針/セキュリティ

- ビジネスロジックは持たず、SQLのみを責務とする
- `oauth_tokens.credentials_id` はユニーク（Upsertで整合）
- 秘密値（client_secret, access_token等）はログに出さない

## テスト項目

- 正常系: 資格情報の追加/取得、Upsertが更新になることの検証
- 例外系: DB接続失敗時のエラー伝播
