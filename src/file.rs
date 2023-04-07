use clang::{EntityKind, EntityVisitResult};

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

pub struct IncludeDirective {
    pub file: String,
    pub location: Location,
}

pub struct Function {
    pub name: String,
    pub is_definition: bool,
    pub location: Location,
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
            includes: Vec::new(),
            global_variables: Vec::new(),
            functions: Vec::new(),
        }
    }

    fn add_include_directive(&mut self, entity: clang::Entity) {
        let include_directive = IncludeDirective {
            location: Location::from_clang(entity.get_location().unwrap()),
            file: entity.get_name().unwrap_or("".to_string()),
        };
        self.includes.push(include_directive);
    }

    fn add_function(&mut self, entity: clang::Entity) {
        let function = Function {
            name: entity.get_name().unwrap_or_default(),
            is_definition: entity.is_definition(),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
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
