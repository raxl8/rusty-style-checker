use crate::file::SourceFile;

pub struct RuleG7;

impl super::Rule for RuleG7 {
    fn analyze(&self, source_file: &SourceFile) {
        for (i, line) in source_file.contents.iter().enumerate() {
            match line.chars().last() {
                Some(c) if c == ' ' || c == '\t' => {
                    println!("{}:{}: C-G7 Violation", source_file.path, i + 1)
                }
                _ => (),
            }
        }
    }
}
