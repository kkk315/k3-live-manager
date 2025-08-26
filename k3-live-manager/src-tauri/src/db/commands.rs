use crate::db::models::{AddCredentialPayload, ServiceCredential};
use crate::db::setup::AppState;
use tauri::State;

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
pub async fn generate_google_auth_url(
    credential_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.oauth_service.generate_auth_url(credential_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn finalize_google_oauth(
    code: String,
    credential_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.oauth_service.exchange_code_and_save_token(code, credential_id).await.map_err(|e| e.to_string())
}

