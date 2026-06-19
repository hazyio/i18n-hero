mod misc;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

use crate::completion::completion::Completion;
use crate::completion::completion_data::CompletionData;
use crate::logger::macros::exit_and_error;
use crate::lsp::misc::get_quote_string;

pub async fn start_lsp(workspace: PathBuf) {
    let projects = {
        match crate::project::ProjectsConfig::load_from_file(
            &workspace.join(crate::init::FILE_NAME),
        ) {
            Ok(cfg) => cfg,
            Err(e) => {
                exit_and_error!("Failed to load config: {}", e);
            }
        }
    };
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        state: Arc::new(RwLock::new(Completion::new())),
        documents: Arc::new(RwLock::new(HashMap::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

struct Backend {
    pub(crate) client: Client,
    state: Arc<RwLock<Completion>>,
    documents: Arc<RwLock<HashMap<Uri, String>>>,
}
impl Backend {
    pub async fn get_completion_line(
        &self,
        line: &str,
        character: u32,
    ) -> Option<CompletionResponse> {
        match get_quote_string(line, character) {
            None => {
                return None;
            }
            Some(quote) => {
                // self.client
                //     .log_message(MessageType::INFO, format!("hover: {:?}", string))
                //     .await;
                self.state.read().await.get_completions(quote).await
            }
        }
    }
}
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
        let mut d = HashMap::new();
        d.insert("en".to_string(), "English traja".to_string());
        self.state
            .write()
            .await
            .add_completion(CompletionData::new("hello.world", d));
    }
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.read().await;
        let text = documents.get(&uri).cloned().unwrap_or_default();
        if let Some(line) = text.lines().nth(position.line as usize) {
            return Ok(self.get_completion_line(line, position.character).await);
        };
        Ok(None)
        // match {
        //     None => Ok(None),
        //     Some(line) => {
        //         self.client
        //             .log_message(MessageType::INFO, format!("completion: {:?}", line))
        //             .await;
        //         let c = self.state.read().await;
        //         let completions = c.get_all_completions();
        //         let items: Vec<CompletionItem> = completions
        //             .iter()
        //             .map(|data| CompletionItem {
        //                 label: data.completion.clone(),
        //                 kind: Some(CompletionItemKind::TEXT),
        //                 detail: Some(data.hover.clone()),
        //                 ..CompletionItem::default()
        //             })
        //             .collect();
        //         Ok(Some(CompletionResponse::Array(items)))
        //     }
        // }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // self.client
        //     .log_message(MessageType::INFO, format!("hover: {:?}", params))
        //     .await;
        Ok(None)
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .write()
            .await
            .insert(params.text_document.uri, params.text_document.text);
    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Save only opened files.
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // if you declared TextDocumentSyncKind::FULL, this has the whole new text
        if let Some(change) = params.content_changes.into_iter().next() {
            self.documents
                .write()
                .await
                .insert(params.text_document.uri, change.text);
        }
    }
}
