use clap::Parser;

use crate::utils::args;

pub(crate) mod init;
pub(crate) mod loaders;
pub(crate) mod lsp;
pub(crate) mod project;
pub(crate) mod utils;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    match args.start {
        args::StartKind::Lsp { config } => {
            lsp::start_lsp(config).await;
        }
        args::StartKind::Init { dir, loader, name } => {
            init::init(&dir, loader, name);
        }
    };
}
