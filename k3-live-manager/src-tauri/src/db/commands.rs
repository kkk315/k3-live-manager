use crate::db::models::{AddCredentialPayload, ServiceCredential};
use crate::services::credential_service::CredentialService;
use tauri::State;

#[tauri::command]
pub async fn get_service_credentials(
    state: State<'_, CredentialService>,
) -> Result<Vec<ServiceCredential>, String> {
    state.get_all_credentials().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_service_credential(
    payload: AddCredentialPayload,
    state: State<'_, CredentialService>,
) -> Result<ServiceCredential, String> {
    state.add_credential(payload).await.map_err(|e| e.to_string())
}

