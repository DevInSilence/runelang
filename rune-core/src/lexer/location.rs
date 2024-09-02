#[derive(Debug, Clone)]
pub struct Location {
    pub start_index: usize,
    pub end_index: Option<usize>,
    pub line: usize,
    pub column: usize,
    pub filename: &'static str,
}

impl Location {
    pub fn new(
        filename: &'static str,
        start_index: usize,
        end_index: Option<usize>,
        column: usize,
        line: usize,
    ) -> Self {
        Self {
            start_index,
            end_index,
            line,
            column,
            filename,
        }
    }
}
