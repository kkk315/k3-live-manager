use super::repositories::{CredentialRepository, SqliteCredentialRepository};
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

// The state that will be managed by Tauri
pub struct AppState(pub Arc<dyn CredentialRepository + Send + Sync>);

// Initializes the database and sets up the repository in the app state.
pub async fn init(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // In development, the database file `app.sqlite` is in the root of the project (`k3-live-manager`),
    // which is the parent directory of `src-tauri`.
    let pool = SqlitePool::connect("sqlite:../app.sqlite").await?;

    // The `migrate!` macro looks for the migrations directory relative to `CARGO_MANIFEST_DIR`.
    // `CARGO_MANIFEST_DIR` is `k3-live-manager/src-tauri` for the backend.
    sqlx::migrate!("./migrations").run(&pool).await?;

    let repo = Arc::new(SqliteCredentialRepository::new(pool));
    app_handle.manage(AppState(repo));

    Ok(())
}

// This function is only compiled for tests.
// It sets up an in-memory SQLite database and runs migrations.
#[cfg(test)]
pub async fn init_test_db() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
