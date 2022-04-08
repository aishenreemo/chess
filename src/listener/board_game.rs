use crate::game::{Game, Piece};
use crate::{Command, MoveType};

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

fn handle_mouse_on_board(game: &Game, pos: (i32, i32)) -> Vec<Command> {
    let (column, row) = into_relative_position(game, pos);

    match game.get_square(column, row) {
        Some(piece) if !is_piece_ally(game, piece) && has_focused_square(game) => vec![
            Command::ChangeTurn,
            Command::Unfocus,
            Command::Move {
                variant: MoveType::Capture,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            },
        ],
        Some(piece) if !is_piece_ally(game, piece) && !has_focused_square(game) => {
            vec![Command::Unfocus]
        }
        Some(piece) if is_piece_ally(game, piece) => vec![Command::Focus(column, row)],
        None if has_focused_square(game) => vec![
            Command::ChangeTurn,
            Command::Unfocus,
            Command::Move {
                variant: MoveType::NonCapture,
                from: game.cache.data.focused_square.unwrap(),
                to: (column, row),
            },
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
