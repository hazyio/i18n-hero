use dialoguer::{Confirm, Input, Select};
use std::path::PathBuf;

use crate::{
    loaders::LoadersKind,
    logger::macros::{exit_and_error, log_error},
    project::{ProjectSetting, ProjectsConfig},
};

static FILE_NAME: &str = "i18n-hero.toml";

pub fn collect() {
    let dir = collect_dir("Project directory");
    let translation_location = collect_dir("Translation Location");
    let file_path = dir.join(FILE_NAME);
    if file_path.exists() {
        exit_and_error!("{} already exists at {}", FILE_NAME, file_path.display());
    }
    let name = collect_name(&dir);
    let loader = collect_loader();

    if Confirm::new()
        .with_prompt(format!(
            "Create i18n-hero configuration in {} with loader {}?",
            dir.display(),
            match loader {
                LoadersKind::RustI18n => "rust-i18n",
            }
        ))
        .interact()
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to read confirmation: {}", e);
        })
    {
        create_project(dir, loader, name, translation_location);
    } else {
        println!("Aborting.");
    }
}
fn collect_dir(title: &str) -> PathBuf {
    let select_dir = Input::<String>::new()
        .with_prompt(title)
        .default(".".into())
        .interact_text()
        .unwrap();

    let valid_path = build_path(&select_dir);
    if valid_path.exists() {
        if !valid_path.is_dir() {
            log_error!("Path is not a directory: {} ", select_dir);
            return collect_dir(title);
        }
        return PathBuf::from(select_dir);
    } else {
        log_error!(
            "Directory does not exist: {}, path must be relative to cwd",
            select_dir
        );
        return collect_dir(title);
    }
}
fn collect_name(dir: &PathBuf) -> String {
    let p = build_path(&dir.to_str().unwrap());
    let default_name = { p.file_name().and_then(|s| s.to_str()).unwrap() };
    Input::new()
        .with_prompt("Project name")
        .default(default_name.into())
        .interact_text()
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to read project name: {}", e);
        })
}
fn collect_loader() -> LoadersKind {
    let select_lib = Select::new()
        .with_prompt("Which i18n library are you using?")
        .items(&["rust-i18n"])
        .default(0)
        .interact()
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to read loader selection: {}", e);
        });
    match select_lib {
        0 => LoadersKind::RustI18n,
        _ => unreachable!(),
    }
}
fn build_path(path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(path)
}

pub fn create_project(
    path: PathBuf,
    loader: crate::loaders::LoadersKind,
    name: String,
    translation: PathBuf,
) {
    let file_path = path.join(FILE_NAME);
    let config = ProjectsConfig {
        project: vec![ProjectSetting::new(&name, &path, loader, &translation)],
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
