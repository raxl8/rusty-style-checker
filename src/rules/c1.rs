use crate::file::{block::Block, SourceFile};

pub struct RuleC1;

fn process_blocks(source_file: &SourceFile, block: &Block, depth: u32) {
    if depth > 10 {
        return;
    }
    if depth >= 3 {
        println!(
            "{}:{} C-C1 Violation",
            source_file.path,
            block.location.as_ref().unwrap().line
        );
    }
    let mut inline_depth = 0;
    for child in block.children.iter() {
        if let Some(initial_token) = &child.initial_token {
            match initial_token.spelling.as_str() {
                "if" | "for" | "while" => {
                    inline_depth = 1;
                }
                "else if" => inline_depth += 1,
                _ => (),
            }
            process_blocks(source_file, &child, depth + inline_depth);
        }
    }
}

impl super::Rule for RuleC1 {
    fn analyze(&self, source_file: &SourceFile) {
        for func in source_file.functions.iter() {
            process_blocks(source_file, &func.block, 0);
        }
    }
}
