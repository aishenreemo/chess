//! chess gaem
//! simple chess implementation written in rust
#![deny(missing_docs)]

mod board;
pub mod piece;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

use board::render_graphical_board;

/// chess board should has 8 each side
pub const BOARD_SIDE_LENGTH: u32 = 8;
/// window size
pub const WINDOW_SIZE: u32 = 512;
/// board size inside the window
pub const BOARD_SIZE: u32 = 400;
/// canvas width per square in board
pub const CELL_WIDTH: u32 = BOARD_SIZE / BOARD_SIDE_LENGTH;

fn render(
    canvas: &mut WindowCanvas,
    board: &board::Board,
    pieces: &Texture,
) -> Result<(), Box<dyn std::error::Error>> {
    // fill background
    canvas.set_draw_color(Color::RGB(250, 229, 210));
    canvas.clear();

    render_graphical_board(canvas, board, pieces)?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sdl2 context
    let ctx = sdl2::init()?;

    let video_subsystem = ctx.video()?;

    // window
    let window = video_subsystem
        .window("chess gaem", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()?;

    // canvas
    let mut canvas: WindowCanvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();
    let pieces = texture_creator.load_texture("assets/chess_pieces.png")?;

    let board = board::Board::default();

    render(&mut canvas, &board, &pieces)?;

    let mut event_pump = ctx.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
