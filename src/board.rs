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
            let piece = chessboard.get_square(column, row);

            // ignore empty squares
            if piece.is_none() {
                continue;
            }

            // ignore non-ally pieces
            if piece.unwrap().color != cached.current_turn {
                continue;
            }

            let square_index = row * 8 + column;
            match piece {
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
                Some(piece) if piece.variant == PieceVariant::Pawn => output.insert(
                    square_index,
                    generate_pawn_moves(chessboard, cached, column, row),
                ),
                Some(piece) if piece.variant == PieceVariant::Knight => output.insert(
                    square_index,
                    generate_knight_moves(chessboard, cached, column, row),
                ),
                Some(piece) if piece.variant == PieceVariant::King => output.insert(
                    square_index,
                    generate_king_moves(chessboard, cached, column, row),
                ),
                _ => continue,
            };
        }
    }
    output
}

fn generate_sliding_moves(
    chessboard: &Board,
    cached: &Cache,
    column: usize,
    row: usize,
    sliding_piece_type: usize,
) -> Vec<Move> {
    let mut output = vec![];
    let direction_offsets = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    let start_index = if sliding_piece_type == 2 { 4 } else { 0 };
    let end_index = if sliding_piece_type == 1 { 4 } else { 8 };

    let mut i = start_index;
    while i < end_index {
        let mut j = 1i32;
        let offset = direction_offsets[i];
        loop {
            let target_column = column as i32 + offset.0 * j;
            let target_row = row as i32 + offset.1 * j;

            if !(0..8).contains(&target_column) || !(0..8).contains(&target_row) {
                break;
            }

            let (target_column, target_row) = (target_column as usize, target_row as usize);

            let target_square = chessboard.get_square(target_column, target_row);
            match target_square {
                Some(piece) if cached.current_turn == piece.color => break,
                Some(_piece) => {
                    output.push(Move {
                        start: (column, row),
                        target: (target_column, target_row),
                    });
                    break;
                }
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

fn generate_pawn_moves(chessboard: &Board, cached: &Cache, column: usize, row: usize) -> Vec<Move> {
    let mut pawn_legal_moves = vec![];
    let is_pawn_ally = cached.current_turn == cached.player_color;
    let start_row = if is_pawn_ally { 6 } else { 1 };

    // forward
    let pawn_forward_offset = if is_pawn_ally { -1 } else { 1 };
    let pawn_forward_limit = if start_row == row { 2 } else { 1 };
    let mut i: u32 = 1;
    while i <= pawn_forward_limit {
        let offset = pawn_forward_offset * i as i32;
        let target_row = row as i32 + offset;
        if !(0..8).contains(&target_row) {
            break;
        }

        match chessboard.get_square(column, target_row as usize) {
            Some(_piece) => break,
            None => pawn_legal_moves.push(Move {
                start: (column, row),
                target: (column, target_row as usize),
            }),
        }
        i += 1;
    }

    let pawn_eating_offsets = vec![(-1, pawn_forward_offset), (1, pawn_forward_offset)];
    for offset in pawn_eating_offsets.iter() {
        let target_row = row as i32 + offset.1;
        let target_column = column as i32 + offset.0;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);
        let target_square = chessboard.get_square(target_column, target_row);

        match target_square {
            Some(piece) if cached.current_turn != piece.color => pawn_legal_moves.push(Move {
                start: (column, row),
                target: (target_column, target_row),
            }),
            None if Some((target_column, row)) == cached.recent_advancing_pawn => pawn_legal_moves
                .push(Move {
                    start: (column, row),
                    target: (target_column, target_row),
                }),
            _ => continue,
        }
    }
    pawn_legal_moves
}

fn generate_knight_moves(
    chessboard: &Board,
    cached: &Cache,
    column: usize,
    row: usize,
) -> Vec<Move> {
    let mut knight_legal_moves: Vec<Move> = vec![];

    let knight_directions = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    for offset in knight_directions.iter() {
        let target_row = row as i32 + offset.0;
        let target_column = column as i32 + offset.1;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);
        let target_square = chessboard.get_square(target_column, target_row);

        match target_square {
            Some(piece) if cached.current_turn == piece.color => continue,
            _ => knight_legal_moves.push(Move {
                start: (column, row),
                target: (target_column, target_row),
            }),
        }
    }
    knight_legal_moves
}

fn generate_king_moves(chessboard: &Board, cached: &Cache, column: usize, row: usize) -> Vec<Move> {
    let mut king_legal_moves: Vec<Move> = vec![];
    let is_king_ally_to_player = cached.player_color == cached.current_turn;
    let king_initial_row = if is_king_ally_to_player { 7 } else { 0 };
    let is_valid_to_castling: bool = {
        let slice: Vec<&bool> = cached.is_castling_pieces_unmoved
            [if is_king_ally_to_player { 3..6 } else { 0..3 }]
        .iter()
        .collect();
        (*slice[0] || *slice[2]) && *slice[1]
    };

    let king_directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for offset in king_directions.iter() {
        let target_row = row as i32 + offset.0;
        let target_column = column as i32 + offset.1;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);

        let target_square = chessboard.get_square(target_column, target_row);

        match target_square {
            Some(piece) if cached.current_turn == piece.color => continue,
            _ => king_legal_moves.push(Move {
                start: (column, row),
                target: (target_column, target_row),
            }),
        }
    }

    if (cached.king_initial_column, king_initial_row) == (column, row) && is_valid_to_castling {
        let mut castling_side = [false, false];
        let offsets = [-1, 1];
        for (i, offset) in offsets.iter().enumerate() {
            let mut j: i32 = 0;
            loop {
                j += 1;
                let target_column = (column as i32 + offset * j) as usize;
                if !(0..8).contains(&target_column) {
                    break;
                }

                let target_square = chessboard.get_square(target_column, row);
                match target_square {
                    Some(piece)
                        if piece.variant == PieceVariant::Castle
                            && [0, 7].contains(&target_column) =>
                    {
                        castling_side[i] = true;
                        break;
                    }
                    Some(_) => break,
                    None => continue,
                }
            }
        }

        if castling_side[0] {
            king_legal_moves.push(Move {
                start: (column, row),
                target: (column - 2, row),
            });
        }
        if castling_side[1] {
            king_legal_moves.push(Move {
                start: (column, row),
                target: (column + 2, row),
            })
        }
    }

    king_legal_moves
}

pub fn get_target_squares(moves: &[Move]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for move_data in moves.iter() {
        output.push(move_data.target);
    }
    output
}

pub fn get_castling_move_data(move_data_target: (usize, usize)) -> Move {
    match move_data_target {
        (1, i) => Move {
            start: (0, i),
            target: (2, i),
        },
        (2, i) => Move {
            start: (0, i),
            target: (3, i),
        },
        (5, i) => Move {
            start: (7, i),
            target: (4, i),
        },
        (6, i) => Move {
            start: (7, i),
            target: (5, i),
        },
        (_, _) => unreachable!(),
    }
}

pub fn is_cursor_inside_promoting_selection(x: u32, y: u32) -> bool {
    let margin_top_bottom = (constants::BOARD_IN_WINDOW_SIZE - constants::SQUARE_IN_BOARD_SIZE) / 2;
    let margin_left_right = constants::SQUARE_IN_BOARD_SIZE;
    let margin_offset: u32 = 10;
    let width = constants::SQUARE_IN_BOARD_SIZE * 6 + margin_offset * 2;
    let height = constants::SQUARE_IN_BOARD_SIZE + margin_offset * 2;
    let rect_x = constants::BOARD_X_OFFSET + margin_left_right as f64 - margin_offset as f64;
    let rect_y = constants::BOARD_Y_OFFSET + margin_top_bottom as f64 - margin_offset as f64;

    rect_x < x as f64
        && rect_y < y as f64
        && (x as f64) < rect_x + width as f64
        && (y as f64) < rect_y + height as f64
}

pub fn is_move_promoting_pawn(move_data: &Move, chessboard: &Board, cached: &Cache) -> bool {
    let end_row = if cached.current_turn == cached.player_color {
        0
    } else {
        7
    };
    chessboard.get_square(move_data.target.0, move_data.target.1)
        == Some(&Piece {
            variant: PieceVariant::Pawn,
            color: cached.current_turn,
        })
        && end_row == move_data.target.1
}

pub fn is_move_advancing_pawn(move_data: &Move, chessboard: &Board) -> bool {
    let (column, row) = move_data.target;
    let target_square = chessboard.get_square(column, row);

    match target_square {
        Some(piece) if piece.variant == PieceVariant::Pawn => {
            let diff = row as i32 - move_data.start.1 as i32;
            diff.abs() == 2
        }
        _ => false,
    }
}

pub fn is_move_en_passant(move_data: &Move, chessboard: &Board, cached: &Cache) -> bool {
    let (column, row) = move_data.target;
    let target_square = chessboard.get_square(column, row);
    let pawn_forward_offset = if cached.current_turn == cached.player_color {
        -1
    } else {
        1
    };
    match target_square {
        Some(piece) if piece.variant == PieceVariant::Pawn => {
            cached.recent_advancing_pawn
                == Some((column, (row as i32 - pawn_forward_offset) as usize))
        }
        _ => false,
    }
}
pub fn is_move_castling(move_data: &Move, cached: &Cache) -> bool {
    let enemy_king = (cached.king_initial_column, 0);
    let ally_king = (cached.king_initial_column, 7);
    let is_target = |target: &(usize, usize)| {
        move_data.target == (target.0 + 2, target.1) || move_data.target == (target.0 - 2, target.1)
    };
    (move_data.start == enemy_king && is_target(&enemy_king))
        || (move_data.start == ally_king && is_target(&ally_king))
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
    (*chessboard).pieces[move_data.target.1][move_data.target.0] =
        chessboard.pieces[move_data.start.1][move_data.start.0].take();
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
    for (row, squares) in board.pieces.into_iter().enumerate() {
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

            if let Some(ref p) = square {
                piece::render_graphical_piece(canvas, pieces_texture, p, x, y)?
            }
        }
    }

    Ok(())
}

pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn init() -> Self {
        let pieces = [[None; 8]; 8];

        Self { pieces }
    }

    pub fn color(teamcolor: &PieceColor) -> Self {
        let mut pieces = [[None; 8]; 8];

        let piece_exception_variant = match *teamcolor {
            PieceColor::Black => [PieceVariant::King, PieceVariant::Queen],
            PieceColor::White => [PieceVariant::Queen, PieceVariant::King],
        };

        // initial rows
        let rows = match *teamcolor {
            PieceColor::White => [1, 6, 0, 7],
            PieceColor::Black => [6, 1, 7, 0],
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
                variant: piece_exception_variant[0],
                color,
            }),
            4 => Some(Piece {
                variant: piece_exception_variant[1],
                color,
            }),
            _ => unreachable!(),
        };

        // for each file
        for column in 0..8 {
            // everything in rank 1 and 6 will be a pawn
            pieces[rows[0]][column] = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::Black,
            });
            pieces[rows[1]][column] = Some(Piece {
                variant: PieceVariant::Pawn,
                color: PieceColor::White,
            });

            // pieces on rank 7 and 0
            pieces[rows[2]][column] = handle_color(column, PieceColor::Black);
            pieces[rows[3]][column] = handle_color(column, PieceColor::White);
        }

        Self { pieces }
    }

    pub fn get_square(&self, column: usize, row: usize) -> Option<&Piece> {
        self.pieces.get(row)?.get(column)?.as_ref()
    }
}

#[derive(Debug)]
pub struct Move {
    pub start: (usize, usize),
    pub target: (usize, usize),
}
