use std::path::PathBuf;

pub mod args;
pub mod project;
pub(crate) mod project_config;

pub fn get_cwd() -> PathBuf {
    // why unwrap, well the program is fucked if it can't even read it current directory.
    std::env::current_dir().unwrap()
}

pub fn join_relate_to_cwd(path: &str) -> PathBuf {
    let cwd = get_cwd();

    if path == "." {
        // just return the current working directory
        return get_cwd();
    }

    // Strip leading "./" or ".\" if present
    let clean_path = path
        .strip_prefix("./")
        .unwrap_or_else(|| path.strip_prefix(".\\").unwrap_or(path));

    cwd.join(clean_path)
}
