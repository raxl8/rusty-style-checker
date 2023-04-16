use crate::{file::{SourceFile, FileKind}, naming::is_snake_case, reporter::Reporter};

pub struct RuleF2;

impl super::Rule for RuleF2 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        for func in source_file.functions.iter() {
            if !is_snake_case(&func.name) {
                reporter.report(
                    source_file.path.clone(),
                    Some(func.location.line),
                    "C-F2 Violation",
                );
            }
        }
    }
}
