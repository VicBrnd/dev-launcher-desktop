mod commands;
mod config;
mod framework;
mod script;
mod types;

use config::load_or_initialize_config;
use types::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            projects: tauri::async_runtime::Mutex::new(Vec::new()),
        })
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            load_or_initialize_config();

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::fetch_projects,
            commands::fetch_package_json,
            commands::run_script_project,
            commands::add_project,
            commands::remove_project
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de l'application Tauri");
}