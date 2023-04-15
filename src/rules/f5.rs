use crate::file::SourceFile;

pub struct RuleF5;

impl super::Rule for RuleF5 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            let param_count = func.params.len();
            if param_count > 4 {
                println!(
                    "{}:{}: C-F5 Violation",
                    source_file.path.display(), func.location.line
                );
            }
        }
    }
}
