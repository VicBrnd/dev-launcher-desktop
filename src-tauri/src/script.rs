use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const PACKAGE_MANAGERS: &[(&str, &str)] = &[
    ("pnpm", "pnpm-lock.yaml"),
    ("yarn", "yarn.lock"),
    ("bun", "bun.lockb"),
    ("npm", "package-lock.json"),
];

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub manager: String,
    pub scripts: HashMap<String, String>,
}

pub fn detect_package_manager_and_scripts(path: &PathBuf) -> Option<PackageInfo> {
    let package_json_path = path.join("package.json");

    // Vérifier si `package.json` existe
    if !package_json_path.exists() {
        return None;
    }

    // Détecter le gestionnaire de paquets
    let package_manager = PACKAGE_MANAGERS
        .iter()
        .find_map(|(manager, lock_file)| {
            if path.join(lock_file).exists() {
                Some(manager.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "npm".to_string());

    // Lire les scripts du fichier `package.json`
    let file = File::open(&package_json_path).ok()?;
    let reader = BufReader::new(file);
    let package_json: Value = serde_json::from_reader(reader).ok()?;

    let scripts = package_json
        .get("scripts")
        .and_then(Value::as_object)
        .map(|scripts| {
            scripts
                .iter()
                .map(|(name, cmd)| (name.clone(), cmd.as_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    Some(PackageInfo {
        manager: package_manager,
        scripts,
    })
}
