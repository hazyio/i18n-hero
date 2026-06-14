use std::sync::OnceLock;

use clap::Parser;

use crate::{
    logger::macros::{exit_and_error, log_info},
    utils::args,
};

pub(crate) mod init;
pub(crate) mod loaders;
pub(crate) mod logger;
pub(crate) mod lsp;
pub(crate) mod project;
pub(crate) mod utils;

static VERBOSE: OnceLock<bool> = OnceLock::new();
#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let e = VERBOSE.set(args.verbose);
    if let Err(_) = e {
        exit_and_error!("Failed to set verbose flag")
    };
    match args.start {
        args::StartKind::Lsp { workspace } => {
            println!("{}",workspace.display());
          
            lsp::start_lsp(workspace).await;
        }
        args::StartKind::Init => {
            init::collect();
        }
    };
}
