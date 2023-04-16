use self::a3::RuleA3;
use self::c1::RuleC1;
use self::c3::RuleC3;
use self::f2::RuleF2;
use self::f3::RuleF3;
use self::f4::RuleF4;
use self::f5::RuleF5;
use self::f6::RuleF6;
use self::f8::RuleF8;
use self::g1::RuleG1;
use self::g2::RuleG2;
use self::g3::RuleG3;
use self::g4::RuleG4;
use self::g5::RuleG5;
use self::g6::RuleG6;
use self::g7::RuleG7;
use self::g8::RuleG8;
use self::h1::RuleH1;
use self::h2::RuleH2;
use self::l2::RuleL2;
use self::o3::RuleO3;
use self::o4::RuleO4;
use self::v1::RuleV1;
use crate::file::SourceFile;
use crate::reporter::Reporter;

pub mod a3;
pub mod c1;
pub mod c3;
pub mod f2;
pub mod f3;
pub mod f4;
pub mod f5;
pub mod f6;
pub mod f8;
pub mod g1;
pub mod g2;
pub mod g3;
pub mod g4;
pub mod g5;
pub mod g6;
pub mod g7;
pub mod g8;
pub mod h1;
pub mod h2;
pub mod l2;
pub mod o3;
pub mod o4;
pub mod v1;

pub trait Rule {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter);
}

pub struct RuleExecutor {
    reporter: Reporter,
    rules: Vec<Box<dyn Rule>>,
}

impl RuleExecutor {
    pub fn new() -> Self {
        RuleExecutor {
            reporter: Reporter::new(),
            rules: vec![
                Box::new(RuleA3),
                Box::new(RuleC1),
                Box::new(RuleC3),
                Box::new(RuleF2),
                Box::new(RuleF3),
                Box::new(RuleF4),
                Box::new(RuleF5),
                Box::new(RuleF6),
                Box::new(RuleF8),
                Box::new(RuleG1),
                Box::new(RuleG2),
                Box::new(RuleG3),
                Box::new(RuleG4),
                Box::new(RuleG5),
                Box::new(RuleG6),
                Box::new(RuleG7),
                Box::new(RuleG8),
                Box::new(RuleH1),
                Box::new(RuleH2),
                Box::new(RuleL2),
                Box::new(RuleO3),
                Box::new(RuleO4),
                Box::new(RuleV1),
            ],
        }
    }

    pub fn run(&mut self, source_file: &SourceFile) {
        for rule in &self.rules {
            rule.analyze(source_file, &mut self.reporter)
        }
    }

    pub fn report(&self) {
        self.reporter.print();
    }
}
