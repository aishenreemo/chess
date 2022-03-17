//!
use crate::board::board_pos_into_canvas_pos;
use crate::{BOARD_SIDE_LENGTH, BOARD_SIZE};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

/// render a graphical piece to canvas
pub fn render_graphical_piece(
    canvas: &mut WindowCanvas,
    piece: &ColoredPiece,
    texture: &Texture,
    column: u32,
    row: u32,
) -> Result<(), String> {
    if &ColoredPiece::Empty == piece {
        return Err("Unknown piece".to_owned());
    }

    let cell_width = BOARD_SIZE / BOARD_SIDE_LENGTH;
    let (x, y) = board_pos_into_canvas_pos(column, row);
    let piece_rect = get_piece_rect(piece)?;
    canvas.copy(
        texture,
        piece_rect,
        Rect::new(x as i32, y as i32, cell_width, cell_width),
    )?;

    Ok(())
}

fn get_piece_rect(cpiece: &ColoredPiece) -> Result<Rect, String> {
    match cpiece {
        ColoredPiece::W(piece) => match piece {
            Piece::Pawn => Ok(Rect::new(45 * 5, 0, 45, 45)),
            Piece::Bishop => Ok(Rect::new(45 * 2, 0, 45, 45)),
            Piece::Knight => Ok(Rect::new(45 * 3, 0, 45, 45)),
            Piece::Castle => Ok(Rect::new(45 * 4, 0, 45, 45)),
            Piece::Queen => Ok(Rect::new(45, 0, 45, 45)),
            Piece::King => Ok(Rect::new(0, 0, 45, 45)),
        },
        ColoredPiece::B(piece) => match piece {
            Piece::Pawn => Ok(Rect::new(45 * 5, 45, 45, 45)),
            Piece::Bishop => Ok(Rect::new(45 * 2, 45, 45, 45)),
            Piece::Knight => Ok(Rect::new(45 * 3, 45, 45, 45)),
            Piece::Castle => Ok(Rect::new(45 * 4, 45, 45, 45)),
            Piece::Queen => Ok(Rect::new(45, 45, 45, 45)),
            Piece::King => Ok(Rect::new(0, 45, 45, 45)),
        },
        ColoredPiece::Empty => Err("Unknown piece.".to_owned()),
    }
}

#[derive(Copy, Clone, PartialEq)]
/// A colored piece in the chess board
pub enum ColoredPiece {
    /// a black piece
    B(Piece),
    /// a white piece
    W(Piece),
    /// neutral
    Empty,
}

#[derive(Copy, Clone, PartialEq)]
/// a chess piece
pub enum Piece {
    /// pawn
    Pawn,
    /// bishop
    Bishop,
    /// horse / knight
    Knight,
    /// rook / castle
    Castle,
    /// queen
    Queen,
    /// king
    King,
}
