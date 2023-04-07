use clang::{EntityKind, EntityVisitResult};

#[derive(Debug)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn from_clang(location: clang::source::SourceLocation) -> Self {
        let file_location = location.get_file_location();
        Self {
            line: file_location.line,
            column: file_location.column,
        }
    }
}

#[derive(Debug)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

pub struct IncludeDirective {
    pub file: String,
    pub location: Location,
}

pub struct Param {
    name: String,
}

pub struct Function {
    pub name: String,
    pub is_definition: bool,
    pub params: Vec<Param>,
    pub location: Location,
    pub range: Option<Range>,
}

pub struct Variable {
    pub is_constant: bool,
    pub location: Location,
}

pub struct SourceFile {
    pub path: String,
    pub includes: Vec<IncludeDirective>,
    pub global_variables: Vec<Variable>,
    pub functions: Vec<Function>,
}

impl SourceFile {
    pub fn new(path: String) -> Self {
        SourceFile {
            path,
            includes: vec![],
            global_variables: vec![],
            functions: vec![],
        }
    }

    fn add_include_directive(&mut self, entity: clang::Entity) {
        let include_directive = IncludeDirective {
            file: entity.get_name().unwrap_or("".to_string()),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
        self.includes.push(include_directive);
    }

    fn add_function(&mut self, entity: clang::Entity) {
        let mut function = Function {
            name: entity.get_name().unwrap_or_default(),
            is_definition: entity.is_definition(),
            params: vec![],
            location: Location::from_clang(entity.get_location().unwrap()),
            range: None,
        };
        entity.visit_children(|child, _| {
            match child.get_kind() {
                EntityKind::ParmDecl => {
                    let param = Param {
                        name: child.get_name().unwrap_or_default(),
                    };
                    function.params.push(param);
                }
                EntityKind::CompoundStmt => {
                    let file = child
                        .get_location()
                        .unwrap()
                        .get_file_location()
                        .file
                        .unwrap();
                    let source_range = child.get_range().unwrap();
                    let start_location = file.get_offset_location(
                        source_range.get_start().get_file_location().offset + 2,
                    );
                    let end_location = file
                        .get_offset_location(source_range.get_end().get_file_location().offset - 2);
                    let range = Range {
                        start: Location::from_clang(start_location),
                        end: Location::from_clang(end_location),
                    };
                    function.range = Some(range);
                }
                _ => (),
            }
            EntityVisitResult::Continue
        });
        self.functions.push(function);
    }

    fn add_global_variable(&mut self, entity: clang::Entity) {
        let variable = Variable {
            location: Location::from_clang(entity.get_location().unwrap()),
            is_constant: match entity.get_type() {
                Some(t) => t.is_const_qualified(),
                _ => false,
            },
        };
        self.global_variables.push(variable);
    }

    pub fn from_clang(path: String, unit: clang::TranslationUnit) -> Self {
        let mut instance = Self::new(path);
        let root = unit.get_entity();
        root.visit_children(|child, _| {
            if !child.is_in_main_file() {
                return EntityVisitResult::Continue;
            }
            match child.get_kind() {
                EntityKind::FunctionDecl => instance.add_function(child),
                EntityKind::VarDecl => instance.add_global_variable(child),
                EntityKind::InclusionDirective => instance.add_include_directive(child),
                _ => (),
            }
            EntityVisitResult::Continue
        });
        instance
    }
}
