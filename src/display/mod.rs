mod board_game;
mod promote_selection;
mod start_menu;
mod team_selection;

use crate::config::{Config, Palette};
use crate::game::{Game, GameState};
use crate::Error;
use crate::Textures;

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

pub fn render(
    canvas: &mut WindowCanvas,
    configuration: &Config,
    game: &Game,
    textures: &Textures,
) -> Result<(), Error> {
    use GameState::*;
    match game.state {
        StartMenu => start_menu::render(canvas, configuration, game),
        TeamSelection => team_selection::render(canvas, configuration, game),
        BoardGame => board_game::render(canvas, configuration, game, textures),
        PromoteSelection => promote_selection::render(canvas, configuration, game, textures),
    }
}
