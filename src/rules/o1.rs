use regex::Regex;

use crate::{file::{SourceFile, FileKind}, reporter::Reporter};

const UNWANTED_FILES: &[&str] = &[
    // Prerequisites
    r".*\.d$",

    // Object files
    r".*\.o$",
    r".*\.ko$",
    r".*\.obj$",
    r".*\.elf$",

    // Linker output
    r".*\.ilk$",
    r".*\.map$",
    r".*\.exp$",

    // Precompiled Headers
    r".*\.gch$",
    r".*\.pch$",

    // Libraries
    r".*\.lib$",
    r".*\.a$",
    r".*\.la$",
    r".*\.lo$",

    // Shared objects (inc. Windows DLLs)
    r".*\.dll$",
    r".*\.so$",
    r".*\.so\..*$",
    r".*\.dylib$",

    // Executables
    r".*\.exe$",
    r".*\.out$",
    r".*\.app$",
    r".*\.i.*86$",
    r".*\.x86_64$",
    r".*\.hex$",

    // Debug files
    r".*\.su$",
    r".*\.idb$",
    r".*\.pdb$",

    // Kernel Module Compile Results
    r".*\.mod.*$",
    r".*\.cmd$",
    r"^modules\.order$",
    r"^Module\.symvers$",
    r"^Mkfile\.old$",
    r"^dkms\.conf$",

    // gcc coverage testing tool files
    r".*\.gcno$",
    r".*\.gcda$",
    r".*\.gcov$",

    // Temporary files
    r".*~.*",
    r".*#.*",

    // Valgrind core dump files
    r"^vgcore\.\d+$"
];

const UNWANTED_BINARY_MAGIC : &[&[u8]] = &[
    // ELF
    b"\x7fELF",

    // EXE
    b"MZ",

    // Mach-O
    b"\xfe\xed\xfa\xce",

    // PE
    b"\x4d\x5a",
];

pub struct RuleO1 {
    unwanted_files: Vec<Regex>,
}

impl RuleO1 {
    pub fn new() -> Self {
        let mut unwanted_files = Vec::with_capacity(UNWANTED_FILES.len());
        for pattern in UNWANTED_FILES {
            unwanted_files.push(Regex::new(pattern).unwrap());
        }
        Self { unwanted_files }
    }
}

impl super::Rule for RuleO1 {
    fn analyze(&self, source_file: &SourceFile, reporter: &mut Reporter) {
        if source_file.kind != FileKind::Other {
            return;
        }
        for pattern in &self.unwanted_files {
            if pattern.is_match(&source_file.file_name) {
                reporter.report(source_file.path.clone(), None,
                    "C-O1 Violation");
                return;
            }
        }
        if source_file.first_bytes.len() > 0 {
            for magic in UNWANTED_BINARY_MAGIC {
                if source_file.first_bytes.starts_with(magic) {
                    reporter.report(source_file.path.clone(), None,
                    "C-O1 Violation");
                    return;
                }
            }
        }
    }
}
