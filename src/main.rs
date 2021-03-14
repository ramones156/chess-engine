mod board;
mod pieces;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use graphics::grid::Grid;
use piston::event_loop::{EventSettings, Events};
use piston::{RenderEvent, EventLoop};
use crate::board::Board;
use pieces::Piece;
use crate::pieces::PieceColor;

fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("tsjess", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
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
        pieces: Piece::load_from_fen("8/3n4/4q3/8/8/8/8/8"),
        moving_color: PieceColor::WHITE,
        selected: None,
        released: None,
        selected_piece: Piece::default(),
        cursor_pos: [0.0, 0.0],
    };

    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(e) = events.next(&mut window) {
        board.event(&e);
        if let Some(args) = e.render_args() {
            board.render(&args);
        }
    }
}