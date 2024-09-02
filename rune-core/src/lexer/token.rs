use super::{location::Location, token_kind::TokenKind};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Location,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Location, lexeme: String) -> Self {
        Self { kind, loc, lexeme }
    }
}
