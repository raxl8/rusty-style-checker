use std::borrow::BorrowMut;

use clang::token::TokenKind;

use crate::file::{
    block::{Block, BlockType, Token},
    Location, SourceFile,
};

pub struct RuleL2;

fn get_first_tokens_each_line(block: &Block) -> Vec<Token> {
    let mut tokens = block.tokens.iter();
    let mut tokens_list: Vec<Token> = Vec::new();

    while let Some(token) = tokens.next() {
        tokens_list.push(token.clone());
        if token.kind == TokenKind::Keyword {
            match token.spelling.as_str() {
                "case" | "default" => {
                    while let Some(next) = tokens.next() {
                        if next.kind == TokenKind::Punctuation && next.spelling == ":" {
                            break;
                        }
                    }
                    continue;
                }
                _ => {}
            }
        }
        while let Some(next) = tokens.next() {
            if next.kind == TokenKind::Punctuation && next.spelling == ";" {
                break;
            }
        }
    }
    tokens_list
}

fn verify_indent(source_file: &SourceFile, location: &Location, depth: u32) -> bool {
    if location.column != depth * 4 + 1 {
        return false;
    }
    let line = &source_file.contents[location.line as usize - 1];
    for i in 0..location.column - 1 {
        match line.chars().nth(i as usize) {
            Some(' ') => {}
            _ => return false,
        }
    }
    true
}

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }

    let tokens: Vec<Token> = get_first_tokens_each_line(block);
    let mut indent: u32 = 0;
    let mut is_case: bool = false;
    for token in tokens.iter() {
        if block.init_type == BlockType::Switch && token.kind == TokenKind::Keyword {
            match token.spelling.as_str() {
                "case" | "default" => {
                    indent = 0;
                    is_case = true;
                }
                _ => {}
            }
        }
        if !verify_indent(source_file, &token.location, depth + indent) {
            println!(
                "{}:{}: L-L2 Violation",
                source_file.path.display(),
                token.location.line
            );
        }
        if is_case {
            indent = 1;
            is_case = false;
        }
    }
    let mut children = block.children.iter().peekable();
    while let Some(current) = children.next() {
        let mut prev = current;
        while let Some(next) = children.peek() {
            match next.init_type {
                BlockType::Else | BlockType::ElseIf => {
                    if prev.is_oneliner && !verify_indent(source_file, &next.range.start, depth) {
                        println!(
                            "{}:{}: L-L2 Violation",
                            source_file.path.display(),
                            next.range.start.line
                        );
                    }
                    if !next.is_oneliner && !verify_indent(source_file, &next.range.end, depth) {
                        println!(
                            "{}:{}: L-L2 Violation",
                            source_file.path.display(),
                            next.range.end.line
                        );
                    }
                    process_blocks(source_file, &next, depth + 1);
                }
                _ => break,
            }
            prev = children.next().unwrap();
        }
        if !verify_indent(source_file, &current.range.start, depth) {
            println!(
                "{}:{}: L-L2 Violation",
                source_file.path.display(),
                current.range.start.line
            );
        }
        if !current.is_oneliner && !verify_indent(source_file, &current.range.end, depth) {
            println!(
                "{}:{}: L-L2 Violation",
                source_file.path.display(),
                current.range.end.line
            );
        }
        process_blocks(source_file, &current, depth + 1);
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
