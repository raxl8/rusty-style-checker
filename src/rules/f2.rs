use crate::{file::SourceFile, naming::is_snake_case, reporter::Reporter};

pub struct RuleF2;

impl super::Rule for RuleF2 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
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
