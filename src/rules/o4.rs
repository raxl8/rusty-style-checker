use crate::{file::SourceFile, naming::is_snake_case};

pub struct RuleO4;

impl super::Rule for RuleO4 {
    fn analyze(&self, source_file: &SourceFile) {
        let stem = source_file.path.file_stem().unwrap().to_str().unwrap();
        if !is_snake_case(stem) {
            println!(
                "{}: C-O4 Violation",
                source_file.path.display()
            );
        }
    }
}
