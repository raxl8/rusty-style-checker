use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleH2;

impl super::Rule for RuleH2 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.path.ends_with(".h") {
            if !source_file.include_guarded {
                reporter.report(source_file.path.clone(), None, "C-H2 Violation");
            }
        }
    }
}
