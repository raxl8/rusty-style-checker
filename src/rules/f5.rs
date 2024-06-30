use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleF5;

impl super::Rule for RuleF5 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        for func in source_file.functions.iter() {
            let param_count = func.params.len();
            if param_count > 4 {
                reporter.report(
                    source_file.path.clone(),
                    Some(func.location.line),
                    "C-F5 Violation",
                );
            }
        }
    }
}
