use std::collections::HashSet;

use clang::token::TokenKind;

use crate::file::{
    block::{Block, BlockType, Token},
    SourceFile,
};

pub struct RuleL2;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }

    let mut lines: HashSet<u32> = HashSet::new();
    let tokens: Vec<Token> = block
        .tokens
        .iter()
        .filter(|token| {
            if lines.contains(&token.location.line) {
                return false;
            }
            lines.insert(token.location.line);
            true
        })
        .cloned()
        .collect();
    let mut indent: u32 = 0;
    let mut is_case: bool = false;
    for token in tokens.iter() {
        if block.init_type == BlockType::Switch {
            if token.kind == TokenKind::Keyword && token.spelling == "case" {
                indent = 0;
                is_case = true;
            }
        }
        if token.location.column != (depth + indent) * 4 + 1 {
            println!(
                "{}:{}: L-L2 Violation",
                source_file.path, token.location.line
            );
        }
        if is_case {
            indent = 1;
            is_case = false;
        }
    }
    for child in block.children.iter() {
        if child.location.column != depth * 4 + 1 {
            println!(
                "{}:{}: L-L2 Violation",
                source_file.path, child.location.line
            );
        }
        process_blocks(source_file, child, depth + 1);
    }
}

impl super::Rule for RuleL2 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, block, 1);
            }
        }
    }
}
