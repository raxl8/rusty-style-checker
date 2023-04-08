use crate::file::{Block, SourceFile};

pub struct RuleC3;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    for goto in block.gotos.iter() {
        println!("{}:{}: C-C3 Violation", source_file.path, goto.line);
    }
    for branch in block.branches.iter() {
        process_blocks(source_file, &branch.child, depth + 1);
    }
}

impl super::Rule for RuleC3 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            process_blocks(source_file, &func.block, 1);
        }
    }
}
