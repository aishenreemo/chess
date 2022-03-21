use crate::piece::{render_graphical_piece, ColoredPiece, Piece};
use crate::{BOARD_SIDE_LENGTH, BOARD_SIZE, CELL_WIDTH, WINDOW_SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

const BOARD_X: f64 = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
const BOARD_Y: f64 = WINDOW_SIZE as f64 * 0.05;

pub fn render_graphical_board(
    canvas: &mut WindowCanvas,
    board: &Board,
    pieces_texture: &Texture,
) -> Result<(), Box<dyn std::error::Error>> {
    // stroke the chess board border
    let board_rect = Rect::new(BOARD_X as i32, BOARD_Y as i32, BOARD_SIZE, BOARD_SIZE);
    canvas.set_draw_color(Color::RGB(122, 95, 71));
    canvas.draw_rect(board_rect)?;

    // render squares
    for square in board.squares.iter() {
        let column = square.index % BOARD_SIDE_LENGTH;
        let row = square.index / BOARD_SIDE_LENGTH;

        // calculate the position of the current cell
        let (x, y) = board_pos_into_canvas_pos(column, row);
        let board_rect = Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_WIDTH);

        if square.is_focused {
            canvas.set_draw_color(Color::RGB(104, 113, 143));
            canvas.fill_rect(board_rect)?;
        } else if square.index % 2 != 0 && row % 2 != 0 || square.index % 2 == 0 && row % 2 == 0 {
            canvas.set_draw_color(Color::RGB(122, 95, 71));
            canvas.fill_rect(board_rect)?;
        }
    }

    for square in board.squares.iter() {
        if square.piece == ColoredPiece::Empty {
            continue;
        }

        let column = square.index % BOARD_SIDE_LENGTH;
        let row = square.index / BOARD_SIDE_LENGTH;
        render_graphical_piece(canvas, &square.piece, pieces_texture, column, row)?;
    }

    Ok(())
}

pub fn board_pos_into_canvas_pos(column: u32, row: u32) -> (u32, u32) {
    (
        (column * CELL_WIDTH) + BOARD_X as u32,
        (row * CELL_WIDTH) + BOARD_Y as u32,
    )
}

pub fn is_cursor_inside_board(x: u32, y: u32) -> bool {
    x > BOARD_X as u32
        && x < BOARD_X as u32 + BOARD_SIZE
        && y > BOARD_Y as u32
        && y < BOARD_Y as u32 + BOARD_SIZE
}

pub fn canvas_pos_into_board_pos(x: u32, y: u32) -> (u32, u32) {
    (
        // column * width + offset = x
        // (x - offset / width) = column
        (x - BOARD_X as u32) / CELL_WIDTH as u32,
        (y - BOARD_Y as u32) / CELL_WIDTH as u32,
    )
}

pub struct Board {
    pub squares: Vec<Square>,
}

impl Board {
    fn init_squares() -> Vec<Square> {
        let start_game_notation: Vec<char> =
            "CHBQKBHCPPPPPPPP////ppppppppchbqkbhc".chars().collect();
        let mut squares: Vec<Square> = vec![];
        let mut cursor = 0;

        while cursor < start_game_notation.len() {
            use ColoredPiece::*;
            use Piece::*;

            if squares.len() > 64 {
                panic!("pieces overflow")
            }

            match start_game_notation.get(cursor) {
                Some(c) if c.is_numeric() => {
                    let num = c.to_digit(10).unwrap();
                    for _ in 0..num {
                        squares.push(Square::empty(squares.len()));
                    }
                }
                Some(&'/') => {
                    let num = 8 - (squares.len() % 8);
                    for _ in 0..num {
                        squares.push(Square::empty(squares.len()));
                    }
                }
                Some(&'C') => squares.push(Square::piece(squares.len(), B(Castle))),
                Some(&'H') => squares.push(Square::piece(squares.len(), B(Knight))),
                Some(&'B') => squares.push(Square::piece(squares.len(), B(Bishop))),
                Some(&'Q') => squares.push(Square::piece(squares.len(), B(Queen))),
                Some(&'K') => squares.push(Square::piece(squares.len(), B(King))),
                Some(&'P') => squares.push(Square::piece(squares.len(), B(Pawn))),
                Some(&'c') => squares.push(Square::piece(squares.len(), W(Castle))),
                Some(&'h') => squares.push(Square::piece(squares.len(), W(Knight))),
                Some(&'b') => squares.push(Square::piece(squares.len(), W(Bishop))),
                Some(&'q') => squares.push(Square::piece(squares.len(), W(Queen))),
                Some(&'k') => squares.push(Square::piece(squares.len(), W(King))),
                Some(&'p') => squares.push(Square::piece(squares.len(), W(Pawn))),
                Some(c) => panic!("unexpected '{}'", c),
                _ => unreachable!(),
            }
            cursor += 1;
        }

        if squares.len() != 64 {
            panic!("expected 64 squares, got {}\n", squares.len());
        }

        squares
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: Board::init_squares(),
        }
    }
}

#[derive(Debug)]
pub struct Square {
    pub piece: ColoredPiece,
    pub is_focused: bool,
    pub index: u32,
}

impl Square {
    fn empty(index: usize) -> Self {
        Self {
            piece: ColoredPiece::Empty,
            index: index as u32,
            is_focused: false,
        }
    }

    fn piece(index: usize, piece: ColoredPiece) -> Self {
        Self {
            piece,
            index: index as u32,
            is_focused: false,
        }
    }
}
