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
            Some(_piece) => moves.insert(Move {
                variant: MoveType::Capture,
                from,
                to: (target_column, target_row),
            }),
            _ => moves.insert(Move {
                variant: MoveType::NonCapture,
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
                Knight => generate_knight_moves(game, (column, row)),
                Pawn => generate_pawn_moves(game, (column, row)),
                _ => continue,
            };

            moves.extend(piece_moves)
        }
    }

    moves
}
