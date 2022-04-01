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
    PromotePawn(piece::PieceVariant),
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

    let piece = chessboard.get_square(column, row);
    let is_focused = cached.focused_square.is_some();

    match piece {
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

fn handle_mouse_keypress_on_promoting_pawn(x: i32) -> State {
    use piece::PieceVariant;
    let margin_offset: u32 = 10;
    let rect_x = (constants::BOARD_X_OFFSET + constants::SQUARE_IN_BOARD_SIZE as f64
        - margin_offset as f64) as i32;
    let constant = ((constants::SQUARE_IN_BOARD_SIZE * 6 + margin_offset * 2) / 5) as i32;
    let promoting_pieces = [
        PieceVariant::Queen,
        PieceVariant::Castle,
        PieceVariant::Knight,
        PieceVariant::Bishop,
    ];
    for (i, piece_variant) in promoting_pieces.iter().enumerate() {
        let piece_x =
            rect_x + ((i as i32 + 1) * constant) - (constants::SQUARE_IN_BOARD_SIZE as i32 / 2);
        if piece_x < x && x < piece_x + constants::SQUARE_IN_BOARD_SIZE as i32 {
            return State::PromotePawn(*piece_variant);
        }
    }
    State::PromotePawn(PieceVariant::Pawn)
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
                GS::PromotingPawn if board::is_cursor_inside_promoting_selection(x, y) => {
                    handle_mouse_keypress_on_promoting_pawn(x as i32)
                }
                _ => State::Unknown,
            }
        }
        MouseButton::Left => State::Unfocus,
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
            // make the selected square blue
            cached.focused_square = Some((column, row));
            // make the square yellow that is targetable by the focused piece
            cached.target_squares = board::get_target_squares(
                cached.available_moves.get(&square_index).unwrap_or(&vec![]),
            );
            // update the canvas
            render(canvas, chessboard, pieces_texture, cached)?;
        }
        State::Unfocus => {
            // clear the cache
            cached.focused_square = None;
            cached.target_squares = vec![];
            // update the canvas
            render(canvas, chessboard, pieces_texture, cached)?;
        }
        State::Move(move_data) => {
            // unfocus if its not a valid move
            if !cached.target_squares.contains(&move_data.target) {
                cached.focused_square = None;
                cached.target_squares = vec![];

                render(canvas, chessboard, pieces_texture, cached)?;
                return Ok(());
            }

            let mut is_move_advancing_pawn_bool = false;
            board::move_board_piece(chessboard, &move_data);
            if board::is_move_promoting_pawn(&move_data, chessboard, cached) {
                cached.recent_promoting_pawn = Some(move_data.target);
                cached.current_game_state = cache::GameState::PromotingPawn;
                render(canvas, chessboard, pieces_texture, cached)?;
                return Ok(());
            } else if board::is_move_castling(&move_data, cached) {
                // move the castle
                board::move_board_piece(
                    chessboard,
                    &board::get_castling_move_data(move_data.target),
                );
            } else if board::is_move_advancing_pawn(&move_data, chessboard) {
                cached.recent_advancing_pawn = Some(move_data.target);
                is_move_advancing_pawn_bool = true;
            } else if board::is_move_en_passant(&move_data, chessboard, cached) {
                let (column, row) = cached.recent_advancing_pawn.take().unwrap();
                chessboard.pieces[row][column] = None;
            }

            if !is_move_advancing_pawn_bool {
                cached.recent_advancing_pawn = None;
            }
            // check if the pieces used for castling is moved
            for (index, pos_data) in cached.castling_pieces_initial_position.iter().enumerate() {
                // ignore if it's already moved
                if !cached.is_castling_pieces_unmoved[index] {
                    continue;
                }

                let piece = chessboard.get_square(pos_data.0[0], pos_data.0[1]);
                if piece.is_none() {
                    cached.is_castling_pieces_unmoved[index] = false;
                    continue;
                }
                if piece.unwrap().variant != pos_data.1 {
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

            // update canvas
            render(canvas, chessboard, pieces_texture, cached)?;
        }

        State::SelectTeam(color) => {
            // if the user selected a team

            // re-initialize the board based on the user choice
            *chessboard = board::Board::color(&color);
            // change the current game state
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

        State::PromotePawn(variant) => {
            if variant == piece::PieceVariant::Pawn {
                return Ok(());
            }
            let (column, row) = cached.recent_promoting_pawn.take().unwrap();
            chessboard.pieces[row][column] = Some(piece::Piece {
                variant,
                color: cached.current_turn,
            });

            cached.current_game_state = cache::GameState::OngoingGame;
            cached.current_turn = if cached.current_turn == PieceColor::White {
                PieceColor::Black
            } else {
                PieceColor::White
            };

            // unfocus the focused square
            cached.focused_square = None;
            cached.target_squares = vec![];
            cached.available_moves = board::generate_moves(chessboard, cached);

            // update canvas
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
        // user is choosing whether black or white
        GS::SelectingTeam => {
            canvas.set_draw_color(Color::RGB(250, 229, 210));
            canvas.clear();

            render_graphical_selection(canvas)?;
            canvas.present();
        }
        // render the game based on the data
        GS::OngoingGame => {
            // fill background
            canvas.set_draw_color(Color::RGB(250, 229, 210));
            canvas.clear();

            board::render_graphical_board(canvas, board, pieces_texture, cached)?;

            canvas.present();
        }
        GS::PromotingPawn => {
            canvas.set_draw_color(Color::RGB(250, 229, 210));
            canvas.clear();

            board::render_graphical_board(canvas, board, pieces_texture, cached)?;
            render_promoting_selection(canvas, cached, pieces_texture)?;
            canvas.present();
        }
        GS::_YouWin => {}
        GS::_YouLose => {}
    }
    Ok(())
}

fn render_promoting_selection(
    canvas: &mut WindowCanvas,
    cached: &crate::cache::Cache,
    pieces_texture: &Texture,
) -> Result<(), Error> {
    use piece::PieceVariant;
    let margin_top_bottom = (constants::BOARD_IN_WINDOW_SIZE - constants::SQUARE_IN_BOARD_SIZE) / 2;
    let margin_left_right = constants::SQUARE_IN_BOARD_SIZE;
    let margin_offset: u32 = 10;

    let main_rect = Rect::new(
        (constants::BOARD_X_OFFSET + margin_left_right as f64 - margin_offset as f64) as i32,
        (constants::BOARD_Y_OFFSET + margin_top_bottom as f64 - margin_offset as f64) as i32,
        constants::SQUARE_IN_BOARD_SIZE * 6 + margin_offset * 2,
        constants::SQUARE_IN_BOARD_SIZE + margin_offset * 2,
    );
    canvas.set_draw_color(Color::RGB(250, 229, 210));
    canvas.fill_rect(main_rect)?;
    canvas.set_draw_color(Color::RGB(122, 95, 71));
    canvas.draw_rect(main_rect)?;

    let constant = ((constants::SQUARE_IN_BOARD_SIZE * 6 + margin_offset * 2) / 5) as i32;
    let promoting_pieces = [
        PieceVariant::Queen,
        PieceVariant::Castle,
        PieceVariant::Knight,
        PieceVariant::Bishop,
    ];
    for (i, piece_variant) in promoting_pieces.iter().enumerate() {
        let x = main_rect.x() + ((i as i32 + 1) * constant)
            - (constants::SQUARE_IN_BOARD_SIZE as i32 / 2);
        let y = main_rect.y() + margin_offset as i32;
        let piece = piece::Piece {
            variant: *piece_variant,
            color: cached.current_turn,
        };
        piece::render_graphical_piece(canvas, pieces_texture, &piece, x as u32, y as u32)?;
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

    // make canvas
    let mut canvas: WindowCanvas = window.into_canvas().build()?;

    // used to draw the chess pieces on the canvas
    let texture_creator = canvas.texture_creator();
    let pieces_texture = texture_creator.load_texture("assets/chess_pieces.png")?;

    // game data
    let mut cached = cache::Cache::init();
    let mut chessboard = board::Board::init();

    // render the initialized game
    render(&mut canvas, &chessboard, &pieces_texture, &cached)?;

    // event listener
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
