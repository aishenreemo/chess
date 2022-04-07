use crate::config::Config;
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

fn is_cursor_inside_quit_rect(configuration: &Config, x: i32, y: i32) -> bool {
    let window_size = configuration.window_size;
    let button_width = window_size * 0.30;
    let button_height = window_size * 0.12;

    let quit_rect = Rect::new(
        ((window_size - button_width) / 2.0) as i32,
        ((window_size - button_height) / 2.0 + button_height * 1.2) as i32,
        button_width as u32,
        button_height as u32,
    );

    quit_rect.contains_point((x, y))
}

fn is_cursor_inside_play_rect(configuration: &Config, x: i32, y: i32) -> bool {
    let window_size = configuration.window_size;
    let button_width = window_size * 0.30;
    let button_height = window_size * 0.12;

    let play_rect = Rect::new(
        ((window_size - button_width) / 2.0) as i32,
        ((window_size - button_height) / 2.0) as i32,
        button_width as u32,
        button_height as u32,
    );

    play_rect.contains_point((x, y))
}

fn handle_mousedown(mouse_btn: MouseButton, x: i32, y: i32, configuration: &Config) -> Command {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_quit_rect(configuration, x, y) => Command::Quit,
        MouseButton::Left if is_cursor_inside_play_rect(configuration, x, y) => Command::Play,
        _ => Command::Idle,
    }
}

fn handle_keydown(keycode: Option<Keycode>) -> Command {
    match keycode {
        Some(Keycode::Escape) => Command::Quit,
        _ => Command::Idle,
    }
}

pub fn handle_event(event: Event, configuration: &Config) -> Command {
    match event {
        Event::Quit { .. } => Command::Quit,
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mousedown(mouse_btn, x, y, configuration),
        _ => Command::Idle,
    }
}
