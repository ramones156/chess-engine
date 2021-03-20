mod board;
mod pieces;

extern crate graphics;
use piston_window::OpenGL;
use crate::board::Board;
use piston_window::{WindowSettings, Events, EventSettings, RenderEvent, PistonWindow};

fn main() {
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("tsjess", [400, 400])
        .graphics_api(OpenGL::V3_2)
        .decorated(true)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut board = Board::new("q7/3n4/4qR2/8/8/8/8/8");
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        board.event(&e);
        if let Some(args) = e.render_args() {
            board.render(&args);
        }
    }
}