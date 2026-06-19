use crate::completion::completion_doc::CompletionDoc;
use std::collections::HashMap;
use tower_lsp_server::ls_types::{CompletionItem, CompletionItemKind, Documentation};

pub struct CompletionData {
    pub completion: String,
    pub documentation: CompletionDoc,
}
impl CompletionData {
    pub fn new(completion: &str, doc: HashMap<String, String>) -> Self {
        CompletionData {
            completion: String::from(completion),
            documentation: CompletionDoc::new(doc),
        }
    }
    pub fn get_completion(&self) -> CompletionItem {
        CompletionItem {
            label: self.completion.clone(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(String::from("Translation")),
            insert_text: Some(self.completion.clone()),
            documentation: Some(self.documentation.get_md()),
            ..CompletionItem::default()
        }
    }
}
