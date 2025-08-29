# 仕様書: Service `CredentialService`

対象実装: `src-tauri/src/services/credential_service.rs`

## 概要

- 目的: 資格情報の取得/追加など、Repositoryを用いたビジネスロジックの入口。

## I/O 契約

- `new(repo: Arc<dyn CredentialRepository>) -> Self`
- `get_all_credentials() -> anyhow::Result<Vec<ServiceCredential>>`
- `add_credential(payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential>`
- 補助: `get_credential_names() -> anyhow::Result<Vec<String>>`

## 設計方針

- 責務: バリデーションの追加余地を持つが、現状は委譲中心。
- 依存: `CredentialRepository`
- セキュリティ: `client_secret` のログ出力は禁止。

## 関連仕様

- Command: `doc/specs/src-tauri_src_db_commands.get_service_credentials.md`
- Command: `doc/specs/src-tauri_src_db_commands.add_service_credential.md`
- Repository: `doc/specs/src-tauri_src_db_repositories.md`

## テスト項目

- 正常系: 取得/追加が成功し値を返す
- 例外系: Repository層のエラーが適切に伝播する
