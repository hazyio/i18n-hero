use tower_lsp_server::{LspService, Server};

use crate::lsp::Backend;

pub(crate) mod lsp;
pub(crate) mod trans;
pub(crate) mod utils;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
