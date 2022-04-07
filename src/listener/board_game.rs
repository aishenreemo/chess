use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn handle_keydown(keycode: Option<Keycode>) -> Command {
    match keycode {
        Some(Keycode::Escape) => Command::ExitGame,
        _ => Command::Idle,
    }
}

pub fn handle_event(event: Event) -> Command {
    match event {
        Event::Quit { .. } => Command::Quit,
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        _ => Command::Idle,
    }
}
