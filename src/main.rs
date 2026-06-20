use std::env;
use std::fs::File;
use std::io::{BufRead, Seek};
use std::sync::OnceLock;

use clap::Parser;

use crate::utils::args::StartKind;
use crate::{logger::macros::exit_and_error, utils::args};

pub mod completion;
pub(crate) mod init;
pub(crate) mod loaders;
pub(crate) mod logger;
pub(crate) mod lsp;
pub(crate) mod utils;

static VERBOSE: OnceLock<bool> = OnceLock::new();
#[tokio::main]
async fn main() {
    let log_path = env::temp_dir().join("i18n-hero.log");
    let args = args::Args::parse();

    // only initialize the file-writing subscriber when we're actually going to log to it
    if !matches!(args.start, args::StartKind::Log) {
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true) // append, don't truncate on every launch
            .open(&log_path)
            .unwrap();
        tracing_subscriber::fmt()
            .with_writer(log_file)
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    let e = VERBOSE.set(args.verbose);
    if let Err(_) = e {
        exit_and_error!("Failed to set verbose flag")
    };

    match args.start {
        args::StartKind::Lsp { workspace } => {
            lsp::start_lsp(workspace).await;
        }
        args::StartKind::Init => {
            init::collect();
        }
        StartKind::Log => {
            let mut file = std::fs::File::open(&log_path).unwrap();
            file.seek(std::io::SeekFrom::End(0)).unwrap();
            let mut reader = std::io::BufReader::new(file);
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap() > 0 {
                    print!("{line}");
                } else {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
            }
        }
    };
}
