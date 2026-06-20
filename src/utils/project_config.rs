use std::path::PathBuf;

use crate::loaders::LoadersKind;
use crate::utils::project::Projects;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSetting {
    pub name: String,
    pub root: String,
    pub loader: LoadersKind,
    pub locales: String,
}

impl ProjectSetting {
    pub fn new(name: &str, root: &PathBuf, loader: LoadersKind, translation: &PathBuf) -> Self {
        Self {
            name: name.to_string(),
            root: root.as_path().to_str().unwrap_or_default().into(),
            loader,
            locales: translation.to_string_lossy().to_string(),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ProjectsConfig {
    pub project: Vec<ProjectSetting>,
}

impl ProjectsConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Projects, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path.display(), e))?;
        let load_projects: ProjectsConfig = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file {}: {}", path.display(), e))?;
        Ok(crate::utils::project::Projects::from(load_projects))
    }
}
