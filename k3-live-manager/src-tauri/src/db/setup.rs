use super::repositories::SqliteRepository;
use crate::services::{
    credential_service::CredentialService,
    oauth_service::OAuthService,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

// A single state struct to hold all services
pub struct AppState {
    pub credential_service: CredentialService,
    pub oauth_service: OAuthService,
}

// Initializes the database and sets up all services in the app state.
pub async fn init(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite:../app.sqlite").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Create a single repository instance, wrapped in an Arc for shared ownership
    let repo = Arc::new(SqliteRepository::new(pool));

    // Create services, passing a clone of the repository Arc to each
    let credential_service = CredentialService::new(repo.clone());
    let oauth_service = OAuthService::new(repo.clone(), repo.clone());

    // Create the final AppState and manage it
    let app_state = AppState {
        credential_service,
        oauth_service,
    };
    app_handle.manage(app_state);

    Ok(())
}

// This function is only compiled for tests.
#[cfg(test)]
pub async fn init_test_db() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
