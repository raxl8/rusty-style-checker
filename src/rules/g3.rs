use crate::{file::SourceFile, reporter::Reporter};

pub struct RuleG3;

impl super::Rule for RuleG3 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
    }
}
