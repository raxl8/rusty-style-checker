use crate::{file::SourceFile, naming::is_snake_case, reporter::Reporter};

pub struct RuleO4;

impl super::Rule for RuleO4 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        let stem = source_file.path.file_stem().unwrap().to_str().unwrap();
        if !is_snake_case(stem) {
            reporter.report(source_file.path.clone(), None, "C-O4 Violation");
        }
    }
}
