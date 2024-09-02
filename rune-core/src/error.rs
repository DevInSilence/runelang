use core::fmt;

use crate::lexer::location::Location;

pub struct CoreError {
    pub loc: Option<Location>,
    pub message: String,
}

impl CoreError {
    pub fn new(loc: Option<Location>, message: String) -> Self {
        Self { loc, message }
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.loc {
            write!(
                f,
                "{}:{}:{}: {}",
                loc.filename, loc.line, loc.column, self.message
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::location::Location;

    #[test]
    fn test_core_error_display() {
        let loc = Location::new("test.rn", 0, None, 1, 1);
        let error = CoreError::new(Some(loc), "Test error".to_string());
        assert_eq!(error.to_string(), "test.rn:1:1: Test error");
    }
}
