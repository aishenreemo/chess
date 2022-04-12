use crate::game::{Game, PieceVariant};
use crate::produce::{Move, MoveType};
use crate::Command;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

fn is_cursor_inside_promoting_selection(game: &Game, pos: (i32, i32)) -> bool {
    let margin_top_bottom = (game.cache.window_size.1 - game.cache.square_size.1) / 2.0;
    let margin_left_right = game.cache.square_size.0;
    let margin_offset: u32 = 10;

    let main_rect = Rect::new(
        (game.cache.board_offset.0 + margin_left_right - margin_offset as f32) as i32,
        (game.cache.board_offset.1 + margin_top_bottom - margin_offset as f32) as i32,
        game.cache.square_size.0 as u32 * 6 + margin_offset * 2,
        game.cache.square_size.1 as u32 + margin_offset * 2,
    );
    main_rect.contains_point(pos)
}

fn handle_mouse_on_promoting_selection(game: &Game, pos: (i32, i32)) -> Vec<Command> {
    use PieceVariant::*;
    let margin_offset: u32 = 10;
    let rect_x =
        (game.cache.board_offset.0 + game.cache.square_size.0 as f32 - margin_offset as f32) as i32;
    let constant = ((game.cache.board_offset.0 as u32 * 6 + margin_offset * 2) / 5) as i32;

    for (i, piece_variant) in [Queen, Castle, Knight, Bishop].into_iter().enumerate() {
        let piece_x = rect_x + ((i as i32 + 1) * constant) - (game.cache.square_size.0 as i32 / 2);
        if piece_x < pos.0 && pos.0 < piece_x + game.cache.square_size.0 as i32 {
            return vec![
                Command::Move(Move {
                    variant: MoveType::Promotion(piece_variant),
                    from: game.cache.data.focused_square.unwrap(),
                    to: game.cache.data.recent_promoting_pawn.unwrap(),
                }),
                Command::Unfocus,
                Command::ChangeTurn,
            ];
        }
    }
    vec![]
}

fn handle_mousedown(game: &Game, mouse_btn: MouseButton, pos: (i32, i32)) -> Vec<Command> {
    match mouse_btn {
        MouseButton::Left if is_cursor_inside_promoting_selection(game, pos) => {
            handle_mouse_on_promoting_selection(game, pos)
        }
        MouseButton::Left => vec![],
        _ => vec![],
    }
}

fn handle_keydown(keycode: Option<Keycode>) -> Vec<Command> {
    match keycode {
        Some(Keycode::Escape) => vec![Command::ExitGame],
        _ => vec![Command::Idle],
    }
}

pub fn handle_event(event: Event, game: &Game) -> Vec<Command> {
    match event {
        Event::Quit { .. } => vec![Command::Quit],
        Event::KeyDown { keycode, .. } => handle_keydown(keycode),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => handle_mousedown(game, mouse_btn, (x, y)),
        _ => vec![Command::Idle],
    }
}
