mod commands;
mod models;
mod services;

use commands::config::AppState;
use models::AppConfig;
use services::ConfigService;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Disable GPU acceleration on Windows to fix rendering artifacts (diagonal lines)
    #[cfg(windows)]
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-gpu");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            // Get app data directory
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

            let config_path = app_dir.join("config.json");

            // Load or create config
            let config = if config_path.exists() {
                ConfigService::load(&config_path).unwrap_or_default()
            } else {
                AppConfig::default()
            };

            app.manage(AppState {
                config: Mutex::new(config),
                config_path,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::load_config,
            commands::save_config,
            commands::create_profile,
            commands::update_profile,
            commands::delete_profile,
            commands::create_source,
            commands::update_source,
            commands::delete_source,
            commands::set_active_profile,
            // Filesystem commands
            commands::list_items,
            commands::get_items_with_status,
            commands::validate_path,
            // Symlink commands
            commands::toggle_item,
            commands::check_symlink_permissions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
