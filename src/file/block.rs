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

#[derive(Debug)]
pub struct Block {
    pub initial_token: Option<Token>,
    pub expression_range: Option<Range>,
    pub location: Option<Location>,
    pub is_oneliner: bool,
    pub tokens: Vec<Token>,
    pub children: Vec<Block>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            initial_token: None,
            expression_range: None,
            location: None,
            is_oneliner: true,
            tokens: vec![],
            children: vec![],
        }
    }
}

impl Block {
    fn try_find_location(
        self: &mut Self,
        token: Token,
        tokens: &mut Iter<Token>,
        expression_depth: &mut u32,
        depth: &mut u32,
    ) {
        if token.kind != TokenKind::Punctuation {
            return;
        }
        match token.spelling.as_str() {
            "(" => {
                *expression_depth += 1;
            }
            ")" => {
                *expression_depth -= 1;
                if *expression_depth == 0 {
                    self.expression_range = Some(Range {
                        start: self.initial_token.clone().unwrap().location.clone(),
                        end: token.location.clone(),
                    });
                    let next = tokens.next();
                    if let Some(next) = next {
                        if next.spelling == "{" {
                            self.location = Some(next.location.clone());
                            self.is_oneliner = false;
                            *depth += 1;
                        } else {
                            self.location = Some(token.location.clone());
                            self.tokens.push(token.clone());
                        }
                    }
                }
            }
            "{" => {
                if *expression_depth == 0 {
                    self.location = Some(token.location.clone());
                    self.is_oneliner = false;
                    *depth += 1;
                }
            }
            _ => (),
        }
    }

    fn try_find_new_block(self: &mut Self, token: &Token, tokens: &mut Iter<Token>) {
        match token.kind {
            TokenKind::Keyword => match token.spelling.as_str() {
                "else" => {
                    if let Some(next) = tokens.clone().next() {
                        if next.spelling == "if" {
                            let new_block = Block::from_tokens(
                                tokens,
                                Token {
                                    spelling: "else if".to_string(),
                                    kind: TokenKind::Keyword,
                                    location: token.location.clone(),
                                },
                            );
                            self.children.push(new_block);
                        } else {
                            let new_block = Block::from_tokens(tokens, token.clone());
                            self.children.push(new_block);
                        }
                    }
                }
                "if" | "while" | "for" | "switch" => {
                    let new_block = Block::from_tokens(tokens, token.clone());
                    self.children.push(new_block);
                }
                _ => self.tokens.push(token.clone()),
            },
            _ => self.tokens.push(token.clone()),
        }
    }

    pub fn from_tokens(tokens: &mut Iter<Token>, initial_token: Token) -> Self {
        let mut block = Self::default();
        block.initial_token = Some(initial_token);
        let mut expression_depth = 0;
        let mut depth = 0;
        while let Some(token) = tokens.next() {
            if let None = block.location {
                block.try_find_location(token.clone(), tokens, &mut expression_depth, &mut depth);
                continue;
            }
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
                if block.is_oneliner
                    && next.location.line > block.location.clone().unwrap().line + 1
                {
                    break;
                }
            }
        }
        block
    }
}
