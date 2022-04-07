use crate::config::Config;
use crate::game::TeamColor;
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

fn is_cursor_inside_white_rect(
    configuration: &Config,
    x: i32,
    y: i32,
    canvas: &WindowCanvas,
) -> bool {
    let default_window_size = configuration.window_size;
    let window_size = canvas
        .output_size()
        .ok()
        .unwrap_or((default_window_size as u32, default_window_size as u32));
    let window_size = (window_size.0 as f32, window_size.1 as f32);

    let board_size = window_size.0 * (400.0 / default_window_size);
    let board_x_offset = window_size.0 * (56.0 / default_window_size);
    let board_y_offset = (window_size.1 - board_size / 2.0) / 2.0;
    let padding = 10.0;

    let white_rect = Rect::new(
        (board_x_offset + padding) as i32,
        (board_y_offset + padding) as i32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
    );

    white_rect.contains_point((x, y))
}

fn is_cursor_inside_black_rect(
    configuration: &Config,
    x: i32,
    y: i32,
    canvas: &WindowCanvas,
) -> bool {
    let default_window_size = configuration.window_size;
    let window_size = canvas
        .output_size()
        .ok()
        .unwrap_or((default_window_size as u32, default_window_size as u32));
    let window_size = (window_size.0 as f32, window_size.1 as f32);

    let board_size = window_size.0 * (400.0 / default_window_size);
    let board_x_offset = window_size.0 * (56.0 / default_window_size);
    let board_y_offset = (window_size.1 - board_size / 2.0) / 2.0;
    let padding = 10.0;

    let black_rect = Rect::new(
        (board_x_offset + (board_size / 2.0) + padding) as i32,
        (board_y_offset + padding) as i32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
    );

    black_rect.contains_point((x, y))
}

fn handle_mousedown(
    mouse_btn: MouseButton,
    x: i32,
    y: i32,
    configuration: &Config,
    canvas: &WindowCanvas,
) -> Command {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_white_rect(configuration, x, y, canvas) => {
            Command::SelectTeam(TeamColor::Black)
        }
        MouseButton::Left if is_cursor_inside_black_rect(configuration, x, y, canvas) => {
            Command::SelectTeam(TeamColor::White)
        }
        _ => Command::Idle,
    }
}

fn handle_keydown(_keycode: Option<Keycode>) -> Command {
    Command::Idle
}

pub fn handle_event(event: Event, configuration: &Config, canvas: &WindowCanvas) -> Command {
    match event {
        Event::Quit { .. } => Command::Quit,
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mousedown(mouse_btn, x, y, configuration, canvas),
        _ => Command::Idle,
    }
}
