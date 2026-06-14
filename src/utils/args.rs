use std::path::PathBuf;

use clap::Parser;

use crate::{loaders::LoadersKind, utils::get_cwd};

#[derive(Parser, Debug)]
#[command(name = "i18n-hero", about = "An lsp for i18n", long_about = None,version,about)]
pub struct Args {
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub start: StartKind,
}
#[derive(Parser, Debug)]

pub(crate) enum StartKind {
    #[command(name = "lsp", about = "Starts lsp server")]
    Lsp {
        #[arg( long = "workspace", help = "The workspace directory", value_parser = validate_workspace, default_value = ".")]
        workspace: PathBuf,
    },
    #[command(name = "init", about = "Initializes i18n-hero configuration")]
    Init,
}
fn validate_workspace(s: &str) -> Result<PathBuf, String> {
    let path = validate_dir(s)?;

    let config_file = path.join("i18n-hero.toml");
    if !config_file.exists() {
        return Err(format!(
            "Cannot start lsp, config file do not exists in: {}",
            get_cwd().join(s).display()
        ));
    }
    Ok(path)
}

fn validate_relative_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    let current_dir = get_cwd();
    let valid_path = current_dir.join(&path);
    if valid_path.exists() {
        if !valid_path.is_dir() {
            return Err(format!("Path is not a directory: {}", s));
        }
        // Return the original path, not the absolute one, to keep it relative
        Ok(path)
    } else {
        Err(format!(
            "Directory does not exist: {}, path must be relative to {}",
            s,
            current_dir.display()
        ))
    }
}
fn validate_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", s));
        }
        Ok(path)
    } else {
        Err(format!("Directory does not exist: {}", s))
    }
}

fn validate_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        if !path.is_file() {
            return Err(format!("Path is not a file: {}", s));
        }
        Ok(path)
    } else {
        Err(format!("File does not exist: {}", s))
    }
}
