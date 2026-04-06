use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub stopwords: Vec<String>,
    #[serde(default)]
    pub ignore_sections: Vec<String>,
    #[serde(default)]
    pub ignore_crossref_sections: Vec<String>,
    #[serde(default)]
    pub ignore_paths: Vec<String>,
    #[serde(default)]
    pub allowlist_concepts: Vec<String>,
    #[serde(default)]
    pub scope_prefix: Option<String>,
}

fn load_from_path(path: &Path) -> Option<Config> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn load_config(config_path: Option<&str>, target_path: &str) -> Config {
    if let Some(path) = config_path {
        if let Some(cfg) = load_from_path(Path::new(path)) {
            return cfg;
        }
        return Config::default();
    }

    let target = Path::new(target_path);
    let root = if target.is_file() {
        target.parent().unwrap_or(target)
    } else {
        target
    };
    let candidate = root.join("lint-ai.json");
    if let Some(cfg) = load_from_path(&candidate) {
        return cfg;
    }

    Config::default()
}

pub fn normalize_list(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|v| v.trim().to_lowercase())
        .filter(|v| !v.is_empty())
        .collect()
}
