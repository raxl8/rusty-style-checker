use crate::file::{SourceFile, Function};

pub struct RuleG2;

impl super::Rule for RuleG2 {
    fn analyze(&self, source_file: &SourceFile) {
        let definitions: Vec<Function> = source_file
            .functions
            .iter()
            .filter(|function| function.is_definition)
            .cloned()
            .collect();
        for (current, next) in definitions.iter().zip(definitions.iter().skip(1)) {
            if current.range.end.line != next.location.line - 2 {
                println!("{}:{}: C-G2 Violation", source_file.path, next.location.line);
            }
        }
    }
}
