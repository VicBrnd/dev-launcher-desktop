// src-tauri/src/config.rs

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = ".dld-config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub added_folders: Vec<ProjectFolder>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectFolder {
    pub id: String,
    pub path: String,
    pub name: String,
    pub framework: String,
    pub framework_url: Option<String>,
}

/// Obtient le chemin du fichier de configuration.
pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Impossible de trouver le dossier utilisateur")
        .join(CONFIG_FILE_NAME)
}

/// Charge ou crée le fichier de configuration.
pub fn load_or_create_config() -> Config {
    if !config_path().exists() {
        save_config(&Config {
            added_folders: Vec::new(),
        });
    }

    let mut file =
        File::open(&config_path()).expect("Impossible d'ouvrir le fichier de configuration");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Erreur lors de la lecture du fichier de configuration");

    serde_json::from_str(&contents).unwrap_or_else(|_| Config {
        added_folders: Vec::new(),
    })
}

pub fn save_config(config: &Config) {
    let contents =
        serde_json::to_string_pretty(config).expect("Erreur lors de la conversion en JSON");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&config_path())
        .expect("Impossible de créer ou d'écrire dans le fichier de configuration");
    file.write_all(contents.as_bytes())
        .expect("Erreur lors de l'écriture du fichier de configuration");
}