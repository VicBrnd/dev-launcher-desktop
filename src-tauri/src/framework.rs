use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const FRAMEWORKS: &[(&str, &str)] = &[
    ("next", "Next.js"),
    ("react", "React"),
    ("vue", "Vue.js"),
    ("svelte", "Svelte"),
    ("angular", "Angular"),
];

pub fn detect_framework(path: &PathBuf) -> Option<String> {
    let package_json_path = path.join("package.json");

    if !package_json_path.exists() {
        return Some("Inconnu".to_string());
    }

    let file = File::open(&package_json_path).ok()?;
    let reader = BufReader::new(file);
    let package_json: Value = serde_json::from_reader(reader).ok()?;

    let deps = package_json.get("dependencies").and_then(Value::as_object);
    let dev_deps = package_json.get("devDependencies").and_then(Value::as_object);

    let detected_framework = FRAMEWORKS
        .iter()
        .filter_map(|(key, tech_name)| {
            if deps.and_then(|d| d.get(*key)).is_some()
                || dev_deps.and_then(|d| d.get(*key)).is_some()
            {
                Some(tech_name.to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    if detected_framework.is_empty() {
        Some("Inconnu".to_string())
    } else {
        Some(detected_framework.into_iter().collect::<Vec<_>>().join(", "))
    }
}
