mod board;
mod pieces;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use graphics::grid::Grid;
use piston::event_loop::{EventSettings, Events};
use piston::{RenderEvent, EventLoop, UpdateEvent};
use crate::board::Board;
use pieces::Piece;
use graphics::Image;
use graphics::rectangle::square;
use std::path::Path;
use piston_window::TextureSettings;

fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("checkerboard", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // Create a new game and run it.
    let mut board = Board {
        gl: GlGraphics::new(opengl),
        grid: Grid {
            cols: 8,
            rows: 8,
            units: 50.0,
        },
        pieces: Piece::default_board(),
    };

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            board.render(&args, &mut gl);
        }
        if let Some(args) = e.update_args() {}
    }
}