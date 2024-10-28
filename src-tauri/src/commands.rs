// src-tauri/src/commands.rs

use crate::config::{load_or_create_config, save_config, ProjectFolder};
use crate::framework::detect_framework;
use crate::script::detect_package_manager_and_scripts;
use crate::types::{PackageInfo, AppState};

use serde_json::json;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;

#[tauri::command]
pub fn get_package_info(path: String) -> Option<PackageInfo> {
    let path = PathBuf::from(path);
    detect_package_manager_and_scripts(&path).map(|info| PackageInfo {
        manager: info.manager,
        scripts: info.scripts,
    })
}

#[tauri::command]
pub async fn execute_script<R: Runtime>(
    app: AppHandle<R>,
    manager: String,
    command: String,
    path: String,
    project_id: u32, // project_id ajouté
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("La fenêtre principale n'a pas été trouvée")?;

    tokio::spawn(async move {
        let cmd_result = TokioCommand::new(&manager)
            .arg("run")
            .arg(&command)
            .current_dir(&path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match cmd_result {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let window_clone = window.clone();
                    tokio::spawn(async move {
                        let reader = BufReader::new(stdout);
                        let mut lines = reader.lines();
                        while let Ok(Some(line)) = lines.next_line().await {
                            let payload = json!({
                                "project_id": project_id,
                                "output": line,
                            });
                            println!("Emitting script_output: {:?}", payload);
                            let _ = window_clone.emit("script_output", payload);
                        }
                    });
                }

                if let Some(stderr) = child.stderr.take() {
                    let window_clone = window.clone();
                    tokio::spawn(async move {
                        let reader = BufReader::new(stderr);
                        let mut lines = reader.lines();
                        while let Ok(Some(line)) = lines.next_line().await {
                            let payload = json!({
                                "project_id": project_id,
                                "output": line,
                            });
                            println!("Emitting script_error: {:?}", payload);
                            let _ = window_clone.emit("script_error", payload);
                        }
                    });
                }

                let _ = child.wait().await;
            }
            Err(e) => {
                let payload = json!({
                    "project_id": project_id,
                    "output": format!("Erreur lors de l'exécution du script: {}", e),
                });
                let _ = window.emit("script_error", payload);
            }
        }
    });

    Ok(())
}


#[tauri::command]
pub fn get_projects() -> Vec<ProjectFolder> {
    let config = load_or_create_config();
    config.added_folders
}

/// Commande pour supprimer un projet par son ID.
#[tauri::command]
pub async fn delete_project(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    let mut config = load_or_create_config();
    let initial_len = config.added_folders.len();
    config.added_folders.retain(|project| project.id != id);
    save_config(&config);

    if config.added_folders.len() < initial_len {
        // Supprimer également de l'état global
        let mut projects = state.projects.lock().await;
        projects.retain(|project| project.id != id);
        println!("Projet avec ID {} supprimé.", id);
        Ok(())
    } else {
        Err(format!("Projet avec ID {} non trouvé.", id))
    }
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
                window
                    .emit(
                        "folder_error",
                        "Le dossier sélectionné ne contient pas de package.json.",
                    )
                    .expect("Erreur lors de l'émission de l'événement d'erreur");
                return;
            }

            let mut config = load_or_create_config();

            if config
                .added_folders
                .iter()
                .any(|folder| folder.path == config_path_str)
            {
                window
                    .emit("folder_error", "Ce dossier est déjà ajouté.")
                    .expect("Erreur lors de l'émission de l'événement d'erreur");
                return;
            }

            let framework_info = detect_framework(&path_buf);
            let framework = framework_info
                .as_ref()
                .map(|f| f.name.clone())
                .unwrap_or_else(|| "Inconnu".to_string());
            let framework_url = framework_info.map(|f| f.url);

            let new_folder = ProjectFolder {
                id: uuid::Uuid::new_v4().to_string(),
                path: config_path_str.clone(),
                name: path_buf.file_name().unwrap().to_string_lossy().to_string(),
                framework,
                framework_url,
            };

            config.added_folders.push(new_folder.clone());
            save_config(&config);

            window
                .emit(
                    "folder_success",
                    serde_json::to_string(&new_folder).unwrap(),
                )
                .expect("Erreur lors de l'émission de l'événement de succès");
        } else {
            window
                .emit("folder_error", "Aucun dossier sélectionné.".to_string())
                .expect("Erreur lors de l'émission de l'événement d'erreur");
        }
    });

    Ok(())
}