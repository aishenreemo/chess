use crate::constants;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub fn render_graphical_piece(
    canvas: &mut WindowCanvas,
    pieces_texture: &Texture,
    piece: &Piece,
    x: u32,
    y: u32,
) -> Result<(), String> {
    let rect = Rect::new(
        x as i32,
        y as i32,
        constants::SQUARE_IN_BOARD_SIZE,
        constants::SQUARE_IN_BOARD_SIZE,
    );

    canvas.copy(pieces_texture, get_piece_rect(piece), rect)?;
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
        PieceColor::Black => texture_width,
        PieceColor::White => 0,
    };

    Rect::new(x as i32, y as i32, texture_width, texture_width)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub variant: PieceVariant,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceVariant {
    Pawn,
    Bishop,
    Knight,
    Castle,
    Queen,
    King,
}
