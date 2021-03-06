use crate::produce::{self, Move};
use crate::Error;

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use std::collections::HashSet;

pub struct Game {
    pub state: GameState,
    pub texture_creator: TextureCreator<WindowContext>,
    pub board: [[Option<Piece>; 8]; 8],
    pub cache: Cache,
}

impl Game {
    pub fn get_square(&self, column: usize, row: usize) -> Option<&Piece> {
        self.board.get(row)?.get(column)?.as_ref()
    }
}

pub struct Cache {
    pub window_size: (f32, f32),
    pub board_size: (f32, f32),
    pub board_offset: (f32, f32),
    pub square_size: (f32, f32),
    pub data: GameData,
}

pub struct GameData {
    pub focused_square: Option<(usize, usize)>,
    pub recent_advancing_pawn: Option<(usize, usize)>,
    pub recent_promoting_pawn: Option<(usize, usize)>,
    pub current_turn: TeamColor,
    pub player_color: TeamColor,
    pub available_moves: HashSet<Move>,
    pub danger_squares: Vec<(usize, usize)>,
    pub is_valid_castling: [[bool; 2]; 2],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub variant: PieceVariant,
    pub color: TeamColor,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PieceVariant {
    King,
    Queen,
    Castle,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TeamColor {
    White,
    Black,
}

pub enum GameState {
    StartMenu,
    TeamSelection,
    BoardGame,
    PromoteSelection,
}

pub fn initialize_game(canvas: &WindowCanvas) -> Result<Game, Error> {
    Ok(Game {
        state: GameState::StartMenu,
        board: [[None; 8]; 8],
        texture_creator: canvas.texture_creator(),
        cache: initialize_cache(canvas)?,
    })
}

fn initialize_cache(canvas: &WindowCanvas) -> Result<Cache, Error> {
    let window_size = canvas.output_size()?;
    let window_size = (window_size.0 as f32, window_size.1 as f32);
    let board_size = (
        window_size.0 * (400.0 / window_size.0),
        window_size.1 * (400.0 / window_size.1),
    );

    let board_offset = ((window_size.0 - board_size.0) / 2.0, (board_size.1 * 0.05));
    let square_size = (board_size.0 / 8.0, board_size.1 / 8.0);

    Ok(Cache {
        window_size,
        board_size,
        board_offset,
        square_size,
        data: initialize_data(),
    })
}

pub fn initialize_data() -> GameData {
    GameData {
        focused_square: None,
        recent_advancing_pawn: None,
        recent_promoting_pawn: None,
        current_turn: TeamColor::White,
        player_color: TeamColor::White,
        available_moves: HashSet::new(),
        danger_squares: vec![],
        is_valid_castling: [[true; 2]; 2],
    }
}

pub fn init_chess_position(game: &mut Game, color: TeamColor) {
    use PieceVariant::*;

    let mut board = [[None; 8]; 8];
    let (king_queen_order, initial_rows) = match color {
        TeamColor::Black => ([King, Queen], [6, 1, 7, 0]),
        TeamColor::White => ([Queen, King], [1, 6, 0, 7]),
    };
    let init_piece_on_column = |x: usize, color: TeamColor| match x {
        0 | 7 => Some(Piece {
            variant: Castle,
            color,
        }),
        1 | 6 => Some(Piece {
            variant: Knight,
            color,
        }),
        2 | 5 => Some(Piece {
            variant: Bishop,
            color,
        }),
        3 => Some(Piece {
            variant: king_queen_order[0],
            color,
        }),
        4 => Some(Piece {
            variant: king_queen_order[1],
            color,
        }),
        _ => unreachable!("unknown column"),
    };

    for column in 0..8 {
        board[initial_rows[0]][column] = Some(Piece {
            variant: Pawn,
            color: TeamColor::Black,
        });
        board[initial_rows[1]][column] = Some(Piece {
            variant: Pawn,
            color: TeamColor::White,
        });
        board[initial_rows[2]][column] = init_piece_on_column(column, TeamColor::Black);
        board[initial_rows[3]][column] = init_piece_on_column(column, TeamColor::White);
    }

    game.board = board;
    game.cache.data.available_moves = produce::generate_moves(game);
}
