use std::{iter::Peekable, slice::Iter};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockType {
    If,
    Else,
    ElseIf,
    For,
    While,
    DoWhile,
    Switch,
    Unnamed,
}

impl BlockType {
    fn from_token(spelling: &str) -> Self {
        match spelling {
            "if" => Self::If,
            "else" => Self::Else,
            "for" => Self::For,
            "while" => Self::While,
            "do" => Self::DoWhile,
            "switch" => Self::Switch,
            _ => Self::Unnamed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub init_type: BlockType,
    pub expression_range: Option<Range>,
    pub range: Range,
    pub location: Location,
    pub is_oneliner: bool,
    pub tokens: Vec<Token>,
    pub children: Vec<Block>,
}

impl Block {
    fn find_expression_range(
        initial_token: Token,
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Option<Range> {
        let mut expression_range: Option<Range> = None;
        let mut depth = 0;
        for next in tokens.by_ref() {
            match next.spelling.as_str() {
                "(" => depth += 1,
                ")" => depth -= 1,
                _ => (),
            }
            if depth == 0 {
                expression_range = Some(Range {
                    start: initial_token.location,
                    end: next.location.clone(),
                });
                break;
            }
        }
        expression_range
    }

    fn try_find_location(
        self: &mut Self,
        token: Token,
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<(), ()> {
        if token.kind == TokenKind::Punctuation && token.spelling == "{" {
            self.range.start = token.location.clone();
            self.is_oneliner = false;
            return Ok(());
        }
        match self.init_type {
            BlockType::If
            | BlockType::ElseIf
            | BlockType::For
            | BlockType::While
            | BlockType::Switch => match tokens.peek() {
                Some(next) => {
                    if next.spelling != "(" {
                        return Err(());
                    }
                }
                None => return Err(()),
            },
            _ => (),
        }
        match self.init_type {
            BlockType::If
            | BlockType::ElseIf
            | BlockType::For
            | BlockType::While
            | BlockType::Switch => {
                self.expression_range = Self::find_expression_range(token, tokens);
            }
            BlockType::Unnamed => {
                while let Some(next) = tokens.peek() {
                    if next.spelling == "{" {
                        break;
                    }
                    tokens.next();
                }
            }
            _ => (),
        }
        if let Some(next) = tokens.peek() {
            if next.spelling == "{" {
                self.range.start = next.location.clone();
                self.is_oneliner = false;
                tokens.next();
            } else if self.init_type != BlockType::Unnamed {
                self.range.start = next.location.clone();
            } else {
                return Err(());
            }
        }
        Ok(())
    }

    fn try_find_new_block(self: &mut Self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        let spelling = token.spelling.as_str();
        match token.kind {
            TokenKind::Keyword => match spelling {
                "else" => {
                    if let Some(next) = tokens.peek() {
                        if next.spelling == "if" {
                            tokens.next();
                            self.children.push(Block::from_tokens(
                                tokens,
                                BlockType::ElseIf,
                                token.clone(),
                            ));
                        } else {
                            self.children.push(Block::from_tokens(
                                tokens,
                                BlockType::Else,
                                token.clone(),
                            ));
                        }
                    }
                }
                "if" | "while" | "for" | "do" | "switch" => {
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
        tokens: &mut Peekable<Iter<Token>>,
        init_type: BlockType,
        initial_token: Token,
    ) -> Self {
        let mut block = Block {
            init_type,
            expression_range: None,
            location: initial_token.location.clone(),
            range: Range {
                start: initial_token.location.clone(),
                end: initial_token.location.clone(),
            },
            is_oneliner: true,
            tokens: vec![],
            children: vec![],
        };
        block.try_find_location(initial_token, tokens);
        while let Some(token) = tokens.next() {
            if token.kind == TokenKind::Punctuation {
                match token.spelling.as_str() {
                    "{" => {
                        block.children.push(Block::from_tokens(
                            tokens,
                            BlockType::Unnamed,
                            token.clone(),
                        ));
                        continue;
                    }
                    "}" => {
                        block.range.end = token.location.clone();
                        if block.init_type == BlockType::DoWhile {
                            if let Some(next) = tokens.peek() {
                                if next.spelling == "while" {
                                    let next = tokens.next();
                                    block.expression_range =
                                        Self::find_expression_range(next.unwrap().clone(), tokens);
                                    tokens.next(); // ; token
                                }
                            }
                        }
                        break;
                    }
                    _ => (),
                }
            }
            block.try_find_new_block(token, tokens);
            if let Some(next) = tokens.peek() {
                if block.is_oneliner && next.location.line > block.range.start.line {
                    break;
                }
            }
        }
        block
    }
}
