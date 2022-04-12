use crate::game::{Game, Piece, PieceVariant};
use crate::produce::{Move, MoveType};
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

fn is_cursor_inside_board(game: &Game, pos: (i32, i32)) -> bool {
    let board_rect = Rect::new(
        game.cache.board_offset.0 as i32,
        game.cache.board_offset.1 as i32,
        game.cache.board_size.0 as u32,
        game.cache.board_size.1 as u32,
    );

    board_rect.contains_point(pos)
}

fn is_piece_ally(game: &Game, piece: &Piece) -> bool {
    piece.color == game.cache.data.current_turn
}

fn has_focused_square(game: &Game) -> bool {
    game.cache.data.focused_square.is_some()
}

fn into_relative_position(game: &Game, pos: (i32, i32)) -> (usize, usize) {
    (
        ((pos.0 as f32 - game.cache.board_offset.0) / game.cache.square_size.0 as f32) as usize,
        ((pos.1 as f32 - game.cache.board_offset.1) / game.cache.square_size.1 as f32) as usize,
    )
}

fn is_move_promotion(game: &Game, row: usize) -> bool {
    let focused_square = game.cache.data.focused_square.unwrap_or((8, 8));
    let from = game.get_square(focused_square.0, focused_square.1);
    let is_ally = game.cache.data.current_turn == game.cache.data.player_color;
    let end_row = if is_ally { 0 } else { 7 };
    match from {
        Some(piece) if row == end_row => {
            is_piece_ally(game, piece) && piece.variant == PieceVariant::Pawn
        }
        _ => false,
    }
}

fn is_move_advancing_pawn(game: &Game, row: usize) -> bool {
    let focused_square = game.cache.data.focused_square.unwrap_or((8, 8));
    let from = game.get_square(focused_square.0, focused_square.1);
    let is_ally = game.cache.data.current_turn == game.cache.data.player_color;
    let start_row = if is_ally { 6 } else { 1 };
    let end_row = if is_ally { 4 } else { 3 };
    match from {
        Some(piece) if row == end_row && focused_square.1 == start_row => {
            is_piece_ally(game, piece) && piece.variant == PieceVariant::Pawn
        }
        _ => false,
    }
}

fn is_move_enpassant(game: &Game, column: usize) -> bool {
    let focused_square = game.cache.data.focused_square.unwrap_or((8, 8));
    let from = game.get_square(focused_square.0, focused_square.1);
    match from {
        Some(piece)
            if Some((column, focused_square.1)) == game.cache.data.recent_advancing_pawn =>
        {
            is_piece_ally(game, piece) && piece.variant == PieceVariant::Pawn
        }
        _ => false,
    }
}

fn handle_mouse_on_board(game: &Game, pos: (i32, i32)) -> Vec<Command> {
    let (column, row) = into_relative_position(game, pos);

    match game.get_square(column, row) {
        _ if is_move_promotion(game, row) => vec![Command::Promote((column, row))],
        _ if is_move_advancing_pawn(game, row) => vec![
            Command::Move(Move {
                variant: MoveType::AdvancePawn,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            }),
            Command::Unfocus,
            Command::ChangeTurn,
        ],
        _ if is_move_enpassant(game, column) => vec![
            Command::Move(Move {
                variant: MoveType::EnPassant,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            }),
            Command::Unfocus,
            Command::ChangeTurn,
        ],
        Some(piece) if !is_piece_ally(game, piece) && has_focused_square(game) => vec![
            Command::Unfocus,
            Command::Move(Move {
                variant: MoveType::Capture,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            }),
            Command::ChangeTurn,
        ],
        Some(piece) if !is_piece_ally(game, piece) && !has_focused_square(game) => {
            vec![Command::Unfocus]
        }
        Some(piece) if is_piece_ally(game, piece) => vec![Command::Focus(column, row)],
        None if has_focused_square(game) => vec![
            Command::Unfocus,
            Command::Move(Move {
                variant: MoveType::NonCapture,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            }),
            Command::ChangeTurn,
        ],
        _ => vec![],
    }
}

fn handle_mousedown(game: &Game, mouse_btn: MouseButton, pos: (i32, i32)) -> Vec<Command> {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_board(game, pos) => handle_mouse_on_board(game, pos),
        MouseButton::Left => vec![Command::Unfocus],
        _ => vec![],
    }
}

fn handle_keydown(keycode: Option<Keycode>) -> Vec<Command> {
    match keycode {
        Some(Keycode::Escape) => vec![Command::ExitGame],
        _ => vec![Command::Idle],
    }
}

pub fn handle_event(event: Event, game: &Game) -> Vec<Command> {
    match event {
        Event::Quit { .. } => vec![Command::Quit],
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mousedown(game, mouse_btn, (x, y)),
        _ => vec![Command::Idle],
    }
}
