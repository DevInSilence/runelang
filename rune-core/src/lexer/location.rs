#[derive(Debug, Clone)]
pub struct Location {
    pub start_index: usize,
    pub end_index: usize,
    pub line: usize,
    pub column: usize,
    pub filename: &'static str,
}

impl Location {
    pub fn new(filename: &'static str, index: usize, column: usize, line: usize) -> Self {
        Self {
            start_index: index,
            end_index: index,
            line,
            column,
            filename,
        }
    }

    pub fn new_with_range(
        filename: &'static str,
        start_index: usize,
        column: usize,
        line: usize,
        end_index: usize,
    ) -> Self {
        Self {
            filename,
            start_index,
            end_index,
            line,
            column,
        }
    }
}
