use crate::error::CoreError;

use super::token::Token;

#[allow(unused)]
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

    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<CoreError>> {
        let mut errs = Vec::new();

        while !self.is_eof() {
            errs.push(CoreError::new(None, "Not implemented".to_string()));
        }

        if errs.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(errs)
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.index >= self.source.len()
    }
}
