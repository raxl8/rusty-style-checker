use crate::file::SourceFile;

pub struct RuleF4;

impl super::Rule for RuleF4 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if let Some(range) = &func.body {
                for line in range.start.line..range.end.line + 1 {
                    let body_line_num = line - range.start.line + 1;
                    if body_line_num > 20 {
                        println!(
                            "{}:{}: C-F4 Violation",
                            source_file.path.display(), line
                        );
                    }
                }
            }
        }
    }
}
