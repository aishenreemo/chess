//! Gameboard controller.

use piston::input::GenericEvent;

use crate::board::Chess;

/// Handles events for Sudoku game.
pub struct Controller {
    /// Stores the gameboard state.
    pub board: Chess,
}

impl Controller {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Chess) -> Self {
        Self {
            board: gameboard,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, _e: &E) {

    }
}