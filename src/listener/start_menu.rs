use crate::game::Game;
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

fn is_cursor_inside_quit_rect(game: &Game, pos: (i32, i32)) -> bool {
    let window_size = game.cache.window_size;
    let button_width = window_size.0 * 0.30;
    let button_height = window_size.1 * 0.12;

    let quit_rect = Rect::new(
        ((window_size.0 - button_width) / 2.0) as i32,
        ((window_size.1 - button_height) / 2.0 + button_height * 1.2) as i32,
        button_width as u32,
        button_height as u32,
    );

    quit_rect.contains_point(pos)
}

fn is_cursor_inside_play_rect(game: &Game, pos: (i32, i32)) -> bool {
    let window_size = game.cache.window_size;
    let button_width = window_size.0 * 0.30;
    let button_height = window_size.1 * 0.12;

    let play_rect = Rect::new(
        ((window_size.0 - button_width) / 2.0) as i32,
        ((window_size.1 - button_height) / 2.0) as i32,
        button_width as u32,
        button_height as u32,
    );

    play_rect.contains_point(pos)
}

fn handle_mousedown(game: &Game, mouse_btn: MouseButton, pos: (i32, i32)) -> Command {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_quit_rect(game, pos) => Command::Quit,
        MouseButton::Left if is_cursor_inside_play_rect(game, pos) => Command::Play,
        _ => Command::Idle,
    }
}

fn handle_keydown(keycode: Option<Keycode>) -> Command {
    match keycode {
        Some(Keycode::Escape) => Command::Quit,
        _ => Command::Idle,
    }
}

pub fn handle_event(event: Event, game: &Game) -> Command {
    match event {
        Event::Quit { .. } => Command::Quit,
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mousedown(game, mouse_btn, (x, y)),
        _ => Command::Idle,
    }
}
