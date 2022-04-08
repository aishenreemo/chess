use crate::game::Game;
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

pub fn into_relative_position(game: &Game, pos: (i32, i32)) -> (usize, usize) {
    (
        ((pos.0 as f32 - game.cache.board_offset.0) / game.cache.square_size.0 as f32) as usize,
        ((pos.1 as f32 - game.cache.board_offset.1) / game.cache.square_size.1 as f32) as usize,
    )
}

fn handle_mouse_on_board(game: &Game, pos: (i32, i32)) -> Vec<Command> {
    let (column, row) = into_relative_position(game, pos);
    println!("{column} - {row}");
    vec![Command::Idle]
}

fn handle_mousedown(game: &Game, mouse_btn: MouseButton, pos: (i32, i32)) -> Vec<Command> {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_board(game, pos) => handle_mouse_on_board(game, pos),
        _ => vec![Command::Idle],
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
