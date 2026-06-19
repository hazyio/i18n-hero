use std::path::PathBuf;

pub mod args;

pub fn get_cwd() -> PathBuf {
    // why unwrap, well the program is fucked if it can't even read it current directory.
    std::env::current_dir().unwrap()
}

