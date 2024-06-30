use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleG4;

impl super::Rule for RuleG4 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        let non_const_global_vars = source_file
            .global_variables
            .iter()
            .filter(|var| !var.is_constant && var.is_definition);
        for var in non_const_global_vars {
            reporter.report(
                source_file.path.clone(),
                Some(var.location.line),
                "C-G4 Violation",
            );
        }
    }
}
