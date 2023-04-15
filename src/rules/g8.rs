use crate::file::SourceFile;

pub struct RuleG8;

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
    fn analyze(&self, source_file: &SourceFile) {
        let mut empty_lines = get_last_empty_lines(&source_file.contents);
        empty_lines.pop(); // Remove the last empty line as it falls under C-A3
        for i in 0..empty_lines.len() {
            let line_number: usize = source_file.contents.len() - empty_lines.len() + i;
            println!("{}:{}: C-G8 Violation", source_file.path.display(), line_number);
        }
    }
}
