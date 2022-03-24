mod board;
mod cache;
mod constants;
pub mod piece;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};

pub type Error = Box<dyn ::std::error::Error>;

enum State {
    Quitting,
    Focus { column: usize, row: usize },
    Unknown,
}

fn handle_mouse_keypress(mouse_btn: MouseButton, x: i32, y: i32) -> State {
    match mouse_btn {
        MouseButton::Left if board::is_cursor_inside_board(x as u32, y as u32) => {
            let (column, row) = board::into_relative_position(x as u32, y as u32);
            State::Focus {
                column: column as usize,
                row: row as usize,
            }
        }
        _ => State::Unknown,
    }
}

fn handle_keyboard_keypress(keycode: Option<Keycode>) -> State {
    match keycode {
        Some(Keycode::Escape) => State::Quitting,
        _ => State::Unknown,
    }
}

fn handle_event(event: sdl2::event::Event) -> State {
    match event {
        Event::Quit { .. } => State::Quitting,
        Event::KeyDown { keycode, .. } => handle_keyboard_keypress(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mouse_keypress(mouse_btn, x, y),
        _ => State::Unknown,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    board: &board::Board,
    pieces_texture: &Texture,
    cached: &crate::cache::Cache,
) -> Result<(), Box<dyn std::error::Error>> {
    // fill background
    canvas.set_draw_color(Color::RGB(250, 229, 210));
    canvas.clear();

    board::render_graphical_board(canvas, board, pieces_texture, cached)?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), Error> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("chess", constants::WINDOW_SIZE, constants::WINDOW_SIZE)
        .position_centered()
        .build()?;

    let mut canvas: WindowCanvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();
    let pieces_texture = texture_creator.load_texture("assets/chess_pieces.png")?;

    let mut cached = cache::Cache::init();
    let chessboard = board::Board::init();
    render(&mut canvas, &chessboard, &pieces_texture, &cached)?;

    let mut events = sdl_context.event_pump().unwrap();
    'keep_alive: loop {
        for event in events.poll_iter() {
            match handle_event(event) {
                State::Quitting => break 'keep_alive,
                State::Focus { column, row } => {
                    // unfocus the last focused square
                    if let Some(lfs) = cached.focused_square {
                        if chessboard.get_square(lfs.0, lfs.1).is_some() {
                            cached.focused_square = None;
                        }
                    }

                    // focus the square if its a piece
                    if let Some(square) = chessboard.get_square(column, row) {
                        if square.piece != None {
                            // memoize the pointers to unfocus later
                            cached.focused_square = Some((column, row));
                        }
                    }

                    render(&mut canvas, &chessboard, &pieces_texture, &cached)?;
                }
                State::Unknown => (),
            }
        }

        // 60 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
