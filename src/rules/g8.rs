use crate::{
    file::{FileKind, SourceFile},
    reporter::Reporter,
};

pub struct RuleG8;

fn get_first_empty_lines(lines: &Vec<String>) -> Vec<String> {
    let mut empty_lines: Vec<String> = Vec::new();

    for line in lines.iter() {
        if line.trim().is_empty() {
            empty_lines.push(line.clone());
        } else {
            break;
        }
    }

    empty_lines.reverse();
    empty_lines
}

fn get_last_empty_lines(lines: &Vec<String>) -> Vec<String> {
    let mut empty_lines: Vec<String> = Vec::new();

    for line in lines.iter().rev() {
        if line.trim().is_empty() {
            empty_lines.push(line.clone());
        } else {
            break;
        }
    }

    empty_lines.reverse();
    empty_lines
}

impl super::Rule for RuleG8 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        match source_file.kind {
            FileKind::Source | FileKind::Makefile => {}
            _ => return,
        }

        let first_empty_lines = get_first_empty_lines(&source_file.contents);
        let mut last_empty_lines = get_last_empty_lines(&source_file.contents);
        last_empty_lines.pop(); // Remove the last empty line as it falls under C-A3
        for (i, _) in first_empty_lines.iter().enumerate() {
            let line_number: usize = i + 1;
            reporter.report(
                source_file.path.clone(),
                Some(line_number as u32),
                "C-G8 Violation",
            );
        }
        for (i, _) in last_empty_lines.iter().enumerate() {
            let line_number: usize = source_file.contents.len() - last_empty_lines.len() + i;
            reporter.report(
                source_file.path.clone(),
                Some(line_number as u32),
                "C-G8 Violation",
            );
        }
    }
}
