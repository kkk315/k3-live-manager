use crate::db::repositories::CredentialRepository;
use std::sync::Arc;

// CredentialRepositoryトレイトに依存する新しい構造体
#[allow(dead_code)]
pub struct CredentialService {
    repo: Arc<dyn CredentialRepository + Send + Sync>,
}

#[allow(dead_code)]
impl CredentialService {
    pub fn new(repo: Arc<dyn CredentialRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    // DBから取得した資格情報の名前だけを返す、というビジネスロジック
    pub async fn get_credential_names(&self) -> anyhow::Result<Vec<String>> {
        let creds = self.repo.get_all_credentials().await?;
        let names = creds.into_iter().map(|c| c.service_name).collect();
        Ok(names)
    }
}

// --- CredentialServiceのユニットテスト ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::ServiceCredential;
    use async_trait::async_trait;

    // 1. テスト用のモックリポジトリを定義
    #[derive(Default)]
    struct MockCredentialRepository;

    #[async_trait]
    impl CredentialRepository for MockCredentialRepository {
        // 2. DBには一切アクセスせず、ハードコードされたテストデータを返す
        async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>> {
            Ok(vec![
                ServiceCredential {
                    id: 1,
                    service_name: "service1".to_string(),
                    client_id: "id1".into(),
                    client_secret: "secret1".into(),
                },
                ServiceCredential {
                    id: 2,
                    service_name: "service2".to_string(),
                    client_id: "id2".into(),
                    client_secret: "secret2".into(),
                },
            ])
        }
    }

    #[tokio::test]
    async fn test_get_credential_names_with_mock() {
        // 3. MockCredentialRepositoryをサービスに「注入」する
        let mock_repo = Arc::new(MockCredentialRepository::default());
        let service = CredentialService::new(mock_repo);

        // 4. サービスを実行
        let names = service.get_credential_names().await.unwrap();

        // 5. 結果を検証（DB接続なしでロジックをテストできている）
        assert_eq!(names, vec!["service1", "service2"]);
    }
}
