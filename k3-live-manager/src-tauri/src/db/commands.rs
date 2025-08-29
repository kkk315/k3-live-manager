use crate::db::models::{AddCredentialPayload, ServiceCredential};
use crate::db::setup::AppState;
use tauri::State;
use tokio::sync::oneshot;
use crate::oauth_server;

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
    let port = 1421; // Or find an available port dynamically
    let redirect_url = format!("http://localhost:{}/oauth/callback", port);

    // Generate the auth URL with the temporary server's redirect URL
    let auth_url = state.oauth_service.generate_auth_url(credential_id, &redirect_url).await.map_err(|e| e.to_string())?;

    // Spawn the server in a background task
    let oauth_service = state.oauth_service.clone();
    tauri::async_runtime::spawn(async move {
        // Start the OAuth server
        if let Err(e) = oauth_server::start_oauth_server(tx, port).await {
            eprintln!("OAuth server error: {:?}", e);
            return;
        }
    });

    // Spawn another task to wait for the callback and process the token
    let oauth_service_clone = state.oauth_service.clone();
    tauri::async_runtime::spawn(async move {
        // Wait for the code and state from the server
        match rx.await {
            Ok((code, _state)) => {
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