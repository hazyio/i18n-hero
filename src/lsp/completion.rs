use std::collections::HashMap;
pub struct CompletionData {
    pub completion: String,
    pub hover: String,
}
pub struct Completion {
    pub data: HashMap<String, CompletionData>,
}

impl Completion {
    pub fn new() -> Self {
        Completion {
            data: HashMap::new(),
        }
    }
    pub fn add_completion(&mut self, key: String, completion: String, hover: String) {
        self.data.insert(key, CompletionData { completion, hover });
    }
    pub fn get_completion(&self, key: &str) -> Option<&CompletionData> {
        self.data.get(key)
    }
    pub fn get_all_completions(&self) -> Vec<&CompletionData> {
        self.data.values().collect()
    }
    pub fn get_hover(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|data| data.hover.as_str())
    }
    pub fn get_keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
}
