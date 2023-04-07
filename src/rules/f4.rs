use crate::file::SourceFile;

pub struct RuleF4;

impl super::Rule for RuleF4 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            match &func.range {
                Some(range) => {
                    let line_count = range.end.line - range.start.line + 1;
                    if line_count > 20 {
                        println!(
                            "{}:{}: C-F4 Violation",
                            source_file.path, func.location.line
                        );
                    }
                }
                _ => (),
            }
        }
    }
}
