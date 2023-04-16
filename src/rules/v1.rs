use crate::{file::{SourceFile, FileKind}, naming::{is_snake_case, is_upper_snake_case}, reporter::Reporter};

pub struct RuleV1;

impl super::Rule for RuleV1 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }
        for macro_definition in source_file.macro_definitions.iter() {
            if !is_upper_snake_case(macro_definition.name.as_str()) {
                reporter.report(
                    source_file.path.clone(),
                    Some(macro_definition.location.line),
                    "C-V1 Violation",
                );
            }
        }
        for type_definition in source_file.type_definitions.iter() {
            if !is_snake_case(type_definition.name.as_str())
                || !type_definition.name.ends_with("_t")
                {
                    reporter.report(
                        source_file.path.clone(),
                        Some(type_definition.location.line),
                        "C-V1 Violation",
                        );
                }
        }
        let global_const_variables = source_file
            .global_variables
            .iter()
            .filter(|variable| variable.is_constant);
        for variable in global_const_variables {
            if !is_upper_snake_case(variable.name.as_str()) {
                reporter.report(
                    source_file.path.clone(),
                    Some(variable.location.line),
                    "C-V1 Violation",
                );
            }
        }
    }
}
