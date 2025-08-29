use crate::db::repositories::{CredentialRepository, TokenRepository};
use crate::db::models::{AddTokenPayload, ServiceCredential};
use anyhow::Context;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, TokenResponse};
use std::sync::Arc;

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
}
