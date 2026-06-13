use std::path::PathBuf;

use clap::Parser;

use crate::loaders::LoadersKind;

#[derive(Parser, Debug)]
#[command(name = "i18n-hero", about = "An lsp for i18n", long_about = None,version,about)]
pub struct Args {
    #[command(subcommand)]
    pub start: StartKind,
}
#[derive(Parser, Debug)]

pub(crate) enum StartKind {
    #[command(name = "lsp", about = "Starts lsp server")]
    Lsp {
        #[arg(short = 'c', long = "config", help = "The config file", value_parser = validate_file, default_value = "./i18n-hero.toml")]
        config: PathBuf,
    },
    #[command(name = "init", about = "Initializes i18n-hero configuration")]
    Init {
        #[arg(long = "dir", help = "Project directory, must be a relative path", value_parser = validate_relative_dir, default_value = ".")]
        dir: PathBuf,
        #[arg(
            long = "loader",
            help = "The loader to use",
            value_enum,
            default_value = "rust-i18n"
        )]
        loader: LoadersKind,
        #[arg(long = "name", help = "Project name", default_value = "my-project")]
        name: String,
    },
}

fn validate_relative_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
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
