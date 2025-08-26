use super::models::{AddCredentialPayload, AddTokenPayload, OauthToken, ServiceCredential};
use async_trait::async_trait;
use sqlx::SqlitePool;

// --- Credential Repository ---
#[async_trait]
pub trait CredentialRepository {
    async fn get_all_credentials(&self) -> anyhow::Result<Vec<ServiceCredential>>;
    async fn add_credential(&self, payload: AddCredentialPayload) -> anyhow::Result<ServiceCredential>;
}

// --- Token Repository ---
#[async_trait]
pub trait TokenRepository {
    async fn add_token(&self, payload: AddTokenPayload) -> anyhow::Result<OauthToken>;
    #[allow(dead_code)]
    async fn get_token_by_credential_id(&self, credential_id: i64) -> anyhow::Result<Option<OauthToken>>;
}

// --- Concrete Implementation ---
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CredentialRepository for SqliteRepository {
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

#[async_trait]
impl TokenRepository for SqliteRepository {
    async fn add_token(&self, payload: AddTokenPayload) -> anyhow::Result<OauthToken> {
        let token = sqlx::query_as::<_, OauthToken>(
            "INSERT INTO oauth_tokens (credentials_id, access_token, refresh_token, expires_at, scope) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )
        .bind(payload.credentials_id)
        .bind(payload.access_token)
        .bind(payload.refresh_token)
        .bind(payload.expires_at)
        .bind(payload.scope)
        .fetch_one(&self.pool)
        .await?;
        Ok(token)
    }

    async fn get_token_by_credential_id(&self, credential_id: i64) -> anyhow::Result<Option<OauthToken>> {
        let token = sqlx::query_as::<_, OauthToken>("SELECT * FROM oauth_tokens WHERE credentials_id = ?")
            .bind(credential_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(token)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::setup::init_test_db;

    #[tokio::test]
    async fn test_add_and_get_credential() {
        let pool = init_test_db().await.unwrap();
        let repo = SqliteRepository::new(pool);
        let payload = AddCredentialPayload {
            service_name: "test".to_string(),
            client_id: "id".to_string(),
            client_secret: "secret".to_string(),
        };
        repo.add_credential(payload).await.unwrap();
        let creds = repo.get_all_credentials().await.unwrap();
        assert_eq!(creds.len(), 1);
    }

    #[tokio::test]
    async fn test_add_and_get_token() {
        let pool = init_test_db().await.unwrap();
        let repo = SqliteRepository::new(pool);

        // We need a credential first
        let cred_payload = AddCredentialPayload {
            service_name: "test".to_string(),
            client_id: "id".to_string(),
            client_secret: "secret".to_string(),
        };
        let cred = repo.add_credential(cred_payload).await.unwrap();

        let token_payload = AddTokenPayload {
            credentials_id: cred.id,
            access_token: "test_access".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_at: "never".to_string(),
            scope: Some("read".to_string()),
        };

        let added_token = repo.add_token(token_payload).await.unwrap();
        assert_eq!(added_token.access_token, "test_access");

        let fetched_token = repo.get_token_by_credential_id(cred.id).await.unwrap().unwrap();
        assert_eq!(fetched_token.access_token, "test_access");
    }
}
