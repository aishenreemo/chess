pub mod board;
mod cache;
mod constants;
pub mod piece;

use piece::PieceColor;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub type Error = Box<dyn ::std::error::Error>;

enum State {
    Quit,
    Focus { column: usize, row: usize },
    Unfocus,
    Move(board::Move),
    SelectTeam(PieceColor),
    Unknown,
}

fn handle_mouse_keypress_on_ongoing_game(
    x: u32,
    y: u32,
    cached: &cache::Cache,
    chessboard: &board::Board,
) -> State {
    let (column, row) = board::into_relative_position(x, y);
    let (column, row) = (column as usize, row as usize);

    let square = chessboard.get_square(column, row).unwrap();
    let is_focused = cached.focused_square.is_some();

    match square.piece {
        Some(piece) if piece.color != cached.current_turn && is_focused => {
            let prev_move = cached.focused_square.unwrap();
            State::Move(board::Move {
                start: prev_move,
                target: (column, row),
            })
        }
        Some(piece) if piece.color != cached.current_turn && !is_focused => State::Unfocus,
        Some(piece) if piece.color == cached.current_turn => State::Focus { column, row },
        None if is_focused => {
            let prev_move = cached.focused_square.unwrap();
            State::Move(board::Move {
                start: prev_move,
                target: (column, row),
            })
        }
        _ => State::Unknown,
    }
}

fn handle_mouse_keypress_on_selecting_team(x: u32) -> State {
    if x < constants::WINDOW_SIZE / 2 {
        State::SelectTeam(PieceColor::White)
    } else {
        State::SelectTeam(PieceColor::Black)
    }
}

fn handle_mouse_keypress(
    mouse_btn: MouseButton,
    x: i32,
    y: i32,
    cached: &cache::Cache,
    chessboard: &board::Board,
) -> State {
    let (x, y) = (x as u32, y as u32);
    match mouse_btn {
        MouseButton::Left if board::is_cursor_inside_board(x, y) => {
            use cache::GameState as GS;
            match cached.current_game_state {
                GS::SelectingTeam => handle_mouse_keypress_on_selecting_team(x),
                GS::OngoingGame => handle_mouse_keypress_on_ongoing_game(x, y, cached, chessboard),
                _ => State::Unknown,
            }
        }
        _ => State::Unknown,
    }
}

fn handle_keyboard_keypress(keycode: Option<Keycode>) -> State {
    match keycode {
        Some(Keycode::Escape) => State::Quit,
        _ => State::Unknown,
    }
}

