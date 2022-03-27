use crate::cache::Cache;
use crate::constants;
use crate::piece::{self, Piece, PieceColor, PieceVariant};
use crate::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;

pub fn generate_moves(chessboard: &Board, cached: &Cache) -> HashMap<usize, Vec<Move>> {
    let mut output = HashMap::new();
    for column in 0..8 {
        for row in 0..8 {
            let square = chessboard.get_square(column, row).unwrap();

            // ignore empty squares
            if square.piece.is_none() {
                continue;
            }

            // ignore non-ally pieces
            if square.piece.unwrap().color != cached.current_turn {
                continue;
            }

            let square_index = row * 8 + column;
            match square.piece {
                Some(piece) if piece.variant == PieceVariant::Queen => output.insert(
                    square_index,
                    generate_sliding_moves(chessboard, cached, column, row, 0),
                ),
                Some(piece) if piece.variant == PieceVariant::Castle => output.insert(
                    square_index,
                    generate_sliding_moves(chessboard, cached, column, row, 1),
                ),
                Some(piece) if piece.variant == PieceVariant::Bishop => output.insert(
                    square_index,
                    generate_sliding_moves(chessboard, cached, column, row, 2),
                ),
                Some(piece) if piece.variant == PieceVariant::Pawn => {
                    let mut pawn_legal_moves = vec![];
                    let ally_color = &square.piece.unwrap().color;
                    let is_direction_forward = cached.current_turn == cached.player_color;
                    let start_row = if is_direction_forward { 6 } else { 1 };
                    let start_square_index = row * 8 + column;
                    let num_directions = if start_row == row { 2 } else { 1 };

                    let mut i = 0;
                    while i < num_directions {
                        let offset = (i as i32 + 1) * if is_direction_forward { -8 } else { 8 };
                        let target_square_index = (start_square_index as i32 + offset) as usize;
                        let target_column = target_square_index % 8;
                        let target_row = target_square_index / 8;

                        let target_square =
                            chessboard.get_square(target_column, target_row).unwrap();
                        match target_square.piece {
                            // if it found a piece
                            Some(_) => break,
                            // if it's an empty square
                            None => pawn_legal_moves.push(Move {
                                start: (column, row),
                                target: (target_column, target_row),
                            }),
                        }

                        i += 1;
                    }

                    let pawn_eating_directions = if is_direction_forward {
                        [-9, -7]
                    } else {
                        [7, 9]
                    };
                    for (index, offset) in pawn_eating_directions.iter().enumerate() {
                        // the pawn is at left/right edge, therefore the left/right square doesnt exist
                        if column == 0 && index == 0 || column == 7 && index == 1 {
                            continue;
                        }

                        let target_square_index = (start_square_index as i32 + offset) as usize;
                        let target_column = target_square_index % 8;
                        let target_row = target_square_index / 8;

                        let target_square =
                            chessboard.get_square(target_column, target_row).unwrap();
                        match target_square.piece {
                            // if it found an enemy piece
                            Some(piece) if &piece.color != ally_color => {
                                pawn_legal_moves.push(Move {
                                    start: (column, row),
                                    target: (target_column, target_row),
                                })
                            }
                            _ => continue,
                        }
                    }
                    output.insert(start_square_index, pawn_legal_moves)
                }
                _ => continue,
            };
        }
    }
    output
}

pub fn generate_sliding_moves(
    chessboard: &Board,
    cached: &Cache,
    column: usize,
    row: usize,
    sliding_piece_type: usize,
) -> Vec<Move> {
    let mut output = vec![];
    let ally_color = &chessboard
        .get_square(column, row)
        .unwrap()
        .piece
        .unwrap()
        .color;
    let direction_offsets = [-8, 8, -1, 1, -9, -7, 7, 9];
    let start_index = if sliding_piece_type == 2 { 4 } else { 0 };
    let end_index = if sliding_piece_type == 1 { 4 } else { 8 };

    let start_square_index = row * 8 + column;

    let mut i = start_index;
    while i < end_index {
        let offset = direction_offsets[i];
        let mut j = 0;
        while j < cached.num_squares_to_edge[start_square_index][i] {
            let target_square_index =
                (start_square_index as i32 + offset * (j as i32 + 1)) as usize;
            let target_column = target_square_index % 8;
            let target_row = target_square_index / 8;

            let target_square = chessboard.get_square(target_column, target_row).unwrap();
            match target_square.piece {
                // if it's an ally square
                Some(piece) if ally_color == &piece.color => break,
                // if it's non-ally square
                Some(_) => {
                    output.push(Move {
                        start: (column, row),
                        target: (target_column, target_row),
                    });
                    break;
                }
                // if it's an empty square
                None => output.push(Move {
                    start: (column, row),
                    target: (target_column, target_row),
                }),
            }
            j += 1;
        }
        i += 1;
    }

    output
}

pub fn get_target_squares(moves: &[Move]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for move_data in moves.iter() {
        output.push(move_data.target);
    }
    output
}

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

pub fn move_board_piece(chessboard: &mut Board, move_data: &Move) {
    chessboard
        .get_square_mut(move_data.target.0, move_data.target.1)
        .unwrap()
        .piece = chessboard
        .get_square_mut(move_data.start.0, move_data.start.1)
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

            let cell_rect = Rect::new(
                x as i32,
                y as i32,
                constants::SQUARE_IN_BOARD_SIZE,
                constants::SQUARE_IN_BOARD_SIZE,
            );
            let cell_rect_focused = Rect::new(
                x as i32 + 1,
                y as i32 + 1,
                constants::SQUARE_IN_BOARD_SIZE - 2,
                constants::SQUARE_IN_BOARD_SIZE - 2,
            );

            if column % 2 != 0 && row % 2 == 0 || column % 2 == 0 && row % 2 != 0 {
                canvas.set_draw_color(Color::RGB(122, 95, 71));
                canvas.fill_rect(cell_rect)?;
            }

            if cached.target_squares.contains(&(column, row)) {
                canvas.set_draw_color(Color::RGB(222, 194, 133));
                canvas.fill_rect(cell_rect_focused)?;
            } else if cached.focused_square == Some((column, row)) {
                canvas.set_draw_color(Color::RGB(104, 113, 143));
                canvas.fill_rect(cell_rect_focused)?;
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
        let squares = [[Square {
            piece: None,
            is_focused: false,
        }; 8]; 8];

        Self { squares }
    }

    pub fn color(teamcolor: &PieceColor) -> Self {
        let mut squares = [[Square {
            piece: None,
            is_focused: false,
        }; 8]; 8];

        let piece_exception_variant = match *teamcolor {
            PieceColor::Black => (PieceVariant::King, PieceVariant::Queen),
            PieceColor::White => (PieceVariant::Queen, PieceVariant::King),
        };
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
                variant: piece_exception_variant.0,
                color,
            }),
            4 => Some(Piece {
                variant: piece_exception_variant.1,
                color,
            }),
            _ => unreachable!(),
        };

        let rows = match *teamcolor {
            PieceColor::White => (1, 6, 0, 7),
            PieceColor::Black => (6, 1, 7, 0),
        };

        // for each file
        for column in 0..8 {
            // everything in rank 1 or 6 will be a black pawn
            squares[rows.0][column].piece = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::Black,
            });
            // everything in rank 6 or 1 will be a white pawn
            squares[rows.1][column].piece = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::White,
            });

            squares[rows.2][column].piece = handle_color(column, PieceColor::Black);
            squares[rows.3][column].piece = handle_color(column, PieceColor::White);
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

#[derive(Debug)]
pub struct Move {
    pub start: (usize, usize),
    pub target: (usize, usize),
}
