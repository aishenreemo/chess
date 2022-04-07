use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn render(canvas: &mut WindowCanvas, configuration: &Config, game: &Game) -> Result<(), Error> {
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

    super::render_canvas_background(canvas, &configuration.palette)?;
    super::render_graphical_text(canvas, game, configuration, chess_rect, "CHESS")?;
    super::render_graphical_text(canvas, game, configuration, play_rect, "Play")?;
    super::render_graphical_text(canvas, game, configuration, quit_rect, "Quit")?;

    canvas.present();
    Ok(())
}
