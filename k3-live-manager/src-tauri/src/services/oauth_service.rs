use crate::db::repositories::{CredentialRepository, TokenRepository};
use crate::db::models::{AddTokenPayload, ServiceCredential};
use anyhow::Context;
use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
};
use std::sync::Arc;

// Google OAuth2 Client using oauth2 v4.4.0 API
fn create_google_oauth_client(
    credential: &ServiceCredential,
) -> anyhow::Result<BasicClient> {
    let client_id = ClientId::new(credential.client_id.clone());
    let client_secret = ClientSecret::new(credential.client_secret.clone());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    // The redirect URI must be configured in the Google Cloud Console
    let redirect_url = RedirectUrl::new("http://localhost:1421/oauth/callback".to_string())?;

    let client = BasicClient::new(
        client_id, 
        Some(client_secret), 
        auth_url, 
        Some(token_url)
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}

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

    pub async fn generate_auth_url(&self, credential_id: i64) -> anyhow::Result<String> {
        let all_creds = self.credential_repo.get_all_credentials().await?;
        let credential = all_creds.iter().find(|c| c.id == credential_id).context("Credential not found")?;

        let client = create_google_oauth_client(credential)?;

        let (authorize_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("https://www.googleapis.com/auth/youtube.readonly".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .url();

        Ok(authorize_url.to_string())
    }

    pub async fn exchange_code_and_save_token(&self, code: String, credential_id: i64) -> anyhow::Result<()> {
        let all_creds = self.credential_repo.get_all_credentials().await?;
        let credential = all_creds.iter().find(|c| c.id == credential_id).context("Credential not found")?;

        let client = create_google_oauth_client(credential)?;

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await?;

        let payload = AddTokenPayload {
            credentials_id: credential_id,
            access_token: token_result.access_token().secret().to_string(),
            refresh_token: token_result.refresh_token().map_or("".to_string(), |t| t.secret().to_string()),
            expires_at: token_result.expires_in().map_or("".to_string(), |d| d.as_secs().to_string()),
            scope: Some(token_result.scopes().map_or("".to_string(), |s| s.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))),
        };

        self.token_repo.add_token(payload).await?;

        Ok(())
    }
}
