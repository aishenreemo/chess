use crate::game::{Game, TeamColor};
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

fn is_cursor_inside_white_rect(game: &Game, pos: (i32, i32)) -> bool {
    let padding = 10.0;
    let white_rect = Rect::new(
        (game.cache.board_offset.0 + padding) as i32,
        ((game.cache.window_size.1 - game.cache.board_size.1) + padding) as i32,
        ((game.cache.board_size.0 / 2.0) - 2.0 * padding) as u32,
        ((game.cache.board_size.1 / 2.0) - 2.0 * padding) as u32,
    );

    white_rect.contains_point(pos)
}

fn is_cursor_inside_black_rect(game: &Game, pos: (i32, i32)) -> bool {
    let padding = 10.0;
    let black_rect = Rect::new(
        (game.cache.board_offset.0 + (game.cache.board_size.0 / 2.0) + padding) as i32,
        ((game.cache.window_size.1 - game.cache.board_size.1) + padding) as i32,
        ((game.cache.board_size.0 / 2.0) - 2.0 * padding) as u32,
        ((game.cache.board_size.1 / 2.0) - 2.0 * padding) as u32,
    );

    black_rect.contains_point(pos)
}

fn handle_mousedown(game: &Game, mouse_btn: MouseButton, pos: (i32, i32)) -> Vec<Command> {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_white_rect(game, pos) => {
            vec![Command::SelectTeam(TeamColor::White)]
        }
        MouseButton::Left if is_cursor_inside_black_rect(game, pos) => {
            vec![Command::SelectTeam(TeamColor::Black)]
        }
        _ => vec![Command::Idle],
    }
}

fn handle_keydown(_keycode: Option<Keycode>) -> Vec<Command> {
    vec![Command::Idle]
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
