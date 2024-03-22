use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub path: Option<PathBuf>,
}

impl Location {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            line: 0,
            column: 0,
            path,
        }
    }

    pub fn increment_line(mut self) -> Self {
        self.line += 1;
        self
    }
}
