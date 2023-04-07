use crate::file::SourceFile;

pub mod g4;
pub mod g5;
pub mod o3;

use self::g4::RuleG4;
use self::g5::RuleG5;
use self::o3::RuleO3;

pub trait Rule {
    fn analyze(&self, source_file: &SourceFile);
}

pub struct RuleExecutor {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleExecutor {
    pub fn new() -> Self {
        RuleExecutor {
            rules: vec![Box::new(RuleO3), Box::new(RuleG4), Box::new(RuleG5)],
        }
    }

    pub fn run(&self, source_file: &SourceFile) {
        for rule in &self.rules {
            rule.analyze(source_file)
        }
    }
}
