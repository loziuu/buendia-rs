static TO_REMOVE: [char; 20] = [
    '$',
    '%',
    '^',
    '*',
    '(',
    ')',
    ',',
    '\"',
    '.',
    ';',
    ':',
    '\'',
    '?',
    '!',
    '<',
    '>',
    '#',
    '@',
    96u8 as char,
    '_',
];

static TO_WHITESPACE: [char; 2] = ['-', '&'];

pub fn tokenize(line: &str) -> Vec<String> {
    line
        .replace(|c: char| !c.is_ascii() || TO_REMOVE.contains(&c), "")
        .replace(|c: char| TO_WHITESPACE.contains(&c), " ")
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let line = "@!@#$^&*(@)Hello!@#@*&, __world! This is `a test.";
        let tokens = tokenize(line);
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], "hello");
        assert_eq!(tokens[1], "world");
        assert_eq!(tokens[2], "this");
        assert_eq!(tokens[3], "is");
        assert_eq!(tokens[4], "a");
        assert_eq!(tokens[5], "test");
    }
}

