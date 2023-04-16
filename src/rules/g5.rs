use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleG5;

impl super::Rule for RuleG5 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

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
