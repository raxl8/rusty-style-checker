use crate::file::SourceFile;

pub struct RuleG6;

impl super::Rule for RuleG6 {
    fn analyze(&self, source_file: &SourceFile) {
        for (i, line) in source_file.contents.iter().enumerate() {
            if line.chars().any(|c| c == '\r') {
                println!("{}:{}: C-G6 Violation", source_file.path, i + 1);
            }
        }
    }
}
