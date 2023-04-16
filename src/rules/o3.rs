use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleO3;

impl super::Rule for RuleO3 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        let count = source_file
            .functions
            .iter()
            .filter(|func| func.is_definition)
            .count();
        if count > 5 {
            reporter.report(source_file.path.clone(), None, "C-O3 Violation");
        }
    }
}
