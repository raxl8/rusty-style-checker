use crate::{file::SourceFile, naming::is_snake_case};

pub struct RuleF2;

impl super::Rule for RuleF2 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if !is_snake_case(&func.name) {
                println!(
                    "{}:{}: C-F2 Violation",
                    source_file.path.display(), func.location.line
                );
            }
        }
    }
}
