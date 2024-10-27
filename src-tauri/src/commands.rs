// src-tauri/src/commands.rs
use tokio::io::{AsyncBufReadExt, BufReader}; // Importer `AsyncBufReadExt` pour utiliser `lines`
use tauri::{AppHandle, Runtime, Manager, Emitter};
use tauri_plugin_dialog::{DialogExt, FilePath};
use crate::config::{load_or_create_config, save_config, ProjectFolder};
use crate::framework::detect_framework;
use chrono::Local;
use crate::script::detect_package_manager_and_scripts;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Serialize;
use tokio::process::Command as TokioCommand;
use std::process::Stdio;

#[derive(Serialize)]
pub struct PackageInfo {
    manager: String,
    scripts: HashMap<String, String>,
}

#[tauri::command]
pub fn get_package_info(path: String) -> Option<PackageInfo> {
    let path = PathBuf::from(path);
    detect_package_manager_and_scripts(&path).map(|info| PackageInfo { manager: info.manager, scripts: info.scripts })
}

#[tauri::command]
pub async fn execute_script<R: Runtime>(
    app: AppHandle<R>,
    manager: String,
    command: String,
    path: String,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("La fenêtre principale n'a pas été trouvée")?;

    // Exécuter le script de manière asynchrone et lire les sorties
    tokio::spawn(async move {
        let mut cmd = TokioCommand::new(manager)
            .arg("run")
            .arg(command)
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Erreur lors de l'exécution du script");

        // Lire la sortie standard (stdout)
        if let Some(stdout) = cmd.stdout.take() {
            let window_clone = window.clone();  // Clone pour stdout
            let reader = BufReader::new(stdout);
            tokio::spawn(async move {
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = window_clone.emit("script_output", line);
                }
            });
        }

        // Lire la sortie d'erreur (stderr)
        if let Some(stderr) = cmd.stderr.take() {
            let window_clone = window.clone();  // Clone pour stderr
            let reader = BufReader::new(stderr);
            tokio::spawn(async move {
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let _ = window_clone.emit("script_error", line);
                }
            });
        }
    });

    Ok(())
}

#[tauri::command]
pub fn get_projects() -> Vec<ProjectFolder> {
    let config = load_or_create_config();
    config.added_folders
}

#[tauri::command]
pub fn select_folder<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("La fenêtre principale n'a pas été trouvée")?;

    app.dialog().file().pick_folder(move |folder_path| {
        if let Some(FilePath::Path(path_buf)) = folder_path {
            let config_path_str = path_buf.to_string_lossy().to_string();

            if !path_buf.join("package.json").exists() {
                window.emit("folder_error", "Le dossier sélectionné ne contient pas de package.json.")
                    .expect("Erreur lors de l'émission de l'événement d'erreur");
                return;
            }

            let mut config = load_or_create_config();

            if config.added_folders.iter().any(|folder| folder.path == config_path_str) {
                window.emit("folder_error", "Ce dossier est déjà ajouté.")
                    .expect("Erreur lors de l'émission de l'événement d'erreur");
                return;
            }

            let framework = detect_framework(&path_buf).unwrap_or_else(|| "Inconnu".to_string());

            let new_folder = ProjectFolder {
                path: config_path_str.clone(),
                name: path_buf.file_name().unwrap().to_string_lossy().to_string(),
                added_on: Local::now().format("%Y-%m-%d").to_string(),
                framework,
            };

            config.added_folders.push(new_folder.clone());
            save_config(&config);

            window.emit("folder_success", serde_json::to_string(&new_folder).unwrap())
                .expect("Erreur lors de l'émission de l'événement de succès");
        } else {
            window.emit("folder_error", "Aucun dossier sélectionné.".to_string())
                .expect("Erreur lors de l'émission de l'événement d'erreur");
        }
    });

    Ok(())
}
