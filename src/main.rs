//! chess gaem
//! simple chess implementation written in rust
#![deny(missing_docs)]
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const BOARD_SIDE_LENGTH: u32 = 8;
const WINDOW_SIZE: u32 = 512;
const BOARD_SIZE: u32 = 400;

fn render(canvas: &mut WindowCanvas) -> Result<(), String> {
    use sdl2::rect::Rect;

    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);

    // fill background
    canvas.set_draw_color(white);
    canvas.clear();

    // stroke the chess board border
    let board_x = (WINDOW_SIZE as f64 - BOARD_SIZE as f64) / 2.0;
    let board_y = WINDOW_SIZE as f64 * 0.05;
    let board_rect = Rect::new(board_x as i32, board_y as i32, BOARD_SIZE, BOARD_SIZE);
    canvas.set_draw_color(black);
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
        let x = (column * cell_width) + board_x as u32;
        let y = (row * cell_width) + board_y as u32;

        // adjustments for even rows
        let x = if row % 2 == 0 { x } else { x - cell_width };

        let board_rect = Rect::new(x as i32, y as i32, cell_width, cell_width);
        canvas.fill_rect(board_rect)?;
        cell_index += 1;
    }

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

    render(&mut canvas)?;

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
        render(&mut canvas)?;

        // 60 fps
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
