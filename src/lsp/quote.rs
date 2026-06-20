pub struct Quote {
    pub text: String,
    pub start: u32,
    pub end: u32,
    pub quote: char,
    pub complete: bool,
}

impl Quote {
    pub fn len(&self) -> usize {
        self.text.len()
    }
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
    pub fn from_line(line: &str, character: u32) -> Option<Self> {
        let mut started = false;
        let mut start = 0;
        let mut starting_char = '?';
        for (i, c) in line.char_indices() {
            if c == '"' || c == '\'' || c == '`' {
                if starting_char == '?' {
                    starting_char = c;
                    started = true;
                    start = i ;
                    continue;
                }
            }
            if c == starting_char && started {
                if character >= start as u32 && character <= i as u32 {
                    // complete text within quote, remove quote
                    return Some(Quote {
                        text: line[(start + 1)..=i - 1].to_string(),
                        end: character,
                        quote: starting_char,
                        start: start as u32,
                        complete: true,
                    });
                }
                // reset quote
                started = false;
                starting_char = '?';
            }
        }
        // unclosed string at end of line — still in progress, cursor likely inside it
        if started && character as usize >= start {
            // return Some(line[start..].to_string());
            return Some(Quote {
                text: line[(start + 1)..character as usize].to_string(),
                end: character,
                quote: starting_char,
                start: start as u32,
                complete: false,
            });
        }
        None
    }
}
