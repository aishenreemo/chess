use crate::piece::PieceColor;

pub struct Cache {
    pub focused_square: Option<(usize, usize)>,
    pub current_turn: crate::piece::PieceColor,
    pub current_game_state: GameState,
}

impl Cache {
    pub fn init() -> Self {
        Self {
            focused_square: None,
            current_turn: PieceColor::White,
            current_game_state: GameState::SelectingTeam,
        }
    }
}

pub enum GameState {
    SelectingTeam,
    OngoingGame,
    _YouWin,
    _YouLose,
}
