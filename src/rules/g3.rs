use crate::file::SourceFile;

pub struct RuleG3;

impl super::Rule for RuleG3 {
    fn analyze(&self, source_file: &SourceFile) {
        let non_const_global_vars = source_file
            .global_variables
            .iter()
            .filter(|var| !var.is_constant);
        for var in non_const_global_vars {
            println!("{}:{}: C-G3 Violation", source_file.path, var.location.line);
        }
    }
}
