//! chess board structures

/// board side length
pub const SIDE_LENGTH: usize = 8;

/// chess game board structure
pub struct Chess {
    /// stores the content of the cells
    pub cells: [[ChessCell; SIDE_LENGTH]; SIDE_LENGTH]
}

impl Chess {
    /// creates a new chessboard
    pub fn new() -> Self {
        Self {
            cells: [[ChessCell::Empty; SIDE_LENGTH]; SIDE_LENGTH],
        }
    }
}

#[derive(Copy, Clone)]
/// the container
pub enum ChessCell {
    /// cell has no piece
    Empty,
    /// cell has a piece
    Piece,
}