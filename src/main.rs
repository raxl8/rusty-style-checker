mod file;
mod naming;
mod reporter;
mod rules;

use std::path::PathBuf;

use file::SourceFile;
use ignore::WalkBuilder;
use rules::RuleExecutor;

const IGNORED_FOLDERS: [&str; 2] = ["./tests/", "./bonus/"];

fn process_file(rule_executor: &mut RuleExecutor, path: PathBuf, index: &clang::Index) {
    let tu = index
        .parser(&path)
        .detailed_preprocessing_record(true)
        .parse();
    if let Ok(tu) = tu {
        let source_file = SourceFile::from_clang(path, tu);
        rule_executor.run(&source_file);
    }
}

fn main() {
    let clang = clang::Clang::new().unwrap();
    let index = clang::Index::new(&clang, false, false);
    let mut rule_executor = RuleExecutor::new();
    let entries = WalkBuilder::new(".")
        .ignore(false)
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter(|e| !IGNORED_FOLDERS.into_iter().any(|d| e.path().starts_with(d)));
    for entry in entries {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "c" || extension == "h" {
                let path = entry.path();
                process_file(&mut rule_executor, path.to_path_buf(), &index);
            }
        }
    }
    rule_executor.report();
}
