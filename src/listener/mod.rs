mod board_game;
mod start_menu;
mod team_selection;

use crate::config::Config;
use crate::game::{Game, GameState};
use crate::Command;

use sdl2::event::Event;
use sdl2::render::WindowCanvas;

pub fn handle_event(
    event: Event,
    game: &Game,
    configuration: &Config,
    canvas: &WindowCanvas,
) -> Command {
    match game.state {
        GameState::StartMenu => start_menu::handle_event(event, configuration),
        GameState::TeamSelection => team_selection::handle_event(event, configuration, canvas),
        GameState::BoardGame => board_game::handle_event(event),
    }
}
