use super::models::{AddCredentialPayload, ServiceCredential};
use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait CredentialRepository {
    async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>>;
    async fn add_credential(&self, payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential>;
}

pub struct SqliteCredentialRepository {
    pool: SqlitePool,
}

impl SqliteCredentialRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CredentialRepository for SqliteCredentialRepository {
    async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>> {
        let creds = sqlx::query_as::<_, ServiceCredential>("SELECT * FROM service_credentials")
            .fetch_all(&self.pool)
            .await?;
        Ok(creds)
    }

    async fn add_credential(&self, payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential> {
        let cred = sqlx::query_as::<_, ServiceCredential>(
            "INSERT INTO service_credentials (service_name, client_id, client_secret) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(payload.service_name)
        .bind(payload.client_id)
        .bind(payload.client_secret)
        .fetch_one(&self.pool)
        .await?;
        Ok(cred)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::setup::init_test_db;

    // Helper to access the pool from the repository for tests
    impl SqliteCredentialRepository {
        fn pool(&self) -> &sqlx::SqlitePool {
            &self.pool
        }
    }

    #[tokio::test]
    async fn test_get_all_credentials() {
        // 1. Set up an in-memory test database and repository
        let pool = init_test_db().await.unwrap();
        let repo = SqliteCredentialRepository::new(pool);

        // 2. Insert test data
        sqlx::query("INSERT INTO service_credentials (id, service_name, client_id, client_secret) VALUES (1, 'test_service', 'test_id', 'test_secret')")
            .execute(repo.pool())
            .await
            .unwrap();

        // 3. Call the function with the test repository
        let credentials = repo.get_all_credentials().await.unwrap();

        // 4. Assert the results
        assert_eq!(credentials.len(), 1);
        assert_eq!(credentials[0].service_name, "test_service");
    }

    #[tokio::test]
    async fn test_add_credential() {
        // 1. Set up an in-memory test database and repository
        let pool = init_test_db().await.unwrap();
        let repo = SqliteCredentialRepository::new(pool);

        // 2. Create a payload
        let payload = AddCredentialPayload {
            service_name: "new_service".to_string(),
            client_id: "new_id".to_string(),
            client_secret: "new_secret".to_string(),
        };

        // 3. Call the add function
        let added_cred = repo.add_credential(payload).await.unwrap();

        // 4. Assert the returned credential
        assert_eq!(added_cred.service_name, "new_service");
        assert_eq!(added_cred.client_id, "new_id");

        // 5. Assert that the credential was actually inserted
        let all_creds = repo.get_all_credentials().await.unwrap();
        assert_eq!(all_creds.len(), 1);
        assert_eq!(all_creds[0].service_name, "new_service");
    }
}
