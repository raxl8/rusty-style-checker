use crate::{file::{SourceFile, block::{Block, BlockType}}, reporter::Reporter};

pub struct RuleL4;

fn process_blocks(source_file: &SourceFile, reporter: &mut Reporter, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }

    if block.init_type == BlockType::Unnamed {
        if block.range.start.line - block.location.line != 1 {
            reporter.report(
                source_file.path.clone(),
                Some(block.range.start.line),
                "C-L4 Violation",
            );
        }
    }

    let mut children = block.children.iter().peekable();
    while let Some(current) = children.next() {
        let mut prev = current;
        while let Some(next) = children.peek() {
            match next.init_type {
                BlockType::Else | BlockType::ElseIf => {
                    if !prev.is_oneliner && next.location.line != prev.range.end.line {
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
        if !current.is_oneliner && current.range.start.line != current.location.line {
            reporter.report(
                source_file.path.clone(),
                Some(current.range.start.line),
                "C-L4 Violation",
            );
        }
        process_blocks(source_file, reporter, &current, depth + 1);
    }
}

impl super::Rule for RuleL4 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        for func in source_file.functions.iter() {
            if let Some(block) = func.block.as_ref() {
                process_blocks(source_file, reporter, block, 0);
            }
        }
    }
}
