use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleF6;

impl super::Rule for RuleF6 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        for func in source_file.functions.iter() {
            if !func.is_variadic && func.is_type_variadic {
                reporter.report(
                    source_file.path.clone(),
                    Some(func.location.line),
                    "C-F6 Violation",
                );
            }
        }
    }
}
