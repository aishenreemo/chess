use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn render(canvas: &mut WindowCanvas, configuration: &Config, game: &Game) -> Result<(), Error> {
    let default_window_size = configuration.window_size;
    let window_size = canvas.output_size()?;
    let window_size = (window_size.0 as f32, window_size.1 as f32);
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
    let black_rect = Rect::new(
        (board_x_offset + (board_size / 2.0) + padding) as i32,
        (board_y_offset + padding) as i32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
        ((board_size / 2.0) - 2.0 * padding) as u32,
    );
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.fill_rect(white_rect)?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.fill_rect(black_rect)?;
    canvas.present();
    Ok(())
}
