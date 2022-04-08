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
                _ => continue,
            };

            moves.extend(piece_moves)
        }
    }

    moves
}
