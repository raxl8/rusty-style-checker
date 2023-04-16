use crate::{
    file::{FileKind, SourceFile},
    reporter::Reporter,
};
use regex::Regex;

const C_HEADER_REGEX: &str = concat!(
    r"^/\*\n",
    r"\*\* EPITECH PROJECT, [0-9]{4}\n",
    r"\*\* \S.+\n",
    r"\*\* File description:\n",
    r"(\*\* .*\n)+",
    r"\*/(\n|$)",
);

const MAKEFILE_HEADER_REGEX: &str = concat!(
    r"^##\n",
    r"## EPITECH PROJECT, [0-9]{4}\n",
    r"## \S.+\n",
    r"## File description:\n",
    r"(## .*\n)+",
    r"##(\n|$)",
);

pub struct RuleG1 {
    c_header_regex: Regex,
    makefile_header_regex: Regex,
}

impl RuleG1 {
    pub fn new() -> Self {
        Self {
            c_header_regex: Regex::new(C_HEADER_REGEX).unwrap(),
            makefile_header_regex: Regex::new(MAKEFILE_HEADER_REGEX).unwrap(),
        }
    }
}

impl super::Rule for RuleG1 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        match source_file.kind {
            FileKind::Source | FileKind::Makefile => {}
            _ => return,
        }

        let header_lines = &source_file.contents[0..std::cmp::min(6, source_file.contents.len())];
        let header = header_lines.join("\n");

        match source_file.kind {
            FileKind::Source => {
                if !self.c_header_regex.is_match(&header) {
                    reporter.report(source_file.path.clone(), None, "C-G1 Violation");
                }
            }
            FileKind::Makefile => {
                if !self.makefile_header_regex.is_match(&header) {
                    reporter.report(source_file.path.clone(), None, "C-G1 Violation");
                }
            }
            _ => (),
        }
    }
}
