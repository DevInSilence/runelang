use super::token::Token;

#[allow(unused)] // just to silence the warning while we're working on this
pub struct Lexer {
    source: &'static str,
    index: usize,
    line: usize,
    column: usize,
    filename: &'static str,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &'static str, filename: &'static str) -> Self {
        Self {
            source,
            index: 0,
            line: 1,
            column: 1,
            filename,
            tokens: Vec::new(),
        }
    }
}
