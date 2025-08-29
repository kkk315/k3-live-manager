# 仕様書: Tauri コマンド `add_service_credential`

対象実装: `src-tauri/src/db/commands.rs` の `add_service_credential`

## 概要

- 目的: 資格情報（service_name, client_id, client_secret）を1件追加。
- 背景/前提: バリデーションはService側（必須チェック等）。

## I/O 契約

- 入力: `payload: AddCredentialPayload { service_name: String, client_id: String, client_secret: String }`
- 出力: `Ok(ServiceCredential)` 追加後のレコード
- エラー: `Err(String)`

## 設計方針

- 層の責務: Commandは受け取りと結果返却のみ。InsertはService→Repository。
- 依存関係: `credential_service.add_credential(payload)`
- セキュリティ: client_secretの取り扱いに注意。ログ出力禁止。

## URL（フロント連携）

- 呼び出し元: `src/pages/AddCredentialPage.tsx` の保存
- 画面URL: `/credentials/add`

## テスト項目

- 正常系: 正しいペイロードで追加され、戻り値が返る。
- 異常系: 必須欠落時にエラー。
