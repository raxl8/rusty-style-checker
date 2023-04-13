mod file;
mod rules;

use file::SourceFile;
use rules::RuleExecutor;
use walkdir::WalkDir;

fn process_file(rule_executor: &RuleExecutor, path: &str, index: &clang::Index) {
    let tu = index
        .parser(path)
        .detailed_preprocessing_record(true)
        .parse()
        .unwrap();
    let source_file = SourceFile::from_clang(path.to_string(), tu);
    rule_executor.run(&source_file);
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
                let path = entry.file_name().to_str().unwrap();
                process_file(&rule_executor, path, &index);
            }
        }
    }
}
