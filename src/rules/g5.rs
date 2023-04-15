use crate::file::SourceFile;

pub struct RuleG5;

impl super::Rule for RuleG5 {
    fn analyze(&self, source_file: &SourceFile) {
        for include in source_file.includes.iter() {
            if !include.file.ends_with(".h") {
                println!(
                    "{}:{}: C-G5 Violation",
                    source_file.path.display(), include.location.line
                );
            }
        }
    }
}
