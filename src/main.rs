extern crate sdl2;

mod config;
mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub type Error = Box<dyn ::std::error::Error>;

fn render_graphical_button(
    canvas: &mut WindowCanvas,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    padding: f32,
    text: &str,
    game: &game::Game,
    configuration: &config::Config,
) -> Result<(), Error> {
    let button_rect = Rect::new(
        x as i32, 
        y as i32,
        width as u32,
        height as u32, 
    );
    let text_rect = Rect::new(
        (x + padding) as i32,
        (y + padding) as i32,
        (width - 2.0 * padding) as u32,
        (height - 2.0 * padding) as u32,
    );
    
    let surface = configuration.font.render(text)
        .blended(configuration.palette.default_light_color)?;

    let texture = game.texture_creator.create_texture_from_surface(&surface)?;
    
    canvas.set_draw_color(configuration.palette.default_dark_color);
    canvas.fill_rect(button_rect)?;
    canvas.copy(&texture, None, text_rect)?;
    Ok(())
}

fn render_canvas_background(
    canvas: &mut WindowCanvas,
    palette: &config::Palette,
) -> Result<(), Error> {
    canvas.set_draw_color(palette.default_background_color);
    canvas.clear();
    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    configuration: &config::Config,
    game: &game::Game,
) -> Result<(), Error> {
    use game::GameState::*;

    match game.state {
        StartMenu => {
            let window_size = configuration.window_size;
            let button_width = window_size * 0.30;
            let button_height = window_size * 0.12;

            render_canvas_background(canvas, &configuration.palette)?;
            render_graphical_button(
                canvas, 
                (window_size - button_width) / 2.0,
                (window_size - button_height) / 2.0,
                button_width,
                button_height,
                3.0,
                "Play",
                game,
                configuration
            )?;
            render_graphical_button(
                canvas, 
                (window_size - button_width) / 2.0,
                (window_size - button_height) / 2.0 + button_height * 1.2,
                button_width,
                button_height,
                3.0,
                "Quit",
                game,
                configuration
            )?;

            canvas.present();
        }
    }

    Ok(())
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
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'running,
                _ => continue,
            }
        }

        // render(&mut canvas, &configuration, &game)?;

        // 40 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 40));
    }
    Ok(())
}
