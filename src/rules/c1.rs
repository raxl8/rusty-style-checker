use crate::file::{
    block::{Block, BlockType},
    SourceFile,
};

pub struct RuleC1;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    if depth >= 3 {
        println!(
            "{}:{} C-C1 Violation",
            source_file.path, block.location.line
        );
    }
    let mut inline_depth = 0;
    for child in block.children.iter() {
        match child.init_type {
            BlockType::If | BlockType::For | BlockType::While => {
                inline_depth = 1;
            }
            BlockType::ElseIf => inline_depth += 1,
            _ => (),
        }
        process_blocks(source_file, child, depth + inline_depth);
    }
}

impl super::Rule for RuleC1 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, block, 0);
            }
        }
    }
}
