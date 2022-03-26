use crate::piece::PieceColor;

pub struct Cache {
    pub focused_square: Option<(usize, usize)>,
    pub target_squares: Vec<(usize, usize)>,
    pub current_turn: crate::piece::PieceColor,
    pub current_game_state: GameState,
    pub available_moves: std::collections::HashMap<usize, Vec<crate::board::Move>>,
    pub num_squares_to_edge: [[usize; 8]; 64],
    pub direction_offsets: [i32; 8],
}

impl Cache {
    pub fn init() -> Self {
        Self {
            focused_square: None,
            target_squares: vec![],
            current_turn: PieceColor::White,
            current_game_state: GameState::SelectingTeam,
            available_moves: std::collections::HashMap::new(),
            num_squares_to_edge: precompute_move_data(),
            direction_offsets: [-8, 8, -1, 1, -9, -7, 7, 9],
        }
    }
}

pub enum GameState {
    SelectingTeam,
    OngoingGame,
    _YouWin,
    _YouLose,
}

fn precompute_move_data() -> [[usize; 8]; 64] {
    use std::cmp::min;

    let mut output = [[0; 8]; 64];
    for column in 0..8 {
        for row in 0..8 {
            let up = row;
            let down = 7 - row;
            let left = column;
            let right = 7 - column;

            let square_index = row * 8 + column;

            output[square_index] = [
                up,
                down,
                left,
                right,
                min(up, left),
                min(up, right),
                min(down, left),
                min(down, right),
            ];
        }
    }
    output
}
