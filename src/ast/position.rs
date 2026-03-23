use std::num::NonZeroUsize;

/// A position in the source code, represented by a line and column number.
pub struct Position {
    pub line: NonZeroUsize,
    pub column: NonZeroUsize,
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
