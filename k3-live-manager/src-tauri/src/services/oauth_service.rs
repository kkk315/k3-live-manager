use crate::db::repositories::{CredentialRepository, TokenRepository};
use crate::db::models::{AddTokenPayload, ServiceCredential};
use anyhow::Context;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, TokenResponse, RefreshToken};
use std::sync::Arc;
use chrono::{NaiveDateTime, Utc, Duration as ChronoDuration, TimeZone};

// Google OAuth2 Client using oauth2 v4.4.0 API
fn create_google_oauth_client(
    credential: &ServiceCredential,
    redirect_url: &str,
) -> anyhow::Result<oauth2::basic::BasicClient> {
    let client_id = ClientId::new(credential.client_id.clone());
    let client_secret = ClientSecret::new(credential.client_secret.clone());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    let redirect_url = RedirectUrl::new(redirect_url.to_string())?;

    let client = oauth2::basic::BasicClient::new(
        client_id, 
        Some(client_secret), 
        auth_url, 
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}

#[derive(Clone)]
pub struct OAuthService {
    credential_repo: Arc<dyn CredentialRepository + Send + Sync>,
    token_repo: Arc<dyn TokenRepository + Send + Sync>,
}

impl OAuthService {
    pub fn new(
        credential_repo: Arc<dyn CredentialRepository + Send + Sync>,
        token_repo: Arc<dyn TokenRepository + Send + Sync>,
    ) -> Self {
        Self { credential_repo, token_repo }
    }

    pub async fn generate_auth_url(&self, credential_id: i64, redirect_url: &str) -> anyhow::Result<(String, String)> {
        let credential = self
            .credential_repo
            .get_credential_by_id(credential_id)
            .await?
            .context("Credential not found")?;

    let client = create_google_oauth_client(&credential, redirect_url)?;

    let (authorize_url, csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/youtube".to_string()))
            .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .add_scope(oauth2::Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .url();

    Ok((authorize_url.to_string(), csrf_token.secret().to_string()))
    }

    pub async fn exchange_code_and_save_token(&self, code: String, credential_id: i64, redirect_url: &str) -> anyhow::Result<()> {
        println!("Starting token exchange for credential_id: {}", credential_id);
        let credential = self
            .credential_repo
            .get_credential_by_id(credential_id)
            .await?
            .context("Credential not found")?;

    let client = create_google_oauth_client(&credential, redirect_url)?;

        let token_result = client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .context("Failed to exchange code for token")?;

    println!("Token exchange successful.");

        let payload = AddTokenPayload {
            credentials_id: credential_id,
            access_token: token_result.access_token().secret().to_string(),
            refresh_token: token_result.refresh_token().map_or("no_refresh_token".to_string(), |t| t.secret().to_string()),
            expires_at: match token_result.expires_in() {
                Some(duration) => {
                    let now = std::time::SystemTime::now();
                    let future = now + duration;
                    let datetime: chrono::DateTime<chrono::Utc> = future.into();
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                },
                None => "2099-12-31 23:59:59".to_string(),
            },
            scope: Some(token_result.scopes().map_or("".to_string(), |s| s.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))),
        };

        self.token_repo.upsert_token(payload).await.context("Failed to save token to database")?;

        println!("Token saved to database.");
        Ok(())
    }

    // Ensure the access token is valid; refresh if expired or within skew seconds
    pub async fn ensure_valid_access_token(&self, credential_id: i64, skew_secs: u64) -> anyhow::Result<(String, String)> {
        let token_opt = self
            .token_repo
            .get_token_by_credential_id(credential_id)
            .await?;

        let token = token_opt.context("Token not found")?;

        // Parse expires_at in UTC "YYYY-MM-DD HH:MM:SS"; if parse fails, treat as expired
        let need_refresh =
        match NaiveDateTime::parse_from_str(&token.expires_at, "%Y-%m-%d %H:%M:%S") {
                Ok(ndt) => {
            let exp = Utc.from_utc_datetime(&ndt);
                    let skew = ChronoDuration::seconds(skew_secs as i64);
                    let threshold = Utc::now() + skew;
                    exp <= threshold
                }
                Err(_) => true,
            };

        if !need_refresh {
            return Ok((token.access_token, token.expires_at));
        }

        // Require refresh_token for refresh flow
        if token.refresh_token.is_empty() || token.refresh_token == "no_refresh_token" {
            anyhow::bail!("No refresh_token available for credential_id={}", credential_id);
        }

        // Perform refresh and return updated token info
        let (access_token, expires_at) = self.refresh_access_token(credential_id).await?;
        Ok((access_token, expires_at))
    }

    // Refresh the access token using the stored refresh_token and persist the new values
    pub async fn refresh_access_token(&self, credential_id: i64) -> anyhow::Result<(String, String)> {
        // Load credential for client configuration
        let credential = self
            .credential_repo
            .get_credential_by_id(credential_id)
            .await?
            .context("Credential not found")?;

        // Load current token to obtain refresh_token
        let current_token = self
            .token_repo
            .get_token_by_credential_id(credential_id)
            .await?
            .context("Token not found")?;

        // Use fixed redirect URL per design
        let redirect_url = "http://localhost:1421/oauth/callback";
        let client = create_google_oauth_client(&credential, redirect_url)?;

        let refresh_token_val = current_token.refresh_token.clone();
        let token_result = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token_val.clone()))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .context("Failed to refresh access token")?;

        // Some providers don't return refresh_token on refresh; keep existing if absent
        let new_refresh_token = token_result
            .refresh_token()
            .map(|t| t.secret().to_string())
            .unwrap_or(refresh_token_val);

        let expires_at = match token_result.expires_in() {
            Some(duration) => {
                let now = std::time::SystemTime::now();
                let future = now + duration;
                let datetime: chrono::DateTime<chrono::Utc> = future.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            None => "2099-12-31 23:59:59".to_string(),
        };

        let payload = AddTokenPayload {
            credentials_id: credential_id,
            access_token: token_result.access_token().secret().to_string(),
            refresh_token: new_refresh_token,
            expires_at: expires_at.clone(),
            scope: Some(token_result.scopes().map_or("".to_string(), |s| s.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))),
        };

        self
            .token_repo
            .upsert_token(payload)
            .await
            .context("Failed to save refreshed token to database")?;

        Ok((token_result.access_token().secret().to_string(), expires_at))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::repositories::{SqliteRepository, CredentialRepository, TokenRepository};
    use crate::db::models::{AddCredentialPayload};
    use crate::db::setup::init_test_db;
    use sqlx::SqlitePool;

    async fn setup_repo() -> (Arc<SqliteRepository>, i64) {
        let pool: SqlitePool = init_test_db().await.unwrap();
        let repo = Arc::new(SqliteRepository::new(pool));
        let cred = repo.add_credential(AddCredentialPayload{
            service_name: "google".into(),
            client_id: "cid".into(),
            client_secret: "csec".into(),
        }).await.unwrap();
        (repo, cred.id)
    }

    #[tokio::test]
    async fn ensure_valid_access_token_returns_existing_when_not_expiring() {
        let (repo, cred_id) = setup_repo().await;
        let svc = OAuthService::new(repo.clone(), repo.clone());

        // Insert token that expires far in the future
        let payload = AddTokenPayload{
            credentials_id: cred_id,
            access_token: "a1".into(),
            refresh_token: "r1".into(),
            expires_at: "2099-12-31 23:59:59".into(),
            scope: Some("s".into()),
        };
        repo.upsert_token(payload).await.unwrap();

        let (at, exp) = svc.ensure_valid_access_token(cred_id, 120).await.unwrap();
        assert_eq!(at, "a1");
        assert_eq!(exp, "2099-12-31 23:59:59");
    }

    #[tokio::test]
    async fn ensure_valid_access_token_errors_without_refresh_token() {
        let (repo, cred_id) = setup_repo().await;
        let svc = OAuthService::new(repo.clone(), repo.clone());

        // Insert expired token and no refresh token
        let payload = AddTokenPayload{
            credentials_id: cred_id,
            access_token: "a1".into(),
            refresh_token: "".into(),
            expires_at: "2000-01-01 00:00:00".into(),
            scope: None,
        };
        repo.upsert_token(payload).await.unwrap();

        let res = svc.ensure_valid_access_token(cred_id, 0).await;
        assert!(res.is_err());
    }
}
