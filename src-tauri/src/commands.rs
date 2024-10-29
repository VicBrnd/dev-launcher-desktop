// src-tauri/src/commands.rs

use crate::config::{load_or_initialize_config, save_config};
use crate::framework::fetch_framework;
use crate::script::detect_package_manager_and_scripts;
use crate::types::{FetchPackageJson, ProjectConfig, AppState};

use serde_json::json;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;


/// Commande pour ajouter un projet.
#[tauri::command]
pub fn add_project<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let main_window = app_handle.get_webview_window("main").ok_or("Fenêtre principale introuvable")?;

    app_handle.dialog().file().pick_folder(move |selected_folder| {
        let project_path = match selected_folder.and_then(|folder| match folder {
            FilePath::Path(path) if path.join("package.json").exists() => Some(path),
            _ => {
                let _ = main_window.emit("folder_error", "Dossier non valide ou package.json manquant.");
                None
            }
        }) {
            Some(path) => path,
            None => return,
        };

        let project_path_str = project_path.to_string_lossy().to_string();
        let mut config_data = load_or_initialize_config();

        if config_data.project_folders.iter().any(|project| project.path == project_path_str) {
            let _ = main_window.emit("folder_error", "Ce dossier est déjà ajouté.");
            return;
        }

        let framework_data = fetch_framework(&project_path);
        let new_project = ProjectConfig {
            id: uuid::Uuid::new_v4().to_string(),
            path: project_path_str.clone(),
            name: project_path.file_name().unwrap().to_string_lossy().to_string(),
            framework: framework_data.as_ref().map(|f| f.name.clone()).unwrap_or_else(|| "Inconnu".to_string()),
            framework_url: framework_data.map(|f| f.url),
        };
        config_data.project_folders.push(new_project.clone());

        if let Err(e) = save_config(&config_data)
            .map_err(|e| e.to_string())
            .and_then(|_| {
                println!("Projet avec ID {} ajouté.", new_project.id);
                serde_json::to_string(&new_project).map_err(|e| e.to_string())
            })
            .and_then(|project_json| main_window.emit("folder_success", project_json).map_err(|e| e.to_string()))
        {
            let _ = main_window.emit("folder_error", format!("Erreur : {}", e));
        }
    });

    Ok(())
}

/// Commande pour supprimer un projet par son ID.
#[tauri::command]
pub async fn remove_project(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    let mut config = load_or_initialize_config();
    let initial_len = config.project_folders.len();
    config.project_folders.retain(|project| project.id != id);

    // Enregistrer les modifications et gérer les erreurs
    save_config(&config).map_err(|e| format!("Erreur lors de la sauvegarde : {}", e))?;

    if config.project_folders.len() < initial_len {
        let mut projects = state.projects.lock().await;
        projects.retain(|project| project.id != id);
        println!("Projet avec ID {} supprimé.", id);
        Ok(())
    } else {
        Err(format!("Projet avec ID {} non trouvé.", id))
    }
}

#[tauri::command]
pub fn fetch_projects() -> Vec<ProjectConfig> {
    let config = load_or_initialize_config();
    config.project_folders
}

#[tauri::command]
pub fn fetch_package_json(path: String) -> Option<FetchPackageJson> {
    let path = PathBuf::from(path);
    detect_package_manager_and_scripts(&path).map(|info| FetchPackageJson {
        manager: info.manager,
        scripts: info.scripts,
    })
}

#[tauri::command]
pub async fn run_script_project<R: Runtime>(
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





