//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::controller::Controller;
use crate::board::SIDE_LENGTH;
use crate::{WINDOW_SIZE, BOARD_SIZE};

/// Stores gameboard view settings.
pub struct ViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// white color
    pub white_color: Color,
    /// black color
    pub black_color: Color,
}

impl ViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> Self {
        Self {
            position: [(WINDOW_SIZE - BOARD_SIZE) / 2.0, WINDOW_SIZE * 0.05],
            size: BOARD_SIZE,
            white_color: [0.8, 0.8, 1.0, 1.0],
            black_color: [0.0, 0.0, 0.2, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct View {
    /// Stores gameboard view settings.
    pub settings: ViewSettings,
}

impl View {
    /// Creates a new gameboard view.
    pub fn new(settings: ViewSettings) -> View {
        View {
            settings: settings,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, _controller: &Controller, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        // Draw board background.
        Rectangle::new(settings.white_color).draw(board_rect, &c.draw_state, c.transform, g);

        let board_area = SIDE_LENGTH.pow(2);
        let box_length = settings.size / SIDE_LENGTH as f64;

        let mut i = 0;
        while board_area > i {
            if i % 2 == 0 {
                i += 1;
                continue;
            }

            let column = i % 8;
            let row = i / 8;

            let y = (row as f64 * box_length) + settings.position[1];
            let x = if row % 2 == 0 {
                (column as f64 * box_length) + settings.position[0]
            } else {
                (column as f64 * box_length) - box_length + settings.position[0]
            };

            let cell_rect = [x, y, box_length, box_length];
            Rectangle::new(settings.black_color).draw(cell_rect, &c.draw_state, c.transform, g);
            i += 1;
        }
    }
}
