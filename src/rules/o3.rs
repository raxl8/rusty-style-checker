use crate::file::SourceFile;

pub struct RuleO3;

impl super::Rule for RuleO3 {
    fn analyze(&self, source_file: &SourceFile) {
        let count = source_file
            .functions
            .iter()
            .filter(|func| func.is_definition)
            .count();
        if count > 5 {
            println!("{}: C-O3 Violation", source_file.path);
        }
    }
}
