pub const WINDOW_SIZE: u32 = 512;
pub const BOARD_IN_WINDOW_SIZE: u32 = 400;
pub const SQUARE_IN_BOARD_SIZE: u32 = BOARD_IN_WINDOW_SIZE / 8;
pub const BOARD_X_OFFSET: f64 = (WINDOW_SIZE as f64 - BOARD_IN_WINDOW_SIZE as f64) / 2.0;
pub const BOARD_Y_OFFSET: f64 = WINDOW_SIZE as f64 * 0.05;