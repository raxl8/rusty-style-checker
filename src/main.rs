mod file;
mod rules;
mod naming;

use std::path::PathBuf;

use file::SourceFile;
use rules::RuleExecutor;
use walkdir::WalkDir;

fn process_file(rule_executor: &RuleExecutor, path: PathBuf, index: &clang::Index) {
    let path_str = path.to_str().unwrap();
    let tu = index
        .parser(path_str)
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
    let rule_executor = RuleExecutor::new();
    let entries = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir());
    for entry in entries {
        let path = entry.clone().into_path();
        if let Some(extension) = path.extension() {
            if extension == "c" || extension == "h" {
                let path = entry.path();
                process_file(&rule_executor, path.to_path_buf(), &index);
            }
        }
    }
}
