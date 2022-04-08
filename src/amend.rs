use crate::game::{self, Game, GameState, TeamColor};
use crate::produce::{self, Move};
use crate::Command;

fn select_team(game: &mut Game, color: TeamColor) {
    game.state = GameState::BoardGame;
    game::init_chess_position(game, color)
}

fn focus_square(game: &mut Game, column: usize, row: usize) {
    game.cache.data.focused_square = Some((column, row));
    game.cache.data.danger_squares = game
        .cache
        .data
        .available_moves
        .iter()
        .filter(|move_data| move_data.from == (column, row))
        .map(|move_data| move_data.to)
        .collect();
}

fn unfocus_square(game: &mut Game) {
    game.cache.data.focused_square = None;
    game.cache.data.danger_squares.clear();
}

fn change_turn(game: &mut Game) {
    game.cache.data.current_turn = if game.cache.data.current_turn == TeamColor::White {
        TeamColor::Black
    } else {
        TeamColor::White
    };
    game.cache.data.available_moves = produce::generate_moves(game);
}

fn move_piece(game: &mut Game, move_data: Move) {
    game.board[move_data.to.1][move_data.to.0] =
        game.board[move_data.from.1][move_data.from.0].take();
}

pub fn update(instructions: Vec<Command>, game: &mut Game) {
    for command in instructions {
        match command {
            Command::Quit => std::process::exit(0),
            Command::Play => game.state = GameState::TeamSelection,
            Command::ExitGame => game.state = GameState::StartMenu,
            Command::SelectTeam(color) => select_team(game, color),
            Command::Focus(c, r) => focus_square(game, c, r),
            Command::ChangeTurn => change_turn(game),
            Command::Unfocus => unfocus_square(game),
            Command::Move(move_data) => move_piece(game, move_data),
            Command::Idle => (),
        }
    }
}
