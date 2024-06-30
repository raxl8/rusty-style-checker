use crate::{
    file::{Function, SourceFile, FileKind},
    reporter::Reporter,
};

pub struct RuleG2;

impl super::Rule for RuleG2 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        let definitions: Vec<Function> = source_file
            .functions
            .iter()
            .filter(|function| function.is_definition)
            .cloned()
            .collect();
        for (current, next) in definitions.iter().zip(definitions.iter().skip(1)) {
            if current.range.end.line != next.location.line - 2 {
                reporter.report(
                    source_file.path.clone(),
                    Some(next.location.line),
                    "C-G2 Violation",
                );
            }
        }
    }
}
