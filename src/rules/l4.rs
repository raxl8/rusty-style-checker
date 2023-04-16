use crate::{
    file::{
        block::{Block, BlockType},
        Range, SourceFile,
    },
    reporter::Reporter,
};

pub struct RuleL4;

fn is_bracket_in_right_place(prev: Option<&Block>, current: &Block) -> bool {
    let range = match &current.expression_range {
        Some(expression_range) => expression_range.clone(),
        _ => Range {
            start: current.location.clone(),
            end: current.location.clone(),
        },
    };
    if let Some(prev) = prev {
        if !prev.is_oneliner && prev.range.end.line != range.start.line {
            return false;
        }
    }
    match current.init_type {
        BlockType::Function => current.range.start.line - range.end.line == 1,
        BlockType::DoWhile => {
            current.range.start.line == current.location.line
                && range.start.line == current.range.end.line
        }
        _ => current.is_oneliner || range.end.line == current.range.start.line,
    }
}

fn process_blocks(source_file: &SourceFile, reporter: &mut Reporter, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }

    if block.init_type == BlockType::Function && !is_bracket_in_right_place(None, block) {
        reporter.report(
            source_file.path.clone(),
            Some(block.location.line),
            "C-L4 Violation",
        );
    }

    let mut children = block.children.iter().peekable();
    while let Some(current) = children.next() {
        let mut prev = current;
        while let Some(next) = children.peek() {
            match next.init_type {
                BlockType::Else | BlockType::ElseIf => {
                    if !is_bracket_in_right_place(Some(prev), next) {
                        reporter.report(
                            source_file.path.clone(),
                            Some(next.location.line),
                            "C-L4 Violation",
                        );
                    }
                    process_blocks(source_file, reporter, &next, depth + 1);
                }
                _ => break,
            }
            prev = children.next().unwrap();
        }

        if !is_bracket_in_right_place(None, current) {
            reporter.report(
                source_file.path.clone(),
                Some(current.location.line),
                "C-L4 Violation",
            );
        }
        process_blocks(source_file, reporter, &current, depth + 1);
    }
}

impl super::Rule for RuleL4 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        let function_definitions = source_file.functions.iter().filter(|f| f.is_definition);
        for func in function_definitions {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, reporter, block, 0);
            }
        }
    }
}
