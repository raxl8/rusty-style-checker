use crate::{file::SourceFile, reporter::Reporter};
use regex::Regex;

pub struct RuleG1;

const C_HEADER_REGEX: &str = concat!(
    r"^/\*\n",
    r"\*\* EPITECH PROJECT, [0-9]{4}\n",
    r"\*\* \S.+\n",
    r"\*\* File description:\n",
    r"(\*\* .*\n)+",
    r"\*/(\n|$)",
);

const MAKEFILE_HEADER_REGEX: &str = concat!(
    r"^/\*\n",
    r"\*\* EPITECH PROJECT, [0-9]{4}\n",
    r"\*\* \S.+\n",
    r"\*\* File description:\n",
    r"(\*\* .*\n)+",
    r"\*/(\n|$)",
);

impl super::Rule for RuleG1 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        let header_lines = &source_file.contents[0..std::cmp::min(6, source_file.contents.len())];
        let header = header_lines.join("\n");
        let header_regex = Regex::new(C_HEADER_REGEX).unwrap();
        if !header_regex.is_match(&header) {
            reporter.report(source_file.path.clone(), None, "C-G1 Violation");
        }
    }
}
