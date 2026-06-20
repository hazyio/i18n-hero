use crate::completion::completion_data::CompletionData;
use crate::lsp::quote::Quote;
use std::collections::HashMap;
use std::str::FromStr;
use tower_lsp_server::ls_types::{
    CompletionResponse, Hover, InlayHint, InlayHintKind, InlayHintLabel, Position, Uri,
};
#[derive(Debug, Clone)]
pub struct Completion {
    pub completion: HashMap<String, CompletionData>,
    pub available_locales: Vec<String>,
    keys: Vec<String>,
}

impl Completion {
    pub fn new() -> Self {
        Completion {
            completion: HashMap::new(),
            keys: Vec::new(),
            available_locales: Vec::new(),
        }
    }
    pub fn set_available_locales(&mut self, locales: &Vec<String>) {
        self.available_locales = locales.clone();
    }
    pub async fn get_hover(&self, quote: &Quote) -> Option<Hover> {
        if quote.text.len() > 1 {
            let c = self.completion.get(&quote.text.to_lowercase())?;
            return Some(c.get_hover(quote, &self.available_locales));
        }

        None
    }
    pub async fn get_goto(&self, quote: &Quote) -> Option<Hover> {
        if quote.text.len() > 1 {
            let c = self.completion.get(&quote.text.to_lowercase())?;
            return Some(c.get_hover(quote, &self.available_locales));
        }

        None
    }
    pub async fn get_hints(&self, text: &String) -> Option<Vec<InlayHint>> {
        if text.is_empty() {
            tracing::info!("empty text passed to get_hints, returning empty hints");
            return (None);
        }
        let mut hints = vec![];
        for (line_num, line) in text.lines().enumerate() {
            if let Some(quote) = Quote::from_line(
                line,
                &Uri::from_file_path("/unused").unwrap(), // this is not used
                Position {
                    line: line_num as u32,
                    character: line.len() as u32, // set cursor to end of line
                },
            ) {
                tracing::info!("found quote: {}", quote.text);

                let Some(completion_entry) = self.completion.get(&quote.text.to_lowercase()) else {
                    tracing::info!("quote not found: {}", quote.text);
                    continue; // not a known key, skip this quote, keep scanning
                };
                let documentation_hint = completion_entry.documentation.get_hint();
                if !documentation_hint.is_empty() {
                    tracing::info!("adding hint: {}", documentation_hint);
                    hints.push(InlayHint {
                        position: Position {
                            line: quote.position.line,
                            character: quote.end,
                        },
                        label: InlayHintLabel::String(format!(": {}", documentation_hint)),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(true),
                        padding_right: None,
                        data: None,
                    });
                }
            }
        }
        (Some(hints))
    }
    pub async fn get_completions(&self, quote: &Quote) -> Option<CompletionResponse> {
        let mut keys_to_show = Vec::new();
        if quote.len() > 0 {
            let keys = self.keys.clone();
            for key in &keys {
                // add keys that start with the text and is different from the text
                if (key.starts_with(&quote.text.to_lowercase())
                    && key != &quote.text.to_lowercase())
                    || key.to_lowercase().contains(&quote.text.to_lowercase())
                {
                    keys_to_show.push(key.clone());
                }
            }
        } else {
            keys_to_show = self.keys.clone();
        }

        let data: Vec<_> = self
            .completion
            .iter()
            .filter(|(key, _)| keys_to_show.contains(key))
            .collect();
        let items = data
            .iter()
            .map(|(_, data)| data.get_completion(&quote, &self.available_locales))
            .collect();

        return Some(CompletionResponse::Array(items));
    }
    pub fn add_completion(&mut self, data: CompletionData) -> &mut Completion {
        self.keys.push(data.completion.clone());
        self.completion
            .insert(data.completion.clone().to_lowercase(), data);
        self
    }
}
