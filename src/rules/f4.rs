use crate::file::SourceFile;

pub struct RuleF4;

impl super::Rule for RuleF4 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            match &func.range {
                Some(range) => {
                    let line_count: i32 = range.end.line as i32 - range.start.line as i32 + 1;
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
