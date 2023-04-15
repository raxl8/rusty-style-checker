use crate::file::SourceFile;

pub struct RuleH2;

impl super::Rule for RuleH2 {
    fn analyze(&self, source_file: &SourceFile) {
        if source_file.path.ends_with(".h") {
            if !source_file.include_guarded {
                println!("{}: C-H2 Violation", source_file.path.display());
            }
        }
    }
}
