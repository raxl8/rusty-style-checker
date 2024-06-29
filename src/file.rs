use std::path::{PathBuf};

use clang::{source::SourceLocation, EntityKind, EntityVisitResult};

use self::block::{Block, Token};

pub mod block;

#[derive(Debug, Default, Clone)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn from_clang(location: SourceLocation) -> Self {
        let file_location = location.get_file_location();
        Self {
            line: file_location.line,
            column: file_location.column,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

pub struct IncludeDirective {
    pub file: String,
    pub location: Location,
}

#[derive(Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub tokens: Vec<Token>,
    pub block: Option<Block>,
    pub location: Location,
    pub body: Option<Range>,
    pub range: Range,
    pub is_definition: bool,
    pub is_variadic: bool,
    pub is_type_variadic: bool,
}

pub struct Variable {
    pub name: String,
    pub location: Location,
    pub is_constant: bool,
    pub is_definition: bool,
}

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: String,
    pub location: Location,
}

pub struct MacroDefinition {
    pub name: String,
    pub location: Location,
}

pub struct SourceFile {
    pub path: PathBuf,
    pub file_name: String,
    pub contents: Vec<String>,
    pub include_guarded: bool,
    pub includes: Vec<IncludeDirective>,
    pub macro_definitions: Vec<MacroDefinition>,
    pub type_definitions: Vec<TypeDefinition>,
    pub global_variables: Vec<Variable>,
    pub functions: Vec<Function>,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap().to_os_string().into_string().unwrap();
        SourceFile {
            path,
            file_name,
            contents: vec![],
            include_guarded: false,
            includes: vec![],
            macro_definitions: vec![],
            type_definitions: vec![],
            global_variables: vec![],
            functions: vec![],
        }
    }

    fn add_include_directive(&mut self, entity: clang::Entity) {
        let include_directive = IncludeDirective {
            file: entity.get_name().unwrap_or_default(),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
        self.includes.push(include_directive);
    }

    fn add_macro_definition(&mut self, entity: clang::Entity) {
        let macro_definition = MacroDefinition {
            name: entity.get_name().unwrap_or_default(),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
        self.macro_definitions.push(macro_definition);
    }

    fn add_type_definition(&mut self, entity: clang::Entity) {
        let type_definition = TypeDefinition {
            name: entity.get_name().unwrap_or_default(),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
        self.type_definitions.push(type_definition);
    }

    fn add_global_variable(&mut self, entity: clang::Entity) {
        let variable = Variable {
            name: entity.get_name().unwrap_or_default(),
            location: Location::from_clang(entity.get_location().unwrap()),
            is_definition: entity.is_definition(),
            is_constant: match entity.get_type() {
                Some(t) => t.is_const_qualified(),
                _ => false,
            },
        };
        self.global_variables.push(variable);
    }

    fn add_function(&mut self, entity: clang::Entity) {
        let range = entity.get_range().unwrap();
        let mut function = Function {
            name: entity.get_name().unwrap_or_default(),
            params: vec![],
            tokens: vec![],
            block: None,
            location: Location::from_clang(entity.get_location().unwrap()),
            range: Range {
                start: Location::from_clang(range.get_start()),
                end: Location::from_clang(range.get_end()),
            },
            body: None,
            is_definition: entity.is_definition(),
            is_variadic: entity.is_variadic(),
            is_type_variadic: entity.get_type().unwrap().is_variadic(),
        };
        for argument in entity.get_arguments().unwrap() {
            let param = Param {
                name: argument.get_name().unwrap_or_default(),
            };
            function.params.push(param);
        }
        if let Some(range) = entity.get_range() {
            for token in range.tokenize() {
                let token = Token::from_clang(token);
                function.tokens.push(token);
            }
        }
        entity.visit_children(|child, _| {
            if child.get_kind() == EntityKind::CompoundStmt {
                let file = child
                    .get_location()
                    .unwrap()
                    .get_file_location()
                    .file
                    .unwrap();
                let source_range = child.get_range().unwrap();
                let start_location = file
                    .get_offset_location(source_range.get_start().get_file_location().offset + 2);
                let end_location =
                    file.get_offset_location(source_range.get_end().get_file_location().offset - 2);
                let range = Range {
                    start: Location::from_clang(start_location),
                    end: Location::from_clang(end_location),
                };
                function.body = Some(range);
            }
            EntityVisitResult::Continue
        });
        let mut tokens = function.tokens.iter().peekable();
        let initial_token = tokens.next().unwrap();
        function.block = Some(Block::from_tokens(
            &mut tokens,
            block::BlockType::Unnamed,
            initial_token.clone(),
        ));
        self.functions.push(function);
    }

    pub fn from_clang(path: PathBuf, unit: clang::TranslationUnit) -> Self {
        let mut instance = Self::new(path);
        let root = unit.get_entity();
        if let Some(file) = unit.get_file(&instance.path) {
            instance.include_guarded = file.is_include_guarded();
            if let Some(contents) = file.get_contents() {
                instance.contents = contents.split('\n').map(|s| s.to_string()).collect();
            }
        }
        root.visit_children(|child, _| {
            if !child.is_in_main_file() {
                return EntityVisitResult::Continue;
            }
            match child.get_kind() {
                EntityKind::InclusionDirective => instance.add_include_directive(child),
                EntityKind::MacroDefinition => instance.add_macro_definition(child),
                EntityKind::TypedefDecl => instance.add_type_definition(child),
                EntityKind::VarDecl => instance.add_global_variable(child),
                EntityKind::FunctionDecl => instance.add_function(child),
                _ => (),
            }
            EntityVisitResult::Continue
        });
        instance
    }
}
