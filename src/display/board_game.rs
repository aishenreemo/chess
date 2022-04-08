use crate::config::Config;
use crate::game::{Game, Piece, PieceVariant, TeamColor};
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
    super::render_canvas_background(canvas, &configuration.palette)?;

    // stroke the chess board border
    let board_rect = Rect::new(
        game.cache.board_offset.0 as i32,
        game.cache.board_offset.1 as i32,
        game.cache.board_size.0 as u32,
        game.cache.board_size.1 as u32,
    );

    canvas.set_draw_color(configuration.palette.default_dark_color);
    canvas.draw_rect(board_rect)?;

    for (row, squares) in game.board.into_iter().enumerate() {
        for (column, square) in squares.into_iter().enumerate() {
            let (x, y) = into_absolute_position(game, (column, row));
            let padding = 1.0;
            let cell_rect = Rect::new(
                x as i32,
                y as i32,
                game.cache.square_size.0 as u32,
                game.cache.square_size.1 as u32,
            );

            let cell_rect_focused = Rect::new(
                (x as f32 + padding) as i32,
                (y as f32 + padding) as i32,
                (game.cache.square_size.0 - 2.0 * padding) as u32,
                (game.cache.square_size.1 - 2.0 * padding) as u32,
            );

            if column % 2 != 0 && row % 2 == 0 || column % 2 == 0 && row % 2 != 0 {
                canvas.set_draw_color(configuration.palette.default_dark_color);
                canvas.fill_rect(cell_rect)?;
            }

            if Some((column, row)) == game.cache.data.focused_square {
                canvas.set_draw_color(configuration.palette.blue);
                canvas.fill_rect(cell_rect_focused)?;
            } else if game.cache.data.danger_squares.contains(&(column, row)) {
                canvas.set_draw_color(configuration.palette.yellow);
                canvas.fill_rect(cell_rect_focused)?;
            }

            if let Some(ref piece) = square {
                render_graphical_piece(canvas, textures, piece, x, y, game.cache.square_size)?;
            }
        }
    }

    canvas.present();
    Ok(())
}

fn into_absolute_position(game: &Game, pos: (usize, usize)) -> (u32, u32) {
    (
        (pos.0 as u32 * game.cache.square_size.0 as u32) + game.cache.board_offset.0 as u32,
        (pos.1 as u32 * game.cache.square_size.1 as u32) + game.cache.board_offset.1 as u32,
    )
}

pub fn render_graphical_piece(
    canvas: &mut WindowCanvas,
    textures: &Textures,
    piece: &Piece,
    x: u32,
    y: u32,
    square_size: (f32, f32),
) -> Result<(), String> {
    let rect = Rect::new(
        x as i32,
        y as i32,
        square_size.0 as u32,
        square_size.1 as u32,
    );

    canvas.copy(&textures.pieces, get_piece_rect(piece), rect)?;
    Ok(())
}

fn get_piece_rect(piece: &Piece) -> Rect {
    let texture_width: u32 = 45;
    let x: u32 = match piece.variant {
        PieceVariant::King => 0,
        PieceVariant::Queen => texture_width,
        PieceVariant::Bishop => texture_width * 2,
        PieceVariant::Knight => texture_width * 3,
        PieceVariant::Castle => texture_width * 4,
        PieceVariant::Pawn => texture_width * 5,
    };
    let y: u32 = match piece.color {
        TeamColor::Black => texture_width,
        TeamColor::White => 0,
    };

    Rect::new(x as i32, y as i32, texture_width, texture_width)
}
