use crate::game::{Game, PieceVariant};

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Move {
    pub variant: MoveType,
    pub from: (usize, usize),
    pub to: (usize, usize),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MoveType {
    Capture,
    NonCapture,
    Castling(usize),
    Promotion(PieceVariant),
    AdvancePawn,
    EnPassant,
}

fn generate_sliding_moves(
    game: &Game,
    from: (usize, usize),
    offset_range: (usize, usize),
) -> HashSet<Move> {
    let mut moves: HashSet<Move> = HashSet::new();
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

    for offset in &direction_offsets[offset_range.0..offset_range.1] {
        let mut i = 1;
        loop {
            let target_column = from.0 as i32 + offset.0 * i;
            let target_row = from.1 as i32 + offset.1 * i;

            if !(0..8).contains(&target_column) || !(0..8).contains(&target_row) {
                break;
            }

            let (target_column, target_row) = (target_column as usize, target_row as usize);

            let target_square = game.get_square(target_column, target_row);
            match target_square {
                Some(piece) if game.cache.data.current_turn == piece.color => break,
                Some(_) => {
                    moves.insert(Move {
                        variant: MoveType::Capture,
                        from,
                        to: (target_column, target_row),
                    });
                    break;
                }
                None => moves.insert(Move {
                    variant: MoveType::NonCapture,
                    from,
                    to: (target_column, target_row),
                }),
            };
            i += 1;
        }
    }

    moves
}

fn generate_king_moves(game: &Game, from: (usize, usize)) -> HashSet<Move> {
    let mut moves: HashSet<Move> = HashSet::new();
    let is_ally = game.cache.data.current_turn == game.cache.data.player_color;

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
        let target_row = from.1 as i32 + offset.1;
        let target_column = from.0 as i32 + offset.0;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);

        match game.get_square(target_column, target_row) {
            Some(piece) if game.cache.data.current_turn == piece.color => continue,
            square => moves.insert(Move {
                from,
                to: (target_column, target_row),
                variant: if square.is_none() {
                    MoveType::NonCapture
                } else {
                    MoveType::Capture
                },
            }),
        };
    }

    let castling_directions = [-1, 1];
    let castling_ptr = if is_ally { 0 } else { 1 };
    let mut is_valid_castling = [false, false];

    for (i, dir) in castling_directions.iter().enumerate() {
        let mut j: i32 = 0;
        loop {
            j += 1;
            let target_column = from.0 as i32 + dir * j;
            if !(0..8).contains(&target_column) {
                break;
            }

            let target_column = target_column as usize;
            match game.get_square(target_column, from.1) {
                Some(piece)
                    if piece.variant == PieceVariant::Castle && [0, 7].contains(&target_column) =>
                {
                    is_valid_castling[i] = true;
                    break;
                }
                Some(_) => break,
                None => continue,
            }
        }
    }

    if is_valid_castling[0] && game.cache.data.is_valid_castling[castling_ptr][0] {
        moves.insert(Move {
            variant: MoveType::Castling(0),
            from,
            to: (from.0 - 2, from.1),
        });
    }

    if is_valid_castling[1] && game.cache.data.is_valid_castling[castling_ptr][1] {
        moves.insert(Move {
            variant: MoveType::Castling(7),
            from,
            to: (from.0 + 2, from.1),
        });
    }

    moves
}

fn generate_knight_moves(game: &Game, from: (usize, usize)) -> HashSet<Move> {
    let mut moves: HashSet<Move> = HashSet::new();

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
        let target_row = from.1 as i32 + offset.1;
        let target_column = from.0 as i32 + offset.0;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);
        let target_square = game.get_square(target_column, target_row);

        match target_square {
            Some(piece) if game.cache.data.current_turn == piece.color => continue,
            square => moves.insert(Move {
                variant: if square.is_none() {
                    MoveType::NonCapture
                } else {
                    MoveType::Capture
                },
                from,
                to: (target_column, target_row),
            }),
        };
    }
    moves
}

fn generate_pawn_promotion_moves(
    moves: &mut HashSet<Move>,
    from: (usize, usize),
    to: (usize, usize),
) -> bool {
    use PieceVariant::*;
    for piece in [Queen, Castle, Knight, Bishop].into_iter() {
        moves.insert(Move {
            variant: MoveType::Promotion(piece),
            from,
            to,
        });
    }
    true
}

fn generate_pawn_moves(game: &Game, from: (usize, usize)) -> HashSet<Move> {
    let mut moves = HashSet::new();
    let is_ally = game.cache.data.current_turn == game.cache.data.player_color;
    let start_row = if !is_ally { 1 } else { 6 };
    let pawn_end_row = if !is_ally { 7 } else { 0 };
    let pawn_forward_offset = if !is_ally { 1 } else { -1 };
    let pawn_forward_limit = if start_row == from.1 { 2 } else { 1 };

    let mut i: u32 = 1;
    while i <= pawn_forward_limit {
        let offset = pawn_forward_offset * i as i32;
        let target_row = from.1 as i32 + offset;

        // if pawn is out of bounds
        if !(0..8).contains(&target_row) {
            break;
        }

        let target_row = target_row as usize;
        let target_square = game.get_square(from.0, target_row);
        match target_square {
            Some(_) => break,
            None if pawn_end_row == target_row => {
                generate_pawn_promotion_moves(&mut moves, from, (from.0, target_row))
            }
            None if i == 2 => moves.insert(Move {
                variant: MoveType::AdvancePawn,
                from,
                to: (from.0, target_row),
            }),
            None => moves.insert(Move {
                variant: MoveType::NonCapture,
                from,
                to: (from.0, target_row),
            }),
        };
        i += 1;
    }

    for offset in [(-1, pawn_forward_offset), (1, pawn_forward_offset)].into_iter() {
        let target_row = from.1 as i32 + offset.1;
        let target_column = from.0 as i32 + offset.0;

        if !(0..8).contains(&target_row) || !(0..8).contains(&target_column) {
            continue;
        }

        let (target_column, target_row) = (target_column as usize, target_row as usize);
        let is_move_enpassant =
            Some((target_column, from.1)) == game.cache.data.recent_advancing_pawn;
        let target_square = game.get_square(target_column, target_row);

        match target_square {
            Some(piece) if game.cache.data.current_turn != piece.color => moves.insert(Move {
                variant: MoveType::Capture,
                from,
                to: (target_column, target_row),
            }),
            None if is_move_enpassant => moves.insert(Move {
                variant: MoveType::EnPassant,
                from,
                to: (target_column, target_row),
            }),
            _ => continue,
        };
    }

    moves
}

pub fn generate_moves(game: &Game) -> HashSet<Move> {
    use PieceVariant::*;

    let mut moves: HashSet<Move> = HashSet::new();

    for row in 0..8 {
        for column in 0..8 {
            let square = game.get_square(column, row);

            if square.is_none() {
                continue;
            }

            let piece = square.unwrap();

            if piece.color != game.cache.data.current_turn {
                continue;
            }

            let piece_moves = match piece.variant {
                Queen => generate_sliding_moves(game, (column, row), (0, 8)),
                Castle => generate_sliding_moves(game, (column, row), (0, 4)),
                Bishop => generate_sliding_moves(game, (column, row), (4, 8)),
                King => generate_king_moves(game, (column, row)),
                Knight => generate_knight_moves(game, (column, row)),
                Pawn => generate_pawn_moves(game, (column, row)),
            };

            moves.extend(piece_moves)
        }
    }

    moves
}
