use super::models::ServiceCredential;
use super::setup::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_service_credentials(
    state: State<'_, AppState>,
) -> Result<Vec<ServiceCredential>, String> {
    let repo = &state.0;
    repo.get_all_credentials()
        .await
        .map_err(|e| e.to_string())
}
