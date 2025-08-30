use crate::db::models::{AddCredentialPayload, ServiceCredential};
use crate::db::setup::AppState;
use tauri::State;
use tokio::sync::oneshot;
use crate::oauth_server;
use serde::Serialize;

// --- Credential Commands ---
#[tauri::command]
pub async fn get_service_credentials(
    state: State<'_, AppState>,
) -> Result<Vec<ServiceCredential>, String> {
    state.credential_service.get_all_credentials().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_service_credential(
    payload: AddCredentialPayload,
    state: State<'_, AppState>,
) -> Result<ServiceCredential, String> {
    state.credential_service.add_credential(payload).await.map_err(|e| e.to_string())
}

// --- OAuth Commands ---
#[tauri::command]
pub async fn start_oauth_flow(
    credential_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let (tx, rx) = oneshot::channel();
    let port = 1421; // fixed by design
    let redirect_url = format!("http://localhost:{}/oauth/callback", port);

    // Generate the auth URL with state and the temporary server's redirect URL
    let (auth_url, expected_state) = state
        .oauth_service
        .generate_auth_url(credential_id, &redirect_url)
        .await
        .map_err(|e| e.to_string())?;

    // Spawn the server in a background task
    tauri::async_runtime::spawn(async move {
        // Start the OAuth server
        if let Err(e) = oauth_server::start_oauth_server(tx, port).await {
            eprintln!("OAuth server error: {:?}", e);
            return;
        }
    });

    // Spawn another task to wait for the callback and process the token
    let oauth_service_clone = state.oauth_service.clone();
    let expected_state_clone = expected_state.clone();
    tauri::async_runtime::spawn(async move {
        // Wait for the code and state from the server
        match rx.await {
            Ok((code, state_val)) => {
                if state_val != expected_state_clone {
                    eprintln!("State mismatch in OAuth callback. Potential CSRF.");
                    return;
                }
                // Finalize OAuth with the received code
                if let Err(e) = oauth_service_clone.exchange_code_and_save_token(code, credential_id, &redirect_url).await {
                    eprintln!("Failed to exchange OAuth code: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to receive OAuth code: {}", e);
            }
        }
    });

    // Return the auth URL immediately
    Ok(auth_url)
}

#[derive(Serialize)]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub expires_at: String,
}

/// Ensure a valid access token is available for the given credential.
/// If the current token is expired or within skew seconds to expire, it will be refreshed.
#[tauri::command]
pub async fn ensure_valid_access_token(
    credential_id: i64,
    skew_secs: i64,
    state: State<'_, AppState>,
) -> Result<AccessTokenInfo, String> {
    let skew = if skew_secs < 0 { 0 } else { skew_secs as u64 };
    let (access_token, expires_at) = state
        .oauth_service
        .ensure_valid_access_token(credential_id, skew)
        .await
        .map_err(|e| e.to_string())?;
    Ok(AccessTokenInfo { access_token, expires_at })
}