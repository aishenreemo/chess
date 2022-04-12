mod board_game;
mod promote_selection;
mod start_menu;
mod team_selection;

use crate::game::{Game, GameState};
use crate::Command;

use sdl2::event::Event;

pub fn handle_event(event: Event, game: &Game) -> Vec<Command> {
    match game.state {
        GameState::StartMenu => start_menu::handle_event(event, game),
        GameState::TeamSelection => team_selection::handle_event(event, game),
        GameState::BoardGame => board_game::handle_event(event, game),
        GameState::PromoteSelection => promote_selection::handle_event(event, game),
    }
}
