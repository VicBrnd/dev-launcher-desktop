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
    pub last_updated: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
}

/// Structure représentant l'état global de l'application.
#[derive(Default)]
pub struct AppState {
    pub projects: Mutex<Vec<Project>>,
}

/// Structure pour PackageInfo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub manager: String,
    pub scripts: HashMap<String, String>,
}
