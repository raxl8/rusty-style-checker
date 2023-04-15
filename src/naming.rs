fn is_snake_case_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='z' | '_')
}

fn is_upper_snake_case_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='Z' | '_')
}

fn has_multiple_chars_in_a_row(string: &str, c: char) -> bool {
    let mut prev_char: Option<char> = None;
    for current_char in string.chars() {
        if prev_char == Some(c) && current_char == c {
            return true;
        }
        prev_char = Some(current_char);
    }
    false
}

fn check_underscores(string: &str) -> bool {
    if string.starts_with('_') || string.ends_with('_') {
        return false;
    }
    !has_multiple_chars_in_a_row(string, '_')
}

pub fn is_snake_case(string: &str) -> bool {
    if string.chars().any(|c| !is_snake_case_char(c)) {
        return false;
    }
    check_underscores(string)
}

pub fn is_upper_snake_case(string: &str) -> bool {
    if string.chars().any(|c| !is_upper_snake_case_char(c)) {
        return false;
    }
    check_underscores(string)
}
