use crate::Error;

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Game {
    pub state: GameState,
    pub texture_creator: TextureCreator<WindowContext>,
    pub board: [[Option<Piece>; 8]; 8],
    pub cache: Cache,
}

pub struct Cache {
    pub window_size: (f32, f32),
    pub board_size: (f32, f32),
    pub board_offset: (f32, f32),
    pub square_size: (f32, f32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub variant: PieceVariant,
    pub color: TeamColor,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
}

pub fn initialize_game(canvas: &WindowCanvas) -> Result<Game, Error> {
    Ok(Game {
        state: GameState::StartMenu,
        board: [[None; 8]; 8],
        texture_creator: canvas.texture_creator(),
        cache: initialize_cache(canvas)?,
    })
}

pub fn initialize_cache(canvas: &WindowCanvas) -> Result<Cache, Error> {
    let window_size = canvas.output_size()?;
    let window_size = (window_size.0 as f32, window_size.1 as f32);
    let board_size = (
        window_size.0 * (400.0 / window_size.0),
        window_size.1 * (400.0 / window_size.1),
    );
    println!("{board_size:?}");
    let board_offset = ((window_size.0 - board_size.0) / 2.0, (board_size.1 * 0.05));
    let square_size = (board_size.0 / 8.0, board_size.1 / 8.0);

    Ok(Cache {
        window_size,
        board_size,
        board_offset,
        square_size,
    })
}

pub fn init_chess_position(game: &mut Game, color: TeamColor) {
    use PieceVariant::*;

    let mut board = [[None; 8]; 8];
    let (king_queen_order, initial_rows) = match color {
        TeamColor::White => ([King, Queen], [6, 1, 7, 0]),
        TeamColor::Black => ([Queen, King], [1, 6, 0, 7]),
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
}
