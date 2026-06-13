pub mod completion;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

use crate::lsp::completion::Completion;

pub async fn start_lsp(config: PathBuf) {
    let config = {
        match crate::project::ProjectsConfig::load_from_file(&config) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Failed to load config: {}", e);
                return;
            }
        }
    };
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        state: Arc::new(RwLock::new(Completion::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    pub(crate) client: Client,
    state: Arc<RwLock<Completion>>,
}
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
        self.state.write().await.add_completion(
            "hello".to_string(),
            "Hello, World!".to_string(),
            "This is a greeting.".to_string(),
        );
    }
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let c = self.state.read().await;
        let completions = c.get_all_completions();
        let items: Vec<CompletionItem> = completions
            .iter()
            .map(|data| CompletionItem {
                label: data.completion.clone(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some(data.hover.clone()),
                ..CompletionItem::default()
            })
            .collect();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        self.client
            .log_message(MessageType::INFO, format!("hover: {:?}", params))
            .await;
        Ok(None)
    }
}
