use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

pub struct RuleH1;

impl super::Rule for RuleH1 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        let function_definitions = source_file
            .functions
            .iter()
            .filter(|function| function.is_definition);
        if let Some(extension) = source_file.path.extension() {
            match extension.to_str().unwrap() {
                "c" => {
                    let static_inline_functions = function_definitions
                        .filter(|function| function.is_static && function.is_inline);
                    for function in static_inline_functions {
                        reporter.report(
                            source_file.path.clone(),
                            Some(function.location.line),
                            "C-H1 Violation",
                        );
                    }
                }
                "h" => {
                    let non_static_inline_functions = function_definitions
                        .filter(|function| !function.is_static || !function.is_inline);
                    for function in non_static_inline_functions {
                        reporter.report(
                            source_file.path.clone(),
                            Some(function.location.line),
                            "C-H1 Violation",
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
