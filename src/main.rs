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

/// chess board should has 8 each side
pub const BOARD_SIDE_LENGTH: u32 = 8;
/// window size
pub const WINDOW_SIZE: u32 = 512;
/// board size inside the window
pub const BOARD_SIZE: u32 = 400;
/// cell width
pub const CELL_WIDTH: u32 = BOARD_SIZE / BOARD_SIDE_LENGTH;

fn render(canvas: &mut WindowCanvas, board: &board::Board, pieces: &Texture) -> Result<(), String> {
    // fill background
    canvas.set_draw_color(Color::RGB(250, 229, 210));
    canvas.clear();

    board::render_graphical_board(canvas, board, pieces)?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    // sdl2 context
    let ctx = sdl2::init()?;

    let video_subsystem = ctx.video()?;

    // window
    let window = video_subsystem
        .window("chess gaem", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    // canvas
    let mut canvas: WindowCanvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let pieces = texture_creator.load_texture("assets/chess_pieces.png")?;

    let board = board::Board::default();

    render(&mut canvas, &board, &pieces)?;

    let mut event_pump = ctx.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // update
        // something in here

        // render
        // this should be lazy
        // render(&mut canvas, &pieces)?;

        // 60 fps
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
