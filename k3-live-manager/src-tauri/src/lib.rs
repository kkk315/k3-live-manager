mod db;
mod services;
mod oauth_server;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                db::setup::init(&handle)
                    .await
                    .expect("failed to initialize database");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            db::commands::get_service_credentials,
            db::commands::add_service_credential,
            db::commands::start_oauth_flow
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
