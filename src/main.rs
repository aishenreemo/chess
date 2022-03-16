#![deny(missing_docs)]
//! Chess Game

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub mod board;
pub mod controller;
pub mod view;

use board::Chess;
use controller::Controller;
use view::{View, ViewSettings};

/// window size
pub const WINDOW_SIZE: f64 = 512.0;
/// chess board size
pub const BOARD_SIZE: f64 = 400.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Chess", [WINDOW_SIZE; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let board = Chess::default();
    let mut controller = Controller::new(board);
    let gameboard_view = View::new(ViewSettings::default());

    while let Some(e) = events.next(&mut window) {
        controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&controller, &c, g);
            });
        }
    }
}
