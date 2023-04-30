use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub column: usize,
    pub line: usize,

    pub line_start: usize,
    pub index: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            column: 1,
            line: 1,

            line_start: 0,
            index: 0,
        }
    }
}
