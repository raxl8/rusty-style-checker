use crate::file::SourceFile;

pub struct RuleA3;

impl super::Rule for RuleA3 {
    fn analyze(&self, source_file: &SourceFile) {
        match source_file.contents.last() {
            Some(last_line) if last_line.is_empty() => (),
            _ => println!("{}: C-A3 Violation", source_file.path.display()),
        }
    }
}
