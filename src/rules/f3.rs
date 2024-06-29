use crate::file::SourceFile;

pub struct RuleF3;

const TAB_WIDTH: usize = 4;
const MAX_LINE_LENGTH: usize = 80;

impl super::Rule for RuleF3 {
    fn analyze(&self, source_file: &SourceFile) {
        for (i, line) in source_file.contents.iter().enumerate() {
            let mut count: usize = 0;
            for c in line.chars() {
                if c == '\t' {
                    count += TAB_WIDTH;
                } else {
                    count += 1;
                }
            }
            if count > MAX_LINE_LENGTH {
                println!("{}:{}: C-F3 Violation", source_file.path.display(), i + 1);
            }
        }
    }
}
