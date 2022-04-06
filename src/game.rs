use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Game {
    pub state: GameState,
    pub texture_creator: TextureCreator<WindowContext>,
}

pub enum GameState {
    StartMenu,
}

pub fn initialize_game(canvas: &WindowCanvas) -> Game {
    Game {
        state: GameState::StartMenu,
        texture_creator: canvas.texture_creator(),
    }
}
