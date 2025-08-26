use super::models::ServiceCredential;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait CredentialRepository {
    async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>>;
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
}
