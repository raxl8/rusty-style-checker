use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleG6;

impl super::Rule for RuleG6 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        match source_file.kind {
            FileKind::Source | FileKind::Makefile => {}
            _ => return,
        }

        for (i, line) in source_file.contents.iter().enumerate() {
            if line.chars().any(|c| c == '\r') {
                reporter.report(
                    source_file.path.clone(),
                    Some(i as u32 + 1),
                    "C-G6 Violation",
                );
            }
        }
    }
}
