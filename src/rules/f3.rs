use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleF3;

const TAB_WIDTH: usize = 4;
const MAX_LINE_LENGTH: usize = 80;

impl super::Rule for RuleF3 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        match source_file.kind {
            FileKind::Source | FileKind::Makefile => {}
            _ => return,
        }

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
                reporter.report(
                    source_file.path.clone(),
                    Some(i as u32 + 1),
                    "C-F3 Violation",
                );
            }
        }
    }
}
