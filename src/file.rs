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
    pub name: String,
}

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub tokens: Vec<Token>,
    pub block: Option<Block>,
    pub location: Location,
    pub range: Option<Range>,
    pub is_definition: bool,
    pub is_variadic: bool,
    pub is_type_variadic: bool,
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
            file: entity.get_name().unwrap_or_default(),
            location: Location::from_clang(entity.get_location().unwrap()),
        };
        self.includes.push(include_directive);
    }

    fn add_function(&mut self, entity: clang::Entity) {
        let mut function = Function {
            name: entity.get_name().unwrap_or_default(),
            params: vec![],
            tokens: vec![],
            block: None,
            location: Location::from_clang(entity.get_location().unwrap()),
            range: None,
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
                function.range = Some(range);
            }
            EntityVisitResult::Continue
        });
        let mut tokens = function.tokens.iter();
        let initial_token = tokens.next().unwrap();
        function.block = Some(Block::from_tokens(
            &mut tokens,
            block::BlockType::Function,
            initial_token.clone(),
        ));
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
