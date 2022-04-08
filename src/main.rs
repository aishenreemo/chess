pub mod config;
pub mod game;

mod display;
mod listener;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use game::GameState;
use game::TeamColor;

pub type Error = Box<dyn ::std::error::Error>;

pub struct Textures<'a> {
    pub pieces: Texture<'a>,
}

pub enum Command {
    SelectTeam(TeamColor),
    ExitGame,
    Play,
    Quit,
    Idle,
}

fn main() -> Result<(), Error> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init()?;

    let configuration = config::initialize_config(&ttf_context)?;
    let window_size = configuration.window_size;
    let window = video_subsystem
        .window("chess", window_size.0 as u32, window_size.1 as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut game = game::initialize_game(&canvas)?;
    let texture_creator = canvas.texture_creator();
    let textures = Textures {
        pieces: texture_creator.load_texture("assets/chess_pieces.png")?,
    };

    // render start menu
    display::render(&mut canvas, &configuration, &game, &textures)?;

    // event loop
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match listener::handle_event(event, &game) {
                Command::Quit => break 'running,
                Command::Play => game.state = GameState::TeamSelection,
                Command::ExitGame => game.state = GameState::StartMenu,
                Command::SelectTeam(color) => {
                    game.state = GameState::BoardGame;
                    game::init_chess_position(&mut game, color);
                }
                Command::Idle => (),
            }
        }

        display::render(&mut canvas, &configuration, &game, &textures)?;

        // 40 loops per second
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 40));
    }

    // process exit
    Ok(())
}
