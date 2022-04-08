use crate::game::{self, Game, GameState, TeamColor};
use crate::Command;

fn select_team(game: &mut Game, color: TeamColor) {
    game.state = GameState::BoardGame;
    game::init_chess_position(game, color)
}

fn focus_square(game: &mut Game, column: usize, row: usize) {
    game.cache.data.focused_square = Some((column, row));
}

fn change_turn(game: &mut Game) {
    game.cache.data.current_turn = if game.cache.data.current_turn == TeamColor::White {
        TeamColor::Black
    } else {
        TeamColor::White
    };
}

fn move_piece(game: &mut Game, from: (usize, usize), to: (usize, usize)) {
    game.board[to.1][to.0] = game.board[from.1][from.0].take();
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
            Command::Unfocus => game.cache.data.focused_square = None,
            Command::Move { from, to, .. } => move_piece(game, from, to),
            Command::Idle => (),
        }
    }
}
