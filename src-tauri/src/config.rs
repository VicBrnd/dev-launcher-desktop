use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::io::Error;
use crate::types::ProjectConfig;

const CONFIG_FILE_NAME: &str = ".dld-config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevLauncherConfig {
    pub project_folders: Vec<ProjectConfig>,
}

/// Obtient le chemin du fichier de configuration.
pub fn get_config_file_path() -> PathBuf {
    dirs::home_dir()
        .expect("Impossible de trouver le dossier utilisateur")
        .join(CONFIG_FILE_NAME)
}

/// Charge le fichier de configuration s'il existe.
pub fn load_config() -> Result<DevLauncherConfig, Error> {
    let config_path = get_config_file_path();
    
    if !Path::new(&config_path).exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Fichier de configuration introuvable",
        ));
    }

    let file = File::open(config_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    
    let config: DevLauncherConfig = serde_json::from_str(&contents).unwrap_or_else(|_| DevLauncherConfig {
        project_folders: Vec::new(),
    });
    Ok(config)
}

/// Initialise une configuration par défaut et la sauvegarde dans le fichier de configuration.
pub fn initialize_config() -> Result<DevLauncherConfig, Error> {
    let default_config = DevLauncherConfig {
        project_folders: Vec::new(),
    };
    save_config(&default_config)?;
    Ok(default_config)
}

/// Charge ou initialise le fichier de configuration.
pub fn load_or_initialize_config() -> DevLauncherConfig {
    load_config().unwrap_or_else(|_| initialize_config().unwrap_or_else(|e| {
        eprintln!("Erreur lors de l'initialisation du fichier de configuration: {:?}", e);
        DevLauncherConfig {
            project_folders: Vec::new(),
        }
    }))
}

/// Sauvegarde les paramètres de configuration dans le fichier de configuration.
pub fn save_config(config: &DevLauncherConfig) -> Result<(), Error> {
    let config_data = serde_json::to_string_pretty(config)?;
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(get_config_file_path())?;
    let mut writer = BufWriter::new(file);
    writer.write_all(config_data.as_bytes())?;
    Ok(())
}
