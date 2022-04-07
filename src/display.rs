use crate::config::{Config, Palette};
use crate::game::{Game, GameState};
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

fn render_canvas_background(canvas: &mut WindowCanvas, palette: &Palette) -> Result<(), Error> {
    canvas.set_draw_color(palette.default_background_color);
    canvas.clear();
    Ok(())
}

fn render_graphical_text(
    canvas: &mut WindowCanvas,
    game: &Game,
    configuration: &Config,
    text_rect: Rect,
    text: &str,
) -> Result<(), Error> {
    let surface = configuration
        .font
        .render(text)
        .blended(configuration.palette.default_dark_color)?;

    let texture = game.texture_creator.create_texture_from_surface(&surface)?;

    canvas.copy(&texture, None, text_rect)?;
    Ok(())
}

fn render_start_menu(
    canvas: &mut WindowCanvas,
    configuration: &Config,
    game: &Game,
) -> Result<(), Error> {
    let window_size = configuration.window_size;
    let button_width = window_size * 0.30;
    let button_height = window_size * 0.12;

    let chess_rect = Rect::new(
        ((window_size - button_width * 2.0) / 2.0) as i32,
        ((window_size - button_height * 2.0) / 2.0 - button_height * 2.0) as i32,
        (button_width * 2.0) as u32,
        (button_height * 1.5) as u32,
    );
    let play_rect = Rect::new(
        ((window_size - button_width) / 2.0) as i32,
        ((window_size - button_height) / 2.0) as i32,
        button_width as u32,
        button_height as u32,
    );
    let quit_rect = Rect::new(
        ((window_size - button_width) / 2.0) as i32,
        ((window_size - button_height) / 2.0 + button_height * 1.2) as i32,
        button_width as u32,
        button_height as u32,
    );
    render_canvas_background(canvas, &configuration.palette)?;
    render_graphical_text(canvas, game, configuration, chess_rect, "CHESS")?;
    render_graphical_text(canvas, game, configuration, play_rect, "Play")?;
    render_graphical_text(canvas, game, configuration, quit_rect, "Quit")?;

    canvas.present();
    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, configuration: &Config, game: &Game) -> Result<(), Error> {
    use GameState::*;
    match game.state {
        StartMenu => render_start_menu(canvas, configuration, game),
    }
}
