use crate::config::Config;
use crate::game::{Game, Piece, PieceVariant};
use crate::Error;
use crate::Textures;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn render(
    canvas: &mut WindowCanvas,
    configuration: &Config,
    game: &Game,
    textures: &Textures,
) -> Result<(), Error> {
    use PieceVariant::*;
    let margin_top_bottom = (game.cache.window_size.1 - game.cache.square_size.1) / 2.0;
    let margin_left_right = game.cache.square_size.0;
    let margin_offset: u32 = 10;

    let main_rect = Rect::new(
        (game.cache.board_offset.0 + margin_left_right - margin_offset as f32) as i32,
        (game.cache.board_offset.1 + margin_top_bottom - margin_offset as f32) as i32,
        game.cache.square_size.0 as u32 * 6 + margin_offset * 2,
        game.cache.square_size.1 as u32 + margin_offset * 2,
    );
    canvas.set_draw_color(configuration.palette.default_light_color);
    canvas.fill_rect(main_rect)?;
    canvas.set_draw_color(configuration.palette.default_dark_color);
    canvas.draw_rect(main_rect)?;

    let constant = ((game.cache.square_size.0 as u32 * 6 + margin_offset * 2) / 5) as i32;

    for (i, piece_variant) in [Queen, Castle, Knight, Bishop].into_iter().enumerate() {
        let x = main_rect.x() + ((i as i32 + 1) * constant) - (game.cache.square_size.0 as i32 / 2);
        let y = main_rect.y() + margin_offset as i32;
        let piece = Piece {
            variant: piece_variant,
            color: game.cache.data.current_turn,
        };
        super::board_game::render_graphical_piece(
            canvas,
            textures,
            &piece,
            x as u32,
            y as u32,
            game.cache.square_size,
        )?;
    }
    canvas.present();
    Ok(())
}
