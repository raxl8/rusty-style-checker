use std::slice::Iter;

use clang::token::TokenKind;

use crate::file::{Location, Range};

#[derive(Debug, Clone)]
pub struct Token {
    pub spelling: String,
    pub kind: TokenKind,
    pub location: Location,
}

impl Token {
    pub fn from_clang(token: clang::token::Token) -> Self {
        let location = Location::from_clang(token.get_location());
        Self {
            spelling: token.get_spelling(),
            kind: token.get_kind(),
            location,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockType {
    If,
    Else,
    ElseIf,
    For,
    While,
    Switch,
    Function,
}

impl BlockType {
    fn from_token(spelling: &str) -> Self {
        match spelling {
            "if" => Self::If,
            "else" => Self::Else,
            "for" => Self::For,
            "while" => Self::While,
            "switch" => Self::Switch,
            _ => Self::Function,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub init_type: BlockType,
    pub expression_range: Option<Range>,
    pub location: Location,
    pub start: Option<Location>,
    pub is_oneliner: bool,
    pub tokens: Vec<Token>,
    pub children: Vec<Block>,
}

impl Block {
    fn try_find_location(
        self: &mut Self,
        token: Token,
        tokens: &mut Iter<Token>,
        depth: &mut u32,
    ) -> Result<(), ()> {
        match self.init_type {
            BlockType::If
            | BlockType::ElseIf
            | BlockType::For
            | BlockType::While
            | BlockType::Switch => {
                let next = tokens.next();
                match next {
                    Some(next) => {
                        if next.spelling != "(" {
                            return Err(());
                        }
                    }
                    None => return Err(()),
                }
            }
            _ => (),
        }
        match self.init_type {
            BlockType::If
            | BlockType::ElseIf
            | BlockType::For
            | BlockType::While
            | BlockType::Switch => {
                let mut depth = 1;
                for next in tokens.by_ref() {
                    match next.spelling.as_str() {
                        "(" => depth += 1,
                        ")" => depth -= 1,
                        _ => (),
                    }
                    if depth == 0 {
                        self.expression_range = Some(Range {
                            start: token.location,
                            end: next.location.clone(),
                        });
                        break;
                    }
                }
            }
            BlockType::Function => {
                while let Some(_) = tokens.next() {
                    if let Some(next) = tokens.clone().next() {
                        if next.spelling == "{" {
                            break;
                        }
                    }
                }
            }
            _ => (),
        }
        if let Some(next) = tokens.clone().next() {
            if next.spelling == "{" {
                tokens.next();
                self.start = Some(next.location.clone());
                self.is_oneliner = false;
                *depth += 1;
            } else if self.init_type != BlockType::Function {
                self.start = Some(next.location.clone());
            } else {
                return Err(());
            }
        }
        Ok(())
    }

    fn try_find_new_block(self: &mut Self, token: &Token, tokens: &mut Iter<Token>) {
        let spelling = token.spelling.as_str();
        match token.kind {
            TokenKind::Keyword => match spelling {
                "else" => {
                    if let Some(next) = tokens.clone().next() {
                        if next.spelling == "if" {
                            tokens.next();
                            let new_block =
                                Block::from_tokens(tokens, BlockType::ElseIf, token.clone());
                            self.children.push(new_block);
                        } else {
                            let new_block =
                                Block::from_tokens(tokens, BlockType::Else, token.clone());
                            self.children.push(new_block);
                        }
                    }
                }
                "if" | "while" | "for" | "switch" => {
                    let new_block = Block::from_tokens(
                        tokens,
                        BlockType::from_token(token.spelling.as_str()),
                        token.clone(),
                    );
                    self.children.push(new_block);
                }
                _ => self.tokens.push(token.clone()),
            },
            _ => self.tokens.push(token.clone()),
        }
    }

    pub fn from_tokens(
        tokens: &mut Iter<Token>,
        init_type: BlockType,
        initial_token: Token,
    ) -> Self {
        let mut block = Block {
            init_type,
            expression_range: None,
            location: initial_token.location.clone(),
            start: None,
            is_oneliner: true,
            tokens: vec![],
            children: vec![],
        };
        let mut depth = 0;
        block.try_find_location(initial_token, tokens, &mut depth);
        while let Some(token) = tokens.next() {
            if token.kind == TokenKind::Punctuation {
                if token.spelling == "{" {
                    depth += 1;
                } else if token.spelling == "}" {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            block.try_find_new_block(token, tokens);
            if let Some(next) = tokens.clone().next() {
                if block.is_oneliner && next.location.line > block.start.clone().unwrap().line {
                    break;
                }
            }
        }
        block
    }
}
