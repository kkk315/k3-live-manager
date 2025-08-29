# 仕様書: Tauri コマンド `get_service_credentials`

対象実装: `src-tauri/src/db/commands.rs` の `get_service_credentials`

## 概要

- 目的: 登録済みのサービス資格情報一覧を取得してUIに返す。
- 背景/前提: SQLiteの`service_credentials`テーブルから全件取得。

## I/O 契約

- 入力: なし
- 出力: `Ok(Vec<ServiceCredential>)`
- エラー: `Err(String)`

## 設計方針

- 層の責務: Commandは橋渡し。取得はCredentialService→Repository。
- 依存関係: `credential_service.get_all_credentials()`
- セキュリティ: 秘密情報（client_secret）は返却仕様上含まれるのでUI側での表示/取り扱いに注意。

## URL（フロント連携）

- 呼び出し元: `src/pages/CredentialsListPage.tsx` 初期表示でinvoke。
- 画面URL: `/credentials`

## テスト項目

- 正常系: 複数件/0件の取得。
- 異常系: DB接続不可時にエラー文字列を返す。
