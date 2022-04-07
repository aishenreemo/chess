pub mod config;
mod display;
pub mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use display::render;

pub type Error = Box<dyn ::std::error::Error>;

enum Command {
    Quit,
    Idle,
}

fn handle_keyboard_event(keycode: Option<Keycode>) -> Command {
    match keycode {
        Some(Keycode::Escape) => Command::Quit,
        _ => Command::Idle,
    }
}

fn handle_event(event: Event) -> Command {
    match event {
        Event::KeyDown { keycode, .. } => handle_keyboard_event(keycode),
        Event::Quit { .. } => Command::Quit,
        _ => Command::Idle,
    }
}

fn main() -> Result<(), Error> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init()?;

    let configuration = config::initialize_config(&ttf_context)?;
    let window_size = configuration.window_size as u32;
    let window = video_subsystem
        .window("chess", window_size, window_size)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let game = game::initialize_game(&canvas);

    render(&mut canvas, &configuration, &game)?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match handle_event(event) {
                Command::Quit => break 'running,
                Command::Idle => (),
            }
        }

        // render(&mut canvas, &configuration, &game)?;

        // 40 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 40));
    }
    Ok(())
}
