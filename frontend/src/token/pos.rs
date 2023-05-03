use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Pos {
    pub fn update(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            _ => self.column += 1,
        }

        self.index += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.line_start = self.index + 1;
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
