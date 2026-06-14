use std::path::PathBuf;

use crate::loaders::LoadersKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSetting {
    name: String,
    root: String,
    loader: LoadersKind,
}

impl ProjectSetting {
    pub fn new(name: &str, root: &PathBuf, loader: LoadersKind, translation: &PathBuf) -> Self {
        Self {
            name: name.to_string(),
            root: root.as_path().to_str().unwrap_or_default().into(),
            loader,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ProjectsConfig {
    pub project: Vec<ProjectSetting>,
}

impl ProjectsConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path.display(), e))?;
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file {}: {}", path.display(), e))
    }
}
