mod misc;
pub mod quote;

use crate::completion::completion::Completion;
use crate::completion::completion_data::CompletionData;
use crate::logger::macros::exit_and_error;
use crate::lsp::quote::Quote;
use crate::utils::join_relate_to_cwd;
use crate::utils::project::{Project, Projects};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tracing_subscriber::fmt;

pub async fn start_lsp(workspace: PathBuf) {
    let projects = {
        match crate::utils::project_config::ProjectsConfig::load_from_file(
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
        state: Arc::new(RwLock::new(projects)),
        documents: Arc::new(RwLock::new(HashMap::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
struct Backend {
    pub(crate) client: Client,
    state: Arc<RwLock<Projects>>,
    documents: Arc<RwLock<HashMap<Uri, String>>>,
}
impl Backend {
    pub async fn get_project(&self, uri: &Uri) -> Option<Project> {
        let uri = uri.to_file_path().to_owned()?.to_str()?.to_string();
        if let Some(project) = self.state.read().await.get_project(&*uri) {
            return Some(project);
        }
        None
    }
    pub async fn compute_hints(&self, text: &String, uri: Uri) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        let project = self.get_project(&uri).await;
        if let Some(project) = project {
            let get_hints = project.completion.get_hints(text).await;
            if let Some(neq_hints) = get_hints {
                hints = neq_hints;
            }
        }

        hints
    }
}
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                definition_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),

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
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        if let Some(quote) =
            quote::Quote::from_completion_params(&params, self.documents.read().await)
        {
            if let Some(project) = self.get_project(&quote.uri).await {
                tracing::info!("project found");
                return Ok(project.completion.get_completions(&quote).await);
            };
        }
        Ok(None)
    }
    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri;
        let range = params.range;
        let start_line = range.start.line as usize;
        let end_line = range.end.line as usize;
        let read = self.documents.read().await;
        let documents = read.get(&uri);
        if let Some(document) = documents {
            let hints = self.compute_hints(&document, uri).await;
            // filter the returned hints to the visible range instead
            let visible: Vec<InlayHint> = hints
                .into_iter()
                .filter(|h| {
                    h.position.line >= start_line as u32 && h.position.line <= end_line as u32
                })
                .collect();
            return Ok(Some(visible));
        }

        Ok((None))
    }
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let target_uri = Uri::from_file_path(join_relate_to_cwd("./locales/en.yaml")).unwrap();

        if let Some(quote) = quote::Quote::from_goto_params(&params, self.documents.read().await) {
            let location = Location {
                uri: target_uri,
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 6,
                    },
                },
            };
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        if let Some(quote) = quote::Quote::from_hover_params(&params, self.documents.read().await) {
            if let Some(project) = self.get_project(&quote.uri).await {
                tracing::info!("project found");
                return Ok(project.completion.get_hover(&quote).await);
            };
        }

        Ok(None)
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .write()
            .await
            .insert(params.text_document.uri.clone(), params.text_document.text);
    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Save only opened files.
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);
    }
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            self.documents
                .write()
                .await
                .insert(params.text_document.uri.clone(), text);
        }
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // if you declared TextDocumentSyncKind::FULL, this has the whole new text
        if let Some(change) = params.content_changes.into_iter().next() {
            self.documents
                .write()
                .await
                .insert(params.text_document.uri.clone(), change.text);
        }
    }
}