fn handle_event(
    event: sdl2::event::Event,
    cached: &cache::Cache,
    chessboard: &board::Board,
) -> State {
    match event {
        Event::Quit { .. } => State::Quit,
        Event::KeyDown { keycode, .. } => handle_keyboard_keypress(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mouse_keypress(mouse_btn, x, y, cached, chessboard),
        _ => State::Unknown,
    }
}

fn handle_state(
    state: State,
    canvas: &mut WindowCanvas,
    pieces_texture: &Texture,
    cached: &mut cache::Cache,
    chessboard: &mut board::Board,
) -> Result<(), Error> {
    match state {
        State::Focus { column, row } => {
            let square_index = row * 8 + column;
            // focus the square
            cached.focused_square = Some((column, row));
            cached.target_squares = board::get_target_squares(
                cached.available_moves.get(&square_index).unwrap_or(&vec![]),
            );
            render(canvas, chessboard, pieces_texture, cached)?;
        }
        State::Unfocus => {
            cached.focused_square = None;
            cached.target_squares = vec![];
        }
        State::Move(move_data) => {
            // unfocus if its not a valid move
            if !cached.target_squares.contains(&move_data.target) {
                cached.focused_square = None;
                cached.target_squares = vec![];

                render(canvas, chessboard, pieces_texture, cached)?;
                return Ok(());
            }

            if board::is_move_castling(&move_data, cached) {
                board::move_board_piece(chessboard, &move_data);
                board::move_board_piece(
                    chessboard,
                    &board::get_castling_move_data(move_data.target),
                );
            } else {
                // move the piece
                board::move_board_piece(chessboard, &move_data);
            }

            // check if the pieces used for castling is moved
            for (index, pos_data) in cached.castling_pieces_initial_position.iter().enumerate() {
                if !cached.is_castling_pieces_unmoved[index] {
                    continue;
                }

                let square = chessboard.get_square(pos_data.0[0], pos_data.0[1]).unwrap();
                if square.piece.is_none() {
                    cached.is_castling_pieces_unmoved[index] = false;
                    continue;
                }
                if square.piece.unwrap().variant != pos_data.1 {
                    cached.is_castling_pieces_unmoved[index] = false;
                }
            }

            // change the turn
            cached.current_turn = if cached.current_turn == PieceColor::White {
                PieceColor::Black
            } else {
                PieceColor::White
            };

            // unfocus the focused square
            cached.focused_square = None;
            cached.target_squares = vec![];
            cached.available_moves = board::generate_moves(chessboard, cached);

            render(canvas, chessboard, pieces_texture, cached)?;
        }
        State::SelectTeam(color) => {
            *chessboard = board::Board::color(&color);
            cached.current_game_state = cache::GameState::OngoingGame;
            cached.player_color = color;
            cached.available_moves = board::generate_moves(chessboard, cached);
            cached.castling_pieces_initial_position =
                cache::precompute_castling_pieces_init_pos(cached);
            cached.king_initial_column = if cached.player_color == PieceColor::White {
                4
            } else {
                3
            };

            render(canvas, chessboard, pieces_texture, cached)?;
        }
        State::Unknown => (),
        _ => unreachable!(),
    }
    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    board: &board::Board,
    pieces_texture: &Texture,
    cached: &crate::cache::Cache,
) -> Result<(), Error> {
    use cache::GameState as GS;
    match cached.current_game_state {
        GS::SelectingTeam => {
            canvas.set_draw_color(Color::RGB(250, 229, 210));
            canvas.clear();

            render_graphical_selection(canvas)?;
            canvas.present();
        }
        GS::OngoingGame => {
            // fill background
            canvas.set_draw_color(Color::RGB(250, 229, 210));
            canvas.clear();

            board::render_graphical_board(canvas, board, pieces_texture, cached)?;

            canvas.present();
        }
        GS::_YouWin => {}
        GS::_YouLose => {}
    }
    Ok(())
}

fn render_graphical_selection(canvas: &mut WindowCanvas) -> Result<(), Error> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let rect = Rect::new(
        constants::BOARD_X_OFFSET as i32,
        constants::BOARD_Y_OFFSET as i32,
        constants::BOARD_IN_WINDOW_SIZE / 2,
        constants::BOARD_IN_WINDOW_SIZE,
    );
    canvas.fill_rect(rect)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let rect = Rect::new(
        constants::BOARD_X_OFFSET as i32 + (constants::BOARD_IN_WINDOW_SIZE / 2) as i32,
        constants::BOARD_Y_OFFSET as i32,
        constants::BOARD_IN_WINDOW_SIZE / 2,
        constants::BOARD_IN_WINDOW_SIZE,
    );
    canvas.fill_rect(rect)?;
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
    let mut chessboard = board::Board::init();
    render(&mut canvas, &chessboard, &pieces_texture, &cached)?;

    let mut events = sdl_context.event_pump().unwrap();
    'keep_alive: loop {
        for event in events.poll_iter() {
            match handle_event(event, &cached, &chessboard) {
                State::Quit => break 'keep_alive,
                other_state => handle_state(
                    other_state,
                    &mut canvas,
                    &pieces_texture,
                    &mut cached,
                    &mut chessboard,
                )?,
            }
        }

        // 60 fps
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
