use crate::db::models::{AddCredentialPayload, ServiceCredential};
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

    //--- Pass-through methods ---
    pub async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>> {
        self.repo.get_all_credentials().await
    }

    pub async fn add_credential(&self, payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential> {
        // In a real app, you might have validation or other business logic here
        self.repo.add_credential(payload).await
    }

    //--- Business logic methods ---
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
    use async_trait::async_trait;

    // 1. テスト用のモックリポジトリを定義
    #[derive(Default)]
    struct MockCredentialRepository {
        credentials: Vec<ServiceCredential>,
    }

    #[async_trait]
    impl CredentialRepository for MockCredentialRepository {
        async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>> {
            Ok(self.credentials.clone())
        }

        async fn add_credential(&self, payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential> {
            let new_cred = ServiceCredential {
                id: (self.credentials.len() + 1) as i64, // simple id generation
                service_name: payload.service_name,
                client_id: payload.client_id,
                client_secret: payload.client_secret,
            };
            // In a real mock, you might want to actually add to the vec
            // to test interactions between add and get.
            Ok(new_cred)
        }
    }

    #[tokio::test]
    async fn test_add_credential_with_mock() {
        let mock_repo = Arc::new(MockCredentialRepository::default());
        let service = CredentialService::new(mock_repo.clone());

        let payload = AddCredentialPayload {
            service_name: "test".to_string(),
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
        };

        let result = service.add_credential(payload).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().service_name, "test");
    }
}
