use opengl_graphics::{GlGraphics, Texture};
use piston::input::{RenderArgs};
use graphics::grid::Grid;
use crate::pieces::{Piece, PieceType};
use graphics::*;
use graphics::rectangle::square;
use self::piston_window::{G2dTexture, G2dTextureContext, TextureContext, TextureSettings};
use std::path::Path;

extern crate piston_window;

const SIZE: usize = 64;

pub struct Board {
    // OpenGL drawing backend.
    pub gl: GlGraphics,
    pub grid: Grid,
    pub pieces: [Piece; SIZE],
}

impl Board {
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        let white: [f32; 4] = color::hex("F0D9B5");
        let black: [f32; 4] = color::hex("946f51");

        let checker_square = rectangle::square(0.0, 0.0, 50.0);
        let pieces = self.pieces;

        for i in 0..SIZE {
            let file = i % 8;
            let rank = i / 8;
            let x: f64 = (file * 50) as f64;
            let y: f64 = (rank * 50) as f64;
            let color_state = (file + rank) % 2 != 0;
            gl.draw(args.viewport(), |c, gl| {
                if color_state {
                    rectangle(black, checker_square, c.transform.trans(x, y), gl);
                } else {
                    rectangle(white, checker_square, c.transform.trans(x, y), gl);
                };
            });

            let piece = &pieces[i];
            match piece.piece_type {
                PieceType::EMPTY => {}
                _ => {
                    let piece = pieces[i];
                    let image = Image::new().rect(square(x, y, 50.0));
                    let texture = piece.get_icon();

                    gl.draw(args.viewport(), |c, gl| {
                        image.draw(&texture, &c.draw_state, c.transform, gl);
                    });
                    // let image = Image::new().rect(square(x, y, 50.0));
                    // image.draw(&piece.get_icon(), &c.draw_state, c.transform, gl);
                }
            }
        }
    }
}