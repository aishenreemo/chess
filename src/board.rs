use crate::constants;
use crate::piece::{self, Piece, PieceColor, PieceVariant};
use crate::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub fn is_cursor_inside_board(x: u32, y: u32) -> bool {
    x > constants::BOARD_X_OFFSET as u32
        && x < constants::BOARD_X_OFFSET as u32 + constants::BOARD_IN_WINDOW_SIZE
        && y > constants::BOARD_Y_OFFSET as u32
        && y < constants::BOARD_Y_OFFSET as u32 + constants::BOARD_IN_WINDOW_SIZE
}

/// convert relative position of board (column, row) into canvas position (x, y)
pub fn into_absolute_position(column: u32, row: u32) -> (u32, u32) {
    (
        (column * constants::SQUARE_IN_BOARD_SIZE) + constants::BOARD_X_OFFSET as u32,
        (row * constants::SQUARE_IN_BOARD_SIZE) + constants::BOARD_Y_OFFSET as u32,
    )
}

/// convert absolute position of canvas (x, y) into board position (column, row)
pub fn into_relative_position(x: u32, y: u32) -> (u32, u32) {
    (
        (x - constants::BOARD_X_OFFSET as u32) / constants::SQUARE_IN_BOARD_SIZE as u32,
        (y - constants::BOARD_Y_OFFSET as u32) / constants::SQUARE_IN_BOARD_SIZE as u32,
    )
}

pub fn move_board_piece(
    chessboard: &mut Board,
    prev_column: usize,
    prev_row: usize,
    column: usize,
    row: usize,
) {
    chessboard.get_square_mut(column, row).unwrap().piece = chessboard
        .get_square_mut(prev_column, prev_row)
        .unwrap()
        .piece
        .take();
}

pub fn render_graphical_board(
    canvas: &mut WindowCanvas,
    board: &Board,
    pieces_texture: &Texture,
    cached: &crate::cache::Cache,
) -> Result<(), Error> {
    // stroke the chess board border
    let board_rect = Rect::new(
        constants::BOARD_X_OFFSET as i32,
        constants::BOARD_Y_OFFSET as i32,
        constants::BOARD_IN_WINDOW_SIZE,
        constants::BOARD_IN_WINDOW_SIZE,
    );
    canvas.set_draw_color(Color::RGB(122, 95, 71));
    canvas.draw_rect(board_rect)?;

    // render squares
    // for each row
    for (row, squares) in board.squares.into_iter().enumerate() {
        // for each column
        for (column, square) in squares.into_iter().enumerate() {
            let (x, y) = into_absolute_position(column as u32, row as u32);

            let square_rect = Rect::new(
                x as i32,
                y as i32,
                constants::SQUARE_IN_BOARD_SIZE,
                constants::SQUARE_IN_BOARD_SIZE,
            );

            if cached.focused_square == Some((column, row)) {
                canvas.set_draw_color(Color::RGB(104, 113, 143));
                canvas.fill_rect(square_rect)?;
            } else if column % 2 != 0 && row % 2 == 0 || column % 2 == 0 && row % 2 != 0 {
                canvas.set_draw_color(Color::RGB(122, 95, 71));
                canvas.fill_rect(square_rect)?;
            }

            if let Some(ref p) = square.piece {
                piece::render_graphical_piece(canvas, pieces_texture, p, x, y)?
            }
        }
    }

    Ok(())
}

pub struct Board {
    pub squares: [[Square; 8]; 8],
}

impl Board {
    pub fn init() -> Self {
        let mut squares = [[Square {
            piece: None,
            is_focused: false,
        }; 8]; 8];
        let handle_color = |x: usize, color: PieceColor| match x {
            0 | 7 => Some(Piece {
                variant: PieceVariant::Castle,
                color,
            }),
            1 | 6 => Some(Piece {
                variant: PieceVariant::Knight,
                color,
            }),
            2 | 5 => Some(Piece {
                variant: PieceVariant::Bishop,
                color,
            }),
            3 => Some(Piece {
                variant: PieceVariant::King,
                color,
            }),
            4 => Some(Piece {
                variant: PieceVariant::Queen,
                color,
            }),
            _ => unreachable!(),
        };

        // for each file
        for column in 0..8 {
            // everything in rank 1 will be a black pawn
            squares[1][column].piece = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::Black,
            });
            // everything in rank 6 will be a white pawn
            squares[6][column].piece = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::White,
            });

            squares[0][column].piece = handle_color(column, PieceColor::Black);
            squares[7][column].piece = handle_color(column, PieceColor::White);
        }

        Self { squares }
    }

    pub fn get_square(&self, column: usize, row: usize) -> Option<&Square> {
        self.squares.get(row)?.get(column)
    }

    pub fn get_square_mut(&mut self, column: usize, row: usize) -> Option<&mut Square> {
        self.squares.get_mut(row)?.get_mut(column)
    }
}

#[derive(Clone, Copy)]
pub struct Square {
    pub piece: Option<Piece>,
    pub is_focused: bool,
}
