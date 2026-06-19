use std::collections::HashMap;
use std::fmt::format;
use tower_lsp_server::ls_types::{Documentation, MarkupContent, MarkupKind};

pub struct CompletionDoc {
    pub doc: HashMap<String, String>,
}
impl CompletionDoc {
    pub fn new(doc: HashMap<String, String>) -> Self {
        CompletionDoc { doc }
    }
    pub fn get_md(&self) -> Documentation {
        let doc = self
            .doc
            .iter()
            .map(|(locale, data)| String::from(format!("- {}: {}", locale, data)))
            .collect();
        Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: doc,
        })
    }
}
