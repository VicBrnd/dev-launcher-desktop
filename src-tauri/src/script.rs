use crate::types::FetchPackageJson;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const PACKAGE_MANAGERS_AND_LOCKFILES: &[(&str, &str)] = &[
    ("pnpm", "pnpm-lock.yaml"),
    ("yarn", "yarn.lock"),
    ("bun", "bun.lockb"),
    ("npm", "package-lock.json"),
];

/// Détecte le gestionnaire de paquets et extrait les scripts définis dans package.json
pub fn detect_package_manager_and_scripts(project_dir: &PathBuf) -> Option<FetchPackageJson> {
    // Définir le chemin vers le fichier `package.json`
    let package_json_file_path = project_dir.join("package.json");

    // Vérifie l'existence de `package.json`
    if !package_json_file_path.exists() {
        return None;
    }

    // Identifier le gestionnaire de paquets en fonction de la présence des fichiers de lock
    let detected_package_manager = PACKAGE_MANAGERS_AND_LOCKFILES
        .iter()
        .find_map(|(manager_name, lockfile_name)| {
            if project_dir.join(lockfile_name).exists() {
                Some(manager_name.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "npm".to_string());

    // Ouvrir et lire le fichier `package.json`
    let package_file = File::open(&package_json_file_path).ok()?;
    let package_file_reader = BufReader::new(package_file);
    let package_json_data: Value = serde_json::from_reader(package_file_reader).ok()?;

    // Extraire les scripts définis dans le champ "scripts" de `package.json`
    let scripts_map = package_json_data
        .get("scripts")
        .and_then(Value::as_object)
        .map(|scripts_object| {
            scripts_object
                .iter()
                .map(|(script_name, script_command)| {
                    (script_name.clone(), script_command.as_str().unwrap_or("").to_string())
                })
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    // Log pour vérifier les scripts extraits
    println!("Gestionnaire de paquets détecté: {}", detected_package_manager);
    println!("Scripts extraits de package.json : {:?}", scripts_map);

    // Retourne le gestionnaire de paquets et les scripts extraits
    Some(FetchPackageJson {
        manager: detected_package_manager,
        scripts: scripts_map,
    })
}
