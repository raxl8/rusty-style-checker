use clang::token::TokenKind;

use crate::file::{SourceFile, block::Block};

pub struct RuleF8;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    for token in block.tokens.iter() {
        if token.kind == TokenKind::Comment {
            println!(
                "{}:{}: C-F8 Violation",
                source_file.path.display(), token.location.line
            );
        }
    }
    for child in block.children.iter() {
        process_blocks(source_file, child, depth + 1);
    }
}

impl super::Rule for RuleF8 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, block, 0);
            }
        }
    }
}
