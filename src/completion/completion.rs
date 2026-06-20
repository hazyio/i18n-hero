use crate::completion::completion_data::CompletionData;
use crate::lsp::quote::Quote;
use std::collections::HashMap;
use tower_lsp_server::Client;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionResponse, Hover, MessageType, Position,
};
pub struct Completion {
    pub completion: HashMap<String, CompletionData>,
    keys: Vec<String>,
}

impl Completion {
    pub fn new() -> Self {
        Completion {
            completion: HashMap::new(),
            keys: Vec::new(),
        }
    }
    pub fn get_hover(&self, quote: &Quote, position: Position) -> Option<Hover> {
        if quote.text.len() > 1 {
            let c = self.completion.get(&quote.text.to_lowercase())?;
            return Some(c.get_hover(quote, position));
        }

        None
    }
    pub async fn get_completions(
        &self,
        quote: Quote,
        position: Position,
        _client: &Client,
    ) -> Option<CompletionResponse> {
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
            .map(|(_, data)| data.get_completion(&quote, position))
            .collect();

        return Some(CompletionResponse::Array(items));
    }
    pub fn add_completion(&mut self, data: CompletionData) {
        self.keys.push(data.completion.clone());
        self.completion
            .insert(data.completion.clone().to_lowercase(), data);
    }
}
