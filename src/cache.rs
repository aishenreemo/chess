use crate::piece::PieceColor;

pub struct Cache {
    pub focused_square: Option<(usize, usize)>,
    pub current_turn: crate::piece::PieceColor,
}

impl Cache {
    pub fn init() -> Self {
        Self {
            focused_square: None,
            current_turn: PieceColor::White,
        }
    }
}