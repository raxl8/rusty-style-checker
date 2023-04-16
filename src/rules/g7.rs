use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleG7;

impl super::Rule for RuleG7 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        for (i, line) in source_file.contents.iter().enumerate() {
            match line.chars().last() {
                Some(c) if c == ' ' || c == '\t' => {
                    reporter.report(
                        source_file.path.clone(),
                        Some(i as u32 + 1),
                        "C-G7 Violation",
                    );
                }
                _ => (),
            }
        }
    }
}
