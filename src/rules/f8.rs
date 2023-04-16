use clang::token::TokenKind;

use crate::{file::{SourceFile, block::Block, FileKind}, reporter::Reporter};

pub struct RuleF8;

fn process_blocks(source_file: &SourceFile, reporter: &mut Reporter, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    for token in block.tokens.iter() {
        if token.kind == TokenKind::Comment {
            reporter.report(
                source_file.path.clone(),
                Some(token.location.line),
                "C-F8 Violation",
            );
        }
    }
    for child in block.children.iter() {
        process_blocks(source_file, reporter, child, depth + 1);
    }
}

impl super::Rule for RuleF8 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Source {
            return;
        }

        for func in source_file.functions.iter() {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, reporter, block, 0);
            }
        }
    }
}
