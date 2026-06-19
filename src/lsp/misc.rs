pub fn get_quote_string(line: &str, character: u32) -> Option<String> {
    let mut started = false;
    let mut start = 0;
    let mut starting_char = '?';
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
            started = false;
            starting_char = '?';
            if character >= start as u32 && character <= i as u32 {
                // return whole quoted string
                return Some(line[start..=i].to_string());
            }
        }
    }
    // unclosed string at end of line — still in progress, cursor likely inside it
    if started && character as usize >= start {
        return Some(line[start..].to_string());
    }
    None
}
