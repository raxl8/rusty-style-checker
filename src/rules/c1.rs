use crate::file::{Branch, SourceFile};

pub struct RuleC1;

fn process_branches(source_file: &SourceFile, branches: &Vec<Branch>, depth: u32) {
    for branch in branches.iter() {
        if depth >= 3 {
            println!(
                "{}:{}: C-C1 Violation",
                source_file.path, branch.location.line
            );
        }
        process_branches(source_file, &branch.children, depth + 1);
    }
}

impl super::Rule for RuleC1 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            process_branches(&source_file, &func.branches, 1);
        }
    }
}
