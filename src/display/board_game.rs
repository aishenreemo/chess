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

    let default_window_size = configuration.window_size;
    let window_size = canvas.output_size()?;

    let board_size = window_size.0 as f32 * (400.0 / default_window_size);
    let square_size = board_size / 8.0;
    let board_x_offset = (window_size.0 as f32 - board_size) / 2.0;
    let board_y_offset = window_size.0 as f32 * 0.05;

    // stroke the chess board border
    let board_rect = Rect::new(
        board_x_offset as i32,
        board_y_offset as i32,
        board_size as u32,
        board_size as u32,
    );

    canvas.set_draw_color(configuration.palette.default_dark_color);
    canvas.draw_rect(board_rect)?;

    for (row, squares) in game.board.into_iter().enumerate() {
        for (column, square) in squares.into_iter().enumerate() {
            let (x, y) = into_absolute_position(
                column as u32,
                row as u32,
                board_x_offset,
                board_y_offset,
                square_size,
            );

            let cell_rect = Rect::new(x as i32, y as i32, square_size as u32, square_size as u32);

            if column % 2 != 0 && row % 2 == 0 || column % 2 == 0 && row % 2 != 0 {
                canvas.fill_rect(cell_rect)?;
            }

            if let Some(ref piece) = square {
                render_graphical_piece(canvas, textures, piece, x, y, square_size)?;
            }
        }
    }

    canvas.present();
    Ok(())
}

fn into_absolute_position(
    column: u32,
    row: u32,
    x_offset: f32,
    y_offset: f32,
    square_size: f32,
) -> (u32, u32) {
    (
        (column * square_size as u32) + x_offset as u32,
        (row * square_size as u32) + y_offset as u32,
    )
}

pub fn render_graphical_piece(
    canvas: &mut WindowCanvas,
    textures: &Textures,
    piece: &Piece,
    x: u32,
    y: u32,
    square_size: f32,
) -> Result<(), String> {
    let rect = Rect::new(x as i32, y as i32, square_size as u32, square_size as u32);

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
