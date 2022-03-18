use crate::piece::{render_graphical_piece, ColoredPiece, Piece};
use crate::{BOARD_SIDE_LENGTH, BOARD_SIZE, CELL_WIDTH, WINDOW_SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub fn render_graphical_board(
    canvas: &mut WindowCanvas,
    board: &Board,
    pieces: &Texture,
) -> Result<(), String> {
    // stroke the chess board border
    let board_x = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
    let board_y = WINDOW_SIZE as f64 * 0.05;
    let board_rect = Rect::new(board_x as i32, board_y as i32, BOARD_SIZE, BOARD_SIZE);
    canvas.set_draw_color(Color::RGB(122, 95, 71));
    canvas.draw_rect(board_rect)?;

    // stroke squares inside the chess board
    let board_area = BOARD_SIDE_LENGTH.pow(2);
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
        let x = if row % 2 == 0 { x } else { x - CELL_WIDTH };

        let board_rect = Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_WIDTH);
        canvas.fill_rect(board_rect)?;
        cell_index += 1;
    }

    // render pieces
    for (i, piece) in board.squares.iter().enumerate() {
        let column = i as u32 % BOARD_SIDE_LENGTH;
        let row = i as u32 / BOARD_SIDE_LENGTH;

        render_graphical_piece(canvas, piece, pieces, column, row)?;
    }
    Ok(())
}

pub fn board_pos_into_canvas_pos(column: u32, row: u32) -> (u32, u32) {
    let board_x = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
    let board_y = WINDOW_SIZE as f64 * 0.05;
    (
        (column * CELL_WIDTH) + board_x as u32,
        (row * CELL_WIDTH) + board_y as u32,
    )
}

pub struct Board {
    pub squares: Vec<ColoredPiece>,
}

impl Board {
    fn init_squares() -> Vec<ColoredPiece> {
        let start_game_notation: Vec<char> =
            "CHBQKBHCPPPPPPPP////ppppppppchbqkbhc".chars().collect();
        let mut pieces: Vec<ColoredPiece> = vec![];
        let mut cursor = 0;

        while cursor < start_game_notation.len() {
            if pieces.len() > 64 {
                panic!("pieces overflow")
            }
            match start_game_notation.get(cursor) {
                Some(c) if c.is_numeric() => {
                    let num = c.to_digit(10).unwrap();
                    for _ in 0..num {
                        pieces.push(ColoredPiece::Empty);
                    }
                }
                Some(&'/') => {
                    let num = 8 - (pieces.len() % 8);
                    for _ in 0..num {
                        pieces.push(ColoredPiece::Empty);
                    }
                }
                Some(&'C') => pieces.push(ColoredPiece::B(Piece::Castle)),
                Some(&'H') => pieces.push(ColoredPiece::B(Piece::Knight)),
                Some(&'B') => pieces.push(ColoredPiece::B(Piece::Bishop)),
                Some(&'Q') => pieces.push(ColoredPiece::B(Piece::Queen)),
                Some(&'K') => pieces.push(ColoredPiece::B(Piece::King)),
                Some(&'P') => pieces.push(ColoredPiece::B(Piece::Pawn)),
                Some(&'c') => pieces.push(ColoredPiece::W(Piece::Castle)),
                Some(&'h') => pieces.push(ColoredPiece::W(Piece::Knight)),
                Some(&'b') => pieces.push(ColoredPiece::W(Piece::Bishop)),
                Some(&'q') => pieces.push(ColoredPiece::W(Piece::Queen)),
                Some(&'k') => pieces.push(ColoredPiece::W(Piece::King)),
                Some(&'p') => pieces.push(ColoredPiece::W(Piece::Pawn)),
                Some(c) => panic!("unexpected '{}'", c),
                _ => unreachable!(),
            }
            cursor += 1;
        }
        if pieces.len() != 64 {
            panic!("expected 64 squares, got {}\n", pieces.len());
        }

        pieces
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: Board::init_squares(),
        }
    }
}
