use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn render(canvas: &mut WindowCanvas, configuration: &Config, game: &Game) -> Result<(), Error> {
    let window_size = game.cache.window_size;
    let text_width = window_size.0 * 0.30;
    let text_height = window_size.1 * 0.12;

    let text_rect = Rect::new(
        ((window_size.0 - text_width) / 2.0) as i32,
        (window_size.1 * 0.05) as i32,
        text_width as u32,
        text_height as u32,
    );

    super::render_canvas_background(canvas, &configuration.palette)?;
    super::render_graphical_text(canvas, game, configuration, text_rect, "Select Team")?;

    let padding = 10.0;
    let white_rect = Rect::new(
        (game.cache.board_offset.0 + padding) as i32,
        ((game.cache.window_size.1 - game.cache.board_size.1) + padding) as i32,
        ((game.cache.board_size.0 / 2.0) - 2.0 * padding) as u32,
        ((game.cache.board_size.1 / 2.0) - 2.0 * padding) as u32,
    );
    let black_rect = Rect::new(
        (game.cache.board_offset.0 + (game.cache.board_size.0 / 2.0) + padding) as i32,
        ((game.cache.window_size.1 - game.cache.board_size.1) + padding) as i32,
        ((game.cache.board_size.0 / 2.0) - 2.0 * padding) as u32,
        ((game.cache.board_size.1 / 2.0) - 2.0 * padding) as u32,
    );

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.fill_rect(white_rect)?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.fill_rect(black_rect)?;
    canvas.present();
    Ok(())
}
