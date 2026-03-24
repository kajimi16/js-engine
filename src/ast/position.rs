use std::num::NonZeroUsize;

/// A position in the source code, represented by a line and column number.
pub struct Position {
    pub line: NonZeroUsize,
    pub column: NonZeroUsize,
}
impl Default for Position {
    fn default() -> Self {
        Self::new(1, 1)
    }
}


impl Position {
    /// Creates a new `Position` from a line and column number.
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line: NonZeroUsize::new(line).expect("line number must be greater than 0"),
            column: NonZeroUsize::new(column).expect("column number must be greater than 0"),
        }
    }

    pub fn line(&self) -> usize {
        self.line.get()
    }
    
    pub fn column(&self) -> usize {
        self.column.get()
    }
}

// A linear position in the source code, represented by a single byte offset.
pub struct LinearPosition {
    pos: usize,
}

// A span in the source code, represented by a start and end position.
pub struct Span {
    start: Position,
    end: Position,
}

// A linear span in the source code, represented by a start and end linear position.
pub struct LinearSpan {
    start: LinearPosition,
    end: LinearPosition,
}
