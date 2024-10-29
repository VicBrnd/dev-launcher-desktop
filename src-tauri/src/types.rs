// src-tauri/src/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::async_runtime::Mutex;

/// Structure représentant un projet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub framework: Option<String>,
    pub framework_url: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
}

// Structure représentant la configuration d'un projet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub id: String,
    pub path: String,
    pub name: String,
    pub framework: String,
    pub framework_url: Option<String>,
}

/// Structure représentant l'état global de l'application.
#[derive(Default)]
pub struct AppState {
    pub projects: Mutex<Vec<Project>>,
}

/// Structure pour FetchPackageJson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchPackageJson {
    pub manager: String,
    pub scripts: HashMap<String, String>,
}
