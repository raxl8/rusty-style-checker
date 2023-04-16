use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleA3;

impl super::Rule for RuleA3 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        match source_file.contents.last() {
            Some(last_line) if last_line.is_empty() => (),
            _ => reporter.report(source_file.path.clone(), None, "C-A3 Violation"),
        }
    }
}
