use std::path::PathBuf;

struct Report {
    pub path: PathBuf,
    pub line: Option<u32>,
    pub message: String,
}

pub struct Reporter {
    pub advanced_rules: bool,
    reports: Vec<Report>,
}

impl Reporter {
    pub fn new(advanced_rules: bool) -> Self {
        Self {
            advanced_rules,
            reports: Vec::new(),
        }
    }

    pub fn report(&mut self, path: PathBuf, line: Option<u32>, message: &str) {
        self.reports.push(Report {
            path,
            line,
            message: message.to_string(),
        });
    }

    pub fn print(&self) {
        for report in self.reports.iter() {
            if let Some(line) = report.line {
                println!("{}:{}: {}", report.path.display(), line, report.message);
            } else {
                println!("{}: {}", report.path.display(), report.message);
            }
        }
    }
}
