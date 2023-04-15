use crate::file::SourceFile;

pub struct RuleF6;

impl super::Rule for RuleF6 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if !func.is_variadic && func.is_type_variadic {
                println!(
                    "{}:{}: C-F6 Violation",
                    source_file.path.display(), func.location.line
                );
            }
        }
    }
}
