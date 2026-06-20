use std::collections::HashMap;
use tokio::sync::RwLockReadGuard;
use tower_lsp_server::ls_types::{
    CompletionParams, GotoDefinitionParams, HoverParams, InlayHintParams, Position, Uri,
};

pub struct Quote {
    pub text: String,
    pub start: u32,
    pub end: u32,
    pub quote: char,
    pub complete: bool,
    pub uri: Uri,
    pub position: Position,
}

impl Quote {
    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
    pub fn from_line(line: &str, uri: &Uri, position: Position) -> Option<Self> {
        let mut started = false;
        let mut start = 0;
        let mut starting_char = '?';
        let character = position.character;
        for (i, c) in line.char_indices() {
            if c == '"' || c == '\'' || c == '`' {
                if starting_char == '?' {
                    starting_char = c;
                    started = true;
                    start = i;
                    continue;
                }
            }
            if c == starting_char && started {
                // make sure the cursor is inside the quote
                // if character >= start as u32 && character <= i as u32 {
                if character >= start as u32 {
                    // complete text within quote, remove quote
                    return Some(Quote {
                        text: line[(start + 1)..=i - 1].to_string(),
                        end: (i + 1) as u32, // ends at current character, add one to account for the quote
                        quote: starting_char,
                        start: start as u32,
                        complete: true,
                        uri: uri.clone(),
                        position,
                    });
                }
                // reset quote
                started = false;
                starting_char = '?';
            }
        }
        // unclosed string at end of line — still in progress, cursor likely inside it
        if started && character as usize >= start {
            return Some(Quote {
                text: line[(start + 1)..character as usize].to_string(),
                end: character,
                quote: starting_char,
                start: start as u32,
                complete: false,
                uri: uri.clone(),
                position,
            });
        }
        None
    }
    fn read_line(
        uri: &Uri,
        position: Position,
        documents: RwLockReadGuard<HashMap<Uri, String>>,
    ) -> Option<String> {
        let text = &documents.get(&uri)?;
        if text.is_empty() {
            return None;
        }
        if let Some(line) = text.lines().nth(position.line as usize) {
            return Some(line.to_string());
        }
        None
    }
    pub fn from_completion_params(
        params: &CompletionParams,
        document: RwLockReadGuard<HashMap<Uri, String>>,
    ) -> Option<Self> {
        let positon = params.text_document_position.position;
        let params = params.clone();
        let uri = params.text_document_position.text_document.uri;
        if let Some(line) = Quote::read_line(&uri, params.text_document_position.position, document)
        {
            return Quote::from_line(&line, &uri, positon);
        }

        None
    }
    pub fn from_hover_params(
        params: &HoverParams,
        document: RwLockReadGuard<HashMap<Uri, String>>,
    ) -> Option<Self> {
        let positon = params.text_document_position_params.position;
        let params = params.clone();
        let uri = params.text_document_position_params.text_document.uri;
        if let Some(line) = Quote::read_line(&uri, positon, document) {
            return Quote::from_line(&line, &uri, positon);
        }

        None
    }
    pub fn from_goto_params(
        params: &GotoDefinitionParams,
        document: RwLockReadGuard<HashMap<Uri, String>>,
    ) -> Option<Self> {
        let positon = params.text_document_position_params.position;
        let params = params.clone();
        let uri = params.text_document_position_params.text_document.uri;
        if let Some(line) = Quote::read_line(&uri, positon, document) {
            return Quote::from_line(&line, &uri, positon);
        }

        None
    }
}
