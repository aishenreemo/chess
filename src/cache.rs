use crate::piece::{PieceColor, PieceVariant};

pub struct Cache {
    pub focused_square: Option<(usize, usize)>,
    pub target_squares: Vec<(usize, usize)>,
    pub current_turn: crate::piece::PieceColor,
    pub player_color: crate::piece::PieceColor,
    pub current_game_state: GameState,
    pub available_moves: std::collections::HashMap<usize, Vec<crate::board::Move>>,
    pub king_initial_column: usize,
    pub is_castling_pieces_unmoved: [bool; 6],
    pub castling_pieces_initial_position: [([usize; 2], PieceVariant); 6],
    pub recent_promoting_pawn: Option<(usize, usize)>,
}

impl Cache {
    pub fn init() -> Self {
        Self {
            focused_square: None,
            target_squares: vec![],
            current_turn: PieceColor::White,
            player_color: PieceColor::White,
            current_game_state: GameState::SelectingTeam,
            available_moves: std::collections::HashMap::new(),
            king_initial_column: 4,
            is_castling_pieces_unmoved: [true, true, true, true, true, true],
            castling_pieces_initial_position: [([2; 2], PieceVariant::King); 6],
            recent_promoting_pawn: None,
        }
    }
}

pub enum GameState {
    SelectingTeam,
    OngoingGame,
    PromotingPawn,
    _YouWin,
    _YouLose,
}

pub fn precompute_castling_pieces_init_pos(cached: &Cache) -> [([usize; 2], PieceVariant); 6] {
    [
        ([0, 0], PieceVariant::Castle),
        if cached.player_color == PieceColor::White {
            ([4, 0], PieceVariant::King)
        } else {
            ([3, 0], PieceVariant::King)
        },
        ([7, 0], PieceVariant::Castle),
        ([0, 7], PieceVariant::Castle),
        if cached.player_color == PieceColor::White {
            ([4, 7], PieceVariant::King)
        } else {
            ([3, 7], PieceVariant::King)
        },
        ([7, 7], PieceVariant::Castle),
    ]
}
