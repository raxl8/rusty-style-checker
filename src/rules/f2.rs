use crate::file::SourceFile;

pub struct RuleF2;

fn is_snake_case_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='z' | '_')
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

fn is_snake_case(string: &String) -> bool {
    if string.chars().any(|c| !is_snake_case_char(c)) {
        return false;
    }
    if string.starts_with("_") || string.ends_with("_") {
        return false;
    }
    !has_multiple_chars_in_a_row(string, '_')
}

impl super::Rule for RuleF2 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if !is_snake_case(&func.name) {
                println!(
                    "{}:{}: C-F2 Violation",
                    source_file.path, func.location.line
                );
            }
        }
    }
}
