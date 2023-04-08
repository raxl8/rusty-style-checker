use crate::file::{Block, SourceFile};

pub struct RuleC1;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    for branch in block.branches.iter() {
        if depth >= 3 {
            println!(
                "{}:{}: C-C1 Violation",
                source_file.path, branch.location.line
            );
        }
        process_blocks(source_file, &branch.child, depth + 1);
    }
}

impl super::Rule for RuleC1 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            process_blocks(source_file, &func.block, 1);
        }
    }
}
