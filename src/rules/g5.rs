use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleG5;

impl super::Rule for RuleG5 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        for include in source_file.includes.iter() {
            if !include.file.ends_with(".h") {
                reporter.report(
                    source_file.path.clone(),
                    Some(include.location.line),
                    "C-G5 Violation",
                );
            }
        }
    }
}
