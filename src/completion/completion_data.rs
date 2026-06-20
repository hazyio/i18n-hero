use crate::completion::completion_doc::{CompletionDoc, CompletionDocData};
use crate::lsp::quote::Quote;
use std::collections::HashMap;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionItemKind, CompletionTextEdit, Hover, HoverContents, Position, Range,
    TextEdit,
};
#[derive(Debug, Clone)]

pub struct CompletionData {
    pub completion: String,
    pub documentation: CompletionDoc,
}
impl CompletionData {
    pub fn new(completion: &str, doc: HashMap<String, CompletionDocData>) -> Self {
        CompletionData {
            completion: String::from(completion),
            documentation: CompletionDoc::new(doc),
        }
    }
    pub fn get_hover(&self, quote: &Quote, available_locales: &Vec<String>) -> Hover {
        Hover {
            contents: HoverContents::Markup(self.documentation.get_markup(quote.text.as_str(),available_locales)),
            range: Some(Range {
                start: Position {
                    line: quote.position.line,
                    character: quote.start + 1, // add quote
                },
                end: Position {
                    line: quote.position.line,
                    character: quote.position.character,
                },
            }),
        }
    }
    pub fn get_completion(&self, quote: &Quote, available_locales: &Vec<String>) -> CompletionItem {
        let new_text = {
            let mut d = self.completion.clone();
            if !quote.complete {
                d.push_str(&quote.quote.to_string());
            }
            d
        };
        CompletionItem {
            label: self.completion.clone(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(String::from("Translation")),
            filter_text: Some(quote.text.clone()),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: Range {
                    start: Position {
                        line: quote.position.line,
                        character: quote.start + 1, // add quote
                    },
                    end: Position {
                        line: quote.position.line,
                        character: quote.position.character,
                    },
                },
                new_text: new_text,
            })),
            documentation: Some(self.documentation.get_md_doc(quote.text.as_str(),available_locales)),
            ..CompletionItem::default()
        }
    }
}
