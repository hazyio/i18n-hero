use crate::completion::completion_data::CompletionData;
use std::collections::HashMap;
use tower_lsp_server::ls_types::CompletionResponse;

pub struct Completion {
    pub data: HashMap<String, CompletionData>,
}

impl Completion {
    pub fn new() -> Self {
        Completion {
            data: HashMap::new(),
        }
    }
    pub async fn get_completions(&self, quote: String) -> Option<CompletionResponse> {
        if quote.len() > 1 {
            // self.ge
            let items = self
                .data
                .iter()
                .map(|(_, data)| data.get_completion())
                .collect();
            return Some(CompletionResponse::Array(items));
        }
        None
    }
    pub fn add_completion(&mut self, data: CompletionData) {
        self.data.insert(data.completion.clone(), data);
    }

    pub fn get_keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
}
