use crate::piece::ColoredPiece;
use crate::{BOARD_SIDE_LENGTH, BOARD_SIZE, WINDOW_SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn render_graphical_board(canvas: &mut WindowCanvas) -> Result<(), String> {
    // stroke the chess board border
    let board_x = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
    let board_y = WINDOW_SIZE as f64 * 0.05;
    let board_rect = Rect::new(board_x as i32, board_y as i32, BOARD_SIZE, BOARD_SIZE);
    canvas.set_draw_color(Color::RGB(122, 95, 71));
    canvas.draw_rect(board_rect)?;

    // stroke squares inside the chess board
    let board_area = BOARD_SIDE_LENGTH.pow(2);
    let cell_width = BOARD_SIZE / BOARD_SIDE_LENGTH;
    let mut cell_index = 0;
    while cell_index < board_area {
        // skip the white square since the background is white
        if cell_index % 2 == 0 {
            cell_index += 1;
            continue;
        }

        // column and row are integers between 0-7
        let column = cell_index % BOARD_SIDE_LENGTH;
        let row = cell_index / BOARD_SIDE_LENGTH;

        // calculate the position of the current cell
        let (x, y) = board_pos_into_canvas_pos(column, row);

        // adjustments for even rows
        let x = if row % 2 == 0 { x } else { x - cell_width };

        let board_rect = Rect::new(x as i32, y as i32, cell_width, cell_width);
        canvas.fill_rect(board_rect)?;
        cell_index += 1;
    }

    Ok(())
}

pub fn board_pos_into_canvas_pos(column: u32, row: u32) -> (u32, u32) {
    let board_x = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
    let board_y = WINDOW_SIZE as f64 * 0.05;
    let cell_width = BOARD_SIZE / BOARD_SIDE_LENGTH;
    (
        (column * cell_width) + board_x as u32,
        (row * cell_width) + board_y as u32,
    )
}

pub struct Board {
    pub squares: [ColoredPiece; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [ColoredPiece::Empty; 64],
        }
    }
}
