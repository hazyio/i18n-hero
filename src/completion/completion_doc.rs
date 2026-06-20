use std::collections::HashMap;
use tower_lsp_server::ls_types::{Documentation, MarkupContent, MarkupKind};
#[derive(Debug, Clone)]

pub struct CompletionDocData {
    pub info: String,
    pub file: String,
}
impl CompletionDocData {
    pub fn new(info: &str, file: impl Into<String>) -> Self {
        CompletionDocData {
            info: String::from(info),
            file: file.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CompletionDoc {
    pub doc: HashMap<String, CompletionDocData>,
}
impl CompletionDoc {
    pub fn new(doc: HashMap<String, CompletionDocData>) -> Self {
        CompletionDoc { doc }
    }
    pub fn get_md_doc(&self, key: &str, available_locales: &Vec<String>) -> Documentation {
        Documentation::MarkupContent(self.get_markup(key, available_locales))
    }
    pub fn get_markup(&self, key: &str, available_locales: &Vec<String>) -> MarkupContent {
        let generated_doc = {
            let mut doc = String::from(format!("**{}**\n\n", key));
            for locale in available_locales {
                if let Some(data) = self.doc.get(locale) {
                    doc.push_str(&format!("- **{}**: {} \n", locale, data.info));
                } else {
                    doc.push_str(&format!("- **{}**: - \n", locale));
                }
            }
            doc
        };
        MarkupContent {
            kind: MarkupKind::Markdown,
            value: generated_doc,
        }
    }
    pub fn get_hint(&self) -> String {
        let default_locale = "en";
        let d = self.doc.get(default_locale);
        match d {
            None => String::from(""),
            Some(d) => d.info.clone(),
        }
    }
}
