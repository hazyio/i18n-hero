use std::path::PathBuf;

use crate::project::{ProjectSetting, ProjectsConfig};

static FILE_NAME: &str = "i18n-hero.toml";

pub fn init(path: &PathBuf, loader: crate::loaders::LoadersKind, name: String) {
    let file_path = path.join(FILE_NAME);
    if file_path.exists() {
        println!("{} already exists at {}", FILE_NAME, file_path.display());
        return;
    }
    let config = ProjectsConfig {
        project: vec![ProjectSetting::new(&name, path, loader)],
    };
    let toml = toml::to_string(&config);
    match toml {
        Ok(config) => {
            std::fs::write(file_path, config).expect("Failed to write i18n-hero.toml");
        }
        Err(e) => {
            eprintln!("Failed to serialize project settings: {}", e);
        }
    }
}
